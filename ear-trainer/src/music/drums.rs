//! Drum Pattern System
//!
//! Provides drum patterns for different styles (jazz, latin, funk, etc.)
//! Uses General MIDI drum note numbers.

use serde::{Deserialize, Serialize};

/// General MIDI Drum Note Numbers
pub mod gm_drums {
    pub const KICK: u8 = 36;         // Bass Drum 1
    pub const SNARE: u8 = 38;        // Acoustic Snare
    pub const SIDE_STICK: u8 = 37;   // Side Stick
    pub const CLOSED_HH: u8 = 42;    // Closed Hi-Hat
    pub const OPEN_HH: u8 = 46;      // Open Hi-Hat
    pub const PEDAL_HH: u8 = 44;     // Pedal Hi-Hat
    pub const RIDE: u8 = 51;         // Ride Cymbal 1
    pub const RIDE_BELL: u8 = 53;    // Ride Bell
    pub const CRASH: u8 = 49;        // Crash Cymbal 1
    pub const HIGH_TOM: u8 = 50;     // High Tom
    pub const MID_TOM: u8 = 47;      // Low-Mid Tom
    pub const LOW_TOM: u8 = 45;      // Low Tom
    pub const CLAVES: u8 = 75;       // Claves
    pub const COWBELL: u8 = 56;      // Cowbell
    pub const SHAKER: u8 = 70;       // Maracas
}

/// A single drum hit in a pattern
#[derive(Debug, Clone, Copy)]
pub struct DrumHit {
    /// Beat position within the bar (0.0 = downbeat)
    pub beat: f32,
    /// MIDI note number (GM drum map)
    pub note: u8,
    /// Velocity (0.0-1.0)
    pub velocity: f32,
}

impl DrumHit {
    pub fn new(beat: f32, note: u8, velocity: f32) -> Self {
        Self { beat, note, velocity }
    }
}

/// Drum pattern style
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, Default)]
pub enum DrumStyle {
    /// No drums
    #[default]
    Off,
    /// Simple metronome (hi-hat only)
    Metronome,
    /// Jazz ride pattern
    JazzRide,
    /// Jazz brushes feel
    JazzBrushes,
    /// Bossa nova
    BossaNova,
    /// Funk pattern
    Funk,
    /// Rock/Pop basic
    Rock,
}

