//! Conrad Cork's LEGO Bricks Jazz Harmony System
//!
//! Bricks are named 4-bar chord patterns that function as reusable harmonic units.
//! They can be transposed to any key and combined with "joins" to create longer progressions.

use super::chord::{Chord, ChordQuality, Note};
use super::progression::{ChordChange, Progression};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

/// Scale degree relative to key center
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum ScaleDegree {
    I,
    bII,
    II,
    bIII,
    III,
    IV,
    bV,
    V,
    bVI,
    VI,
    bVII,
    VII,
}

impl ScaleDegree {
    /// Convert scale degree to actual Note given a key
    pub fn to_note(&self, key: Note) -> Note {
        let semitones = match self {
            ScaleDegree::I => 0,
            ScaleDegree::bII => 1,
            ScaleDegree::II => 2,
            ScaleDegree::bIII => 3,
            ScaleDegree::III => 4,
            ScaleDegree::IV => 5,
            ScaleDegree::bV => 6,
            ScaleDegree::V => 7,
            ScaleDegree::bVI => 8,
            ScaleDegree::VI => 9,
            ScaleDegree::bVII => 10,
            ScaleDegree::VII => 11,
        };
        key.transpose(semitones)
    }

    /// Get Roman numeral representation
    pub fn symbol(&self) -> &'static str {
        match self {
            ScaleDegree::I => "I",
            ScaleDegree::bII => "bII",
            ScaleDegree::II => "II",
            ScaleDegree::bIII => "bIII",
            ScaleDegree::III => "III",
            ScaleDegree::IV => "IV",
            ScaleDegree::bV => "bV",
            ScaleDegree::V => "V",
            ScaleDegree::bVI => "bVI",
            ScaleDegree::VI => "VI",
            ScaleDegree::bVII => "bVII",
            ScaleDegree::VII => "VII",
        }
    }
}

/// Category of brick for organization and difficulty levels
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum BrickCategory {
    /// Resolution patterns (ii-V-I, etc.)
    Cadence,
    /// I-vi-ii-V style patterns
    Turnaround,
    /// Patterns that extend without resolving
    Extension,
    /// Patterns that avoid expected resolution
    Deceptive,
    /// Blues-based patterns
    Blues,
    /// Modal interchange / Coltrane patterns
    Modal,
}

impl BrickCategory {
    pub fn name(&self) -> &'static str {
        match self {
            BrickCategory::Cadence => "Cadence",
            BrickCategory::Turnaround => "Turnaround",
            BrickCategory::Extension => "Extension",
            BrickCategory::Deceptive => "Deceptive",
            BrickCategory::Blues => "Blues",
            BrickCategory::Modal => "Modal",
        }
    }
}

/// A single chord within a brick template, defined by scale degree
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BrickChord {
    /// Scale degree of the chord root
    pub degree: ScaleDegree,
    /// Chord quality
    pub quality: ChordQuality,
    /// Duration in beats
    pub duration: f32,
}

impl BrickChord {
    pub fn new(degree: ScaleDegree, quality: ChordQuality, duration: f32) -> Self {
        Self {
            degree,
            quality,
            duration,
        }
    }

    /// Convert to concrete Chord in a given key
    pub fn to_chord(&self, key: Note) -> Chord {
        let root = self.degree.to_note(key);
        Chord::new(root, self.quality)
    }
}

/// A named 4-bar chord pattern (brick) that can be transposed to any key
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Brick {
    /// Cork's naming convention (e.g., "Launcher", "Sad Cadence")
    pub name: String,
    /// Description for learning
    pub description: String,
    /// Chord changes relative to key (scale degrees)
    pub template: Vec<BrickChord>,
    /// Total duration in beats (typically 16 for 4 bars)
    pub duration_beats: f32,
    /// Category for organization
    pub category: BrickCategory,
    /// Example songs using this brick
    pub examples: Vec<String>,
}

impl Brick {
    pub fn new(
        name: impl Into<String>,
        description: impl Into<String>,
        template: Vec<BrickChord>,
        category: BrickCategory,
    ) -> Self {
        let duration_beats = template.iter().map(|c| c.duration).sum();
        Self {
            name: name.into(),
            description: description.into(),
            template,
            duration_beats,
            category,
            examples: Vec::new(),
        }
    }

