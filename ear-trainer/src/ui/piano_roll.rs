use crate::music::{Chord, ChordScaleMatcher, Note, Scale};
use ratatui::{
    buffer::Buffer,
    layout::Rect,
    style::{Color, Modifier, Style},
    text::{Line, Span},
    widgets::{Block, Borders, Widget},
};

pub struct PianoRoll<'a> {
    pub chord: &'a Chord,
    pub scale: &'a Scale,
    pub show_extensions: bool,
}

impl<'a> PianoRoll<'a> {
    pub fn new(chord: &'a Chord, scale: &'a Scale) -> Self {
        Self {
            chord,
            scale,
            show_extensions: true,
        }
    }

    fn is_black_key(note: u8) -> bool {
        matches!(note % 12, 1 | 3 | 6 | 8 | 10)
    }

    fn note_name(midi: u8) -> String {
        let note = Note::from_midi(midi);
        let octave = (midi / 12) as i8 - 1;
        format!("{}{}", note.sharp_name(), octave)
    }
}

impl<'a> Widget for PianoRoll<'a> {
    fn render(self, area: Rect, buf: &mut Buffer) {
        let block = Block::default()
            .title("Piano Roll View")
            .borders(Borders::ALL)
            .border_style(Style::default().fg(Color::Cyan));

        let inner = block.inner(area);
        block.render(area, buf);

        if inner.height < 3 {
            return;
        }

        let chord_notes = self.chord.notes_in_range(48, 84);
        let scale_notes: Vec<u8> = (48..84)
            .filter(|&midi| {
                let note = Note::from_midi(midi);
                self.scale.contains(note)
            })
            .collect();

        let guide_tones = self.chord.guide_tones();
        let avoid_notes = ChordScaleMatcher::get_avoid_notes(self.chord, self.scale);

        let start_note = 48;
        let end_note = 84;
        let total_keys = (end_note - start_note) as usize;

        let key_width = (inner.width as usize).saturating_sub(2) / total_keys.min(inner.width as usize);
        if key_width == 0 {
            return;
        }

        for i in 0..total_keys {
            let midi = start_note + i as u8;
            if midi >= end_note {
                break;
            }

            let x = inner.x + 1 + (i * key_width) as u16;
            if x >= inner.x + inner.width - 1 {
                break;
            }

            let note = Note::from_midi(midi);
            let is_black = Self::is_black_key(midi);
            let is_chord_tone = chord_notes.contains(&midi);
            let is_scale_note = scale_notes.contains(&midi);
            let is_guide_tone = guide_tones.contains(&note);
            let is_avoid = avoid_notes.contains(&note);

            let (symbol, style) = if is_chord_tone {
                if is_guide_tone {
                    ("●", Style::default().fg(Color::Yellow).add_modifier(Modifier::BOLD))
                } else if self.chord.chord_tone(note) == Some(crate::music::chord::ChordTone::Root) {
                    ("R", Style::default().fg(Color::Red).add_modifier(Modifier::BOLD))
                } else {
                    ("●", Style::default().fg(Color::Green).add_modifier(Modifier::BOLD))
                }
            } else if is_avoid {
                ("×", Style::default().fg(Color::DarkGray))
            } else if is_scale_note {
                ("·", Style::default().fg(Color::Blue))
            } else if is_black {
                ("▪", Style::default().fg(Color::DarkGray))
            } else {
                ("▫", Style::default().fg(Color::Gray))
            };

            for dy in 0..inner.height.saturating_sub(2).min(2) {
                let y = inner.y + 1 + dy;
                if y < inner.y + inner.height - 1 {
                    buf.set_string(x, y, symbol, style);
                }
            }

            if inner.height >= 4 && i % 2 == 0 {
                let label = Self::note_name(midi);
                let label_y = inner.y + inner.height - 2;
                if x + label.len() as u16 <= inner.x + inner.width - 1 {
                    buf.set_string(
                        x,
                        label_y,
                        &label,
                        Style::default().fg(Color::DarkGray),
                    );
                }
            }
        }

        let legend_y = inner.y + 1;
        if inner.height > 3 {
            let legend = vec![
                Span::styled("●", Style::default().fg(Color::Green).add_modifier(Modifier::BOLD)),
                Span::raw(" Chord Tone  "),
                Span::styled("●", Style::default().fg(Color::Yellow).add_modifier(Modifier::BOLD)),
                Span::raw(" Guide Tone  "),
                Span::styled("·", Style::default().fg(Color::Blue)),
                Span::raw(" Scale Note  "),
                Span::styled("×", Style::default().fg(Color::DarkGray)),
                Span::raw(" Avoid"),
            ];
            let legend_line = Line::from(legend);
            buf.set_line(inner.x + 1, legend_y, &legend_line, inner.width - 2);
        }
    }
}

pub fn render_piano_roll<'a>(chord: &'a Chord, scale: &'a Scale) -> PianoRoll<'a> {
    PianoRoll::new(chord, scale)
}
