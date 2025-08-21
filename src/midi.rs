use anyhow::{anyhow, Result};
use midly::{Smf, MidiMessage, MetaMessage, Timing};
use std::collections::VecDeque;
use std::path::{Path, PathBuf};
use std::time::{Duration, Instant};

use crate::audio::{Recording, RecordingEventType};

#[derive(Debug, Clone)]
pub struct MidiEvent {
    pub delta_time: u32,
    pub absolute_time: u64,
    pub event: MidiMessage,
}

#[derive(Debug)]
pub struct MidiPlayer {
    pub current_file: Option<PathBuf>,
    pub events: VecDeque<MidiEvent>,
    pub is_playing: bool,
    pub start_time: Option<Instant>,
    pub current_position: u64,
    pub tempo: u32,
    pub ticks_per_quarter: u16,
    pub total_ticks: u64,
    pub loop_enabled: bool,
}

impl MidiPlayer {
    pub fn new() -> Self {
        Self {
            current_file: None,
            events: VecDeque::new(),
            is_playing: false,
            start_time: None,
            current_position: 0,
            tempo: 500000, // Default tempo (120 BPM)
            ticks_per_quarter: 480,
            total_ticks: 0,
            loop_enabled: false,
        }
    }
    
    pub fn load_file<P: AsRef<Path>>(&mut self, path: P) -> Result<()> {
        let path = path.as_ref();
        let data = std::fs::read(path)?;
        let smf = Smf::parse(&data)?;
        
        self.current_file = Some(path.to_path_buf());
        self.events.clear();
        self.current_position = 0;
        self.is_playing = false;
        self.start_time = None;
        
        match smf.header.timing {
            Timing::Metrical(tpq) => {
                self.ticks_per_quarter = tpq.as_int();
            }
            Timing::Timecode(fps, tpf) => {
                self.ticks_per_quarter = (fps.as_int() as u16) * (tpf as u16);
            }
        }
        
        let mut absolute_time = 0u64;
        let mut all_events = Vec::new();
        
        for track in smf.tracks {
            absolute_time = 0;
            for event in track {
                absolute_time += event.delta.as_int() as u64;
                
                match event.kind {
                    midly::TrackEventKind::Midi { channel, message } => {
                        all_events.push(MidiEvent {
                            delta_time: event.delta.as_int(),
                            absolute_time,
                            event: message,
                        });
                    }
                    midly::TrackEventKind::Meta(MetaMessage::Tempo(tempo)) => {
                        self.tempo = tempo.as_int();
                    }
                    _ => {}
                }
            }
        }
        
        all_events.sort_by_key(|e| e.absolute_time);
        self.total_ticks = all_events.last().map(|e| e.absolute_time).unwrap_or(0);
        self.events = all_events.into();
        
        Ok(())
    }
    
    pub fn play(&mut self) {
        if !self.events.is_empty() {
            self.is_playing = true;
            self.start_time = Some(Instant::now());
        }
    }
    
    pub fn pause(&mut self) {
        self.is_playing = false;
    }
    
    pub fn stop(&mut self) {
        self.is_playing = false;
        self.start_time = None;
        self.current_position = 0;
        
        if let Some(path) = self.current_file.clone() {
            let _ = self.load_file(path);
        }
    }
    
    pub fn toggle_playback(&mut self) {
        if self.is_playing {
            self.pause();
        } else {
            self.play();
        }
    }
    
    pub fn get_pending_events(&mut self) -> Vec<MidiMessage> {
        if !self.is_playing || self.start_time.is_none() {
            return Vec::new();
        }
        
        let elapsed = self.start_time.unwrap().elapsed();
        let current_tick = self.time_to_ticks(elapsed);
        
        let mut pending_events = Vec::new();
        
        while !self.events.is_empty() {
            if let Some(event) = self.events.front() {
                if event.absolute_time <= current_tick {
                    let event = self.events.pop_front().unwrap();
                    pending_events.push(event.event);
                    self.current_position = event.absolute_time;
                } else {
                    break;
                }
            } else {
                break;
            }
        }
        
        if self.events.is_empty() && self.loop_enabled {
            if let Some(path) = self.current_file.clone() {
                let _ = self.load_file(path);
                self.play();
            }
        } else if self.events.is_empty() {
            self.is_playing = false;
        }
        
        pending_events
    }
    
    pub fn seek_to_position(&mut self, position: f32) {
        let position = position.clamp(0.0, 1.0);
        let target_tick = (self.total_ticks as f32 * position) as u64;
        
        if let Some(path) = self.current_file.clone() {
            let _ = self.load_file(path);
            
            let mut absolute_time = 0u64;
            while !self.events.is_empty() {
                if let Some(event) = self.events.front() {
                    if event.absolute_time > target_tick {
                        break;
                    }
                    if let Some(event) = self.events.pop_front() {
                        absolute_time = event.absolute_time;
                    }
                } else {
                    break;
                }
            }
            
            self.current_position = absolute_time;
            
            if self.is_playing {
                let elapsed_time = self.ticks_to_time(self.current_position);
                self.start_time = Some(Instant::now() - elapsed_time);
            }
        }
    }
    
