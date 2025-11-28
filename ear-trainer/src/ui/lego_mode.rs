//! LEGO Bricks Training Mode
//!
//! Quiz system for identifying brick patterns by ear.

use crate::music::{Brick, BrickLibrary, JoinLibrary, Note, QuizDifficulty};
use rand::seq::SliceRandom;
use rand::Rng;

/// State for the LEGO training mode
#[derive(Debug)]
pub struct LegoModeState {
    pub brick_library: BrickLibrary,
    pub join_library: JoinLibrary,
    pub current_quiz: Option<BrickQuiz>,
    pub session_score: QuizScore,
    pub difficulty: QuizDifficulty,
    /// Current brick being played in Listen mode
    pub current_brick_name: Option<String>,
    pub current_key: Note,
}

/// A brick identification quiz question
#[derive(Debug, Clone)]
pub struct BrickQuiz {
    /// The brick being played
    pub target_brick: Brick,
    /// Key it's being played in
    pub target_key: Note,
    /// Multiple choice options (brick names)
    pub options: Vec<String>,
    /// Index of the correct answer in options
    pub correct_idx: usize,
    /// User's answer (None if not answered yet)
    pub user_answer: Option<usize>,
    /// Whether the answer has been revealed
    pub revealed: bool,
}

impl BrickQuiz {
    /// Check if the user's answer is correct
    pub fn is_correct(&self) -> bool {
        self.user_answer == Some(self.correct_idx)
    }

    /// Get the correct answer name
    pub fn correct_answer(&self) -> &str {
        &self.options[self.correct_idx]
    }
}

/// Score tracking for the current session
#[derive(Debug, Clone, Default)]
pub struct QuizScore {
    pub correct: u32,
    pub total: u32,
    pub streak: u32,
    pub best_streak: u32,
}

impl QuizScore {
    pub fn accuracy(&self) -> f32 {
        if self.total == 0 {
            0.0
        } else {
            (self.correct as f32 / self.total as f32) * 100.0
        }
    }

    pub fn record(&mut self, correct: bool) {
        self.total += 1;
        if correct {
            self.correct += 1;
            self.streak += 1;
            if self.streak > self.best_streak {
                self.best_streak = self.streak;
            }
        } else {
            self.streak = 0;
        }
    }

    pub fn reset(&mut self) {
        *self = Self::default();
    }
}

impl LegoModeState {
    pub fn new() -> Self {
        Self {
            brick_library: BrickLibrary::new(),
            join_library: JoinLibrary::new(),
            current_quiz: None,
            session_score: QuizScore::default(),
            difficulty: QuizDifficulty::Beginner,
            current_brick_name: None,
            current_key: Note::C,
        }
    }

    /// Generate a new quiz question
    pub fn generate_quiz(&mut self) {
        let mut rng = rand::thread_rng();

        // Get bricks for current difficulty
        let available_bricks = self.brick_library.for_difficulty(self.difficulty);
        if available_bricks.is_empty() {
            return;
        }

        // Pick a random brick
        let target = (*available_bricks.choose(&mut rng).unwrap()).clone();

        // Pick a random key from common jazz keys
        let common_keys = [Note::C, Note::F, Note::Bb, Note::Eb, Note::G];
        let target_key = *common_keys.choose(&mut rng).unwrap();

        // Generate 3 wrong options + correct
        let mut wrong_options: Vec<String> = available_bricks
            .iter()
            .filter(|b| b.name != target.name)
            .map(|b| b.name.clone())
            .collect();
        wrong_options.shuffle(&mut rng);
        wrong_options.truncate(3);

        // Insert correct answer at random position
        let correct_idx = rng.gen_range(0..=wrong_options.len().min(3));
        let mut options = wrong_options;
        options.insert(correct_idx, target.name.clone());

        // Ensure we have exactly 4 options (pad with more wrong answers if needed)
        while options.len() < 4 {
            let all_names = self.brick_library.names();
            if let Some(&name) = all_names.choose(&mut rng) {
                if !options.contains(&name.to_string()) {
                    options.push(name.to_string());
                }
            }
        }
        options.truncate(4);

        self.current_quiz = Some(BrickQuiz {
            target_brick: target.clone(),
            target_key,
            options,
            correct_idx,
            user_answer: None,
            revealed: false,
        });
    }

    /// Submit an answer for the current quiz
    pub fn submit_answer(&mut self, answer_idx: usize) {
        if let Some(quiz) = &mut self.current_quiz {
            if quiz.revealed {
                return; // Already answered
            }

            quiz.user_answer = Some(answer_idx);
            quiz.revealed = true;

            self.session_score.record(answer_idx == quiz.correct_idx);
        }
    }

