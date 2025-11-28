//! Jazz Piano Voicings
//!
//! Different voicing strategies for chords, from simple shell voicings
//! to sophisticated rootless voicings used by jazz pianists.

use super::chord::{Chord, ChordQuality};
use serde::{Deserialize, Serialize};

/// Types of chord voicings
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, Default)]
pub enum VoicingType {
    /// All chord tones in close position
    #[default]
    Full,
    /// 3rd and 7th only (guide tones)
    Shell,
    /// 3-5-7-9 without root (Bill Evans left hand)
    RootlessA,
    /// 7-9-3-5 without root (inverted)
    RootlessB,
    /// Drop 2 voicing - second voice from top dropped an octave
    Drop2,
}

impl VoicingType {
    pub fn name(&self) -> &'static str {
        match self {
            VoicingType::Full => "Full",
            VoicingType::Shell => "Shell",
            VoicingType::RootlessA => "Rootless A",
            VoicingType::RootlessB => "Rootless B",
            VoicingType::Drop2 => "Drop 2",
        }
    }

    pub fn description(&self) -> &'static str {
        match self {
            VoicingType::Full => "All chord tones in close position",
            VoicingType::Shell => "3rd and 7th only (guide tones)",
            VoicingType::RootlessA => "3-5-7-9 (Bill Evans style, no root)",
            VoicingType::RootlessB => "7-9-3-5 (inverted rootless)",
            VoicingType::Drop2 => "Second voice from top dropped an octave",
        }
    }

    pub fn next(self) -> Self {
        match self {
            VoicingType::Full => VoicingType::Shell,
            VoicingType::Shell => VoicingType::RootlessA,
            VoicingType::RootlessA => VoicingType::RootlessB,
            VoicingType::RootlessB => VoicingType::Drop2,
            VoicingType::Drop2 => VoicingType::Full,
        }
    }

    /// Generate MIDI notes for this voicing
    ///
    /// # Arguments
    /// * `chord` - The chord to voice
    /// * `bass_octave` - Octave for the bass note (typically 2 or 3)
    /// * `voicing_octave` - Base octave for the voicing (typically 3 or 4)
    /// * `range` - (min_midi, max_midi) to clamp output notes
    ///
    /// # Returns
    /// Vector of MIDI note numbers, sorted low to high
    pub fn voice_chord(
        &self,
        chord: &Chord,
        bass_octave: i8,
        voicing_octave: i8,
        range: (u8, u8),
    ) -> VoicedChord {
        let bass = chord
            .bass
            .unwrap_or(chord.root)
            .to_midi(bass_octave);

        let voicing_notes = match self {
            VoicingType::Full => self.full_voicing(chord, voicing_octave),
            VoicingType::Shell => self.shell_voicing(chord, voicing_octave),
            VoicingType::RootlessA => self.rootless_a(chord, voicing_octave),
            VoicingType::RootlessB => self.rootless_b(chord, voicing_octave),
            VoicingType::Drop2 => self.drop2_voicing(chord, voicing_octave),
        };

        // Filter to range
        let filtered: Vec<u8> = voicing_notes
            .into_iter()
            .filter(|&n| n >= range.0 && n <= range.1)
            .collect();

        VoicedChord {
            bass,
            voicing: filtered,
        }
    }

    fn full_voicing(&self, chord: &Chord, octave: i8) -> Vec<u8> {
        chord
            .quality
            .intervals()
            .iter()
            .take(4) // Limit to first 4 notes for dense chords
            .map(|&interval| chord.root.transpose(interval).to_midi(octave))
            .collect()
    }

    fn shell_voicing(&self, chord: &Chord, octave: i8) -> Vec<u8> {
        let intervals = chord.quality.intervals();
        let mut notes = Vec::new();

        // Find 3rd (interval 3 or 4)
        if let Some(&third_int) = intervals.iter().find(|&&i| i == 3 || i == 4) {
            notes.push(chord.root.transpose(third_int).to_midi(octave));
        }

        // Find 7th (interval 10, 11, or 9 for 6th chords)
        if let Some(&seventh_int) = intervals.iter().find(|&&i| i == 10 || i == 11 || i == 9) {
            notes.push(chord.root.transpose(seventh_int).to_midi(octave));
        }

        notes.sort_unstable();
        notes
    }

    fn rootless_a(&self, chord: &Chord, octave: i8) -> Vec<u8> {
        // Type A: 3-5-7-9 (from bottom to top)
        let intervals = chord.quality.intervals();
        let mut notes = Vec::new();

        // 3rd
        if let Some(&third) = intervals.iter().find(|&&i| i == 3 || i == 4) {
            notes.push(chord.root.transpose(third).to_midi(octave));
        }

        // 5th
        if intervals.contains(&7) {
            notes.push(chord.root.transpose(7).to_midi(octave));
        } else if intervals.contains(&6) {
            // Diminished 5th
            notes.push(chord.root.transpose(6).to_midi(octave));
        }

        // 7th
        if let Some(&seventh) = intervals.iter().find(|&&i| i == 10 || i == 11) {
            notes.push(chord.root.transpose(seventh).to_midi(octave));
        } else if intervals.contains(&9) {
            // 6th chord
            notes.push(chord.root.transpose(9).to_midi(octave));
        }

        // 9th (add if available, one octave up)
        if let Some(&ninth) = intervals.iter().find(|&&i| i >= 13 && i <= 15) {
            notes.push(chord.root.transpose(ninth - 12).to_midi(octave + 1));
        } else {
            // Add natural 9th if not in chord
            notes.push(chord.root.transpose(2).to_midi(octave + 1));
        }

        notes.sort_unstable();
        notes
    }

    fn rootless_b(&self, chord: &Chord, octave: i8) -> Vec<u8> {
        // Type B: 7-9-3-5 (from bottom to top) - inverted rootless
        let intervals = chord.quality.intervals();
        let mut notes = Vec::new();

        // 7th at bottom
        if let Some(&seventh) = intervals.iter().find(|&&i| i == 10 || i == 11) {
            notes.push(chord.root.transpose(seventh).to_midi(octave));
        } else if intervals.contains(&9) {
            notes.push(chord.root.transpose(9).to_midi(octave));
        }

        // 9th
        if let Some(&ninth) = intervals.iter().find(|&&i| i >= 13 && i <= 15) {
            notes.push(chord.root.transpose(ninth - 12).to_midi(octave));
        } else {
            notes.push(chord.root.transpose(2).to_midi(octave));
        }

        // 3rd (one octave up)
        if let Some(&third) = intervals.iter().find(|&&i| i == 3 || i == 4) {
            notes.push(chord.root.transpose(third).to_midi(octave + 1));
        }

        // 5th (one octave up)
        if intervals.contains(&7) {
            notes.push(chord.root.transpose(7).to_midi(octave + 1));
        } else if intervals.contains(&6) {
            notes.push(chord.root.transpose(6).to_midi(octave + 1));
        }

        notes.sort_unstable();
        notes
    }

    fn drop2_voicing(&self, chord: &Chord, octave: i8) -> Vec<u8> {
        // Drop 2: Start with close voicing, drop second note from top by an octave
        let mut close = self.full_voicing(chord, octave);
        close.sort_unstable();

        if close.len() >= 3 {
            // Second from top goes down an octave
            let drop_idx = close.len() - 2;
            if close[drop_idx] >= 12 {
                close[drop_idx] -= 12;
            }
        }

        close.sort_unstable();
        close
    }
}

