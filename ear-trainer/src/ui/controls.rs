use super::app::{App, AppMode};
use crossterm::event::{KeyCode, KeyEvent};

pub fn handle_input(app: &mut App, key: KeyEvent) -> bool {
    // Handle LEGO Quiz mode inputs separately
    if app.mode == AppMode::LegoQuiz {
        match key.code {
            KeyCode::Char('q') => return false,
            KeyCode::Char('h') => app.show_help = !app.show_help,
            // Quiz answer selection (1-4)
            KeyCode::Char('1') => app.submit_quiz_answer(0),
            KeyCode::Char('2') => app.submit_quiz_answer(1),
            KeyCode::Char('3') => app.submit_quiz_answer(2),
            KeyCode::Char('4') => app.submit_quiz_answer(3),
            // Space to play current quiz brick or next question
            KeyCode::Char(' ') => {
                if app.lego_state.waiting_for_next() {
                    app.lego_state.next_question();
                }
                app.toggle_play();
            }
            // Enter for next question after answering
            KeyCode::Enter => {
                if app.lego_state.waiting_for_next() {
                    app.lego_state.next_question();
                }
            }
            // d to cycle difficulty
            KeyCode::Char('d') => app.lego_state.cycle_difficulty(),
            // Escape to go back to normal mode
            KeyCode::Esc => {
                app.stop();
                app.mode = AppMode::Listen;
            }
            _ => {}
        }
        return true;
    }

    // Handle LEGO Listen mode
    if app.mode == AppMode::LegoListen {
        match key.code {
            KeyCode::Char('q') => return false,
            KeyCode::Char('h') => app.show_help = !app.show_help,
            KeyCode::Char(' ') => app.toggle_play(),
            // n/p to cycle bricks
            KeyCode::Char('n') => {
                app.stop();
                app.lego_state.next_brick();
            }
            KeyCode::Char('p') => {
                app.stop();
                app.lego_state.prev_brick();
            }
            // k/K to cycle keys
            KeyCode::Char('k') => {
                app.stop();
                app.lego_state.next_key();
            }
            KeyCode::Char('K') => {
                app.stop();
                app.lego_state.prev_key();
            }
            // d to cycle difficulty
            KeyCode::Char('d') => {
                app.lego_state.cycle_difficulty();
                app.stop();
            }
            // Escape to go back to normal mode
            KeyCode::Esc => {
                app.stop();
                app.mode = AppMode::Listen;
            }
            _ => {}
        }
        return true;
    }

    // Normal mode controls
    match key.code {
        KeyCode::Char('q') => return false,
        KeyCode::Char(' ') => app.toggle_play(),
        KeyCode::Char('n') => app.next_progression(),
        KeyCode::Char('p') => app.prev_progression(),
        KeyCode::Char('g') => app.next_genre(),
        KeyCode::Char('G') => app.prev_genre(),
        KeyCode::Char('m') => app.toggle_audio_mode(),
        KeyCode::Char('b') => app.force_ble_rescan(),
        KeyCode::Char('h') => app.show_help = !app.show_help,
        KeyCode::Char('s') => app.show_scales = !app.show_scales,
        KeyCode::Char('v') => app.show_voice_leading = !app.show_voice_leading,
        KeyCode::Char('+') | KeyCode::Char('=') => app.increase_tempo(),
        KeyCode::Char('-') | KeyCode::Char('_') => app.decrease_tempo(),
        // Mode selection
        KeyCode::Char('1') => app.mode = AppMode::Listen,
        KeyCode::Char('2') => app.mode = AppMode::Practice,
        KeyCode::Char('3') => app.mode = AppMode::Quiz,
        KeyCode::Char('4') => app.enter_lego_listen(),
        KeyCode::Char('5') => app.enter_lego_quiz(),
        // Timeline scroll controls
        KeyCode::Char('[') | KeyCode::Left => app.timeline_state.scroll(-4.0),
        KeyCode::Char(']') | KeyCode::Right => app.timeline_state.scroll(4.0),
        // Voicing and swing controls
        KeyCode::Char('V') => app.cycle_voicing(),
        KeyCode::Char('w') => app.toggle_swing(),
        KeyCode::Char('W') => app.cycle_swing_ratio(),
        // Rhythm style control
        KeyCode::Char('r') => app.cycle_rhythm(),
        _ => {}
    }
    true
}