    /// Move to the next question
    pub fn next_question(&mut self) {
        self.generate_quiz();
    }

    /// Check if there's an active unanswered quiz
    pub fn has_active_quiz(&self) -> bool {
        self.current_quiz.as_ref().is_some_and(|q| !q.revealed)
    }

    /// Check if waiting for user to proceed to next question
    pub fn waiting_for_next(&self) -> bool {
        self.current_quiz.as_ref().is_some_and(|q| q.revealed)
    }

    /// Cycle to the next difficulty level
    pub fn cycle_difficulty(&mut self) {
        self.difficulty = self.difficulty.next();
        // Reset quiz on difficulty change
        self.current_quiz = None;
    }

    /// Set a specific brick for listen mode
    pub fn set_current_brick(&mut self, name: &str, key: Note) {
        self.current_brick_name = Some(name.to_string());
        self.current_key = key;
    }

    /// Get the current brick in listen mode
    pub fn get_current_brick(&self) -> Option<&Brick> {
        self.current_brick_name
            .as_ref()
            .and_then(|name| self.brick_library.get(name))
    }

    /// Get progression for current quiz (for playback)
    pub fn quiz_progression(&self) -> Option<crate::music::Progression> {
        self.current_quiz.as_ref().map(|quiz| {
            quiz.target_brick.to_progression(quiz.target_key, 140.0)
        })
    }

    /// Get progression for listen mode (for playback)
    pub fn listen_progression(&self) -> Option<crate::music::Progression> {
        self.get_current_brick()
            .map(|brick| brick.to_progression(self.current_key, 140.0))
    }

    /// Cycle to next brick in listen mode
    pub fn next_brick(&mut self) {
        let bricks = self.brick_library.for_difficulty(self.difficulty);
        if bricks.is_empty() {
            return;
        }

        let current_idx = self
            .current_brick_name
            .as_ref()
            .and_then(|name| bricks.iter().position(|b| b.name == *name))
            .unwrap_or(0);

        let next_idx = (current_idx + 1) % bricks.len();
        self.current_brick_name = Some(bricks[next_idx].name.clone());
    }

    /// Cycle to previous brick in listen mode
    pub fn prev_brick(&mut self) {
        let bricks = self.brick_library.for_difficulty(self.difficulty);
        if bricks.is_empty() {
            return;
        }

        let current_idx = self
            .current_brick_name
            .as_ref()
            .and_then(|name| bricks.iter().position(|b| b.name == *name))
            .unwrap_or(0);

        let prev_idx = if current_idx == 0 {
            bricks.len() - 1
        } else {
            current_idx - 1
        };
        self.current_brick_name = Some(bricks[prev_idx].name.clone());
    }

    /// Cycle to next key
    pub fn next_key(&mut self) {
        self.current_key = self.current_key.transpose(1);
    }

    /// Cycle to previous key
    pub fn prev_key(&mut self) {
        self.current_key = self.current_key.transpose(-1);
    }
}

impl Default for LegoModeState {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_quiz_generation() {
        let mut state = LegoModeState::new();
        state.generate_quiz();

        let quiz = state.current_quiz.as_ref().unwrap();
        assert_eq!(quiz.options.len(), 4);
        assert!(quiz.options.contains(&quiz.target_brick.name));
        assert!(!quiz.revealed);
        assert!(quiz.user_answer.is_none());
    }

    #[test]
    fn test_quiz_scoring() {
        let mut score = QuizScore::default();

        score.record(true);
        assert_eq!(score.correct, 1);
        assert_eq!(score.streak, 1);

        score.record(true);
        assert_eq!(score.correct, 2);
        assert_eq!(score.streak, 2);
        assert_eq!(score.best_streak, 2);

        score.record(false);
        assert_eq!(score.correct, 2);
        assert_eq!(score.total, 3);
        assert_eq!(score.streak, 0);
        assert_eq!(score.best_streak, 2);
    }

    #[test]
    fn test_difficulty_cycle() {
        let mut state = LegoModeState::new();
        assert_eq!(state.difficulty, QuizDifficulty::Beginner);

        state.cycle_difficulty();
        assert_eq!(state.difficulty, QuizDifficulty::Intermediate);

        state.cycle_difficulty();
        assert_eq!(state.difficulty, QuizDifficulty::Advanced);

        state.cycle_difficulty();
        assert_eq!(state.difficulty, QuizDifficulty::Beginner);
    }
}
