# Terminal Piano - Project Status

## 🎯 Project Goal
Build a terminal-based grand piano application with learning features for Homebrew distribution.

## ✅ Completed
- [x] Project initialization
- [x] Created STATUS.md and README.md documentation

## 🚧 In Progress
- [ ] Evaluating audio libraries (Rust/rodio vs Go/beep vs Python/pygame)

## 📋 Planned Features

### Core Piano Functionality
- [ ] Grand piano visual representation in terminal
- [ ] 88-key support with octave navigation (← →)
- [ ] QWERTY keyboard mapping to piano keys
- [ ] High-quality piano sound samples
- [ ] Sustain pedal simulation (spacebar)

### Recording & Playback
- [ ] Record performances to MIDI/WAV
- [ ] Playback recorded sessions
- [ ] Export recordings
- [ ] Metronome with adjustable BPM

### Learning Features
- [ ] Sheet music parser (MusicXML, MIDI input)
- [ ] AI integration for sheet music recognition
- [ ] "Raining notes" visualization for learning
- [ ] Practice mode with tempo adjustment
- [ ] Chord detection and display

### Technical Requirements
- [ ] Cross-platform (macOS, Linux, Windows)
- [ ] Single binary distribution
- [ ] Homebrew formula
- [ ] Minimal dependencies
- [ ] <50MB installed size

## 🔄 Version History

### v0.0.1 (Planning Phase)
- Project structure created
- Requirements gathering
- Technology selection in progress

## 📝 Notes
- Considering Rust for performance and single-binary compilation
- Need to evaluate terminal UI libraries (ratatui for Rust, termui for Go)
- Audio latency must be <20ms for playable experience