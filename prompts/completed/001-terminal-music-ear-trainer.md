<objective>
Design and implement a comprehensive terminal-based music ear training application in Rust that helps musicians internalize chord progressions by visualizing and playing chord tones and navigating scales/modes over progression changes.

This tool is for musicians (especially jazz, soul, funk, and smooth jazz players) who want to develop their ear and understand voice leading, guide tones, and harmonic movement through interactive practice. The end goal is a daily-use tool that makes complex music theory concepts tangible and audible.
</objective>

<context>
Musicians learning to improvise need to:
1. Hear and identify chord tones (root, 3rd, 5th, 7th, extensions) within progressions
2. Understand which scale/mode fits each chord and how to navigate between them
3. Internalize common progressions (ii-V-I, turnarounds, substitutions)
4. Visualize voice leading and guide tone movement

This app should make these concepts interactive and immediate. Use Rust for performance and rich terminal UI capabilities.
</context>

<requirements>

**Core Functionality:**

1. **Dual Audio Output System**
   - MIDI output capability (send to external synths/DAWs via MIDI ports)
   - Built-in synthesis (generate audio internally for standalone use)
   - User-selectable audio mode at runtime
   - Clean separation between audio backends

2. **Chord Progression Engine**
   - Pre-built library of common progressions organized by genre:
     * Jazz: ii-V-I, iii-VI-ii-V, Coltrane changes, rhythm changes
     * Soul/Funk: I-IV-V variations, Stevie Wonder-style progressions
     * Smooth Jazz: extended ii-V-I, modal progressions
     * Pop: I-V-vi-IV and common 4-chord variations
   - Support for passing chords, substitutions (tritone subs, diatonic subs)
   - Random progression generator with musical logic
   - User-defined custom progressions
   - Tempo and key selection

3. **Visualization System** (choose most effective approaches and combine them)
   - **Piano roll view**: Horizontal timeline showing chord tones highlighted on a piano keyboard
   - **Standard notation staff**: Show chord tones in traditional musical notation
   - **Chord tone movement diagram**: Visualize guide tone lines and voice leading between chords
   - **Scale/mode overlay**: Display available scale degrees over current chord with color coding:
     * Chord tones (root, 3rd, 5th, 7th) - bright/primary colors
     * Extensions (9th, 11th, 13th) - secondary colors
     * Avoid notes - dimmed or different marker
   - **Fretboard view** (optional bonus): Guitar/bass fretboard showing positions
   - Real-time highlighting as notes play

4. **Interactive Learning Modes**
   - **Listen mode**: Play progression, highlight chord tones and guide tone movement
   - **Practice mode**: User plays along, app shows current chord and suggested focus tones
   - **Quiz mode**: Play chord tones, user identifies which degree (root, 3rd, 5th, etc.)
   - **Voice leading trainer**: Show multiple voice leading options between chords
   - Loop individual chord transitions for focused practice

5. **Music Theory Intelligence**
   - Automatically determine appropriate scales/modes for each chord:
     * Major 7th → Ionian or Lydian
     * Minor 7th → Dorian, Aeolian, or Phrygian (context-dependent)
     * Dominant 7th → Mixolydian, altered, diminished, whole tone
     * Half-diminished → Locrian
   - Show extensions and tensions available for each chord
   - Highlight guide tones (3rd and 7th) and their resolution patterns
   - Display chord function in progression (tonic, subdominant, dominant)

6. **Terminal UI** (using tui-rs or ratatui)
   - Split-pane layout:
     * Main visualization area (piano roll/notation)
     * Chord progression display showing current and upcoming chords
     * Control panel (tempo, playback controls, mode selection)
     * Information panel (scale degrees, chord tones, current mode/scale)
   - Keyboard controls for all functions
   - Smooth animations for chord transitions
   - Color coding for different note types (chord tones vs. extensions vs. avoid notes)

</requirements>

<implementation>

**Project Structure:**
```
./ear-trainer/
├── Cargo.toml
├── src/
│   ├── main.rs              # Entry point, UI orchestration
│   ├── audio/
│   │   ├── mod.rs           # Audio module exports
│   │   ├── backend.rs       # AudioBackend trait
│   │   ├── midi.rs          # MIDI output implementation
│   │   └── synth.rs         # Built-in synthesis implementation
│   ├── music/
│   │   ├── mod.rs           # Music theory module exports
│   │   ├── chord.rs         # Chord representation and logic
│   │   ├── scale.rs         # Scale/mode definitions
│   │   ├── progression.rs   # Progression library and generator
│   │   └── theory.rs        # Voice leading, guide tones, analysis
│   ├── ui/
│   │   ├── mod.rs           # UI module exports
│   │   ├── app.rs           # Application state
│   │   ├── piano_roll.rs    # Piano roll visualization
│   │   ├── notation.rs      # Standard notation view
│   │   └── controls.rs      # Input handling
│   └── config.rs            # Configuration and presets
└── README.md
```

**Recommended Dependencies:**
- `ratatui` or `tui-rs` - Terminal UI framework
- `crossterm` - Terminal manipulation
- `midir` - MIDI I/O
- `rodio` or `cpal` - Audio playback (for synthesis)
- `serde` and `serde_json` - Configuration and progression storage

