use crate::music::{Chord, Scale, VoiceLeading};
use ratatui::{
    buffer::Buffer,
    layout::Rect,
    style::{Color, Modifier, Style},
    text::{Line, Span},
    widgets::{Block, Borders, Paragraph, Widget},
};

pub struct NotationView<'a> {
    pub chord: &'a Chord,
    pub scale: &'a Scale,
    pub next_chord: Option<&'a Chord>,
    pub show_voice_leading: bool,
}

impl<'a> NotationView<'a> {
    pub fn new(chord: &'a Chord, scale: &'a Scale) -> Self {
        Self {
            chord,
            scale,
            next_chord: None,
            show_voice_leading: false,
        }
    }

    pub fn with_next_chord(mut self, next: &'a Chord) -> Self {
        self.next_chord = Some(next);
        self.show_voice_leading = true;
        self
    }
}

impl<'a> Widget for NotationView<'a> {
    fn render(self, area: Rect, buf: &mut Buffer) {
        let block = Block::default()
            .title("Chord Analysis")
            .borders(Borders::ALL)
            .border_style(Style::default().fg(Color::Cyan));

        let inner = block.inner(area);
        block.render(area, buf);

        if inner.height < 3 {
            return;
        }

        let mut lines = Vec::new();

        lines.push(Line::from(vec![
            Span::styled("Current Chord: ", Style::default().fg(Color::Gray)),
            Span::styled(
                self.chord.name(),
                Style::default().fg(Color::Green).add_modifier(Modifier::BOLD),
            ),
        ]));

        lines.push(Line::from(vec![
            Span::styled("Scale/Mode: ", Style::default().fg(Color::Gray)),
            Span::styled(
                self.scale.name(),
                Style::default().fg(Color::Blue).add_modifier(Modifier::BOLD),
            ),
        ]));

        lines.push(Line::from(""));

        let chord_notes = self.chord.notes();
        let note_names: Vec<String> = chord_notes.iter().map(|n| n.name().to_string()).collect();
        lines.push(Line::from(vec![
            Span::styled("Chord Tones: ", Style::default().fg(Color::Gray)),
            Span::styled(
                note_names.join(", "),
                Style::default().fg(Color::Green),
            ),
        ]));

        let guide_tones = self.chord.guide_tones();
        let guide_names: Vec<String> = guide_tones.iter().map(|n| n.name().to_string()).collect();
        lines.push(Line::from(vec![
            Span::styled("Guide Tones: ", Style::default().fg(Color::Gray)),
            Span::styled(
                guide_names.join(", "),
                Style::default().fg(Color::Yellow),
            ),
        ]));

        let extensions = self.scale.available_extensions(self.chord);
        if !extensions.is_empty() {
            lines.push(Line::from(""));
            lines.push(Line::from(vec![
                Span::styled("Available Extensions:", Style::default().fg(Color::Gray)),
            ]));

            for (note, label) in extensions.iter().take(5) {
                lines.push(Line::from(vec![
                    Span::raw("  "),
                    Span::styled(
                        format!("{}: {}", label, note.name()),
                        Style::default().fg(Color::Cyan),
                    ),
                ]));
            }
        }

        if self.show_voice_leading {
            if let Some(next) = self.next_chord {
                lines.push(Line::from(""));
                lines.push(Line::from(vec![
                    Span::styled("Voice Leading to ", Style::default().fg(Color::Gray)),
                    Span::styled(
                        next.name(),
                        Style::default().fg(Color::Magenta).add_modifier(Modifier::BOLD),
                    ),
                    Span::styled(":", Style::default().fg(Color::Gray)),
                ]));

                let movements = VoiceLeading::analyze(self.chord, next);
                for movement in movements {
                    let arrow = if movement.movement > 0 {
                        "↑"
                    } else if movement.movement < 0 {
                        "↓"
                    } else {
                        "→"
                    };

                    let color = if movement.movement.abs() <= 2 {
                        Color::Green
                    } else {
                        Color::Yellow
                    };

                    lines.push(Line::from(vec![
                        Span::raw("  "),
                        Span::styled(
                            format!("{} {} {}", movement.note.name(), arrow, movement.movement.abs()),
                            Style::default().fg(color),
                        ),
                        Span::styled(" semitones", Style::default().fg(Color::Gray)),
                    ]));
                }
            }
        }

        let paragraph = Paragraph::new(lines);
        paragraph.render(inner, buf);
    }
}

pub fn render_notation<'a>(chord: &'a Chord, scale: &'a Scale) -> NotationView<'a> {
    NotationView::new(chord, scale)
}
