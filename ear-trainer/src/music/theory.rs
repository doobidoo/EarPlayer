use super::chord::{Chord, ChordQuality, Note};
use super::scale::{Scale, ScaleType};

#[derive(Debug, Clone)]
pub struct GuideTone {
    pub note: Note,
    pub from_chord: String,
    pub to_chord: String,
    pub movement: i8,
}

pub struct VoiceLeading;

impl VoiceLeading {
    pub fn analyze(from: &Chord, to: &Chord) -> Vec<GuideTone> {
        let from_guide_tones = from.guide_tones();
        let to_guide_tones = to.guide_tones();
        let mut movements = Vec::new();

        for (i, &from_note) in from_guide_tones.iter().enumerate() {
            if let Some(&to_note) = to_guide_tones.get(i) {
                let movement = (to_note as i8 - from_note as i8 + 12) % 12;
                let movement = if movement > 6 {
                    movement - 12
                } else {
                    movement
                };

                movements.push(GuideTone {
                    note: from_note,
                    from_chord: from.name(),
                    to_chord: to.name(),
                    movement,
                });
            }
        }

        movements
    }

    pub fn smooth_voice_leading(from: &Chord, to: &Chord) -> Vec<(Note, Note, i8)> {
        let from_notes = from.notes();
        let to_notes = to.notes();
        let mut voice_pairs = Vec::new();

        for &from_note in &from_notes {
            let mut best_to = to_notes[0];
            let mut min_distance = 12;

            for &to_note in &to_notes {
                let distance = ((to_note as i8 - from_note as i8).abs() % 12).min(
                    12 - ((to_note as i8 - from_note as i8).abs() % 12)
                );
                if distance < min_distance {
                    min_distance = distance;
                    best_to = to_note;
                }
            }

            let movement = (best_to as i8 - from_note as i8 + 12) % 12;
            let movement = if movement > 6 {
                movement - 12
            } else {
                movement
            };

            voice_pairs.push((from_note, best_to, movement));
        }

        voice_pairs
    }
}

pub struct ChordScaleMatcher;

impl ChordScaleMatcher {
    pub fn get_primary_scale(chord: &Chord) -> Scale {
        let scale_type = match chord.quality {
            ChordQuality::Major7 => ScaleType::Major,
            ChordQuality::Major7sharp11 => ScaleType::Lydian,
            ChordQuality::Minor7 => ScaleType::Dorian,
            ChordQuality::Dominant7 => ScaleType::Mixolydian,
            ChordQuality::Dominant9 => ScaleType::Mixolydian,
            ChordQuality::Dominant7b9 => ScaleType::DiminishedHalfWhole,
            ChordQuality::Dominant7sharp9 => ScaleType::Altered,
            ChordQuality::Dominant7b13 => ScaleType::Altered,
            ChordQuality::Altered => ScaleType::Altered,
            ChordQuality::HalfDiminished => ScaleType::Locrian,
            ChordQuality::Diminished7 => ScaleType::DiminishedWholeHalf,
            ChordQuality::MinorMajor7 => ScaleType::MelodicMinor,
            ChordQuality::MinorMajor9 => ScaleType::MelodicMinor,
            ChordQuality::Major6 => ScaleType::Major,
            ChordQuality::Minor6 => ScaleType::Dorian,
            ChordQuality::Dominant7sus4 => ScaleType::Mixolydian,
            ChordQuality::Major9 => ScaleType::Major,
            ChordQuality::Minor9 => ScaleType::Dorian,
        };

        Scale::new(chord.root, scale_type)
    }

    pub fn get_alternate_scales(chord: &Chord) -> Vec<Scale> {
        let mut scales = vec![Self::get_primary_scale(chord)];

        match chord.quality {
            ChordQuality::Major7 => {
                scales.push(Scale::new(chord.root, ScaleType::Lydian));
            }
            ChordQuality::Minor7 => {
                scales.push(Scale::new(chord.root, ScaleType::Aeolian));
                scales.push(Scale::new(chord.root, ScaleType::Phrygian));
            }
            ChordQuality::Dominant7 | ChordQuality::Dominant9 => {
                scales.push(Scale::new(chord.root, ScaleType::LydianDominant));
                scales.push(Scale::new(chord.root, ScaleType::Altered));
                scales.push(Scale::new(chord.root, ScaleType::WholeTone));
                scales.push(Scale::new(chord.root, ScaleType::DiminishedHalfWhole));
            }
            _ => {}
        }

        scales
    }

    pub fn get_avoid_notes(chord: &Chord, scale: &Scale) -> Vec<Note> {
        let mut avoid_notes = Vec::new();
        let chord_notes = chord.notes();

        match chord.quality {
            ChordQuality::Major7 => {
                let fourth = chord.root.transpose(5);
                if !chord_notes.contains(&fourth) {
                    avoid_notes.push(fourth);
                }
            }
            ChordQuality::Minor7 => {
                let sixth = chord.root.transpose(8);
                if scale.contains(sixth) && !chord_notes.contains(&sixth) {
                    avoid_notes.push(sixth);
                }
            }
            _ => {}
        }

        avoid_notes
    }

    pub fn chord_function(chord: &Chord, key: Note) -> &'static str {
        let interval = (chord.root as i8 - key as i8).rem_euclid(12);

        match interval {
            0 => "Tonic (I)",
            2 => "Supertonic (ii)",
            4 => "Mediant (iii)",
            5 => "Subdominant (IV)",
            7 => "Dominant (V)",
            9 => "Submediant (vi)",
            11 => "Leading Tone (vii)",
            _ => "Unknown",
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_chord_scale_matcher() {
        let cmaj7 = Chord::new(Note::C, ChordQuality::Major7);
        let scale = ChordScaleMatcher::get_primary_scale(&cmaj7);
        assert_eq!(scale.scale_type, ScaleType::Major);
        assert_eq!(scale.root, Note::C);
    }

    #[test]
    fn test_guide_tone_movement() {
        let dm7 = Chord::new(Note::D, ChordQuality::Minor7);
        let g7 = Chord::new(Note::G, ChordQuality::Dominant7);
        let movements = VoiceLeading::analyze(&dm7, &g7);
        assert!(!movements.is_empty());
    }
}
