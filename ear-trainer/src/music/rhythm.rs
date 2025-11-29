//! Rhythm Patterns for Chord Comping
//!
//! Provides basic rhythmic patterns for chord playback,
//! inspired by Band-in-a-Box style accompaniment.

use serde::{Deserialize, Serialize};

/// A single hit in a rhythm pattern
#[derive(Debug, Clone, Copy)]
pub struct RhythmHit {
    /// Beat position (0.0 = downbeat, 0.5 = eighth note, etc.)
    pub beat: f32,
    /// Velocity (0.0-1.0, will be scaled to MIDI velocity)
    pub velocity: f32,
    /// Duration in beats (how long to hold the chord)
    pub duration: f32,
}

impl RhythmHit {
    pub fn new(beat: f32, velocity: f32, duration: f32) -> Self {
        Self { beat, velocity, duration }
    }
}

/// Rhythm pattern style
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, Default)]
pub enum RhythmStyle {
    /// Whole notes - just sustain (original behavior)
    #[default]
    Whole,
    /// Basic jazz comping - Charleston rhythm
    JazzBasic,
    /// Swing comping with anticipations
    SwingComp,
    /// Bossa nova pattern
    BossaNova,
    /// Funk/R&B 16th note pattern
    Funk,
    /// Simple quarter notes
    Quarter,
}

impl RhythmStyle {
    /// Get the pattern for one bar (4 beats)
    pub fn pattern(&self) -> Vec<RhythmHit> {
        match self {
            RhythmStyle::Whole => vec![
                // Just one hit on beat 1, sustained
                RhythmHit::new(0.0, 0.8, 3.5),
            ],

            RhythmStyle::Quarter => vec![
                // Simple quarter notes
                RhythmHit::new(0.0, 0.8, 0.8),
                RhythmHit::new(1.0, 0.6, 0.8),
                RhythmHit::new(2.0, 0.7, 0.8),
                RhythmHit::new(3.0, 0.6, 0.8),
            ],

            RhythmStyle::JazzBasic => vec![
                // Charleston rhythm: beat 1 and the "and" of 2
                RhythmHit::new(0.0, 0.85, 1.0),   // Beat 1
                RhythmHit::new(1.5, 0.7, 0.8),    // And of 2
                RhythmHit::new(3.0, 0.65, 0.5),   // Beat 4 (optional pickup)
            ],

            RhythmStyle::SwingComp => vec![
                // More active swing comping
                RhythmHit::new(0.0, 0.8, 0.5),    // Beat 1
                RhythmHit::new(1.33, 0.6, 0.4),   // Swung "and" of 1 (triplet feel)
                RhythmHit::new(2.0, 0.75, 0.5),   // Beat 3
                RhythmHit::new(3.33, 0.65, 0.4),  // Swung "and" of 3
            ],

            RhythmStyle::BossaNova => vec![
                // Classic bossa pattern
                RhythmHit::new(0.0, 0.75, 0.4),   // Beat 1
                RhythmHit::new(0.75, 0.5, 0.4),   // Dotted eighth
                RhythmHit::new(1.5, 0.7, 0.4),    // And of 2
                RhythmHit::new(2.5, 0.6, 0.4),    // And of 3
                RhythmHit::new(3.0, 0.65, 0.4),   // Beat 4
            ],

            RhythmStyle::Funk => vec![
                // 16th note funk pattern
                RhythmHit::new(0.0, 0.9, 0.2),    // Beat 1
                RhythmHit::new(0.5, 0.5, 0.2),    // And of 1
                RhythmHit::new(0.75, 0.6, 0.2),   // E of 1
                RhythmHit::new(1.5, 0.7, 0.2),    // And of 2
                RhythmHit::new(2.0, 0.8, 0.2),    // Beat 3
                RhythmHit::new(2.75, 0.55, 0.2),  // E of 3
                RhythmHit::new(3.25, 0.65, 0.2),  // A of 4
                RhythmHit::new(3.5, 0.6, 0.2),    // And of 4
            ],
        }
    }

