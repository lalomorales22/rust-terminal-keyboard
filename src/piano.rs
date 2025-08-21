use std::collections::HashMap;
use std::time::Instant;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum NoteType {
    White,
    Black,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct Note {
    pub midi_note: u8,
    pub octave: u8,
    pub note_name: NoteName,
    pub note_type: NoteType,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum NoteName {
    C, CSharp, D, DSharp, E, F, FSharp, G, GSharp, A, ASharp, B,
}

impl NoteName {
    pub fn from_midi(midi_note: u8) -> Self {
        match midi_note % 12 {
            0 => NoteName::C,
            1 => NoteName::CSharp,
            2 => NoteName::D,
            3 => NoteName::DSharp,
            4 => NoteName::E,
            5 => NoteName::F,
            6 => NoteName::FSharp,
            7 => NoteName::G,
            8 => NoteName::GSharp,
            9 => NoteName::A,
            10 => NoteName::ASharp,
            11 => NoteName::B,
            _ => unreachable!(),
        }
    }
    
    pub fn is_black_key(&self) -> bool {
        matches!(self, NoteName::CSharp | NoteName::DSharp | NoteName::FSharp | NoteName::GSharp | NoteName::ASharp)
    }
    
    pub fn to_string(&self) -> &'static str {
        match self {
            NoteName::C => "C",
            NoteName::CSharp => "C#",
            NoteName::D => "D",
            NoteName::DSharp => "D#",
            NoteName::E => "E",
            NoteName::F => "F",
            NoteName::FSharp => "F#",
            NoteName::G => "G",
            NoteName::GSharp => "G#",
            NoteName::A => "A",
            NoteName::ASharp => "A#",
            NoteName::B => "B",
        }
    }
}

impl Note {
    pub fn new(midi_note: u8) -> Self {
        let note_name = NoteName::from_midi(midi_note);
        let octave = midi_note / 12;
        let note_type = if note_name.is_black_key() {
            NoteType::Black
        } else {
            NoteType::White
        };
        
        Self {
            midi_note,
            octave,
            note_name,
            note_type,
        }
    }
    
    pub fn frequency(&self) -> f32 {
        440.0 * 2.0_f32.powf((self.midi_note as f32 - 69.0) / 12.0)
    }
}

#[derive(Debug)]
pub struct Piano {
    pub current_octave: u8,
    pub pressed_keys: HashMap<u8, Instant>, // Track when keys were pressed
    pub sustain_pedal: bool,
    pub volume: f32,
    pub key_mappings: HashMap<char, u8>,
}

impl Piano {
    pub fn new() -> Self {
        let mut piano = Self {
            current_octave: 4,
            pressed_keys: HashMap::new(),
            sustain_pedal: false,
            volume: 0.7,
            key_mappings: HashMap::new(),
        };
        
        piano.setup_key_mappings();
        piano
    }
    
    fn setup_key_mappings(&mut self) {
        // Use keys that don't conflict with controls: avoid 'q' (quit), '[' and ']' (volume)
        let white_keys = ['a', 's', 'd', 'f', 'g', 'h', 'j', 'k', 'l', ';', 'z', 'x', 'c', 'v', 'b', 'n', 'm', ',', '.', '/'];
        let black_keys = ['1', '2', '3', '4', '5', '6', '7', '8', '9', '0', '-', '='];
        
        let base_octave = self.current_octave * 12;
        let mut white_index = 0;
        let mut black_index = 0;
        
        for midi_note in base_octave..base_octave + 12 {
            let note = Note::new(midi_note);
            
            match note.note_type {
                NoteType::White => {
                    if white_index < white_keys.len() {
                        self.key_mappings.insert(white_keys[white_index], midi_note);
                        white_index += 1;
                    }
                }
                NoteType::Black => {
                    if black_index < black_keys.len() {
                        self.key_mappings.insert(black_keys[black_index], midi_note);
                        black_index += 1;
                    }
                }
            }
        }
    }
    
    pub fn press_key(&mut self, midi_note: u8) {
        self.pressed_keys.insert(midi_note, Instant::now());
    }
    
    pub fn release_key(&mut self, midi_note: u8) {
        if !self.sustain_pedal {
            self.pressed_keys.remove(&midi_note);
        }
    }
    
    pub fn toggle_sustain(&mut self) {
        self.sustain_pedal = !self.sustain_pedal;
        if !self.sustain_pedal {
            self.pressed_keys.clear();
        }
    }
    
    pub fn change_octave(&mut self, delta: i8) {
        let new_octave = (self.current_octave as i8 + delta).clamp(0, 8) as u8;
        if new_octave != self.current_octave {
            self.current_octave = new_octave;
            self.setup_key_mappings();
        }
    }
    
    pub fn adjust_volume(&mut self, delta: f32) {
        self.volume = (self.volume + delta).clamp(0.0, 1.0);
    }
    
    pub fn get_midi_note_from_key(&self, key: char) -> Option<u8> {
        self.key_mappings.get(&key).copied()
    }
    
    pub fn get_octave_range(&self) -> (u8, u8) {
        let start = self.current_octave * 12;
        (start, start + 12)
    }
    
    pub fn update(&mut self) {
        // Auto-release keys after 300ms if not using sustain pedal
        if !self.sustain_pedal {
            let now = Instant::now();
            self.pressed_keys.retain(|_, &mut press_time| {
                now.duration_since(press_time).as_millis() < 300
            });
        }
    }
    
    pub fn get_key_layout(&self) -> Vec<(char, Note, bool)> {
        let mut layout = Vec::new();
        
        for (&key, &midi_note) in &self.key_mappings {
            let note = Note::new(midi_note);
            let is_pressed = self.pressed_keys.contains_key(&midi_note);
            layout.push((key, note, is_pressed));
        }
        
        layout.sort_by_key(|(_, note, _)| note.midi_note);
        layout
    }
}

#[derive(Debug, Clone)]
pub struct PianoLayout {
    pub white_keys: Vec<WhiteKey>,
    pub black_keys: Vec<BlackKey>,
    pub width: u16,
    pub height: u16,
}

#[derive(Debug, Clone)]
pub struct WhiteKey {
    pub note: Note,
    pub x: u16,
    pub width: u16,
    pub is_pressed: bool,
    pub key_char: Option<char>,
}

#[derive(Debug, Clone)]
pub struct BlackKey {
    pub note: Note,
    pub x: u16,
    pub width: u16,
    pub is_pressed: bool,
    pub key_char: Option<char>,
}

impl PianoLayout {
    pub fn new(piano: &Piano, terminal_width: u16) -> Self {
        // Show 2 octaves but use much more of the terminal width for wider keys
        let octave_count = 2;
        let white_keys_per_octave = 7;
        let total_white_keys = octave_count * white_keys_per_octave;
        
        // Use 95% of terminal width and ensure minimum key width
        let usable_width = (terminal_width as f32 * 0.95) as u16;
        let key_width = std::cmp::max(6, usable_width / total_white_keys); // Minimum 6 chars wide
        let black_key_width = std::cmp::max(4, (key_width as f32 * 0.7) as u16); // Minimum 4 chars wide
        
        let mut white_keys = Vec::new();
        let mut black_keys = Vec::new();
        
        let (start_note, end_note) = piano.get_octave_range();
        let key_mappings = &piano.key_mappings;
        let char_to_key: HashMap<u8, char> = key_mappings.iter().map(|(&k, &v)| (v, k)).collect();
        
        // Calculate actual piano width and center it
        let actual_piano_width = total_white_keys * key_width;
        let offset_x = (terminal_width.saturating_sub(actual_piano_width)) / 2;
        
        let mut white_key_index = 0;
        
        for midi_note in start_note..end_note.min(start_note + 24) {
            let note = Note::new(midi_note);
            let is_pressed = piano.pressed_keys.contains_key(&midi_note);
            let key_char = char_to_key.get(&midi_note).copied();
            
            match note.note_type {
                NoteType::White => {
                    white_keys.push(WhiteKey {
                        note,
                        x: offset_x + (white_key_index * key_width),
                        width: key_width,
                        is_pressed,
                        key_char,
                    });
                    white_key_index += 1;
                }
                NoteType::Black => {
                    let prev_white_x = if white_key_index > 0 {
                        offset_x + ((white_key_index - 1) * key_width)
                    } else {
                        offset_x
                    };
                    
                    black_keys.push(BlackKey {
                        note,
                        x: prev_white_x + key_width - black_key_width / 2,
                        width: black_key_width,
                        is_pressed,
                        key_char,
                    });
                }
            }
        }
        
        Self {
            white_keys,
            black_keys,
            width: terminal_width,
            height: 12, // Taller piano for better presence
        }
    }
}