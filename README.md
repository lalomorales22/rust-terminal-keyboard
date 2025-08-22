# 🎹 Terminal Piano

<img width="752" height="424" alt="Screenshot 2025-08-22 at 12 37 49 AM" src="https://github.com/user-attachments/assets/981b6ccd-329e-45e2-b691-0e69ffc7f7d9" />

<img width="771" height="462" alt="Screenshot 2025-08-22 at 12 38 45 AM" src="https://github.com/user-attachments/assets/f6efe506-329c-4500-a496-427cc35a90d6" />

A spectacular, fully-featured grand piano in your terminal with colorful note visualization, MIDI playback, and recording capabilities. Experience music in living color with note-specific visual effects and full-width responsive piano layout!

## ✨ Features

### 🎵 Audio & Performance
- **Realistic Grand Piano**: Multi-octave piano with authentic sound samples
- **MIDI File Playback**: Load and play .mid files with full visual synchronization
- **Audio Recording**: Record and playback your performances
- **Volume Control**: Real-time volume adjustment with visual feedback
- **Sustain Pedal**: Hold notes for expressive playing

### 🌈 Visual Effects
- **Colorful Note Visualization**: Each musical note (C, C#, D, etc.) has its own distinctive color
- **Piano Key Lighting**: Keys light up in note-specific colors when played
- **Musical Particle Effects**: Colorful musical symbols (♪♫♬♭) rain down from played keys
- **Velocity-Sensitive Effects**: Louder MIDI notes create more spectacular visual bursts
- **Full-Width Piano Layout**: Responsive piano that adapts to your terminal width

### 🎹 Piano Interface
- **Full-Screen Piano**: Dynamic layout that spans your entire terminal width
- **Multi-Octave Display**: Shows 3-7 octaves depending on terminal width
- **Real-time Key Mapping**: Visual display of keyboard-to-piano key assignments
- **Note Names**: Display of musical note names on keys
- **Octave Navigation**: Easy octave switching with visual indicators

### 🎛️ Controls & Features
- **Keyboard Mapping**: Play using your QWERTY keyboard with intelligent key mapping
- **MIDI Progress**: Visual progress bar during MIDI playback
- **Help System**: Built-in help with F1 key
- **Status Messages**: Real-time feedback for all actions
- **Debug Mode**: Advanced debugging capabilities

## Installation

### Homebrew (coming soon)
```bash
brew tap lalo/tools
brew install terminal-piano
```

### From Source
```bash
git clone https://github.com/yourusername/terminal-piano
cd terminal-piano
cargo build --release  # or make build
./target/release/terminal-piano
```

## Usage

```bash
terminal-piano
```

### 🎯 Keyboard Controls

#### 🎹 Piano Playing
- **First Octave**: `Q W E R T Y U I O P` (white keys) + `2 3 5 6 7 9 0` (black keys)
- **Second Octave**: `A S D F G H J K L ;` (white keys) + `1 4 8 - =` (black keys)
- **Lower Notes**: `Z X C V B N M` (additional lower notes)
- **Sustain Pedal**: `Space` - Hold notes longer
- **Volume Control**: `[ ]` - Decrease/increase volume
- **Octave Control**: `+ _` - Change octave up/down

#### 🎵 MIDI & Recording
- **Load MIDI File**: `L` - Load and select .mid files
- **MIDI Playback**: `Shift + P` (Capital P) - Play/pause MIDI files
- **Recording**: `R` - Start/stop recording your performance
- **Playback Recording**: `p` (lowercase p) - Play your last recording

#### 🎛️ Interface & Settings
- **Help**: `F1` - Show/hide help screen
- **Metronome**: `M` - Toggle metronome on/off
- **Quit**: `Q` or `q` - Exit the application

#### 🌈 Color Mapping
Each note has its own distinctive color:
- **C**: 🔴 Bright Red
- **C#**: 🟠 Warm Orange  
- **D**: 🟡 Golden Yellow
- **D#**: 🟢 Green Yellow
- **E**: 🟢 Lime Green
- **F**: 🟢 Teal Green
- **F#**: 🔵 Sky Blue
- **G**: 🔵 Dodger Blue
- **G#**: 🟣 Blue Violet
- **A**: 🩷 Deep Pink
- **A#**: 🩷 Hot Pink
- **B**: 🟠 Orange

### 🎹 Full-Width Piano Layout
The piano automatically adapts to your terminal width, showing 3-7 octaves:

```
Terminal Width: Narrow (80 cols)          Wide (120+ cols)
┌─┬─┬─┬─┬─┬─┬─┬─┬─┬─┬─┬─┬─┬─┐              ┌─┬─┬─┬─┬─┬─┬─┬─┬─┬─┬─┬─┬─┬─┬─┬─┬─┬─┬─┬─┬─┬─┬─┬─┬─┬─┬─┬─┬─┬─┐
│ │♯│ │♯│ │ │♯│ │♯│ │♯│ │      │ │♯│ │♯│ │ │♯│ │♯│ │♯│ │ │♯│ │♯│ │ │♯│ │♯│ │♯│ │
│ └┬┘ └┬┘ │ └┬┘ └┬┘ └┬┘ │      │ └┬┘ └┬┘ │ └┬┘ └┬┘ └┬┘ │ └┬┘ └┬┘ │ └┬┘ └┬┘ └┬┘ │
│ C│ D│ E│ F│ G│ A│ B│         │ C│ D│ E│ F│ G│ A│ B│ C│ D│ E│ F│ G│ A│ B│ C│ D│ E│
└──┴──┴──┴──┴──┴──┴──┴──┘       └──┴──┴──┴──┴──┴──┴──┴──┴──┴──┴──┴──┴──┴──┴──┴──┴──┘
 Q  W  E  R  T  Y  U             Q  W  E  R  T  Y  U  I  O  P  A  S  D  F  G  H  J
                                          2  3  5  6  7  9  0
```

**Key Features:**
- **Responsive Design**: Piano scales with terminal width
- **Color-Coded Keys**: Each key lights up in its note-specific color
- **Visual Feedback**: Pressed keys show bright color highlights
- **Note Labels**: Musical note names displayed on each key

## 🎼 MIDI Playback Experience

Load MIDI files for a spectacular visual and audio experience:

```bash
# Load MIDI file on startup
terminal-piano song.mid

# Or load within the app
# Press 'L' and select your .mid file
```

### ✨ Visual Effects During Playback
- **Colorful Key Lighting**: Piano keys light up in note-specific colors as the music plays
- **Musical Particle Rain**: Colorful musical symbols (♪♫♬♭) cascade from active keys
- **Velocity-Sensitive Effects**: Louder passages create more spectacular particle bursts
- **Progress Visualization**: See playback progress with a visual timeline
- **Harmonic Colors**: Watch chords and melodies unfold in beautiful color combinations

### 🎵 Supported MIDI Features
- **Full MIDI Event Support**: Note on/off, velocity, timing
- **Polyphonic Playback**: Multiple notes and complex harmonies
- **Tempo Accurate**: Precise timing reproduction
- **Visual Synchronization**: Perfect sync between audio and visual effects

### 🎼 Demo MIDI Files
The repository includes a collection of beautiful MIDI files in the `midi-demos/` folder:

```bash
# Play a demo file directly
terminal-piano midi-demos/debussy-clair-de-lune.mid
terminal-piano midi-demos/bach-invention-1.mid
terminal-piano midi-demos/chopin-minute-waltz.mid

# Or load them within the app using 'L' key
```

**Included Demos:**
- 🌙 **Debussy - Clair de Lune**: Perfect for experiencing the colorful visual effects
- 🎼 **Bach - Invention No. 1**: Great for seeing counterpoint in different colors  
- 🎹 **Chopin - Minute Waltz**: Fast passages showcase velocity-sensitive particle effects
- 🎵 **Mozart - Turkish March**: Classical melody with beautiful harmonic colors
- 🎶 **Beethoven - Für Elise**: Iconic piece demonstrating the full color palette

These demo files are perfect for experiencing the full visual spectacle of Terminal Piano!

## Recording

Recordings are saved in `~/.terminal-piano/recordings/` as both MIDI and WAV files.

## Configuration

Config file location: `~/.terminal-piano/config.toml`

```toml
[audio]
sample_rate = 44100
buffer_size = 256
sound_font = "default"  # or path to .sf2 file

[ui]
color_scheme = "classic"  # classic, neon, minimal
show_notes = true
show_keyboard_hints = true

[midi]
input_device = "auto"  # or specific device name
output_device = "auto"
```

## 💻 System Requirements

### Terminal Requirements
- **UTF-8 Support**: Modern terminal with Unicode character support
- **True Color Support**: 24-bit color terminal for full visual effects
  - Works: VS Code terminal, iTerm2, Alacritty, Kitty, Windows Terminal
  - Limited: Standard Terminal.app, older terminals (fallback to basic colors)
- **Audio Device**: Working audio output for sound playback
- **Minimum Width**: 80 columns recommended (adapts to any width)

### Color Support Troubleshooting
If colors don't appear correctly, try:
```bash
# Enable true color support
export COLORTERM=truecolor
./target/release/terminal-piano

# Or set terminal type
export TERM=xterm-256color
./target/release/terminal-piano
```

### Disk Space
- **Binary**: ~10MB
- **Audio Samples**: Generated on-demand
- **Recordings**: Variable (depends on usage)

## Building from Source

### Prerequisites
- Rust 1.70+ (if using Rust implementation)
- or Go 1.21+ (if using Go implementation)

### Build
```bash
make build
make test
make install
```

## Contributing

We welcome contributions! Please see [CONTRIBUTING.md](CONTRIBUTING.md) for guidelines.

## License

MIT License - see [LICENSE](LICENSE) for details.

## 🚀 Recent Updates & Fixes

### v1.5.0 - Colorful Musical Experience
- ✅ **Fixed MIDI Playback**: Resolved capital P key detection for MIDI play/pause
- 🌈 **Colorful Note Visualization**: Each note now has its own distinctive color
- 🎹 **Full-Width Piano Layout**: Piano now spans entire terminal width with adaptive octave display
- ✨ **Enhanced Visual Effects**: Velocity-sensitive particle effects with musical symbols
- 🎵 **Improved Audio Engine**: Better note triggering and audio device handling
- 🔧 **Key Mapping Overhaul**: Complete keyboard mapping system with multi-octave support
- 🎨 **Responsive Design**: Piano automatically adjusts to terminal width (3-7 octaves)

### Performance Improvements
- Fixed timing calculations for accurate MIDI playback
- Optimized particle rendering for smooth visual effects
- Enhanced audio initialization with better error handling
- Improved key press detection and response

## 🎨 Color Palette

The app uses a carefully crafted color palette for each musical note:
- **Warm Colors**: C, C#, D, B (reds, oranges, yellows)
- **Cool Colors**: F#, G, G# (blues, purples) 
- **Nature Colors**: D#, E, F (greens, teals)
- **Accent Colors**: A, A# (pinks)

## 🤝 Contributing

We welcome contributions! Areas where help is appreciated:
- Additional audio sample formats
- More visual effect patterns  
- Terminal compatibility improvements
- Performance optimizations
- MIDI feature enhancements

## 📄 License

MIT License - see [LICENSE](LICENSE) for details.

## 🙏 Acknowledgments

- Inspired by [piano-rs](https://github.com/ritiek/piano-rs) and terminal music applications
- Built with Rust, Ratatui, and Rodio for audio
- Musical symbols and Unicode support
- Community feedback and testing

---

Made with ♪ and 🌈 by the Terminal Piano team
