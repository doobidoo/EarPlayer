# Ear Trainer - Terminal Music Learning Tool

A comprehensive terminal-based music ear training application built in Rust that helps musicians internalize chord progressions by visualizing and playing chord tones, navigating scales/modes over progression changes.

## Features

### Triple Audio Output System
- **MIDI Output**: Send notes to external synthesizers or DAWs
- **Built-in Synthesis**: Standalone audio generation using piano-like synthesis
- **BLE MIDI**: Automatic Bluetooth Low Energy MIDI for wireless controllers (MD-BT01, WIDI, etc.)
- Cycle through modes at runtime with the 'm' key

### LEGO Bricks Jazz Harmony System
Based on Conrad Cork's approach to jazz harmony, breaking standards into reusable patterns:
- **14 Named Bricks**: Launcher, Sad Launcher, Dizzy, Overrun, Pennies, Raindrop, Long Approach, Honeysuckle, Rhythm, Nowhere, Surprise, Starlight, Countdown, So What
- **8 Key Transition Joins**: Sidewinder, High Jump, Cherokee, Giant Steps, Stairway, Ladybird, Moment's Notice, Back Door
- **Circle of Fourths Navigation**: Understanding key relationships
- **Jazz Standards Analysis**: See how Autumn Leaves, All The Things You Are, Blue Bossa, and Rhythm Changes break down into bricks

### Music Theory Engine
- 50+ pre-built progressions across 6 genres (including LEGO Bricks):
  - **Jazz**: ii-V-I variations, Coltrane changes, rhythm changes, turnarounds
  - **Soul**: Motown progressions, neo-soul, gospel-influenced
  - **Funk**: Dorian vamps, modal funk, chromatic progressions
  - **Smooth Jazz**: Extended ii-V-I, lydian sounds, fusion styles
  - **Pop**: I-V-vi-IV, Beatles-style, modal pop
  - **LEGO Bricks**: Named chord patterns in multiple keys

### Jazz Voicings
- **5 Professional Voicing Types**:
  - Full (all chord tones)
  - Shell (3rd and 7th only)
  - Rootless A (Bill Evans style: 3-5-7-9)
  - Rootless B (inverted: 7-9-3-5)
  - Drop 2 (second voice dropped an octave)
- Cycle through voicings with `V` key

### Swing Timing
- Toggle swing feel with `w` key
- Cycle swing ratios with `W`:
  - Straight (0.5)
  - Light swing (0.58)
  - Hard swing (0.67)

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
- **Listen Mode** (`1`): Play progressions and observe chord tone movement
- **Practice Mode** (`2`): Practice along with visual guides
- **Quiz Mode** (`3`): Test your ear for chord tone identification
- **LEGO Listen Mode** (`4`): Browse and learn individual brick patterns
  - Cycle through bricks with `n`/`p`
  - Change key with `k`/`K`
  - Adjust difficulty with `d`
- **LEGO Quiz Mode** (`5`): Identify bricks by ear
  - Multiple choice quiz (1-4 to answer)
  - Score tracking with accuracy and streaks
  - Three difficulty levels: Beginner, Intermediate, Advanced

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
- `4` - LEGO Listen mode
- `5` - LEGO Quiz mode

#### LEGO Mode Controls
- `n`/`p` - Next/Previous brick (Listen mode)
- `k`/`K` - Next/Previous key (Listen mode)
- `d` - Cycle difficulty level
- `1`-`4` - Answer quiz question (Quiz mode)
- `ESC` - Exit LEGO mode

#### Display & Sound Options
- `s` - Toggle scale display
- `v` - Toggle voice leading analysis
- `V` - Cycle voicing type (Full/Shell/RootlessA/RootlessB/Drop2)
- `w` - Toggle swing feel
- `W` - Cycle swing ratio (Straight/Light/Hard)
- `m` - Cycle audio mode (MIDI -> Synth -> BLE MIDI)
- `b` - Force BLE MIDI rescan
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

### 1. MIDI Output
Connects to the first available MIDI port on your system. Use with:
- External hardware synthesizers
- DAWs (Ableton, Logic, FL Studio)
- Virtual MIDI instruments
- MIDI loopback devices

If no MIDI ports are available, the application will notify you and MIDI output will be disabled.

### 2. Synthesis (Built-in)
Uses Rodio for audio playback with piano-like synthesis. Works out of the box without any external MIDI setup.

Features:
- Multi-oscillator sound with harmonics for richer timbre
- ADSR envelope for natural attack and release
- Anti-aliasing for clean high frequencies

**Note:** The synthesis backend outputs to your system's default audio device via PulseAudio/PipeWire/ALSA.

