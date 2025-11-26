use super::app::{App, AppMode};
use crossterm::event::{KeyCode, KeyEvent};

pub fn handle_input(app: &mut App, key: KeyEvent) -> bool {
    match key.code {
        KeyCode::Char('q') => return false,
        KeyCode::Char(' ') => app.toggle_play(),
        KeyCode::Char('n') => app.next_progression(),
        KeyCode::Char('p') => app.prev_progression(),
        KeyCode::Char('g') => app.next_genre(),
        KeyCode::Char('G') => app.prev_genre(),
        KeyCode::Char('m') => app.toggle_audio_mode(),
        KeyCode::Char('h') => app.show_help = !app.show_help,
        KeyCode::Char('s') => app.show_scales = !app.show_scales,
        KeyCode::Char('v') => app.show_voice_leading = !app.show_voice_leading,
        KeyCode::Char('+') | KeyCode::Char('=') => app.increase_tempo(),
        KeyCode::Char('-') | KeyCode::Char('_') => app.decrease_tempo(),
        KeyCode::Char('1') => app.mode = AppMode::Listen,
        KeyCode::Char('2') => app.mode = AppMode::Practice,
        KeyCode::Char('3') => app.mode = AppMode::Quiz,
        _ => {}
    }
    true
}
