//! Progress persistence for LEGO Bricks training
//!
//! Saves and loads user progress, scores, and preferences.

use chrono::{DateTime, Local, NaiveDate};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::fs;
use std::path::PathBuf;

use crate::music::QuizDifficulty;

/// User progress data that persists between sessions
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct UserProgress {
    /// Total quiz sessions completed
    pub total_sessions: u32,
    /// Total questions answered
    pub total_questions: u32,
    /// Total correct answers
    pub total_correct: u32,
    /// Best streak ever achieved
    pub best_streak_ever: u32,
    /// Current daily streak (consecutive days practiced)
    pub daily_streak: u32,
    /// Last practice date (for tracking daily streaks)
    pub last_practice_date: Option<String>,
    /// Per-brick statistics
    pub brick_stats: HashMap<String, BrickStats>,
    /// Current difficulty preference
    pub difficulty: QuizDifficulty,
    /// Mastered bricks (consistently identified correctly)
    pub mastered_bricks: Vec<String>,
}

/// Statistics for a single brick
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct BrickStats {
    /// Times this brick appeared in quiz
    pub times_seen: u32,
    /// Times correctly identified
    pub times_correct: u32,
    /// Current streak for this brick
    pub current_streak: u32,
    /// Best streak for this brick
    pub best_streak: u32,
    /// Last seen timestamp
    pub last_seen: Option<String>,
}

impl BrickStats {
    /// Calculate accuracy percentage for this brick
    pub fn accuracy(&self) -> f32 {
        if self.times_seen == 0 {
            0.0
        } else {
            (self.times_correct as f32 / self.times_seen as f32) * 100.0
        }
    }

    /// Check if brick is considered "mastered" (80%+ accuracy with 10+ attempts)
    pub fn is_mastered(&self) -> bool {
        self.times_seen >= 10 && self.accuracy() >= 80.0
    }

    /// Record a quiz result for this brick
    pub fn record(&mut self, correct: bool) {
        self.times_seen += 1;
        if correct {
            self.times_correct += 1;
            self.current_streak += 1;
            if self.current_streak > self.best_streak {
                self.best_streak = self.current_streak;
            }
        } else {
            self.current_streak = 0;
        }
        self.last_seen = Some(Local::now().to_rfc3339());
    }
}

impl UserProgress {
    /// Get the default storage path
    pub fn default_path() -> PathBuf {
        dirs::data_local_dir()
            .unwrap_or_else(|| PathBuf::from("."))
            .join("ear-trainer")
            .join("progress.json")
    }

    /// Load progress from disk
    pub fn load() -> Self {
        Self::load_from(Self::default_path())
    }

    /// Load progress from a specific path
    pub fn load_from(path: PathBuf) -> Self {
        if let Ok(contents) = fs::read_to_string(&path) {
            serde_json::from_str(&contents).unwrap_or_default()
        } else {
            Self::default()
        }
    }

    /// Save progress to disk
    pub fn save(&self) -> anyhow::Result<()> {
        self.save_to(Self::default_path())
    }

    /// Save progress to a specific path
    pub fn save_to(&self, path: PathBuf) -> anyhow::Result<()> {
        // Ensure parent directory exists
        if let Some(parent) = path.parent() {
            fs::create_dir_all(parent)?;
        }

        let json = serde_json::to_string_pretty(self)?;
        fs::write(path, json)?;
        Ok(())
    }

    /// Overall accuracy percentage
    pub fn overall_accuracy(&self) -> f32 {
        if self.total_questions == 0 {
            0.0
        } else {
            (self.total_correct as f32 / self.total_questions as f32) * 100.0
        }
    }

    /// Record a quiz session result
    pub fn record_session(&mut self, correct: u32, total: u32, streak: u32) {
        self.total_sessions += 1;
        self.total_questions += total;
        self.total_correct += correct;

        if streak > self.best_streak_ever {
            self.best_streak_ever = streak;
        }

        // Update daily streak
        let today = Local::now().format("%Y-%m-%d").to_string();
        if let Some(last_date) = &self.last_practice_date {
            if last_date == &today {
                // Already practiced today, no change
            } else if is_yesterday(last_date) {
                // Practiced yesterday, increment streak
                self.daily_streak += 1;
            } else {
                // Streak broken, reset to 1
                self.daily_streak = 1;
            }
        } else {
            // First time practicing
            self.daily_streak = 1;
        }
        self.last_practice_date = Some(today);
    }

