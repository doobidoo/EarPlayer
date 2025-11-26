use super::chord::{Chord, ChordQuality, Note};
use serde::{Deserialize, Serialize};
use std::fmt;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum ScaleType {
    Major,
    NaturalMinor,
    HarmonicMinor,
    MelodicMinor,
    Dorian,
    Phrygian,
    Lydian,
    Mixolydian,
    Aeolian,
    Locrian,
    LydianDominant,
    Altered,
    WholeTone,
    DiminishedHalfWhole,
    DiminishedWholeHalf,
    MinorPentatonic,
    MajorPentatonic,
    BluesMajor,
    BluesMinor,
}

impl ScaleType {
    pub fn intervals(&self) -> Vec<i8> {
        match self {
            ScaleType::Major => vec![0, 2, 4, 5, 7, 9, 11],
            ScaleType::NaturalMinor => vec![0, 2, 3, 5, 7, 8, 10],
            ScaleType::HarmonicMinor => vec![0, 2, 3, 5, 7, 8, 11],
            ScaleType::MelodicMinor => vec![0, 2, 3, 5, 7, 9, 11],
            ScaleType::Dorian => vec![0, 2, 3, 5, 7, 9, 10],
            ScaleType::Phrygian => vec![0, 1, 3, 5, 7, 8, 10],
            ScaleType::Lydian => vec![0, 2, 4, 6, 7, 9, 11],
            ScaleType::Mixolydian => vec![0, 2, 4, 5, 7, 9, 10],
            ScaleType::Aeolian => vec![0, 2, 3, 5, 7, 8, 10],
            ScaleType::Locrian => vec![0, 1, 3, 5, 6, 8, 10],
            ScaleType::LydianDominant => vec![0, 2, 4, 6, 7, 9, 10],
            ScaleType::Altered => vec![0, 1, 3, 4, 6, 8, 10],
            ScaleType::WholeTone => vec![0, 2, 4, 6, 8, 10],
            ScaleType::DiminishedHalfWhole => vec![0, 1, 3, 4, 6, 7, 9, 10],
            ScaleType::DiminishedWholeHalf => vec![0, 2, 3, 5, 6, 8, 9, 11],
            ScaleType::MinorPentatonic => vec![0, 3, 5, 7, 10],
            ScaleType::MajorPentatonic => vec![0, 2, 4, 7, 9],
            ScaleType::BluesMajor => vec![0, 2, 3, 4, 7, 9],
            ScaleType::BluesMinor => vec![0, 3, 5, 6, 7, 10],
        }
    }

    pub fn name(&self) -> &'static str {
        match self {
            ScaleType::Major => "Major (Ionian)",
            ScaleType::NaturalMinor => "Natural Minor",
            ScaleType::HarmonicMinor => "Harmonic Minor",
            ScaleType::MelodicMinor => "Melodic Minor",
            ScaleType::Dorian => "Dorian",
            ScaleType::Phrygian => "Phrygian",
            ScaleType::Lydian => "Lydian",
            ScaleType::Mixolydian => "Mixolydian",
            ScaleType::Aeolian => "Aeolian",
            ScaleType::Locrian => "Locrian",
            ScaleType::LydianDominant => "Lydian Dominant",
            ScaleType::Altered => "Altered",
            ScaleType::WholeTone => "Whole Tone",
            ScaleType::DiminishedHalfWhole => "Diminished (H-W)",
            ScaleType::DiminishedWholeHalf => "Diminished (W-H)",
            ScaleType::MinorPentatonic => "Minor Pentatonic",
            ScaleType::MajorPentatonic => "Major Pentatonic",
            ScaleType::BluesMajor => "Blues Major",
            ScaleType::BluesMinor => "Blues Minor",
        }
    }
}

impl fmt::Display for ScaleType {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.name())
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Mode {
    Ionian,
    Dorian,
    Phrygian,
    Lydian,
    Mixolydian,
    Aeolian,
    Locrian,
}

impl Mode {
    pub fn to_scale_type(self) -> ScaleType {
        match self {
            Mode::Ionian => ScaleType::Major,
            Mode::Dorian => ScaleType::Dorian,
            Mode::Phrygian => ScaleType::Phrygian,
            Mode::Lydian => ScaleType::Lydian,
            Mode::Mixolydian => ScaleType::Mixolydian,
            Mode::Aeolian => ScaleType::Aeolian,
            Mode::Locrian => ScaleType::Locrian,
        }
    }
}

#[derive(Debug, Clone)]
pub struct Scale {
    pub root: Note,
    pub scale_type: ScaleType,
}

impl Scale {
    pub fn new(root: Note, scale_type: ScaleType) -> Self {
        Self { root, scale_type }
    }

    pub fn notes(&self) -> Vec<Note> {
        self.scale_type
            .intervals()
            .iter()
            .map(|&interval| self.root.transpose(interval))
            .collect()
    }

    pub fn contains(&self, note: Note) -> bool {
        let interval = (note as i8 - self.root as i8).rem_euclid(12);
        self.scale_type.intervals().contains(&interval)
    }

    pub fn degree(&self, note: Note) -> Option<usize> {
        let interval = (note as i8 - self.root as i8).rem_euclid(12);
        self.scale_type
            .intervals()
            .iter()
            .position(|&i| i == interval)
            .map(|p| p + 1)
    }

    pub fn name(&self) -> String {
        format!("{} {}", self.root, self.scale_type.name())
    }

    pub fn is_chord_tone(&self, chord: &Chord, note: Note) -> bool {
        chord.notes().contains(&note)
    }

    pub fn is_extension(&self, chord: &Chord, note: Note) -> bool {
        self.contains(note) && !self.is_chord_tone(chord, note)
    }

    pub fn available_extensions(&self, chord: &Chord) -> Vec<(Note, &'static str)> {
        let chord_notes = chord.notes();
        let scale_notes = self.notes();
        let mut extensions = Vec::new();

        for note in scale_notes {
            if !chord_notes.contains(&note) {
                let interval = (note as i8 - chord.root as i8).rem_euclid(12);
                let label = match interval {
                    2 => "9th",
                    3 => "b9th",
                    4 => "#9th",
                    5 => "11th",
                    6 => "#11th/b5",
                    8 => "b13th",
                    9 => "13th",
                    _ => "tension",
                };
                extensions.push((note, label));
            }
        }

        extensions
    }
}

impl fmt::Display for Scale {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.name())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_scale_notes() {
        let c_major = Scale::new(Note::C, ScaleType::Major);
        let notes = c_major.notes();
        assert_eq!(
            notes,
            vec![Note::C, Note::D, Note::E, Note::F, Note::G, Note::A, Note::B]
        );
    }

    #[test]
    fn test_scale_contains() {
        let c_major = Scale::new(Note::C, ScaleType::Major);
        assert!(c_major.contains(Note::C));
        assert!(c_major.contains(Note::E));
        assert!(!c_major.contains(Note::Eb));
    }
}