    /// Cycle to next rhythm style
    pub fn next(&self) -> Self {
        match self {
            RhythmStyle::Whole => RhythmStyle::Quarter,
            RhythmStyle::Quarter => RhythmStyle::JazzBasic,
            RhythmStyle::JazzBasic => RhythmStyle::SwingComp,
            RhythmStyle::SwingComp => RhythmStyle::BossaNova,
            RhythmStyle::BossaNova => RhythmStyle::Funk,
            RhythmStyle::Funk => RhythmStyle::Whole,
        }
    }

    /// Get display name
    pub fn name(&self) -> &'static str {
        match self {
            RhythmStyle::Whole => "Whole",
            RhythmStyle::Quarter => "Quarter",
            RhythmStyle::JazzBasic => "Jazz",
            RhythmStyle::SwingComp => "Swing",
            RhythmStyle::BossaNova => "Bossa",
            RhythmStyle::Funk => "Funk",
        }
    }
}

/// Rhythm state tracker for playback
#[derive(Debug, Clone)]
pub struct RhythmState {
    /// Current rhythm style
    pub style: RhythmStyle,
    /// Current pattern
    pattern: Vec<RhythmHit>,
    /// Index of next hit to play
    next_hit_idx: usize,
    /// Last beat position checked
    last_beat: f32,
}

impl RhythmState {
    pub fn new() -> Self {
        let style = RhythmStyle::default();
        Self {
            pattern: style.pattern(),
            style,
            next_hit_idx: 0,
            last_beat: -1.0,
        }
    }

    /// Set the rhythm style
    pub fn set_style(&mut self, style: RhythmStyle) {
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
        self.next_hit_idx = 0;
        self.last_beat = -1.0;
    }

    /// Check if we should play a hit at the current beat position
    /// Returns Some((velocity, duration)) if we should play, None otherwise
    pub fn check_hit(&mut self, current_beat: f32, beats_in_bar: f32) -> Option<(f32, f32)> {
        if self.pattern.is_empty() {
            return None;
        }

        // Wrap beat position to bar
        let beat_in_bar = current_beat % beats_in_bar;

        // Don't trigger on same beat twice
        if (beat_in_bar - self.last_beat).abs() < 0.05 {
            return None;
        }

        // Find if any hit should trigger
        for (i, hit) in self.pattern.iter().enumerate() {
            // Skip already played hits
            if i < self.next_hit_idx {
                continue;
            }

            // Check if we've passed this hit's beat
            let hit_beat = hit.beat % beats_in_bar;
            if beat_in_bar >= hit_beat && self.last_beat < hit_beat {
                self.next_hit_idx = i + 1;
                self.last_beat = beat_in_bar;
                return Some((hit.velocity, hit.duration));
            }
        }

        // Reset if we've wrapped around
        if beat_in_bar < self.last_beat {
            self.next_hit_idx = 0;
        }

        self.last_beat = beat_in_bar;
        None
    }
}

impl Default for RhythmState {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_rhythm_patterns() {
        for style in [
            RhythmStyle::Whole,
            RhythmStyle::Quarter,
            RhythmStyle::JazzBasic,
            RhythmStyle::SwingComp,
            RhythmStyle::BossaNova,
            RhythmStyle::Funk,
        ] {
            let pattern = style.pattern();
            assert!(!pattern.is_empty(), "{:?} pattern should not be empty", style);

            // All beats should be within 0-4
            for hit in &pattern {
                assert!(hit.beat >= 0.0 && hit.beat < 4.0,
                    "{:?} hit beat {} out of range", style, hit.beat);
                assert!(hit.velocity > 0.0 && hit.velocity <= 1.0,
                    "{:?} hit velocity {} out of range", style, hit.velocity);
            }
        }
    }

    #[test]
    fn test_style_cycle() {
        let mut style = RhythmStyle::Whole;
        let start = style;

        // Cycle through all styles
        for _ in 0..6 {
            style = style.next();
        }

        // Should return to start
        assert_eq!(style, start);
    }
}