    /// Record a single brick quiz result
    pub fn record_brick_result(&mut self, brick_name: &str, correct: bool) {
        let stats = self.brick_stats.entry(brick_name.to_string()).or_default();
        stats.record(correct);

        // Check if brick is now mastered
        if stats.is_mastered() && !self.mastered_bricks.contains(&brick_name.to_string()) {
            self.mastered_bricks.push(brick_name.to_string());
        }
    }

    /// Get weak bricks (low accuracy, need more practice)
    pub fn weak_bricks(&self) -> Vec<(&String, &BrickStats)> {
        let mut weak: Vec<_> = self
            .brick_stats
            .iter()
            .filter(|(_, stats)| stats.times_seen >= 3 && stats.accuracy() < 60.0)
            .collect();
        weak.sort_by(|a, b| a.1.accuracy().partial_cmp(&b.1.accuracy()).unwrap());
        weak
    }

    /// Get strong bricks (high accuracy)
    pub fn strong_bricks(&self) -> Vec<(&String, &BrickStats)> {
        let mut strong: Vec<_> = self
            .brick_stats
            .iter()
            .filter(|(_, stats)| stats.times_seen >= 5 && stats.accuracy() >= 80.0)
            .collect();
        strong.sort_by(|a, b| b.1.accuracy().partial_cmp(&a.1.accuracy()).unwrap());
        strong
    }

    /// Get suggested difficulty based on performance
    pub fn suggested_difficulty(&self) -> QuizDifficulty {
        let accuracy = self.overall_accuracy();
        let mastered_count = self.mastered_bricks.len();

        if accuracy >= 85.0 && mastered_count >= 5 {
            QuizDifficulty::Advanced
        } else if accuracy >= 70.0 && mastered_count >= 3 {
            QuizDifficulty::Intermediate
        } else {
            QuizDifficulty::Beginner
        }
    }
}

/// Check if a date string is yesterday
fn is_yesterday(date_str: &str) -> bool {
    if let Ok(date) = NaiveDate::parse_from_str(date_str, "%Y-%m-%d") {
        let today = Local::now().date_naive();
        let yesterday = today.pred_opt().unwrap_or(today);
        date == yesterday
    } else {
        false
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_brick_stats() {
        let mut stats = BrickStats::default();

        stats.record(true);
        assert_eq!(stats.times_seen, 1);
        assert_eq!(stats.times_correct, 1);
        assert_eq!(stats.current_streak, 1);

        stats.record(true);
        assert_eq!(stats.current_streak, 2);
        assert_eq!(stats.best_streak, 2);

        stats.record(false);
        assert_eq!(stats.current_streak, 0);
        assert_eq!(stats.best_streak, 2);

        assert!((stats.accuracy() - 66.67).abs() < 0.1);
    }

    #[test]
    fn test_mastery() {
        let mut stats = BrickStats::default();

        // Not enough attempts
        for _ in 0..8 {
            stats.record(true);
        }
        assert!(!stats.is_mastered());

        // Now has 10 attempts with 100% accuracy
        stats.record(true);
        stats.record(true);
        assert!(stats.is_mastered());
    }

    #[test]
    fn test_progress_accuracy() {
        let mut progress = UserProgress::default();
        progress.total_questions = 20;
        progress.total_correct = 15;

        assert!((progress.overall_accuracy() - 75.0).abs() < 0.01);
    }

    #[test]
    fn test_difficulty_suggestion() {
        let mut progress = UserProgress::default();
        progress.total_questions = 50;
        progress.total_correct = 45; // 90% accuracy

        // Need mastered bricks for advanced
        assert_eq!(progress.suggested_difficulty(), QuizDifficulty::Beginner);

        progress.mastered_bricks = vec![
            "A".to_string(),
            "B".to_string(),
            "C".to_string(),
            "D".to_string(),
            "E".to_string(),
        ];
        assert_eq!(progress.suggested_difficulty(), QuizDifficulty::Advanced);
    }
}
