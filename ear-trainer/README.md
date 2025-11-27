# Ear Trainer - Terminal Music Learning Tool

A comprehensive terminal-based music ear training application built in Rust that helps musicians internalize chord progressions by visualizing and playing chord tones, navigating scales/modes over progression changes.

## Features

### Dual Audio Output System
- **MIDI Output**: Send notes to external synthesizers or DAWs
- **Built-in Synthesis**: Standalone audio generation using sine wave synthesis
- Switchable at runtime with the 'm' key

### Music Theory Engine
- 40+ pre-built progressions across 5 genres:
  - **Jazz**: ii-V-I variations, Coltrane changes, rhythm changes, turnarounds
  - **Soul**: Motown progressions, neo-soul, gospel-influenced
  - **Funk**: Dorian vamps, modal funk, chromatic progressions
  - **Smooth Jazz**: Extended ii-V-I, lydian sounds, fusion styles
  - **Pop**: I-V-vi-IV, Beatles-style, modal pop

### Intelligent Chord-Scale Matching
- Automatic scale/mode selection for each chord type:
  - Major 7th → Ionian or Lydian
  - Minor 7th → Dorian, Aeolian, or Phrygian
  - Dominant 7th → Mixolydian, altered, diminished
  - Half-diminished → Locrian
  - And more...

### Visualizations
1. **Piano Roll View**: Visual keyboard with color-coded notes
   - Green dots: Chord tones
   - Yellow dots: Guide tones (3rd and 7th)
   - Blue dots: Available scale notes
   - Gray X: Avoid notes

2. **Chord Analysis Panel**:
   - Current chord and scale/mode
   - Chord tones and guide tones
   - Available extensions (9th, 11th, 13th)
   - Voice leading analysis showing guide tone movement

3. **Progression Display**:
   - Current progression with all chord changes
   - Beat indicators
   - Tempo and key information

### Learning Modes
- **Listen Mode**: Play progressions and observe chord tone movement
- **Practice Mode**: Practice along with visual guides (planned)
- **Quiz Mode**: Test your ear for chord tone identification (planned)

## Installation

### Prerequisites
- Rust 1.70 or later
- ALSA development libraries (Linux)
- MIDI output device (optional, for MIDI mode)

### Build from Source

```bash
cd ear-trainer
cargo build --release
```

## Usage

### Quick Start

```bash
cd ear-trainer
cargo run --release
```

### Keybindings

#### Playback Controls
- `SPACE` - Play/Pause progression
- `n` - Next progression
- `p` - Previous progression
- `+/-` - Increase/Decrease tempo

#### Navigation
- `g` - Next genre
- `G` - Previous genre (Shift+g)
- `1` - Listen mode
- `2` - Practice mode
- `3` - Quiz mode

#### Display Options
- `s` - Toggle scale display
- `v` - Toggle voice leading analysis
- `m` - Toggle between MIDI and Synthesis audio
- `h` - Toggle help screen

#### Other
- `q` - Quit application

## Understanding the Display

### Piano Roll Legend
- **● (Green)** - Chord tones (root, 3rd, 5th, 7th)
- **● (Yellow)** - Guide tones (3rd and 7th - important for voice leading)
- **R (Red)** - Root note
- **· (Blue)** - Available scale notes (extensions)
- **× (Gray)** - Avoid notes

### Voice Leading
When enabled, the chord analysis panel shows how guide tones move between chords:
- **↑** - Ascending motion
- **↓** - Descending motion
- **→** - Static (same note)
- Green arrows indicate smooth voice leading (≤2 semitones)
- Yellow arrows indicate larger leaps

## Musical Concepts

### Guide Tones
The 3rd and 7th of each chord define its quality and create smooth voice leading when they resolve by step or stay static between chords.

Example in a ii-V-I progression (Dm7 → G7 → Cmaj7):
- Dm7: F (3rd), C (7th)
- G7: B (3rd), F (7th)
- Cmaj7: E (3rd), B (7th)

Notice how F→B and C→E in half-step motions create strong voice leading.

### Chord-Scale Relationships
Each chord type suggests specific scales/modes:
- **Cmaj7** - C Ionian (major scale) or C Lydian
- **Dm7** - D Dorian (in major key context)
- **G7** - G Mixolydian, G Altered, or G Diminished
- **Bm7b5** - B Locrian

