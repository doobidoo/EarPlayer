//! Conrad Cork's LEGO Joins - Key Transitions
//!
//! Joins are named key transitions that connect bricks across different keys.
//! They use circle of fourths relationships as the backbone for modulation.

use super::chord::Note;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

/// A named key transition pattern
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Join {
    /// Cork's naming convention (e.g., "Sidewinder", "High Jump")
    pub name: String,
    /// Interval of key change in semitones (positive = up)
    pub key_shift: i8,
    /// Description of the harmonic movement
    pub description: String,
    /// Example songs using this join
    pub examples: Vec<String>,
}

impl Join {
    pub fn new(
        name: impl Into<String>,
        key_shift: i8,
        description: impl Into<String>,
    ) -> Self {
        Self {
            name: name.into(),
            key_shift,
            description: description.into(),
            examples: Vec::new(),
        }
    }

    pub fn with_examples(mut self, examples: Vec<&str>) -> Self {
        self.examples = examples.into_iter().map(String::from).collect();
        self
    }

    /// Apply this join to get the new key
    pub fn apply(&self, from_key: Note) -> Note {
        from_key.transpose(self.key_shift)
    }

    /// Get interval description
    pub fn interval_name(&self) -> &'static str {
        match self.key_shift.abs() {
            0 => "Unison",
            1 => "Half step",
            2 => "Whole step",
            3 => "Minor 3rd",
            4 => "Major 3rd",
            5 => "Perfect 4th",
            6 => "Tritone",
            7 => "Perfect 5th",
            8 => "Minor 6th",
            9 => "Major 6th",
            10 => "Minor 7th",
            11 => "Major 7th",
            _ => "Octave+",
        }
    }

    /// Get direction string
    pub fn direction(&self) -> &'static str {
        if self.key_shift > 0 {
            "up"
        } else if self.key_shift < 0 {
            "down"
        } else {
            "same"
        }
    }
}

/// Circle of fourths navigation utilities
pub struct CircleOfFourths;

impl CircleOfFourths {
    /// Order of keys moving clockwise (up in 4ths / down in 5ths)
    pub const CLOCKWISE: [Note; 12] = [
        Note::C,
        Note::F,
        Note::Bb,
        Note::Eb,
        Note::Ab,
        Note::Db,
        Note::Gb,
        Note::B,
        Note::E,
        Note::A,
        Note::D,
        Note::G,
    ];

    /// Get the next key moving up in fourths (clockwise)
    pub fn next_up(from: Note) -> Note {
        from.transpose(5) // Up a 4th = +5 semitones
    }

    /// Get the next key moving down in fourths (counter-clockwise)
    pub fn next_down(from: Note) -> Note {
        from.transpose(-5) // Down a 4th = -5 semitones (same as up a 5th)
    }

    /// Get the position of a key on the circle (0-11)
    pub fn position(key: Note) -> usize {
        Self::CLOCKWISE.iter().position(|&k| k == key).unwrap_or(0)
    }

    /// Get the key at a given position on the circle
    pub fn at_position(pos: usize) -> Note {
        Self::CLOCKWISE[pos % 12]
    }

    /// Get the number of steps between two keys on the circle
    /// Positive = clockwise, negative = counter-clockwise
    pub fn steps_between(from: Note, to: Note) -> i8 {
        let from_pos = Self::position(from) as i8;
        let to_pos = Self::position(to) as i8;
        let diff = (to_pos - from_pos).rem_euclid(12) as i8;
        if diff <= 6 {
            diff
        } else {
            diff - 12
        }
    }

    /// Get all keys in a range around a center key
    pub fn nearby_keys(center: Note, range: usize) -> Vec<Note> {
        let pos = Self::position(center);
        let mut keys = Vec::new();
        for i in 0..=(range * 2) {
            let offset = i as isize - range as isize;
            let new_pos = (pos as isize + offset).rem_euclid(12) as usize;
            keys.push(Self::at_position(new_pos));
        }
        keys
    }
}

/// Library of all Cork joins
#[derive(Debug)]
pub struct JoinLibrary {
    joins: HashMap<String, Join>,
}

impl JoinLibrary {
    pub fn new() -> Self {
        let mut library = Self {
            joins: HashMap::new(),
        };
        library.populate();
        library
    }

    fn add(&mut self, join: Join) {
        self.joins.insert(join.name.clone(), join);
    }