### 3. BLE MIDI (Automatic)

The app includes built-in BLE MIDI support that automatically:
- Scans for BLE MIDI devices on startup
- Connects to devices with MIDI-related names (MD-BT01, WIDI, Yamaha, Roland, Korg, CME)
- Reconnects automatically if connection is lost
- Shows connection status in the header (Green=Connected, Yellow=Scanning, Magenta=Disconnected)

**Supported Devices:**
- Yamaha MD-BT01 (wireless MIDI adapter)
- CME WIDI series
- Any device advertising BLE MIDI service

**Prerequisites:**
```bash
# Arch/Manjaro
sudo pacman -S bluez bluez-utils

# Debian/Ubuntu
sudo apt install bluez

# Ensure Bluetooth service is running
sudo systemctl enable --now bluetooth
```

**Usage:**
1. Power on your BLE MIDI device (e.g., MD-BT01 - red LED should blink)
2. Start ear-trainer - it scans automatically
3. Press `m` to cycle to "BLE MIDI" mode
4. Press `b` to force rescan if needed

**Troubleshooting BLE MIDI:**

If auto-connect fails, the app will guide you through prerequisites. You can also check manually:

```bash
# Check Bluetooth is powered on
bluetoothctl show | grep Powered

# List available adapters (app prefers USB BLE dongles)
bluetoothctl list

# Manual scan for BLE devices
bluetoothctl scan le

# Check if device is already paired
bluetoothctl devices
```

**Multiple Bluetooth Adapters:**
If you have multiple Bluetooth adapters (common on laptops with both internal and USB dongles), the app automatically prefers USB BLE dongles over internal adapters, as internal adapters often have limited BLE support.

#### Legacy Bluetooth MIDI (via system)

For devices that don't work with direct BLE MIDI, you can use system-level Bluetooth MIDI:

**Option A: PipeWire (recommended for modern systems):**
```bash
pw-link -o | grep -i midi
```

**Option B: bluez-alsa:**
```bash
# Arch/Manjaro
yay -S bluez-alsa-git
sudo systemctl enable --now bluealsa
```

**Option C: Manual pairing:**
```bash
bluetoothctl
> scan on
> pair XX:XX:XX:XX:XX:XX
> connect XX:XX:XX:XX:XX:XX
> trust XX:XX:XX:XX:XX:XX
```

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
│   │   ├── synth.rs         - Piano-like synthesis
│   │   ├── ble_midi.rs      - BLE MIDI backend (btleplug)
│   │   └── manager.rs       - Audio backend coordinator
│   ├── music/
│   │   ├── chord.rs         - Chord representation
│   │   ├── scale.rs         - Scale/mode definitions
│   │   ├── progression.rs   - Progression library
│   │   ├── theory.rs        - Voice leading analysis
│   │   ├── bricks.rs        - LEGO Bricks patterns
│   │   ├── joins.rs         - Key transition patterns
│   │   ├── voicings.rs      - Jazz voicing algorithms
│   │   └── standards.rs     - Jazz standard breakdowns
│   ├── ui/
│   │   ├── app.rs           - Application state
│   │   ├── piano_roll.rs    - Piano visualization
│   │   ├── notation.rs      - Chord analysis view
│   │   ├── controls.rs      - Input handling
│   │   └── lego_mode.rs     - LEGO training mode UI
│   ├── storage.rs           - Progress persistence
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

### Learn LEGO Bricks
1. Press `4` to enter LEGO Listen mode
2. Press `n`/`p` to cycle through bricks
3. Press `SPACE` to hear each brick pattern
4. Press `k`/`K` to transpose to different keys
5. Notice how each brick is a reusable harmonic unit

### Test Your Ears with LEGO Quiz
1. Press `5` to enter LEGO Quiz mode
2. Press `SPACE` to hear the mystery brick
3. Press `1`-`4` to select your answer
4. Track your accuracy and build streaks
5. Press `d` to increase difficulty as you improve

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

### Completed Features (v0.4.0)
- [x] Quiz mode for ear training tests (LEGO Quiz)
- [x] Swing feel implementation
- [x] Jazz voicing options (5 types)
- [x] Progress tracking and persistence

### Planned Features
- [ ] Custom progression builder
- [ ] Random progression generator with musical logic
- [ ] Interactive practice mode with user input
- [ ] Fretboard visualization for guitar/bass
- [ ] Standard notation rendering
- [ ] MIDI file export
- [ ] Configuration file support
- [ ] Rhythm visualization
- [ ] Microtonal exploration
- [ ] More jazz standards breakdowns

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
