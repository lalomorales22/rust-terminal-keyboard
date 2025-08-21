use serde::{Deserialize, Serialize};
use std::fs;
use std::path::PathBuf;
use anyhow::Result;

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Config {
    pub audio: AudioConfig,
    pub ui: UiConfig,
    pub midi: MidiConfig,
    pub keybindings: KeyBindings,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct AudioConfig {
    pub sample_rate: u32,
    pub buffer_size: u32,
    pub sound_font: Option<String>,
    pub volume: f32,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct UiConfig {
    pub color_scheme: String,
    pub show_notes: bool,
    pub show_keyboard_hints: bool,
    pub animation_speed: f32,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct MidiConfig {
    pub input_device: String,
    pub output_device: String,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct KeyBindings {
    pub white_keys: Vec<char>,
    pub black_keys: Vec<char>,
    pub octave_up: char,
    pub octave_down: char,
    pub volume_up: char,
    pub volume_down: char,
    pub sustain: char,
    pub record: char,
    pub playback: char,
    pub metronome: char,
    pub load: char,
    pub help: char,
    pub quit: char,
}

impl Default for Config {
    fn default() -> Self {
        Self {
            audio: AudioConfig {
                sample_rate: 44100,
                buffer_size: 256,
                sound_font: None,
                volume: 0.7,
            },
            ui: UiConfig {
                color_scheme: "classic".to_string(),
                show_notes: true,
                show_keyboard_hints: true,
                animation_speed: 1.0,
            },
            midi: MidiConfig {
                input_device: "auto".to_string(),
                output_device: "auto".to_string(),
            },
            keybindings: KeyBindings {
                white_keys: vec![
                    'a', 's', 'd', 'f', 'g', 'h', 'j', 'k', 'l', ';',
                    'z', 'x', 'c', 'v', 'b', 'n', 'm', ',', '.', '/'
                ],
                black_keys: vec![
                    '1', '2', '3', '4', '5', '6', '7', '8', '9', '0', '-', '='
                ],
                octave_up: '+',
                octave_down: '_',
                volume_up: ']',
                volume_down: '[',
                sustain: ' ',
                record: 'R',
                playback: 'P',
                metronome: 'M',
                load: 'L',
                help: 'F', // F1 key, represented as 'F' in config
                quit: 'Q',
            },
        }
    }
}

impl Config {
    pub fn load() -> Result<Self> {
        let config_path = Self::config_path()?;
        
        if !config_path.exists() {
            let config = Self::default();
            config.save()?;
            return Ok(config);
        }
        
        let contents = fs::read_to_string(&config_path)?;
        let config: Config = toml::from_str(&contents)?;
        Ok(config)
    }
    
    pub fn save(&self) -> Result<()> {
        let config_path = Self::config_path()?;
        
        if let Some(parent) = config_path.parent() {
            fs::create_dir_all(parent)?;
        }
        
        let contents = toml::to_string_pretty(self)?;
        fs::write(&config_path, contents)?;
        Ok(())
    }
    
    pub fn config_path() -> Result<PathBuf> {
        let home = dirs::home_dir().ok_or_else(|| anyhow::anyhow!("Could not find home directory"))?;
        Ok(home.join(".terminal-piano").join("config.toml"))
    }
    
    pub fn recordings_dir() -> Result<PathBuf> {
        let home = dirs::home_dir().ok_or_else(|| anyhow::anyhow!("Could not find home directory"))?;
        let dir = home.join(".terminal-piano").join("recordings");
        fs::create_dir_all(&dir)?;
        Ok(dir)
    }
}