impl DrumStyle {
    /// Get the pattern for one bar (4 beats)
    pub fn pattern(&self) -> Vec<DrumHit> {
        use gm_drums::*;

        match self {
            DrumStyle::Off => vec![],

            DrumStyle::Metronome => vec![
                // Simple hi-hat quarters
                DrumHit::new(0.0, CLOSED_HH, 0.8),
                DrumHit::new(1.0, CLOSED_HH, 0.5),
                DrumHit::new(2.0, CLOSED_HH, 0.6),
                DrumHit::new(3.0, CLOSED_HH, 0.5),
            ],

            DrumStyle::JazzRide => vec![
                // Classic jazz ride pattern with swing feel
                // "spang-a-lang" pattern
                DrumHit::new(0.0, RIDE, 0.85),        // Beat 1
                DrumHit::new(1.0, RIDE, 0.6),         // Beat 2
                DrumHit::new(1.67, RIDE, 0.5),        // Swung skip note
                DrumHit::new(2.0, RIDE, 0.75),        // Beat 3
                DrumHit::new(3.0, RIDE, 0.6),         // Beat 4
                DrumHit::new(3.67, RIDE, 0.5),        // Swung skip note
                // Hi-hat on 2 and 4
                DrumHit::new(1.0, PEDAL_HH, 0.6),
                DrumHit::new(3.0, PEDAL_HH, 0.6),
            ],

            DrumStyle::JazzBrushes => vec![
                // Brushes pattern - more legato feel
                DrumHit::new(0.0, SIDE_STICK, 0.5),
                DrumHit::new(1.0, SIDE_STICK, 0.4),
                DrumHit::new(2.0, SIDE_STICK, 0.55),
                DrumHit::new(3.0, SIDE_STICK, 0.4),
                // Light hi-hat
                DrumHit::new(1.0, CLOSED_HH, 0.3),
                DrumHit::new(3.0, CLOSED_HH, 0.3),
            ],

            DrumStyle::BossaNova => vec![
                // Classic bossa nova pattern
                // Cross-stick pattern
                DrumHit::new(0.0, SIDE_STICK, 0.6),
                DrumHit::new(1.5, SIDE_STICK, 0.5),
                DrumHit::new(3.0, SIDE_STICK, 0.55),
                // Hi-hat eighths
                DrumHit::new(0.0, CLOSED_HH, 0.5),
                DrumHit::new(0.5, CLOSED_HH, 0.3),
                DrumHit::new(1.0, CLOSED_HH, 0.4),
                DrumHit::new(1.5, CLOSED_HH, 0.3),
                DrumHit::new(2.0, CLOSED_HH, 0.45),
                DrumHit::new(2.5, CLOSED_HH, 0.3),
                DrumHit::new(3.0, CLOSED_HH, 0.4),
                DrumHit::new(3.5, CLOSED_HH, 0.3),
                // Kick pattern
                DrumHit::new(0.0, KICK, 0.7),
                DrumHit::new(2.5, KICK, 0.5),
            ],

            DrumStyle::Funk => vec![
                // Funky 16th note pattern
                // Kick
                DrumHit::new(0.0, KICK, 0.95),
                DrumHit::new(1.5, KICK, 0.7),
                DrumHit::new(2.75, KICK, 0.8),
                // Snare on 2 and 4
                DrumHit::new(1.0, SNARE, 0.9),
                DrumHit::new(3.0, SNARE, 0.9),
                // Ghost notes
                DrumHit::new(1.5, SNARE, 0.3),
                DrumHit::new(2.5, SNARE, 0.25),
                DrumHit::new(3.75, SNARE, 0.35),
                // Hi-hat 16ths
                DrumHit::new(0.0, CLOSED_HH, 0.7),
                DrumHit::new(0.25, CLOSED_HH, 0.4),
                DrumHit::new(0.5, CLOSED_HH, 0.5),
                DrumHit::new(0.75, CLOSED_HH, 0.4),
                DrumHit::new(1.0, CLOSED_HH, 0.6),
                DrumHit::new(1.25, CLOSED_HH, 0.4),
                DrumHit::new(1.5, CLOSED_HH, 0.5),
                DrumHit::new(1.75, CLOSED_HH, 0.4),
                DrumHit::new(2.0, CLOSED_HH, 0.65),
                DrumHit::new(2.25, CLOSED_HH, 0.4),
                DrumHit::new(2.5, OPEN_HH, 0.6),     // Open on "and" of 3
                DrumHit::new(2.75, CLOSED_HH, 0.4),
                DrumHit::new(3.0, CLOSED_HH, 0.6),
                DrumHit::new(3.25, CLOSED_HH, 0.4),
                DrumHit::new(3.5, CLOSED_HH, 0.5),
                DrumHit::new(3.75, CLOSED_HH, 0.4),
            ],

            DrumStyle::Rock => vec![
                // Basic rock beat
                // Kick on 1 and 3
                DrumHit::new(0.0, KICK, 0.9),
                DrumHit::new(2.0, KICK, 0.85),
                // Snare on 2 and 4
                DrumHit::new(1.0, SNARE, 0.9),
                DrumHit::new(3.0, SNARE, 0.9),
                // Hi-hat eighths
                DrumHit::new(0.0, CLOSED_HH, 0.7),
                DrumHit::new(0.5, CLOSED_HH, 0.5),
                DrumHit::new(1.0, CLOSED_HH, 0.6),
                DrumHit::new(1.5, CLOSED_HH, 0.5),
                DrumHit::new(2.0, CLOSED_HH, 0.65),
                DrumHit::new(2.5, CLOSED_HH, 0.5),
                DrumHit::new(3.0, CLOSED_HH, 0.6),
                DrumHit::new(3.5, CLOSED_HH, 0.5),
            ],
        }
    }

    /// Cycle to next drum style
    pub fn next(&self) -> Self {
        match self {
            DrumStyle::Off => DrumStyle::Metronome,
            DrumStyle::Metronome => DrumStyle::JazzRide,
            DrumStyle::JazzRide => DrumStyle::JazzBrushes,
            DrumStyle::JazzBrushes => DrumStyle::BossaNova,
            DrumStyle::BossaNova => DrumStyle::Funk,
            DrumStyle::Funk => DrumStyle::Rock,
            DrumStyle::Rock => DrumStyle::Off,
        }
    }

