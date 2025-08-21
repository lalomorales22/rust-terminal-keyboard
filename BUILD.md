# Building Terminal Piano

## Prerequisites

- Rust 1.70 or later
- Cargo (comes with Rust)
- Audio output device
- Terminal with UTF-8 support

## Build Instructions

1. Clone or navigate to the project directory:
```bash
cd keyboard-terminal
```

2. Build the project:
```bash
cargo build --release
```

3. Run the application:
```bash
./target/release/terminal-piano
```

Or for development:
```bash
cargo run
```

## Features Implemented

✅ **Core Piano Functionality**
- Piano keyboard visual representation (2 octaves, 95% terminal width, centered)
- Wide keys with minimum 6-character width for better visibility
- Taller piano (15+ lines) for prominent display
- Keyboard mapping (optimized layout avoiding conflicts)
- Real-time audio synthesis
- Volume control
- Octave navigation
- Sustain pedal simulation

✅ **MIDI Support**
- MIDI file loading and parsing
- MIDI playback with timing
- MIDI recording to JSON format

✅ **Visual Effects**
- Colorful key press effects
- Particle effects when keys are pressed
- Real-time visual feedback

✅ **User Interface**
- Beautiful terminal UI with Ratatui
- Help system
- Status messages
- Real-time display updates

✅ **Controls**
- `ASDFGHJKL;...` - White piano keys (20 keys total, including H!)
- `1234567890-=` - Black piano keys (12 keys total)  
- `[` / `]` - Volume down/up
- `+` / `_` - Octave up/down (+ key and Shift+- key)
- `Space` - Sustain pedal
- `R` - Record/Stop recording
- `P` - Playback last recording
- `L` - Load MIDI file (with OS dialog)
- `F1` - Toggle help (fixed conflict!)
- `Q` - Quit

## File Structure

- `src/main.rs` - Entry point and CLI handling
- `src/app.rs` - Main application loop and event handling
- `src/piano.rs` - Piano model and key mapping
- `src/audio.rs` - Audio engine and synthesis
- `src/midi.rs` - MIDI file parsing and playback
- `src/ui.rs` - Terminal user interface
- `src/effects.rs` - Visual effects system
- `src/config.rs` - Configuration management
- `src/file_dialog.rs` - Cross-platform file dialogs

## Configuration

The application creates a config file at `~/.terminal-piano/config.toml` with settings for audio, UI, and key bindings. Recordings are saved to `~/.terminal-piano/recordings/`.

## Known Limitations

- Audio synthesis uses simple sine waves (no realistic piano samples)
- TachyonFX integration was simplified due to API changes
- Some advanced MIDI features not yet implemented
- File dialog may not work on all platforms

## Building for Production

For a smaller, optimized binary:
```bash
cargo build --release
strip target/release/terminal-piano  # Optional: remove debug symbols
```

The resulting binary will be approximately 8-15MB and requires no external dependencies.