    pub fn with_examples(mut self, examples: Vec<&str>) -> Self {
        self.examples = examples.into_iter().map(String::from).collect();
        self
    }

    /// Transpose this brick to a specific key, returning concrete ChordChanges
    pub fn transpose(&self, key: Note) -> Vec<ChordChange> {
        self.template
            .iter()
            .map(|bc| ChordChange {
                chord: bc.to_chord(key),
                duration: bc.duration,
            })
            .collect()
    }

    /// Convert this brick to a playable Progression in the given key
    pub fn to_progression(&self, key: Note, tempo: f32) -> Progression {
        let mut prog = Progression::new(
            format!("{} ({})", self.name, key.name()),
            "LEGO Bricks".to_string(),
            key,
            tempo,
        );

        for bc in &self.template {
            prog.add_chord(bc.to_chord(key), bc.duration);
        }

        prog
    }

    /// Get a short analysis string showing the chord functions
    pub fn analysis(&self) -> String {
        self.template
            .iter()
            .map(|bc| format!("{}{}", bc.degree.symbol(), bc.quality.symbol()))
            .collect::<Vec<_>>()
            .join(" → ")
    }
}

/// Library of all Cork bricks
#[derive(Debug)]
pub struct BrickLibrary {
    bricks: HashMap<String, Brick>,
}

impl BrickLibrary {
    pub fn new() -> Self {
        let mut library = Self {
            bricks: HashMap::new(),
        };
        library.populate_cadences();
        library.populate_turnarounds();
        library.populate_deceptive();
        library.populate_modal();
        library
    }

    fn add(&mut self, brick: Brick) {
        self.bricks.insert(brick.name.clone(), brick);
    }

    fn populate_cadences(&mut self) {
        // Launcher - Classic major ii-V-I
        self.add(
            Brick::new(
                "Launcher",
                "Classic major ii-V-I cadence - the most fundamental jazz progression",
                vec![
                    BrickChord::new(ScaleDegree::II, ChordQuality::Minor7, 4.0),
                    BrickChord::new(ScaleDegree::V, ChordQuality::Dominant7, 4.0),
                    BrickChord::new(ScaleDegree::I, ChordQuality::Major7, 8.0),
                ],
                BrickCategory::Cadence,
            )
            .with_examples(vec!["All The Things You Are", "Autumn Leaves", "Satin Doll"]),
        );

        // Sad Launcher - Minor ii-V-i
        self.add(
            Brick::new(
                "Sad Launcher",
                "Minor ii-V-i cadence with half-diminished ii chord",
                vec![
                    BrickChord::new(ScaleDegree::II, ChordQuality::HalfDiminished, 4.0),
                    BrickChord::new(ScaleDegree::V, ChordQuality::Dominant7b9, 4.0),
                    BrickChord::new(ScaleDegree::I, ChordQuality::Minor7, 8.0),
                ],
                BrickCategory::Cadence,
            )
            .with_examples(vec!["Blue Bossa", "Softly As In A Morning Sunrise"]),
        );

        // Dizzy - Tritone substitution
        self.add(
            Brick::new(
                "Dizzy",
                "ii-V-I with tritone substitution (bII7 for V7)",
                vec![
                    BrickChord::new(ScaleDegree::II, ChordQuality::Minor7, 4.0),
                    BrickChord::new(ScaleDegree::bII, ChordQuality::Dominant7, 4.0),
                    BrickChord::new(ScaleDegree::I, ChordQuality::Major7, 8.0),
                ],
                BrickCategory::Cadence,
            )
            .with_examples(vec!["The Girl From Ipanema", "Lady Bird"]),
        );

        // Overrun - Extended resolution
        self.add(
            Brick::new(
                "Overrun",
                "Extended tonic resolution - dwelling on the I chord",
                vec![
                    BrickChord::new(ScaleDegree::I, ChordQuality::Major7, 4.0),
                    BrickChord::new(ScaleDegree::I, ChordQuality::Major7, 4.0),
                    BrickChord::new(ScaleDegree::I, ChordQuality::Major7, 4.0),
                    BrickChord::new(ScaleDegree::I, ChordQuality::Major7, 4.0),
                ],
                BrickCategory::Extension,
            )
            .with_examples(vec![]),
        );

        // Sad Cadence - ii-V to minor
        self.add(
            Brick::new(
                "Sad Cadence",
                "Secondary dominant resolving to minor chord",
                vec![
                    BrickChord::new(ScaleDegree::VI, ChordQuality::Minor7, 4.0),
                    BrickChord::new(ScaleDegree::II, ChordQuality::Dominant7, 4.0),
                    BrickChord::new(ScaleDegree::V, ChordQuality::Minor7, 8.0),
                ],
                BrickCategory::Cadence,
            )
            .with_examples(vec!["Autumn Leaves (B section)"]),
        );

        // Pennies - Minor to major pivot
        self.add(
            Brick::new(
                "Pennies",
                "Minor ii-V resolving to major ii-V (pivot modulation)",
                vec![
                    BrickChord::new(ScaleDegree::III, ChordQuality::HalfDiminished, 4.0),
                    BrickChord::new(ScaleDegree::VI, ChordQuality::Dominant7b9, 4.0),
                    BrickChord::new(ScaleDegree::II, ChordQuality::Minor7, 4.0),
                    BrickChord::new(ScaleDegree::V, ChordQuality::Dominant7, 4.0),
                ],
                BrickCategory::Cadence,
            )
            .with_examples(vec!["Pennies From Heaven"]),
        );

        // Raindrop - Chromatic descent
        self.add(
            Brick::new(
                "Raindrop",
                "Chromatic descending bass line with modal interchange",
                vec![
                    BrickChord::new(ScaleDegree::IV, ChordQuality::Major7, 4.0),
                    BrickChord::new(ScaleDegree::IV, ChordQuality::Minor7, 4.0),
                    BrickChord::new(ScaleDegree::III, ChordQuality::Minor7, 4.0),
                    BrickChord::new(ScaleDegree::bIII, ChordQuality::Dominant7, 4.0),
                ],
                BrickCategory::Cadence,
            )
            .with_examples(vec!["My Funny Valentine", "In A Sentimental Mood"]),
        );
    }

