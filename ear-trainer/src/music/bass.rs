//! Walking Bass Pattern System
//!
//! Provides walking bass lines that follow chord changes,
//! with different styles for jazz, latin, and funk.

use super::chord::{Chord, ChordQuality, Note};
use serde::{Deserialize, Serialize};

/// A single bass note in a pattern
#[derive(Debug, Clone, Copy)]
pub struct BassNote {
    /// Beat position within the bar (0.0 = downbeat)
    pub beat: f32,
    /// Scale degree relative to chord root (0 = root, 2 = third, 4 = fifth, etc.)
    pub degree: i8,
    /// Velocity (0.0-1.0)
    pub velocity: f32,
    /// Whether this is a chromatic approach note
    pub chromatic: bool,
}

impl BassNote {
    pub fn new(beat: f32, degree: i8, velocity: f32) -> Self {
        Self {
            beat,
            degree,
            velocity,
            chromatic: false,
        }
    }

    pub fn chromatic(beat: f32, degree: i8, velocity: f32) -> Self {
        Self {
            beat,
            degree,
            velocity,
            chromatic: true,
        }
    }
}

/// Bass pattern style
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, Default)]
pub enum BassStyle {
    /// No bass
    #[default]
    Off,
    /// Root notes only on beat 1
    RootOnly,
    /// Root and fifth
    RootFifth,
    /// Walking bass (quarter notes)
    Walking,
    /// Latin/Bossa style (root-fifth pattern)
    Latin,
    /// Funk style (syncopated)
    Funk,
}

impl BassStyle {
    /// Get the pattern for one bar (4 beats)
    /// Returns scale degrees relative to chord root
    pub fn pattern(&self) -> Vec<BassNote> {
        match self {
            BassStyle::Off => vec![],

            BassStyle::RootOnly => vec![
                BassNote::new(0.0, 0, 0.9),
            ],

            BassStyle::RootFifth => vec![
                BassNote::new(0.0, 0, 0.9),  // Root
                BassNote::new(2.0, 4, 0.7),  // Fifth
            ],

            BassStyle::Walking => vec![
                BassNote::new(0.0, 0, 0.9),   // Root
                BassNote::new(1.0, 2, 0.7),   // Third
                BassNote::new(2.0, 4, 0.75),  // Fifth
                BassNote::new(3.0, 6, 0.7),   // Seventh (approach to next root)
            ],

            BassStyle::Latin => vec![
                // Bossa nova bass pattern
                BassNote::new(0.0, 0, 0.85),   // Root
                BassNote::new(1.5, 4, 0.6),    // Fifth (anticipated)
                BassNote::new(2.0, 4, 0.7),    // Fifth
                BassNote::new(3.5, 0, 0.5),    // Root (pickup)
            ],

            BassStyle::Funk => vec![
                // Syncopated funk bass
                BassNote::new(0.0, 0, 0.95),   // Root (strong)
                BassNote::new(0.75, 0, 0.5),   // Ghost note
                BassNote::new(1.5, 4, 0.7),    // Fifth
                BassNote::new(2.0, 0, 0.8),    // Root
                BassNote::new(2.5, 2, 0.6),    // Third
                BassNote::new(3.0, 4, 0.75),   // Fifth
                BassNote::new(3.5, 0, 0.5),    // Root (pickup)
            ],
        }
    }

    /// Cycle to next bass style
    pub fn next(&self) -> Self {
        match self {
            BassStyle::Off => BassStyle::RootOnly,
            BassStyle::RootOnly => BassStyle::RootFifth,
            BassStyle::RootFifth => BassStyle::Walking,
            BassStyle::Walking => BassStyle::Latin,
            BassStyle::Latin => BassStyle::Funk,
            BassStyle::Funk => BassStyle::Off,
        }
    }

    /// Get display name
    pub fn name(&self) -> &'static str {
        match self {
            BassStyle::Off => "Off",
            BassStyle::RootOnly => "Root",
            BassStyle::RootFifth => "Root-5th",
            BassStyle::Walking => "Walking",
            BassStyle::Latin => "Latin",
            BassStyle::Funk => "Funk",
        }
    }
}

