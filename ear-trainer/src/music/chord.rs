use serde::{Deserialize, Serialize};
use std::fmt;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum Note {
    C = 0,
    Db = 1,
    D = 2,
    Eb = 3,
    E = 4,
    F = 5,
    Gb = 6,
    G = 7,
    Ab = 8,
    A = 9,
    Bb = 10,
    B = 11,
}

impl Note {
    pub fn from_midi(midi: u8) -> Self {
        match midi % 12 {
            0 => Note::C,
            1 => Note::Db,
            2 => Note::D,
            3 => Note::Eb,
            4 => Note::E,
            5 => Note::F,
            6 => Note::Gb,
            7 => Note::G,
            8 => Note::Ab,
            9 => Note::A,
            10 => Note::Bb,
            11 => Note::B,
            _ => unreachable!(),
        }
    }

    pub fn to_midi(self, octave: i8) -> u8 {
        ((octave as i16 + 1) * 12 + self as i16) as u8
    }

    pub fn transpose(self, semitones: i8) -> Self {
        let new_note = (self as i8 + semitones).rem_euclid(12);
        Self::from_midi(new_note as u8)
    }

    pub fn name(self) -> &'static str {
        match self {
            Note::C => "C",
            Note::Db => "Db",
            Note::D => "D",
            Note::Eb => "Eb",
            Note::E => "E",
            Note::F => "F",
            Note::Gb => "Gb",
            Note::G => "G",
            Note::Ab => "Ab",
            Note::A => "A",
            Note::Bb => "Bb",
            Note::B => "B",
        }
    }

    pub fn sharp_name(self) -> &'static str {
        match self {
            Note::C => "C",
            Note::Db => "C#",
            Note::D => "D",
            Note::Eb => "D#",
            Note::E => "E",
            Note::F => "F",
            Note::Gb => "F#",
            Note::G => "G",
            Note::Ab => "G#",
            Note::A => "A",
            Note::Bb => "A#",
            Note::B => "B",
        }
    }
}

impl fmt::Display for Note {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.name())
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum ChordQuality {
    Major7,           // Cmaj7
    Minor7,           // Cm7
    Dominant7,        // C7
    HalfDiminished,   // Cm7b5
    Diminished7,      // Cdim7
    MinorMajor7,      // Cm(maj7)
    Major6,           // C6
    Minor6,           // Cm6
    Dominant7sus4,    // C7sus4
    Major9,           // Cmaj9
    Minor9,           // Cm9
    Dominant9,        // C9
    Dominant7b9,      // C7b9
    Dominant7sharp9,  // C7#9
    Dominant7b13,     // C7b13
    Altered,          // C7alt (b9, #9, b13)
    MinorMajor9,      // Cm(maj9)
    Major7sharp11,    // Cmaj7#11
}

impl ChordQuality {
    pub fn intervals(&self) -> Vec<i8> {
        match self {
            ChordQuality::Major7 => vec![0, 4, 7, 11],
            ChordQuality::Minor7 => vec![0, 3, 7, 10],
            ChordQuality::Dominant7 => vec![0, 4, 7, 10],
            ChordQuality::HalfDiminished => vec![0, 3, 6, 10],
            ChordQuality::Diminished7 => vec![0, 3, 6, 9],
            ChordQuality::MinorMajor7 => vec![0, 3, 7, 11],
            ChordQuality::Major6 => vec![0, 4, 7, 9],
            ChordQuality::Minor6 => vec![0, 3, 7, 9],
            ChordQuality::Dominant7sus4 => vec![0, 5, 7, 10],
            ChordQuality::Major9 => vec![0, 4, 7, 11, 14],
            ChordQuality::Minor9 => vec![0, 3, 7, 10, 14],
            ChordQuality::Dominant9 => vec![0, 4, 7, 10, 14],
            ChordQuality::Dominant7b9 => vec![0, 4, 7, 10, 13],
            ChordQuality::Dominant7sharp9 => vec![0, 4, 7, 10, 15],
            ChordQuality::Dominant7b13 => vec![0, 4, 7, 10, 20],
            ChordQuality::Altered => vec![0, 4, 7, 10, 13, 15, 20],
            ChordQuality::MinorMajor9 => vec![0, 3, 7, 11, 14],
            ChordQuality::Major7sharp11 => vec![0, 4, 7, 11, 18],
        }
    }