### Available Extensions
The application shows which tensions are available for each chord:
- **9th** - Major 2nd above the root
- **11th** - Perfect 4th above the root
- **13th** - Major 6th above the root
- Altered tensions (b9, #9, #11, b13) for dominant chords

## Progression Library

### Jazz (10 progressions)
- ii-V-I in various keys
- iii-VI-ii-V
- Rhythm Changes
- Coltrane Changes
- Jazz Blues
- And more...

### Soul (10 progressions)
- Classic soul vamps
- Stevie Wonder style
- Motown progressions
- Neo-soul harmonies
- Gospel influenced

### Funk (10 progressions)
- Dorian vamps
- James Brown style
- P-Funk
- Modal funk
- Chromatic funk

### Smooth Jazz (10 progressions)
- Extended ii-V-I
- Lydian sounds
- Bossa nova influenced
- Contemporary ballads
- Fusion styles

### Pop (10 progressions)
- I-V-vi-IV (classic 4-chord)
- 50s progressions
- Beatles-style
- Modal pop
- And more...

## Audio Backends

### MIDI Output
Connects to the first available MIDI port on your system. Use with:
- External hardware synthesizers
- DAWs (Ableton, Logic, FL Studio)
- Virtual MIDI instruments
- MIDI loopback devices

If no MIDI ports are available, the application will notify you and MIDI output will be disabled.

#### Bluetooth MIDI Setup (Linux)

To use Bluetooth MIDI devices, you need to install and configure BLE MIDI support:

**1. Install required packages:**
```bash
# Arch/Manjaro
sudo pacman -S bluez bluez-utils

# Debian/Ubuntu
sudo apt install bluez bluez-tools
```

**2. Enable Bluetooth service:**
```bash
sudo systemctl enable bluetooth
sudo systemctl start bluetooth
```

**3. Pair your Bluetooth MIDI device:**
```bash
bluetoothctl
# Inside bluetoothctl:
power on
agent on
scan on
# Wait for your device to appear, note the MAC address
pair XX:XX:XX:XX:XX:XX
connect XX:XX:XX:XX:XX:XX
trust XX:XX:XX:XX:XX:XX
exit
```

**4. For BLE MIDI (most modern MIDI controllers), use one of these methods:**

**Option A: PipeWire (recommended for modern systems):**
PipeWire with WirePlumber handles BLE MIDI automatically. Check if it's already working:
```bash
pw-link -o | grep -i midi
```

**Option B: bluez-alsa (for ALSA-based setups):**
```bash
# Arch/Manjaro
yay -S bluez-alsa-git

# Start the service
sudo systemctl enable bluealsa
sudo systemctl start bluealsa
```

**Option C: ble2midi (standalone BLE MIDI bridge):**
```bash
# Install from AUR or build from source
yay -S ble2midi-git
# Or use: https://github.com/oxesoft/ble2midi
```

**5. Verify MIDI ports:**
```bash
aconnect -l
# Your BT MIDI device should appear as a client
```

### Synthesis (Built-in)
Uses Rodio for audio playback with simple sine wave synthesis. Works out of the box without any external MIDI setup.

**Note:** The synthesis backend outputs to your system's default audio device via PulseAudio/PipeWire/ALSA.

## Technical Details

### Architecture
- **Music Engine**: Pure Rust music theory implementation
- **Audio Backends**: Trait-based design allows easy extension
- **UI**: Built with Ratatui for rich terminal interfaces
- **Performance**: Optimized for real-time audio and smooth UI updates

### File Structure
```
ear-trainer/
├── src/
│   ├── main.rs              - Entry point and UI orchestration
│   ├── audio/
│   │   ├── backend.rs       - AudioBackend trait
│   │   ├── midi.rs          - MIDI output implementation
│   │   └── synth.rs         - Synthesis implementation
│   ├── music/
│   │   ├── chord.rs         - Chord representation
│   │   ├── scale.rs         - Scale/mode definitions
│   │   ├── progression.rs   - Progression library
│   │   └── theory.rs        - Voice leading analysis
│   ├── ui/
│   │   ├── app.rs           - Application state
│   │   ├── piano_roll.rs    - Piano visualization
│   │   ├── notation.rs      - Chord analysis view
│   │   └── controls.rs      - Input handling
│   └── config.rs            - Configuration
└── Cargo.toml
```

## Examples

### Practice a ii-V-I Progression
1. Launch the application
2. Press `g` to navigate to "Jazz" genre (if not already selected)
3. Press `n` or `p` to find "ii-V-I (C)"
4. Press `SPACE` to start playback
5. Observe the piano roll highlighting chord tones
6. Watch the voice leading panel show guide tone movement

### Learn Smooth Jazz Harmony
1. Press `g` until you reach "Smooth Jazz"
2. Explore extended chords with 9ths, 11ths, and 13ths
3. Notice the Lydian and altered scales used
4. Study the smooth voice leading between complex chords

### Compare MIDI and Synthesis
1. Connect a MIDI device or software instrument
2. Press `m` to switch to MIDI mode
3. Press `SPACE` to play
4. Press `m` again to switch to built-in synthesis
5. Compare the sound and latency

## Troubleshooting

### No MIDI Output
- Ensure you have a MIDI output device connected
- On Linux, check ALSA MIDI devices: `aconnect -o`
- Try a virtual MIDI loopback if no hardware is available

### No Audio from Synthesis
- Check your system's default audio output
- Ensure Rodio can access your audio device
- On Linux, verify ALSA or PulseAudio is running

### Application Won't Build
- Ensure you have Rust 1.70+: `rustc --version`
- Install ALSA development libraries:
  - Debian/Ubuntu: `sudo apt-get install libasound2-dev`
  - Fedora: `sudo dnf install alsa-lib-devel`
  - Arch: `sudo pacman -S alsa-lib`

## Future Enhancements

### Planned Features
- [ ] Custom progression builder
- [ ] Random progression generator with musical logic
- [ ] Interactive practice mode with user input
- [ ] Quiz mode for ear training tests
- [ ] Fretboard visualization for guitar/bass
- [ ] Standard notation rendering
- [ ] MIDI file export
- [ ] Configuration file support
- [ ] Multiple voice leading options
- [ ] Rhythm visualization
- [ ] Swing feel implementation
- [ ] Microtonal exploration

## Contributing

This is a learning tool for musicians. Contributions are welcome, especially:
- Additional progressions in existing genres
- New genre categories
- Alternative chord-scale relationships
- UI improvements
- Audio backend enhancements

## License

This project is provided as-is for educational and practice purposes.

## Acknowledgments

Built for musicians who want to deeply understand harmony, voice leading, and improvisation through interactive practice.

---

**Happy practicing!** 🎵