    fn populate(&mut self) {
        // Sidewinder - Half step up
        self.add(
            Join::new(
                "Sidewinder",
                1,
                "Half-step up modulation (C to Db)",
            )
            .with_examples(vec!["Lady Bird", "Joy Spring"]),
        );

        // High Jump - Fourth up
        self.add(
            Join::new(
                "High Jump",
                5,
                "Fourth up modulation - one step clockwise on circle (C to F)",
            )
            .with_examples(vec!["Autumn Leaves", "All The Things You Are"]),
        );

        // Cherokee - Minor third down
        self.add(
            Join::new(
                "Cherokee",
                -3,
                "Minor third down modulation (C to A)",
            )
            .with_examples(vec!["Cherokee", "Body And Soul"]),
        );

        // Giant Steps - Major third up
        self.add(
            Join::new(
                "Giant Steps",
                4,
                "Major third up modulation - Coltrane cycle (C to E)",
            )
            .with_examples(vec!["Giant Steps", "Countdown"]),
        );

        // Stairway - Whole step up
        self.add(
            Join::new(
                "Stairway",
                2,
                "Whole step up modulation (C to D)",
            )
            .with_examples(vec!["How High The Moon", "Ornithology"]),
        );

        // Ladybird - Fourth down (fifth up)
        self.add(
            Join::new(
                "Ladybird",
                -5,
                "Fourth down modulation - one step counter-clockwise (C to G)",
            )
            .with_examples(vec!["Lady Bird", "Blue Bossa"]),
        );

        // Moment's Notice - Minor third up
        self.add(
            Join::new(
                "Moment's Notice",
                3,
                "Minor third up modulation (C to Eb)",
            )
            .with_examples(vec!["Moment's Notice", "Night And Day"]),
        );

        // Back Door - Whole step down
        self.add(
            Join::new(
                "Back Door",
                -2,
                "Whole step down modulation (C to Bb)",
            )
            .with_examples(vec![]),
        );
    }

    /// Get a join by name
    pub fn get(&self, name: &str) -> Option<&Join> {
        self.joins.get(name)
    }

    /// Get all joins
    pub fn all(&self) -> Vec<&Join> {
        self.joins.values().collect()
    }

    /// Get all join names
    pub fn names(&self) -> Vec<&str> {
        self.joins.keys().map(|s| s.as_str()).collect()
    }

    /// Find joins that match a given key shift
    pub fn by_shift(&self, shift: i8) -> Vec<&Join> {
        self.joins
            .values()
            .filter(|j| j.key_shift == shift)
            .collect()
    }

    /// Find a join that takes you from one key to another
    pub fn find_join(&self, from: Note, to: Note) -> Option<&Join> {
        let shift = (to as i8 - from as i8).rem_euclid(12) as i8;
        // Normalize to -6..+6 range
        let normalized = if shift > 6 { shift - 12 } else { shift };
        self.joins.values().find(|j| j.key_shift == normalized)
    }
}

impl Default for JoinLibrary {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_join_apply() {
        let library = JoinLibrary::new();

        // Sidewinder: C -> Db
        let sidewinder = library.get("Sidewinder").unwrap();
        assert_eq!(sidewinder.apply(Note::C), Note::Db);
        assert_eq!(sidewinder.apply(Note::G), Note::Ab);

        // High Jump: C -> F
        let high_jump = library.get("High Jump").unwrap();
        assert_eq!(high_jump.apply(Note::C), Note::F);
        assert_eq!(high_jump.apply(Note::Bb), Note::Eb);

        // Cherokee: C -> A
        let cherokee = library.get("Cherokee").unwrap();
        assert_eq!(cherokee.apply(Note::C), Note::A);
    }

    #[test]
    fn test_circle_of_fourths() {
        // Up in fourths
        assert_eq!(CircleOfFourths::next_up(Note::C), Note::F);
        assert_eq!(CircleOfFourths::next_up(Note::F), Note::Bb);
        assert_eq!(CircleOfFourths::next_up(Note::G), Note::C);

        // Down in fourths (up in fifths)
        assert_eq!(CircleOfFourths::next_down(Note::C), Note::G);
        assert_eq!(CircleOfFourths::next_down(Note::F), Note::C);
    }

    #[test]
    fn test_find_join() {
        let library = JoinLibrary::new();

        // C to Db should find Sidewinder
        let join = library.find_join(Note::C, Note::Db);
        assert!(join.is_some());
        assert_eq!(join.unwrap().name, "Sidewinder");

        // C to F should find High Jump
        let join = library.find_join(Note::C, Note::F);
        assert!(join.is_some());
        assert_eq!(join.unwrap().name, "High Jump");
    }

    #[test]
    fn test_steps_between() {
        assert_eq!(CircleOfFourths::steps_between(Note::C, Note::F), 1);
        assert_eq!(CircleOfFourths::steps_between(Note::C, Note::G), -1);
        assert_eq!(CircleOfFourths::steps_between(Note::C, Note::Bb), 2);
    }
}
