//! Jazz Standard Song Breakdowns
//!
//! Analyzes jazz standards in terms of LEGO Bricks patterns.
//! Each standard is broken down into its constituent bricks and joins.

use super::bricks::ScaleDegree;
use super::chord::Note;
use serde::{Deserialize, Serialize};

/// A section of a jazz standard (A section, B section, etc.)
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StandardSection {
    /// Section label (A, B, C, etc.)
    pub label: String,
    /// Bars in this section
    pub bars: u8,
    /// Brick references that make up this section
    pub bricks: Vec<BrickRef>,
    /// Key center for this section
    pub key: Note,
}

/// Reference to a brick within a standard
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BrickRef {
    /// Name of the brick being used
    pub brick_name: String,
    /// Starting bar (1-indexed)
    pub start_bar: u8,
    /// Duration in bars
    pub duration_bars: u8,
    /// Key this brick is played in (may differ from section key)
    pub key: Note,
    /// Optional notes about this usage
    pub notes: Option<String>,
}

/// A complete jazz standard breakdown
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Standard {
    /// Name of the standard
    pub name: String,
    /// Composer
    pub composer: String,
    /// Year composed (if known)
    pub year: Option<u16>,
    /// Total bars in the form
    pub total_bars: u8,
    /// Form description (e.g., "AABA", "ABAC")
    pub form: String,
    /// Original key
    pub key: Note,
    /// Tempo range (min, max)
    pub tempo_range: (u16, u16),
    /// Style notes
    pub style: String,
    /// Section breakdowns
    pub sections: Vec<StandardSection>,
    /// Overall brick usage summary
    pub brick_summary: Vec<String>,
    /// Join patterns used
    pub joins_used: Vec<String>,
}

impl Standard {
    /// Get all unique bricks used in this standard
    pub fn unique_bricks(&self) -> Vec<&str> {
        let mut bricks: Vec<&str> = self
            .sections
            .iter()
            .flat_map(|s| s.bricks.iter().map(|b| b.brick_name.as_str()))
            .collect();
        bricks.sort();
        bricks.dedup();
        bricks
    }

    /// Count how many times each brick appears
    pub fn brick_frequency(&self) -> Vec<(&str, usize)> {
        let mut freq: std::collections::HashMap<&str, usize> = std::collections::HashMap::new();
        for section in &self.sections {
            for brick in &section.bricks {
                *freq.entry(&brick.brick_name).or_insert(0) += 1;
            }
        }
        let mut result: Vec<_> = freq.into_iter().collect();
        result.sort_by(|a, b| b.1.cmp(&a.1));
        result
    }
}

/// Library of jazz standard breakdowns
#[derive(Debug)]
pub struct StandardsLibrary {
    standards: Vec<Standard>,
}

impl StandardsLibrary {
    pub fn new() -> Self {
        let mut library = Self {
            standards: Vec::new(),
        };
        library.populate();
        library
    }