/// A chord that has been voiced with specific MIDI notes
#[derive(Debug, Clone)]
pub struct VoicedChord {
    /// Bass note (typically played by left hand or bass instrument)
    pub bass: u8,
    /// Voicing notes (comping hand)
    pub voicing: Vec<u8>,
}

impl VoicedChord {
    /// Get all notes combined (bass + voicing)
    pub fn all_notes(&self) -> Vec<u8> {
        let mut notes = vec![self.bass];
        notes.extend(&self.voicing);
        notes.sort_unstable();
        notes.dedup();
        notes
    }

    /// Check if this voicing would sound good (no collisions, reasonable spread)
    pub fn is_playable(&self) -> bool {
        let all = self.all_notes();
        if all.len() < 2 {
            return false;
        }

        // Check for no very close notes (less than 2 semitones except bass)
        for window in all.windows(2) {
            if window[1] - window[0] < 2 && window[0] > self.bass {
                return false;
            }
        }

        // Check total spread isn't too wide (max 2 octaves for voicing)
        if let (Some(&lowest), Some(&highest)) = (self.voicing.first(), self.voicing.last()) {
            if highest - lowest > 24 {
                return false;
            }
        }

        true
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::music::chord::Note;

    #[test]
    fn test_shell_voicing() {
        let cmaj7 = Chord::new(Note::C, ChordQuality::Major7);
        let voiced = VoicingType::Shell.voice_chord(&cmaj7, 2, 4, (36, 84));

        // Shell should have 3rd (E) and 7th (B)
        assert_eq!(voiced.voicing.len(), 2);
        // E4 = 64, B4 = 71
        assert!(voiced.voicing.contains(&64)); // E4
        assert!(voiced.voicing.contains(&71)); // B4
    }

    #[test]
    fn test_rootless_a() {
        let dm7 = Chord::new(Note::D, ChordQuality::Minor7);
        let voiced = VoicingType::RootlessA.voice_chord(&dm7, 2, 4, (36, 84));

        // Should have F (3rd), A (5th), C (7th), E (9th)
        // No D (root)
        let all = voiced.all_notes();
        assert!(!all.contains(&50)); // D3 should not be in voicing
    }

    #[test]
    fn test_voicing_cycle() {
        let v = VoicingType::Full;
        assert_eq!(v.next(), VoicingType::Shell);
        assert_eq!(v.next().next(), VoicingType::RootlessA);
        assert_eq!(
            v.next().next().next().next().next(),
            VoicingType::Full
        );
    }

    #[test]
    fn test_voiced_chord_playable() {
        let cmaj7 = Chord::new(Note::C, ChordQuality::Major7);
        let voiced = VoicingType::Full.voice_chord(&cmaj7, 2, 4, (36, 84));
        assert!(voiced.is_playable());
    }
}