**Key Design Principles:**

1. **Audio abstraction**: Create an `AudioBackend` trait that both MIDI and synthesis implement, allowing runtime switching

2. **Music theory accuracy**: WHY this matters - musicians will rely on this for learning, so chord-scale relationships must be correct and context-aware (e.g., ii chord in major = Dorian, not Aeolian)

3. **Visualization clarity**: Use Unicode box drawing characters, Braille patterns, or block elements for smooth visual representation. Color should enhance understanding, not confuse.

4. **Progression library organization**: Structure progressions by:
   - Genre/style
   - Complexity level (beginner, intermediate, advanced)
   - Common patterns (turnarounds, vamps, bridges)

5. **Performance**: WHY this matters - audio timing must be precise for musical application. Use proper audio thread management and avoid blocking the UI thread.

**Go Beyond Basics:**
Include as many ingenious visualization ideas as possible. Consider:
- Arrow indicators showing guide tone movement (3rd to 7th, 7th to 3rd)
- Heat map showing most commonly used tones in jazz improvisation
- Timeline view showing the "story" of voice leading through an entire progression
- Side-by-side comparison of different voice leading options
- Integration of rhythm (swing feel, syncopation patterns common in each genre)

**What to Avoid:**
- Don't hardcode chord-scale relationships; build a flexible system that considers harmonic context
- Avoid cluttered UI; progressively disclose information based on selected mode
- Don't assume 12-EDO tuning exclusively (leave room for microtonal exploration)
- Never block the audio thread with UI rendering; keep these concerns separate
</implementation>

<output>
Create the following files:

- `./ear-trainer/Cargo.toml` - Project dependencies and metadata
- `./ear-trainer/src/main.rs` - Entry point with UI loop
- `./ear-trainer/src/audio/backend.rs` - AudioBackend trait definition
- `./ear-trainer/src/audio/midi.rs` - MIDI output implementation
- `./ear-trainer/src/audio/synth.rs` - Synthesis implementation (start with sine waves)
- `./ear-trainer/src/music/chord.rs` - Chord types, voicings, chord tone identification
- `./ear-trainer/src/music/scale.rs` - Scale and mode definitions with degree mappings
- `./ear-trainer/src/music/progression.rs` - Progression library with 10+ common progressions per genre
- `./ear-trainer/src/music/theory.rs` - Voice leading calculator, guide tone finder, chord-scale matcher
- `./ear-trainer/src/ui/app.rs` - Application state machine and mode management
- `./ear-trainer/src/ui/piano_roll.rs` - Piano roll visualization widget
- `./ear-trainer/src/ui/notation.rs` - Standard notation visualization (can start simple)
- `./ear-trainer/src/ui/controls.rs` - Keyboard input handling
- `./ear-trainer/src/config.rs` - Config struct with defaults
- `./ear-trainer/README.md` - Usage instructions, keybindings, feature overview

Each file should be fully implemented with working code, not stubs.
</output>

<verification>
Before declaring complete, verify:

1. **Compilation**: `cargo build --release` completes without errors
2. **Audio test**: Both MIDI and synthesis backends can produce sound
3. **Progression playback**: At least 5 progressions from different genres play correctly
4. **Visualization**: Piano roll displays and updates in real-time with chord changes
5. **Chord-scale matching**: Verify that common chords (Cmaj7, Dm7, G7) map to correct scales
6. **Interactive controls**: Keyboard controls for play/pause, tempo, progression selection work
7. **Voice leading**: Guide tone movement is correctly identified and can be visualized
8. **Documentation**: README includes quick start guide and keybinding reference

Run a full test:
```bash
cd ear-trainer
cargo run --release
```
Then test:
- Select a ii-V-I progression in C
- Verify Dm7 → Dorian, G7 → Mixolydian/Altered, Cmaj7 → Ionian
- Watch visualization show guide tone movement (F→B, A→C)
- Switch audio backend between MIDI and synthesis
</verification>

<success_criteria>
- Rust project compiles and runs on Linux (primary), macOS, Windows
- Both audio backends (MIDI and synthesis) functional and switchable
- Library of 40+ progressions across jazz, soul, funk, pop, smooth jazz
- At least 2 visualization modes working (suggest: piano roll + chord tone movement diagram)
- Accurate chord-scale relationships for all common chord types (maj7, m7, dom7, m7b5, dim7, sus, altered)
- Real-time visual feedback synchronized with audio playback
- Keyboard-driven UI with intuitive controls
- Guide tone voice leading correctly identified and visualized
- Random progression generator produces musically sensible results
- Clear documentation for musicians to start using immediately

The application should feel like a professional music learning tool, not a tech demo. Prioritize musical accuracy and usability.
</success_criteria>

<extended_thinking>
Deeply consider multiple approaches for:
- Visualization strategies: What combination of views provides maximum learning insight?
- Voice leading calculation: How to determine the smoothest movement between chord voicings?
- Audio synthesis: What waveforms and envelope shaping create pleasant, musical sounds?
- Progression generation: What rules ensure random progressions sound intentional and musical?
- Scale selection logic: How to handle ambiguous situations (e.g., minor ii-V-i vs. major)?

Explore creative solutions for representing complex music theory visually in limited terminal space.
</extended_thinking>