    fn populate(&mut self) {
        // Autumn Leaves - Classic ii-V-I study piece
        self.standards.push(Standard {
            name: "Autumn Leaves".to_string(),
            composer: "Joseph Kosma".to_string(),
            year: Some(1945),
            total_bars: 32,
            form: "AABA".to_string(),
            key: Note::G, // Minor, but we use relative major
            tempo_range: (100, 180),
            style: "Medium swing".to_string(),
            sections: vec![
                StandardSection {
                    label: "A1".to_string(),
                    bars: 8,
                    key: Note::G,
                    bricks: vec![
                        BrickRef {
                            brick_name: "Launcher".to_string(),
                            start_bar: 1,
                            duration_bars: 4,
                            key: Note::Bb,
                            notes: Some("Relative major ii-V-I".to_string()),
                        },
                        BrickRef {
                            brick_name: "Sad Launcher".to_string(),
                            start_bar: 5,
                            duration_bars: 4,
                            key: Note::G,
                            notes: Some("Minor ii-V-i to tonic".to_string()),
                        },
                    ],
                },
                StandardSection {
                    label: "A2".to_string(),
                    bars: 8,
                    key: Note::G,
                    bricks: vec![
                        BrickRef {
                            brick_name: "Launcher".to_string(),
                            start_bar: 9,
                            duration_bars: 4,
                            key: Note::Bb,
                            notes: None,
                        },
                        BrickRef {
                            brick_name: "Sad Launcher".to_string(),
                            start_bar: 13,
                            duration_bars: 4,
                            key: Note::G,
                            notes: None,
                        },
                    ],
                },
                StandardSection {
                    label: "B".to_string(),
                    bars: 8,
                    key: Note::Bb,
                    bricks: vec![
                        BrickRef {
                            brick_name: "Long Approach".to_string(),
                            start_bar: 17,
                            duration_bars: 8,
                            key: Note::Bb,
                            notes: Some("Extended approach to Bb".to_string()),
                        },
                    ],
                },
                StandardSection {
                    label: "A3".to_string(),
                    bars: 8,
                    key: Note::G,
                    bricks: vec![
                        BrickRef {
                            brick_name: "Launcher".to_string(),
                            start_bar: 25,
                            duration_bars: 4,
                            key: Note::Bb,
                            notes: None,
                        },
                        BrickRef {
                            brick_name: "Sad Launcher".to_string(),
                            start_bar: 29,
                            duration_bars: 4,
                            key: Note::G,
                            notes: Some("Final resolution".to_string()),
                        },
                    ],
                },
            ],
            brick_summary: vec![
                "Launcher x3".to_string(),
                "Sad Launcher x3".to_string(),
                "Long Approach x1".to_string(),
            ],
            joins_used: vec!["High Jump (Bb -> G minor)".to_string()],
        });

        // All The Things You Are - Modulation showcase
        self.standards.push(Standard {
            name: "All The Things You Are".to_string(),
            composer: "Jerome Kern".to_string(),
            year: Some(1939),
            total_bars: 36,
            form: "AABA".to_string(),
            key: Note::Ab,
            tempo_range: (120, 200),
            style: "Medium up swing".to_string(),
            sections: vec![
                StandardSection {
                    label: "A1".to_string(),
                    bars: 8,
                    key: Note::Ab,
                    bricks: vec![
                        BrickRef {
                            brick_name: "Launcher".to_string(),
                            start_bar: 1,
                            duration_bars: 4,
                            key: Note::Ab,
                            notes: Some("Opens in Ab".to_string()),
                        },
                        BrickRef {
                            brick_name: "Launcher".to_string(),
                            start_bar: 5,
                            duration_bars: 4,
                            key: Note::C,
                            notes: Some("Modulates to C".to_string()),
                        },
                    ],
                },
                StandardSection {
                    label: "A2".to_string(),
                    bars: 8,
                    key: Note::Eb,
                    bricks: vec![
                        BrickRef {
                            brick_name: "Launcher".to_string(),
                            start_bar: 9,
                            duration_bars: 4,
                            key: Note::Eb,
                            notes: Some("Modulates to Eb".to_string()),
                        },
                        BrickRef {
                            brick_name: "Launcher".to_string(),
                            start_bar: 13,
                            duration_bars: 4,
                            key: Note::G,
                            notes: Some("Sets up bridge".to_string()),
                        },
                    ],
                },
                StandardSection {
                    label: "B".to_string(),
                    bars: 8,
                    key: Note::E,
                    bricks: vec![
                        BrickRef {
                            brick_name: "Launcher".to_string(),
                            start_bar: 17,
                            duration_bars: 4,
                            key: Note::E,
                            notes: Some("Distant key center".to_string()),
                        },
                        BrickRef {
                            brick_name: "Launcher".to_string(),
                            start_bar: 21,
                            duration_bars: 4,
                            key: Note::Ab,
                            notes: Some("Returns to tonic".to_string()),
                        },
                    ],
                },
                StandardSection {
                    label: "A3".to_string(),
                    bars: 12,
                    key: Note::Ab,
                    bricks: vec![
                        BrickRef {
                            brick_name: "Launcher".to_string(),
                            start_bar: 25,
                            duration_bars: 4,
                            key: Note::Ab,
                            notes: None,
                        },
                        BrickRef {
                            brick_name: "Dizzy".to_string(),
                            start_bar: 29,
                            duration_bars: 4,
                            key: Note::Ab,
                            notes: Some("Tritone sub turnaround".to_string()),
                        },
                        BrickRef {
                            brick_name: "Overrun".to_string(),
                            start_bar: 33,
                            duration_bars: 4,
                            key: Note::Ab,
                            notes: Some("Extended ending".to_string()),
                        },
                    ],
                },
            ],
            brick_summary: vec![
                "Launcher x7".to_string(),
                "Dizzy x1".to_string(),
                "Overrun x1".to_string(),
            ],
            joins_used: vec![
                "High Jump (Ab -> C)".to_string(),
                "Cherokee (C -> Eb)".to_string(),
                "High Jump (Eb -> G)".to_string(),
                "Ladybird (G -> E)".to_string(),
                "Cherokee (E -> Ab)".to_string(),
            ],
        });

        // Blue Bossa - Latin jazz standard
        self.standards.push(Standard {
            name: "Blue Bossa".to_string(),
            composer: "Kenny Dorham".to_string(),
            year: Some(1963),
            total_bars: 16,
            form: "AB".to_string(),
            key: Note::C, // C minor
            tempo_range: (120, 160),
            style: "Bossa nova".to_string(),
            sections: vec![
                StandardSection {
                    label: "A".to_string(),
                    bars: 8,
                    key: Note::C,
                    bricks: vec![
                        BrickRef {
                            brick_name: "Sad Launcher".to_string(),
                            start_bar: 1,
                            duration_bars: 8,
                            key: Note::C,
                            notes: Some("Minor ii-V-i in C minor".to_string()),
                        },
                    ],
                },
                StandardSection {
                    label: "B".to_string(),
                    bars: 8,
                    key: Note::Db,
                    bricks: vec![
                        BrickRef {
                            brick_name: "Launcher".to_string(),
                            start_bar: 9,
                            duration_bars: 4,
                            key: Note::Db,
                            notes: Some("Brief modulation to Db".to_string()),
                        },
                        BrickRef {
                            brick_name: "Sad Launcher".to_string(),
                            start_bar: 13,
                            duration_bars: 4,
                            key: Note::C,
                            notes: Some("Return to C minor".to_string()),
                        },
                    ],
                },
            ],
            brick_summary: vec![
                "Sad Launcher x2".to_string(),
                "Launcher x1".to_string(),
            ],
            joins_used: vec![
                "Sidewinder (C -> Db)".to_string(),
                "Sidewinder (Db -> C)".to_string(),
            ],
        });

        // Rhythm Changes - Essential standard form
        self.standards.push(Standard {
            name: "Rhythm Changes (I Got Rhythm)".to_string(),
            composer: "George Gershwin".to_string(),
            year: Some(1930),
            total_bars: 32,
            form: "AABA".to_string(),
            key: Note::Bb,
            tempo_range: (160, 320),
            style: "Up-tempo swing".to_string(),
            sections: vec![
                StandardSection {
                    label: "A1".to_string(),
                    bars: 8,
                    key: Note::Bb,
                    bricks: vec![
                        BrickRef {
                            brick_name: "Rhythm".to_string(),
                            start_bar: 1,
                            duration_bars: 8,
                            key: Note::Bb,
                            notes: Some("I-vi-ii-V turnaround pattern".to_string()),
                        },
                    ],
                },
                StandardSection {
                    label: "A2".to_string(),
                    bars: 8,
                    key: Note::Bb,
                    bricks: vec![
                        BrickRef {
                            brick_name: "Rhythm".to_string(),
                            start_bar: 9,
                            duration_bars: 8,
                            key: Note::Bb,
                            notes: None,
                        },
                    ],
                },
                StandardSection {
                    label: "B".to_string(),
                    bars: 8,
                    key: Note::Bb,
                    bricks: vec![
                        BrickRef {
                            brick_name: "Long Approach".to_string(),
                            start_bar: 17,
                            duration_bars: 8,
                            key: Note::Bb,
                            notes: Some("III7-VI7-II7-V7 sequence".to_string()),
                        },
                    ],
                },
                StandardSection {
                    label: "A3".to_string(),
                    bars: 8,
                    key: Note::Bb,
                    bricks: vec![
                        BrickRef {
                            brick_name: "Rhythm".to_string(),
                            start_bar: 25,
                            duration_bars: 8,
                            key: Note::Bb,
                            notes: Some("Final A section".to_string()),
                        },
                    ],
                },
            ],
            brick_summary: vec![
                "Rhythm x3".to_string(),
                "Long Approach x1".to_string(),
            ],
            joins_used: vec![],
        });
    }