    fn populate_turnarounds(&mut self) {
        // Long Approach - iii-VI-ii-V
        self.add(
            Brick::new(
                "Long Approach",
                "Extended approach: iii-VI-ii-V turnaround",
                vec![
                    BrickChord::new(ScaleDegree::III, ChordQuality::Minor7, 4.0),
                    BrickChord::new(ScaleDegree::VI, ChordQuality::Dominant7, 4.0),
                    BrickChord::new(ScaleDegree::II, ChordQuality::Minor7, 4.0),
                    BrickChord::new(ScaleDegree::V, ChordQuality::Dominant7, 4.0),
                ],
                BrickCategory::Turnaround,
            )
            .with_examples(vec!["Have You Met Miss Jones", "I Got Rhythm"]),
        );

        // Honeysuckle - IV minor turnaround
        self.add(
            Brick::new(
                "Honeysuckle",
                "ii-V to IV with minor IV (backdoor progression)",
                vec![
                    BrickChord::new(ScaleDegree::V, ChordQuality::Minor7, 4.0),
                    BrickChord::new(ScaleDegree::I, ChordQuality::Dominant7, 4.0),
                    BrickChord::new(ScaleDegree::IV, ChordQuality::Major7, 4.0),
                    BrickChord::new(ScaleDegree::IV, ChordQuality::Minor7, 4.0),
                ],
                BrickCategory::Turnaround,
            )
            .with_examples(vec!["Honeysuckle Rose", "Stompin' At The Savoy"]),
        );

        // Rhythm - Basic rhythm changes turnaround
        self.add(
            Brick::new(
                "Rhythm",
                "Classic rhythm changes turnaround: I-vi-ii-V",
                vec![
                    BrickChord::new(ScaleDegree::I, ChordQuality::Major7, 4.0),
                    BrickChord::new(ScaleDegree::VI, ChordQuality::Minor7, 4.0),
                    BrickChord::new(ScaleDegree::II, ChordQuality::Minor7, 4.0),
                    BrickChord::new(ScaleDegree::V, ChordQuality::Dominant7, 4.0),
                ],
                BrickCategory::Turnaround,
            )
            .with_examples(vec!["I Got Rhythm", "Oleo", "Anthropology"]),
        );
    }

