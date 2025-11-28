//! Horizontal keyboard reference widget showing current chord analysis

use crate::music::{Chord, ChordScaleMatcher, Note, Scale};
use crate::music::chord::ChordTone;
use ratatui::{
    buffer::Buffer,
    layout::Rect,
    style::{Color, Modifier, Style},
    text::{Line, Span},
    widgets::{Block, Borders, Widget},
};
use super::symbols::{colors, styles, symbols, is_black_key};

/// Compact horizontal keyboard showing one or two octaves with note analysis
pub struct HorizontalKeyboard<'a> {
    /// The chord being displayed
    pub chord: &'a Chord,
    /// The scale associated with this chord
    pub scale: &'a Scale,
    /// Starting MIDI note (typically 48 = C3 or 60 = C4)
    pub start_midi: u8,
    /// Number of octaves to display
    pub octaves: u8,
}

impl<'a> HorizontalKeyboard<'a> {
    pub fn new(chord: &'a Chord, scale: &'a Scale) -> Self {
        Self {
            chord,
            scale,
            start_midi: 48, // C3
            octaves: 2,
        }
    }

    pub fn with_range(mut self, start_midi: u8, octaves: u8) -> Self {
        self.start_midi = start_midi;
        self.octaves = octaves;
        self
    }

    /// Get the display character and style for a note
    fn note_display(&self, midi: u8) -> (&'static str, Style) {
        let note = Note::from_midi(midi);
        let chord_notes = self.chord.notes_in_range(midi, midi + 1);
        let is_chord_tone = !chord_notes.is_empty();
        let guide_tones = self.chord.guide_tones();
        let is_guide = guide_tones.contains(&note);
        let avoid_notes = ChordScaleMatcher::get_avoid_notes(self.chord, self.scale);
        let is_avoid = avoid_notes.contains(&note);
        let is_scale = self.scale.contains(note);

        if is_chord_tone {
            match self.chord.chord_tone(note) {
                Some(ChordTone::Root) => (symbols::ROOT_MARKER, styles::root()),
                Some(ChordTone::Third) => ("3", styles::third()),
                Some(ChordTone::Fifth) => ("5", styles::fifth()),
                Some(ChordTone::Seventh) => ("7", styles::seventh()),
                Some(ChordTone::Ninth) => ("9", styles::extension()),
                Some(ChordTone::Eleventh) => ("11", styles::extension()),
                Some(ChordTone::Thirteenth) => ("13", styles::extension()),
                None => (symbols::NOTE_FILLED, styles::fifth()),
            }
        } else if is_guide {
            // Guide tone not in chord (unusual but handle it)
            (symbols::NOTE_FILLED, styles::seventh())
        } else if is_avoid {
            (symbols::AVOID_NOTE, styles::avoid_note())
        } else if is_scale {
            (symbols::NOTE_SMALL, styles::scale_note())
        } else if is_black_key(midi) {
            (symbols::BLACK_KEY, styles::black_key())
        } else {
            (symbols::WHITE_KEY, styles::white_key())
        }
    }

    /// Get the extension label for a note if it's an available extension
    fn extension_label(&self, note: Note) -> Option<&'static str> {
        let extensions = self.scale.available_extensions(self.chord);
        extensions
            .iter()
            .find(|(n, _)| *n == note)
            .map(|(_, label)| *label)
    }
}

impl<'a> Widget for HorizontalKeyboard<'a> {
    fn render(self, area: Rect, buf: &mut Buffer) {
        let block = Block::default()
            .title(format!(" {} ({}) ", self.chord.name(), self.scale.name()))
            .title_style(styles::title())
            .borders(Borders::ALL)
            .border_style(styles::border());

        let inner = block.inner(area);
        block.render(area, buf);

        if inner.height < 2 || inner.width < 12 {
            return;
        }

        let total_keys = (self.octaves as usize) * 12;
        let key_width = ((inner.width as usize).saturating_sub(2) / total_keys).max(1);

        // Row 1: Note names (C, D, E, etc.)
        // Row 2: Analysis symbols (R, 3, 5, 7, etc.)
        // Row 3: Extension labels (9th, #11, etc.) - optional

        let show_extensions = inner.height >= 3;

        for i in 0..total_keys {
            let midi = self.start_midi + i as u8;
            let x = inner.x + 1 + (i * key_width) as u16;

            if x >= inner.x + inner.width - 1 {
                break;
            }

            let note = Note::from_midi(midi);
            let is_black = is_black_key(midi);

            // Row 1: Note name (only for white keys to save space, or all if wide enough)
            if !is_black || key_width >= 2 {
                let note_name = if is_black {
                    note.name().to_string()
                } else {
                    format!("{}", note.name().chars().next().unwrap_or(' '))
                };

                let name_style = if is_black {
                    Style::default().fg(Color::DarkGray)
                } else {
                    Style::default().fg(Color::White)
                };

                buf.set_string(x, inner.y, &note_name, name_style);
            }

            // Row 2: Analysis symbol
            if inner.height >= 2 {
                let (symbol, style) = self.note_display(midi);
                buf.set_string(x, inner.y + 1, symbol, style);
            }

            // Row 3: Extension label (only for extensions)
            if show_extensions && inner.height >= 3 {
                if let Some(ext_label) = self.extension_label(note) {
                    // Only show first char of extension to fit
                    let short_label: String = ext_label.chars().take(2).collect();
                    buf.set_string(
                        x,
                        inner.y + 2,
                        &short_label,
                        Style::default().fg(Color::Cyan)
                    );
                }
            }
        }

        // If we have space, add a legend at the right
        if inner.width > 40 && inner.height >= 2 {
            let legend_x = inner.x + inner.width - 25;
            let legend = vec![
                Span::styled("R", styles::root()),
                Span::raw("=Root "),
                Span::styled("3", styles::third()),
                Span::raw("/"),
                Span::styled("7", styles::seventh()),
                Span::raw("=Guide"),
            ];
            let legend_line = Line::from(legend);
            buf.set_line(legend_x, inner.y, &legend_line, 25);
        }
    }
}

/// Create a horizontal keyboard widget
pub fn render_horizontal_keyboard<'a>(chord: &'a Chord, scale: &'a Scale) -> HorizontalKeyboard<'a> {
    HorizontalKeyboard::new(chord, scale)
}