    /// Get all standards
    pub fn all(&self) -> &[Standard] {
        &self.standards
    }

    /// Get a standard by name
    pub fn get(&self, name: &str) -> Option<&Standard> {
        self.standards.iter().find(|s| s.name == name)
    }

    /// Get standards that use a specific brick
    pub fn using_brick(&self, brick_name: &str) -> Vec<&Standard> {
        self.standards
            .iter()
            .filter(|s| {
                s.sections
                    .iter()
                    .any(|sec| sec.bricks.iter().any(|b| b.brick_name == brick_name))
            })
            .collect()
    }

    /// Get standards that use a specific join
    pub fn using_join(&self, join_name: &str) -> Vec<&Standard> {
        self.standards
            .iter()
            .filter(|s| s.joins_used.iter().any(|j| j.contains(join_name)))
            .collect()
    }

    /// Get standards by difficulty (based on number of modulations)
    pub fn by_difficulty(&self, level: StandardDifficulty) -> Vec<&Standard> {
        self.standards
            .iter()
            .filter(|s| self.assess_difficulty(s) == level)
            .collect()
    }

    fn assess_difficulty(&self, standard: &Standard) -> StandardDifficulty {
        let unique_keys: std::collections::HashSet<_> =
            standard.sections.iter().map(|s| s.key).collect();
        let modulations = unique_keys.len();
        let join_count = standard.joins_used.len();

        if modulations <= 2 && join_count <= 2 {
            StandardDifficulty::Beginner
        } else if modulations <= 4 && join_count <= 5 {
            StandardDifficulty::Intermediate
        } else {
            StandardDifficulty::Advanced
        }
    }
}

impl Default for StandardsLibrary {
    fn default() -> Self {
        Self::new()
    }
}

/// Difficulty level for standards
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum StandardDifficulty {
    Beginner,
    Intermediate,
    Advanced,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_standards_library() {
        let library = StandardsLibrary::new();
        assert!(!library.all().is_empty());
    }

    #[test]
    fn test_get_standard() {
        let library = StandardsLibrary::new();
        let autumn = library.get("Autumn Leaves");
        assert!(autumn.is_some());
        assert_eq!(autumn.unwrap().total_bars, 32);
    }

    #[test]
    fn test_using_brick() {
        let library = StandardsLibrary::new();
        let launcher_standards = library.using_brick("Launcher");
        assert!(!launcher_standards.is_empty());
    }

    #[test]
    fn test_brick_frequency() {
        let library = StandardsLibrary::new();
        let all_things = library.get("All The Things You Are").unwrap();
        let freq = all_things.brick_frequency();
        // Launcher should be most common
        assert_eq!(freq[0].0, "Launcher");
    }
}
