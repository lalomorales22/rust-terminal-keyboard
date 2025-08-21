# 🎹 Terminal Piano
<img width="326" height="477" alt="Screenshot 2025-08-21 at 10 38 35 AM" src="https://github.com/user-attachments/assets/d26ccf08-f153-4098-9580-d75db9b1e46f" />
<img width="577" height="521" alt="Screenshot 2025-08-21 at 10 37 59 AM" src="https://github.com/user-attachments/assets/aa7368fc-2c71-4439-98f0-f611716586c8" />


A beautiful, fully-featured grand piano in your terminal with AI-powered learning capabilities.

## Features

- **Realistic Grand Piano**: 88-key piano with authentic sound samples
- **Terminal UI**: Beautiful ASCII art representation of a grand piano
- **Keyboard Mapping**: Play using your QWERTY keyboard
- **Recording**: Record and playback your performances
- **AI Learning**: Submit sheet music and watch notes rain down on the keys
- **Portable**: Single binary, works anywhere

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

### Keyboard Controls

#### Playing
- `A-Z, 0-9`: Piano keys (mapped to current octave)
- `←/→`: Navigate octaves
- `↑/↓`: Adjust volume
- `Space`: Sustain pedal

#### Features
- `R`: Start/stop recording
- `P`: Playback last recording
- `M`: Toggle metronome
- `L`: Load sheet music
- `H`: Show help
- `Q`: Quit

### Piano Layout
```
 ┌─┬─┬─┬─┬─┬─┬─┬─┬─┬─┬─┬─┬─┬─┬─┬─┬─┬─┬─┬─┬─┬─┬─┬─┬─┬─┬─┬─┬─┬─┐
 │ │♯│ │♯│ │ │♯│ │♯│ │♯│ │ │♯│ │♯│ │ │♯│ │♯│ │♯│ │ │♯│ │♯│ │ │
 │ └┬┘ └┬┘ │ └┬┘ └┬┘ └┬┘ │ └┬┘ └┬┘ │ └┬┘ └┬┘ └┬┘ │ └┬┘ └┬┘ │ │
 │ C│ D│ E│ F│ G│ A│ B│ C│ D│ E│ F│ G│ A│ B│ C│ D│ E│ F│ G│ A│ B│
 └──┴──┴──┴──┴──┴──┴──┴──┴──┴──┴──┴──┴──┴──┴──┴──┴──┴──┴──┴──┴──┘
  Q  W  E  R  T  Y  U  I  O  P  [  ]  A  S  D  F  G  H  J  K  L
```

## Learning Mode

Drop any sheet music file (PDF, MusicXML, MIDI) onto the app or use:

```bash
terminal-piano --learn song.pdf
```

Watch as notes rain down on the keys, showing you exactly what to play and when!

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

## Requirements

- Terminal with UTF-8 support
- 256-color terminal recommended
- Audio output device
- 50MB free disk space

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

## Acknowledgments

- Sound samples from [source]
- Inspired by [cmus](https://cmus.github.io/) and [piano-rs](https://github.com/ritiek/piano-rs)

---

Made with ♪ by Lalo and contributors