    pub fn get_progress(&self) -> f32 {
        if self.total_ticks == 0 {
            return 0.0;
        }
        (self.current_position as f32 / self.total_ticks as f32).min(1.0)
    }
    
    pub fn get_time_info(&self) -> (Duration, Duration) {
        let current_time = self.ticks_to_time(self.current_position);
        let total_time = self.ticks_to_time(self.total_ticks);
        (current_time, total_time)
    }
    
    fn time_to_ticks(&self, time: Duration) -> u64 {
        let microseconds = time.as_micros() as f64;
        let ticks_per_microsecond = (self.ticks_per_quarter as f64) / (self.tempo as f64);
        (microseconds * ticks_per_microsecond) as u64
    }
    
    fn ticks_to_time(&self, ticks: u64) -> Duration {
        let microseconds_per_tick = (self.tempo as f64) / (self.ticks_per_quarter as f64);
        let total_microseconds = (ticks as f64) * microseconds_per_tick;
        Duration::from_micros(total_microseconds as u64)
    }
    
    pub fn set_loop(&mut self, enabled: bool) {
        self.loop_enabled = enabled;
    }
    
    pub fn is_loop_enabled(&self) -> bool {
        self.loop_enabled
    }
}

#[derive(Debug)]
pub struct MidiRecorder {
    pub recording: Option<Recording>,
    pub is_recording: bool,
}

impl MidiRecorder {
    pub fn new() -> Self {
        Self {
            recording: None,
            is_recording: false,
        }
    }
    
    pub fn start_recording(&mut self) {
        self.recording = Some(Recording::new());
        self.is_recording = true;
    }
    
    pub fn stop_recording(&mut self) -> Option<Recording> {
        self.is_recording = false;
        if let Some(mut recording) = self.recording.take() {
            recording.finish();
            Some(recording)
        } else {
            None
        }
    }
    
    pub fn record_note_on(&mut self, midi_note: u8, velocity: u8) {
        if self.is_recording {
            if let Some(recording) = &mut self.recording {
                recording.add_event(RecordingEventType::NoteOn { midi_note, velocity });
            }
        }
    }
    
    pub fn record_note_off(&mut self, midi_note: u8) {
        if self.is_recording {
            if let Some(recording) = &mut self.recording {
                recording.add_event(RecordingEventType::NoteOff { midi_note });
            }
        }
    }
    
    pub fn record_sustain_pedal(&mut self, pressed: bool) {
        if self.is_recording {
            if let Some(recording) = &mut self.recording {
                recording.add_event(RecordingEventType::SustainPedal { pressed });
            }
        }
    }
    
    pub fn toggle_recording(&mut self) -> Option<Recording> {
        if self.is_recording {
            self.stop_recording()
        } else {
            self.start_recording();
            None
        }
    }
}

pub fn midi_note_to_frequency(midi_note: u8) -> f32 {
    440.0 * 2.0_f32.powf((midi_note as f32 - 69.0) / 12.0)
}

pub fn frequency_to_midi_note(frequency: f32) -> u8 {
    (69.0 + 12.0 * (frequency / 440.0).log2()).round() as u8
}

pub fn note_name_to_midi_note(note_name: &str, octave: u8) -> Result<u8> {
    let base_note = match note_name.to_uppercase().as_str() {
        "C" => 0,
        "C#" | "DB" => 1,
        "D" => 2,
        "D#" | "EB" => 3,
        "E" => 4,
        "F" => 5,
        "F#" | "GB" => 6,
        "G" => 7,
        "G#" | "AB" => 8,
        "A" => 9,
        "A#" | "BB" => 10,
        "B" => 11,
        _ => return Err(anyhow!("Invalid note name: {}", note_name)),
    };
    
    Ok((octave * 12) + base_note)
}

pub fn midi_note_to_note_name(midi_note: u8) -> (String, u8) {
    let note_names = ["C", "C#", "D", "D#", "E", "F", "F#", "G", "G#", "A", "A#", "B"];
    let octave = midi_note / 12;
    let note_index = (midi_note % 12) as usize;
    (note_names[note_index].to_string(), octave)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_midi_note_conversion() {
        assert_eq!(midi_note_to_frequency(69), 440.0);
        assert_eq!(frequency_to_midi_note(440.0), 69);
        
        let (note, octave) = midi_note_to_note_name(69);
        assert_eq!(note, "A");
        assert_eq!(octave, 5);
        
        assert_eq!(note_name_to_midi_note("A", 5).unwrap(), 69);
        assert_eq!(note_name_to_midi_note("C", 4).unwrap(), 60);
    }
    
    #[test]
    fn test_time_conversion() {
        let mut player = MidiPlayer::new();
        player.tempo = 500000; // 120 BPM
        player.ticks_per_quarter = 480;
        
        let one_second = Duration::from_secs(1);
        let ticks = player.time_to_ticks(one_second);
        let time_back = player.ticks_to_time(ticks);
        
        assert!((time_back.as_secs_f64() - 1.0).abs() < 0.01);
    }
}