/// Convert scale degree to semitones for a given chord quality
fn degree_to_semitones(degree: i8, quality: ChordQuality) -> i8 {
    match degree {
        0 => 0,   // Root
        1 => 2,   // Second (whole step)
        2 => match quality {
            ChordQuality::Minor7 | ChordQuality::Minor6 | ChordQuality::Minor9 |
            ChordQuality::MinorMajor7 | ChordQuality::MinorMajor9 |
            ChordQuality::Diminished7 | ChordQuality::HalfDiminished => 3,  // Minor third
            _ => 4,  // Major third
        },
        3 => 5,   // Fourth
        4 => 7,   // Fifth (or diminished fifth for dim chords)
        5 => 9,   // Sixth (major sixth for walking bass)
        6 => match quality {
            ChordQuality::Major7 | ChordQuality::MinorMajor7 |
            ChordQuality::Major9 | ChordQuality::MinorMajor9 |
            ChordQuality::Major7sharp11 => 11,  // Major seventh
            _ => 10,  // Minor seventh
        },
        7 => 12,  // Octave
        _ => (degree % 8) as i8,
    }
}

/// Bass state tracker for playback
#[derive(Debug, Clone)]
pub struct BassState {
    /// Current bass style
    pub style: BassStyle,
    /// Current pattern
    pattern: Vec<BassNote>,
    /// Index of next note to play
    next_note_idx: usize,
    /// Last beat position checked
    last_beat: f32,
    /// Bass octave (MIDI note number for C)
    bass_octave: u8,
}

impl BassState {
    pub fn new() -> Self {
        let style = BassStyle::default();
        Self {
            pattern: style.pattern(),
            style,
            next_note_idx: 0,
            last_beat: -1.0,
            bass_octave: 36, // C2
        }
    }

    /// Set the bass style
    pub fn set_style(&mut self, style: BassStyle) {
        self.style = style;
        self.pattern = style.pattern();
        self.reset();
    }

    /// Cycle to next style
    pub fn cycle_style(&mut self) {
        self.set_style(self.style.next());
    }

    /// Reset for new chord/bar
    pub fn reset(&mut self) {
        self.next_note_idx = 0;
        self.last_beat = -1.0;
    }

    /// Check if we should play a bass note at the current beat position
    /// Returns Some((midi_note, velocity)) if we should play, None otherwise
    pub fn check_note(&mut self, current_beat: f32, beats_in_bar: f32, chord: &Chord) -> Option<(u8, f32)> {
        if self.pattern.is_empty() {
            return None;
        }

        // Wrap beat position to bar
        let beat_in_bar = current_beat % beats_in_bar;

        // Don't trigger on same beat twice
        if (beat_in_bar - self.last_beat).abs() < 0.05 {
            return None;
        }

        // Find if any note should trigger
        for (i, bass_note) in self.pattern.iter().enumerate() {
            // Skip already played notes
            if i < self.next_note_idx {
                continue;
            }

            // Check if we've passed this note's beat
            let note_beat = bass_note.beat % beats_in_bar;
            if beat_in_bar >= note_beat && self.last_beat < note_beat {
                self.next_note_idx = i + 1;
                self.last_beat = beat_in_bar;

                // Calculate MIDI note
                let root_midi = chord.root.to_midi(2); // Bass register (octave 2)
                let semitones = degree_to_semitones(bass_note.degree, chord.quality);
                let midi_note = (root_midi as i8 + semitones).clamp(24, 60) as u8;

                return Some((midi_note, bass_note.velocity));
            }
        }

        // Reset if we've wrapped around
        if beat_in_bar < self.last_beat {
            self.next_note_idx = 0;
        }

        self.last_beat = beat_in_bar;
        None
    }
}

impl Default for BassState {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_bass_patterns() {
        for style in [
            BassStyle::Off,
            BassStyle::RootOnly,
            BassStyle::RootFifth,
            BassStyle::Walking,
            BassStyle::Latin,
            BassStyle::Funk,
        ] {
            let pattern = style.pattern();

            if style == BassStyle::Off {
                assert!(pattern.is_empty());
                continue;
            }

            // All beats should be within 0-4
            for note in &pattern {
                assert!(
                    note.beat >= 0.0 && note.beat < 4.0,
                    "{:?} note beat {} out of range",
                    style,
                    note.beat
                );
                assert!(
                    note.velocity > 0.0 && note.velocity <= 1.0,
                    "{:?} note velocity {} out of range",
                    style,
                    note.velocity
                );
            }
        }
    }

    #[test]
    fn test_style_cycle() {
        let mut style = BassStyle::Off;
        let start = style;

        // Cycle through all styles
        for _ in 0..6 {
            style = style.next();
        }

        // Should return to start
        assert_eq!(style, start);
    }
}