    fn populate_deceptive(&mut self) {
        // Nowhere - Deceptive to iii-VI
        self.add(
            Brick::new(
                "Nowhere",
                "Deceptive resolution: ii-V resolves to iii-VI instead of I",
                vec![
                    BrickChord::new(ScaleDegree::II, ChordQuality::Minor7, 4.0),
                    BrickChord::new(ScaleDegree::V, ChordQuality::Dominant7, 4.0),
                    BrickChord::new(ScaleDegree::III, ChordQuality::Minor7, 4.0),
                    BrickChord::new(ScaleDegree::VI, ChordQuality::Dominant7, 4.0),
                ],
                BrickCategory::Deceptive,
            )
            .with_examples(vec!["Out Of Nowhere", "Satin Doll (bridge)"]),
        );

        // Surprise - To flat VI
        self.add(
            Brick::new(
                "Surprise",
                "Deceptive cadence to bVI major",
                vec![
                    BrickChord::new(ScaleDegree::II, ChordQuality::Minor7, 4.0),
                    BrickChord::new(ScaleDegree::V, ChordQuality::Dominant7, 4.0),
                    BrickChord::new(ScaleDegree::bVI, ChordQuality::Major7, 8.0),
                ],
                BrickCategory::Deceptive,
            )
            .with_examples(vec![]),
        );
    }

    fn populate_modal(&mut self) {
        // Starlight - Giant Steps pattern
        self.add(
            Brick::new(
                "Starlight",
                "Major third cycle (Giant Steps pattern)",
                vec![
                    BrickChord::new(ScaleDegree::I, ChordQuality::Major7, 4.0),
                    BrickChord::new(ScaleDegree::bIII, ChordQuality::Major7, 4.0),
                    BrickChord::new(ScaleDegree::bVI, ChordQuality::Major7, 4.0),
                    BrickChord::new(ScaleDegree::bII, ChordQuality::Major7, 4.0),
                ],
                BrickCategory::Modal,
            )
            .with_examples(vec!["Giant Steps", "Central Park West"]),
        );

        // Countdown - Coltrane changes with dominants
        self.add(
            Brick::new(
                "Countdown",
                "Coltrane changes: major third cycle with V7 approaches",
                vec![
                    BrickChord::new(ScaleDegree::I, ChordQuality::Major7, 2.0),
                    BrickChord::new(ScaleDegree::bIII, ChordQuality::Dominant7, 2.0),
                    BrickChord::new(ScaleDegree::bVI, ChordQuality::Major7, 2.0),
                    BrickChord::new(ScaleDegree::VII, ChordQuality::Dominant7, 2.0),
                    BrickChord::new(ScaleDegree::bV, ChordQuality::Major7, 2.0),
                    BrickChord::new(ScaleDegree::V, ChordQuality::Dominant7, 2.0),
                    BrickChord::new(ScaleDegree::I, ChordQuality::Major7, 4.0),
                ],
                BrickCategory::Modal,
            )
            .with_examples(vec!["Countdown", "Giant Steps"]),
        );

        // So What - Modal vamp
        self.add(
            Brick::new(
                "So What",
                "Modal vamp - Dorian minor 7th chord",
                vec![
                    BrickChord::new(ScaleDegree::I, ChordQuality::Minor7, 8.0),
                    BrickChord::new(ScaleDegree::I, ChordQuality::Minor7, 8.0),
                ],
                BrickCategory::Modal,
            )
            .with_examples(vec!["So What", "Impressions"]),
        );
    }

    /// Get a brick by name
    pub fn get(&self, name: &str) -> Option<&Brick> {
        self.bricks.get(name)
    }

    /// Get all bricks
    pub fn all(&self) -> Vec<&Brick> {
        self.bricks.values().collect()
    }