    pub fn symbol(&self) -> &'static str {
        match self {
            ChordQuality::Major7 => "maj7",
            ChordQuality::Minor7 => "m7",
            ChordQuality::Dominant7 => "7",
            ChordQuality::HalfDiminished => "m7b5",
            ChordQuality::Diminished7 => "dim7",
            ChordQuality::MinorMajor7 => "m(maj7)",
            ChordQuality::Major6 => "6",
            ChordQuality::Minor6 => "m6",
            ChordQuality::Dominant7sus4 => "7sus4",
            ChordQuality::Major9 => "maj9",
            ChordQuality::Minor9 => "m9",
            ChordQuality::Dominant9 => "9",
            ChordQuality::Dominant7b9 => "7b9",
            ChordQuality::Dominant7sharp9 => "7#9",
            ChordQuality::Dominant7b13 => "7b13",
            ChordQuality::Altered => "7alt",
            ChordQuality::MinorMajor9 => "m(maj9)",
            ChordQuality::Major7sharp11 => "maj7#11",
        }
    }
}

impl fmt::Display for ChordQuality {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.symbol())
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ChordTone {
    Root,
    Third,
    Fifth,
    Seventh,
    Ninth,
    Eleventh,
    Thirteenth,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Chord {
    pub root: Note,
    pub quality: ChordQuality,
    pub bass: Option<Note>,
}

impl Chord {
    pub fn new(root: Note, quality: ChordQuality) -> Self {
        Self {
            root,
            quality,
            bass: None,
        }
    }

    pub fn with_bass(root: Note, quality: ChordQuality, bass: Note) -> Self {
        Self {
            root,
            quality,
            bass: Some(bass),
        }
    }

    pub fn notes(&self) -> Vec<Note> {
        self.quality
            .intervals()
            .iter()
            .map(|&interval| self.root.transpose(interval))
            .collect()
    }

    pub fn notes_in_range(&self, min_midi: u8, max_midi: u8) -> Vec<u8> {
        let mut result = Vec::new();
        let intervals = self.quality.intervals();

        for octave in -1..=8 {
            for &interval in &intervals {
                let note = self.root.transpose(interval);
                let midi = note.to_midi(octave);
                if midi >= min_midi && midi <= max_midi {
                    result.push(midi);
                }
            }
        }

        result.sort_unstable();
        result
    }

    pub fn chord_tone(&self, note: Note) -> Option<ChordTone> {
        let interval = (note as i8 - self.root as i8).rem_euclid(12);
        let intervals = self.quality.intervals();

        if intervals.contains(&interval) {
            match interval {
                0 => Some(ChordTone::Root),
                3 | 4 => Some(ChordTone::Third),
                6 | 7 => Some(ChordTone::Fifth),
                9 | 10 | 11 => Some(ChordTone::Seventh),
                13 | 14 | 15 => Some(ChordTone::Ninth),
                17 | 18 => Some(ChordTone::Eleventh),
                20 | 21 => Some(ChordTone::Thirteenth),
                _ => None,
            }
        } else {
            None
        }
    }

    pub fn guide_tones(&self) -> Vec<Note> {
        let intervals = self.quality.intervals();
        let mut tones = Vec::new();

        // 3rd
        if intervals.contains(&3) {
            tones.push(self.root.transpose(3));
        } else if intervals.contains(&4) {
            tones.push(self.root.transpose(4));
        }

        // 7th
        if intervals.contains(&10) {
            tones.push(self.root.transpose(10));
        } else if intervals.contains(&11) {
            tones.push(self.root.transpose(11));
        } else if intervals.contains(&9) {
            tones.push(self.root.transpose(9));
        }

        tones
    }

    pub fn name(&self) -> String {
        if let Some(bass) = self.bass {
            format!("{}{}/{}", self.root, self.quality.symbol(), bass)
        } else {
            format!("{}{}", self.root, self.quality.symbol())
        }
    }
}

impl fmt::Display for Chord {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.name())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_chord_notes() {
        let cmaj7 = Chord::new(Note::C, ChordQuality::Major7);
        let notes = cmaj7.notes();
        assert_eq!(notes, vec![Note::C, Note::E, Note::G, Note::B]);
    }

    #[test]
    fn test_guide_tones() {
        let cmaj7 = Chord::new(Note::C, ChordQuality::Major7);
        let guide_tones = cmaj7.guide_tones();
        assert_eq!(guide_tones, vec![Note::E, Note::B]);
    }
}
