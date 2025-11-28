//! Unicode symbols and color theme constants for the piano roll visualization

use ratatui::style::{Color, Modifier, Style};

/// Unicode symbols used throughout the piano roll
pub mod symbols {
    // Notes
    pub const NOTE_FILLED: &str = "●";
    pub const NOTE_HOLLOW: &str = "○";
    pub const NOTE_SMALL: &str = "·";
    pub const NOTE_DIAMOND: &str = "◆";

    // Piano keys
    pub const BLACK_KEY: &str = "▪";
    pub const WHITE_KEY: &str = "▫";
    pub const KEY_PRESSED: &str = "█";

    // Voice leading arrows
    pub const ARROW_UP: &str = "↑";
    pub const ARROW_DOWN: &str = "↓";
    pub const ARROW_RIGHT: &str = "→";
    pub const ARROW_UP_RIGHT: &str = "↗";
    pub const ARROW_DOWN_RIGHT: &str = "↘";
    pub const LINE_HORIZONTAL: &str = "━";

    // Timeline markers
    pub const PLAYHEAD: &str = "▼";
    pub const PLAYHEAD_LINE: &str = "│";
    pub const BEAT_MARKER: &str = "┊";
    pub const MEASURE_MARKER: &str = "┃";

    // Special markers
    pub const AVOID_NOTE: &str = "×";
    pub const ROOT_MARKER: &str = "R";
    pub const EXTENSION_9: &str = "9";
    pub const EXTENSION_11: &str = "11";
    pub const EXTENSION_13: &str = "13";

    // Box drawing
    pub const BOX_H: &str = "─";
    pub const BOX_V: &str = "│";
    pub const BOX_TL: &str = "┌";
    pub const BOX_TR: &str = "┐";
    pub const BOX_BL: &str = "└";
    pub const BOX_BR: &str = "┘";
    pub const BOX_T: &str = "┬";
    pub const BOX_B: &str = "┴";
    pub const BOX_L: &str = "├";
    pub const BOX_R: &str = "┤";
    pub const BOX_CROSS: &str = "┼";
}

/// Color theme for the piano roll
pub mod colors {
    use super::*;

    // Note colors
    pub const ROOT: Color = Color::Red;
    pub const THIRD: Color = Color::Yellow;
    pub const FIFTH: Color = Color::Green;
    pub const SEVENTH: Color = Color::Yellow;
    pub const EXTENSION: Color = Color::Cyan;
    pub const SCALE_NOTE: Color = Color::Blue;
    pub const AVOID_NOTE: Color = Color::DarkGray;

    // Key colors
    pub const BLACK_KEY: Color = Color::DarkGray;
    pub const WHITE_KEY: Color = Color::Gray;

    // Voice leading
    pub const SMOOTH_MOVEMENT: Color = Color::Green;     // 0-2 semitones
    pub const MODERATE_MOVEMENT: Color = Color::Yellow;  // 3-4 semitones
    pub const LARGE_MOVEMENT: Color = Color::Red;        // 5+ semitones

    // Timeline
    pub const PLAYHEAD: Color = Color::Magenta;
    pub const CURRENT_CHORD: Color = Color::Green;
    pub const PAST_CHORD: Color = Color::DarkGray;
    pub const FUTURE_CHORD: Color = Color::White;

    // UI
    pub const BORDER: Color = Color::Cyan;
    pub const TITLE: Color = Color::Cyan;
    pub const LABEL: Color = Color::DarkGray;
}

/// Pre-built styles for common elements
pub mod styles {
    use super::*;

    pub fn root() -> Style {
        Style::default()
            .fg(colors::ROOT)
            .add_modifier(Modifier::BOLD)
    }

    pub fn third() -> Style {
        Style::default()
            .fg(colors::THIRD)
            .add_modifier(Modifier::BOLD)
    }

    pub fn fifth() -> Style {
        Style::default().fg(colors::FIFTH)
    }

    pub fn seventh() -> Style {
        Style::default()
            .fg(colors::SEVENTH)
            .add_modifier(Modifier::BOLD)
    }

    pub fn extension() -> Style {
        Style::default().fg(colors::EXTENSION)
    }

    pub fn scale_note() -> Style {
        Style::default().fg(colors::SCALE_NOTE)
    }

    pub fn avoid_note() -> Style {
        Style::default().fg(colors::AVOID_NOTE)
    }

    pub fn black_key() -> Style {
        Style::default().fg(colors::BLACK_KEY)
    }

    pub fn white_key() -> Style {
        Style::default().fg(colors::WHITE_KEY)
    }

    pub fn playhead() -> Style {
        Style::default()
            .fg(colors::PLAYHEAD)
            .add_modifier(Modifier::BOLD)
    }

    pub fn current_chord() -> Style {
        Style::default()
            .fg(colors::CURRENT_CHORD)
            .add_modifier(Modifier::BOLD)
    }

    pub fn past_chord() -> Style {
        Style::default()
            .fg(colors::PAST_CHORD)
            .add_modifier(Modifier::DIM)
    }

    pub fn future_chord() -> Style {
        Style::default().fg(colors::FUTURE_CHORD)
    }

    pub fn border() -> Style {
        Style::default().fg(colors::BORDER)
    }

    pub fn title() -> Style {
        Style::default()
            .fg(colors::TITLE)
            .add_modifier(Modifier::BOLD)
    }

    pub fn label() -> Style {
        Style::default().fg(colors::LABEL)
    }

    /// Get style for voice leading arrow based on semitone movement
    pub fn voice_leading(semitones: i8) -> Style {
        let color = match semitones.abs() {
            0..=2 => colors::SMOOTH_MOVEMENT,
            3..=4 => colors::MODERATE_MOVEMENT,
            _ => colors::LARGE_MOVEMENT,
        };
        Style::default().fg(color)
    }

    /// Get voice leading arrow symbol based on movement direction and distance
    pub fn voice_leading_arrow(semitones: i8) -> &'static str {
        match semitones {
            0 => symbols::ARROW_RIGHT,
            1..=2 => symbols::ARROW_UP_RIGHT,
            -2..=-1 => symbols::ARROW_DOWN_RIGHT,
            s if s > 2 => symbols::ARROW_UP,
            _ => symbols::ARROW_DOWN,
        }
    }
}

/// Helper to determine if a MIDI note is a black key
pub fn is_black_key(midi: u8) -> bool {
    matches!(midi % 12, 1 | 3 | 6 | 8 | 10)
}

/// Convert MIDI note to note name with octave
pub fn midi_to_name(midi: u8) -> String {
    let note_names = ["C", "C#", "D", "D#", "E", "F", "F#", "G", "G#", "A", "A#", "B"];
    let note = note_names[(midi % 12) as usize];
    let octave = (midi / 12) as i8 - 1;
    format!("{}{}", note, octave)
}

/// Short note name (just the pitch class, no octave)
pub fn midi_to_pitch_class(midi: u8) -> &'static str {
    const NOTE_NAMES: [&str; 12] = ["C", "C#", "D", "D#", "E", "F", "F#", "G", "G#", "A", "A#", "B"];
    NOTE_NAMES[(midi % 12) as usize]
}