    /// Get display name
    pub fn name(&self) -> &'static str {
        match self {
            DrumStyle::Off => "Off",
            DrumStyle::Metronome => "Click",
            DrumStyle::JazzRide => "Jazz",
            DrumStyle::JazzBrushes => "Brushes",
            DrumStyle::BossaNova => "Bossa",
            DrumStyle::Funk => "Funk",
            DrumStyle::Rock => "Rock",
        }
    }
}

/// Drum state tracker for playback
#[derive(Debug, Clone)]
pub struct DrumState {
    /// Current drum style
    pub style: DrumStyle,
    /// Current pattern
    pattern: Vec<DrumHit>,
    /// Indices of notes that have been played this bar
    played_notes: Vec<bool>,
    /// Last beat position checked
    last_beat: f32,
}

impl DrumState {
    pub fn new() -> Self {
        let style = DrumStyle::default();
        let pattern = style.pattern();
        let played_notes = vec![false; pattern.len()];
        Self {
            pattern,
            style,
            played_notes,
            last_beat: -1.0,
        }
    }

    /// Set the drum style
    pub fn set_style(&mut self, style: DrumStyle) {
        self.style = style;
        self.pattern = style.pattern();
        self.reset();
    }

    /// Cycle to next style
    pub fn cycle_style(&mut self) {
        self.set_style(self.style.next());
    }

    /// Reset for new bar
    pub fn reset(&mut self) {
        self.played_notes = vec![false; self.pattern.len()];
        self.last_beat = -1.0;
    }

    /// Check if we should play drum hits at the current beat position
    /// Returns a Vec of (midi_note, velocity) for all hits that should play
    pub fn check_hits(&mut self, current_beat: f32, beats_in_bar: f32) -> Vec<(u8, f32)> {
        if self.pattern.is_empty() {
            return vec![];
        }

        // Wrap beat position to bar
        let beat_in_bar = current_beat % beats_in_bar;

        // Reset if we've wrapped around to a new bar
        if beat_in_bar < self.last_beat {
            self.reset();
        }

        let mut hits = vec![];

        // Find all hits that should trigger
        for (i, drum_hit) in self.pattern.iter().enumerate() {
            // Skip already played hits
            if self.played_notes.get(i).copied().unwrap_or(true) {
                continue;
            }

            // Check if we've passed this hit's beat
            let hit_beat = drum_hit.beat % beats_in_bar;
            if beat_in_bar >= hit_beat && self.last_beat < hit_beat {
                if let Some(played) = self.played_notes.get_mut(i) {
                    *played = true;
                }
                hits.push((drum_hit.note, drum_hit.velocity));
            }
        }

        self.last_beat = beat_in_bar;
        hits
    }
}

impl Default for DrumState {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_drum_patterns() {
        for style in [
            DrumStyle::Off,
            DrumStyle::Metronome,
            DrumStyle::JazzRide,
            DrumStyle::JazzBrushes,
            DrumStyle::BossaNova,
            DrumStyle::Funk,
            DrumStyle::Rock,
        ] {
            let pattern = style.pattern();

            if style == DrumStyle::Off {
                assert!(pattern.is_empty());
                continue;
            }

            // All beats should be within 0-4
            for hit in &pattern {
                assert!(
                    hit.beat >= 0.0 && hit.beat < 4.0,
                    "{:?} hit beat {} out of range",
                    style,
                    hit.beat
                );
                assert!(
                    hit.velocity > 0.0 && hit.velocity <= 1.0,
                    "{:?} hit velocity {} out of range",
                    style,
                    hit.velocity
                );
                // Verify valid GM drum notes (35-81)
                assert!(
                    hit.note >= 35 && hit.note <= 81,
                    "{:?} hit note {} out of GM drum range",
                    style,
                    hit.note
                );
            }
        }
    }

    #[test]
    fn test_style_cycle() {
        let mut style = DrumStyle::Off;
        let start = style;

        // Cycle through all styles
        for _ in 0..7 {
            style = style.next();
        }

        // Should return to start
        assert_eq!(style, start);
    }
}