    /// Get bricks by category
    pub fn by_category(&self, category: BrickCategory) -> Vec<&Brick> {
        self.bricks
            .values()
            .filter(|b| b.category == category)
            .collect()
    }

    /// Get brick names suitable for a difficulty level
    pub fn for_difficulty(&self, difficulty: QuizDifficulty) -> Vec<&Brick> {
        match difficulty {
            QuizDifficulty::Beginner => {
                // Just basic ii-V-I patterns
                vec!["Launcher", "Sad Launcher", "Overrun"]
                    .into_iter()
                    .filter_map(|name| self.get(name))
                    .collect()
            }
            QuizDifficulty::Intermediate => {
                // Add turnarounds and tritone subs
                vec![
                    "Launcher",
                    "Sad Launcher",
                    "Overrun",
                    "Dizzy",
                    "Long Approach",
                    "Rhythm",
                    "Honeysuckle",
                    "Sad Cadence",
                ]
                .into_iter()
                .filter_map(|name| self.get(name))
                .collect()
            }
            QuizDifficulty::Advanced => {
                // All bricks
                self.all()
            }
        }
    }

    /// Get all brick names
    pub fn names(&self) -> Vec<&str> {
        self.bricks.keys().map(|s| s.as_str()).collect()
    }
}

impl Default for BrickLibrary {
    fn default() -> Self {
        Self::new()
    }
}

/// Quiz difficulty levels for brick identification
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, Default)]
pub enum QuizDifficulty {
    /// Basic ii-V-I patterns only (Launcher, Sad Launcher)
    #[default]
    Beginner,
    /// Add turnarounds, tritone subs
    Intermediate,
    /// All bricks including modal/deceptive
    Advanced,
}

impl QuizDifficulty {
    pub fn name(&self) -> &'static str {
        match self {
            QuizDifficulty::Beginner => "Beginner",
            QuizDifficulty::Intermediate => "Intermediate",
            QuizDifficulty::Advanced => "Advanced",
        }
    }

    pub fn next(self) -> Self {
        match self {
            QuizDifficulty::Beginner => QuizDifficulty::Intermediate,
            QuizDifficulty::Intermediate => QuizDifficulty::Advanced,
            QuizDifficulty::Advanced => QuizDifficulty::Beginner,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_scale_degree_transpose() {
        assert_eq!(ScaleDegree::I.to_note(Note::C), Note::C);
        assert_eq!(ScaleDegree::II.to_note(Note::C), Note::D);
        assert_eq!(ScaleDegree::V.to_note(Note::C), Note::G);
        assert_eq!(ScaleDegree::bII.to_note(Note::C), Note::Db);

        // Test in different keys
        assert_eq!(ScaleDegree::II.to_note(Note::F), Note::G);
        assert_eq!(ScaleDegree::V.to_note(Note::Bb), Note::F);
    }

    #[test]
    fn test_brick_transpose() {
        let library = BrickLibrary::new();
        let launcher = library.get("Launcher").unwrap();

        // In C: Dm7 - G7 - Cmaj7
        let c_changes = launcher.transpose(Note::C);
        assert_eq!(c_changes[0].chord.root, Note::D);
        assert_eq!(c_changes[1].chord.root, Note::G);
        assert_eq!(c_changes[2].chord.root, Note::C);

        // In F: Gm7 - C7 - Fmaj7
        let f_changes = launcher.transpose(Note::F);
        assert_eq!(f_changes[0].chord.root, Note::G);
        assert_eq!(f_changes[1].chord.root, Note::C);
        assert_eq!(f_changes[2].chord.root, Note::F);
    }

    #[test]
    fn test_brick_library() {
        let library = BrickLibrary::new();

        // Should have all the standard bricks
        assert!(library.get("Launcher").is_some());
        assert!(library.get("Sad Launcher").is_some());
        assert!(library.get("Dizzy").is_some());
        assert!(library.get("Nowhere").is_some());

        // Test difficulty filtering
        let beginner = library.for_difficulty(QuizDifficulty::Beginner);
        assert!(beginner.len() <= 3);

        let advanced = library.for_difficulty(QuizDifficulty::Advanced);
        assert!(advanced.len() > beginner.len());
    }
}
