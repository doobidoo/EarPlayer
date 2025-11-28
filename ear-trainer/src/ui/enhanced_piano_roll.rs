//! Enhanced piano roll with vertical timeline visualization

use crate::music::{Chord, ChordScaleMatcher, Note, Scale, VoiceLeading};
use crate::music::chord::ChordTone;
use ratatui::{
    buffer::Buffer,
    layout::Rect,
    style::{Color, Modifier, Style},
    widgets::{Block, Borders, Widget},
};
use super::symbols::{colors, styles, symbols, is_black_key, midi_to_name};
use super::timeline::{TimelineState, TimelineChord};

/// Enhanced piano roll with vertical timeline showing chord progression
pub struct EnhancedPianoRoll<'a> {
    /// Timeline state containing all chords
    pub timeline: &'a TimelineState,
    /// Show voice leading arrows between chords
    pub show_voice_leading: bool,
    /// MIDI range to display
    pub midi_low: u8,
    pub midi_high: u8,
}

impl<'a> EnhancedPianoRoll<'a> {
    pub fn new(timeline: &'a TimelineState) -> Self {
        Self {
            timeline,
            show_voice_leading: true,
            midi_low: 48,  // C3
            midi_high: 84, // C6
        }
    }

    pub fn with_voice_leading(mut self, show: bool) -> Self {
        self.show_voice_leading = show;
        self
    }

    pub fn with_range(mut self, low: u8, high: u8) -> Self {
        self.midi_low = low;
        self.midi_high = high;
        self
    }

    /// Calculate y position for a MIDI note within the given area
    fn midi_to_y(&self, midi: u8, inner: Rect) -> Option<u16> {
        if midi < self.midi_low || midi > self.midi_high {
            return None;
        }

        let range = (self.midi_high - self.midi_low) as u16;
        let available_height = inner.height.saturating_sub(2); // Leave room for header

        if available_height == 0 || range == 0 {
            return None;
        }

        // Invert so high notes are at top
        let note_offset = (self.midi_high - midi) as u16;
        let y = inner.y + 1 + (note_offset * available_height / range).min(available_height - 1);

        Some(y)
    }

    /// Get display style for a note in a chord context
    fn note_style(&self, chord: &Chord, scale: &Scale, midi: u8, is_current: bool) -> (char, Style) {
        let note = Note::from_midi(midi);
        let chord_notes = chord.notes_in_range(midi, midi + 1);
        let is_chord_tone = !chord_notes.is_empty();
        let guide_tones = chord.guide_tones();
        let is_guide = guide_tones.contains(&note);
        let avoid_notes = ChordScaleMatcher::get_avoid_notes(chord, scale);
        let is_avoid = avoid_notes.contains(&note);
        let is_scale = scale.contains(note);

        let base_style = if !is_current {
            // Dim past/future chords
            Style::default().add_modifier(Modifier::DIM)
        } else {
            Style::default()
        };

        if is_chord_tone {
            let color = match chord.chord_tone(note) {
                Some(ChordTone::Root) => colors::ROOT,
                Some(ChordTone::Third) | Some(ChordTone::Seventh) => colors::THIRD,
                _ => colors::FIFTH,
            };
            ('●', base_style.fg(color).add_modifier(Modifier::BOLD))
        } else if is_avoid {
            ('×', base_style.fg(colors::AVOID_NOTE))
        } else if is_scale {
            ('·', base_style.fg(colors::SCALE_NOTE))
        } else {
            (' ', base_style)
        }
    }

    /// Render piano key labels on the left side
    fn render_piano_keys(&self, buf: &mut Buffer, inner: Rect) {
        let label_width = 4u16;

        for midi in self.midi_low..=self.midi_high {
            if let Some(y) = self.midi_to_y(midi, inner) {
                if y >= inner.y && y < inner.y + inner.height {
                    // Only show certain notes to avoid clutter
                    let note = midi % 12;
                    let show_label = matches!(note, 0 | 4 | 7); // C, E, G

                    if show_label {
                        let name = midi_to_name(midi);
                        let style = if is_black_key(midi) {
                            Style::default().fg(Color::DarkGray)
                        } else {
                            Style::default().fg(Color::White)
                        };
                        buf.set_string(inner.x, y, &name[..name.len().min(label_width as usize)], style);
                    }

                    // Draw grid line for white keys
                    if !is_black_key(midi) {
                        let grid_char = if note == 0 { '─' } else { '·' };
                        buf.set_string(
                            inner.x + label_width,
                            y,
                            &grid_char.to_string(),
                            Style::default().fg(Color::DarkGray)
                        );
                    }
                }
            }
        }
    }

    /// Render a single chord column
    fn render_chord_column(
        &self,
        buf: &mut Buffer,
        inner: Rect,
        timeline_chord: &TimelineChord,
        col_x: u16,
        col_width: u16,
    ) {
        // Draw chord name header
        let name = timeline_chord.chord.name();
        let name_style = if timeline_chord.is_current {
            styles::current_chord()
        } else if timeline_chord.is_past {
            styles::past_chord()
        } else {
            styles::future_chord()
        };

        // Truncate name if needed
        let display_name: String = name.chars().take(col_width as usize - 1).collect();
        buf.set_string(col_x, inner.y, &display_name, name_style);

        // Draw vertical separator line
        for y in (inner.y + 1)..(inner.y + inner.height) {
            buf.set_string(col_x + col_width - 1, y, "│", Style::default().fg(Color::DarkGray));
        }

        // Draw notes in this chord
        let chord_notes = timeline_chord.chord.notes_in_range(self.midi_low, self.midi_high);

        for midi in self.midi_low..=self.midi_high {
            if let Some(y) = self.midi_to_y(midi, inner) {
                if y > inner.y && y < inner.y + inner.height - 1 {
                    let (ch, style) = self.note_style(
                        &timeline_chord.chord,
                        &timeline_chord.scale,
                        midi,
                        timeline_chord.is_current,
                    );

                    if ch != ' ' {
                        // Center the note in the column
                        let note_x = col_x + col_width / 2;
                        buf.set_string(note_x, y, &ch.to_string(), style);
                    }
                }
            }
        }

        // Draw playhead if this is the current chord
        if timeline_chord.is_current {
            // Calculate playhead position within the chord column
            let beat_progress = (self.timeline.current_beat - timeline_chord.start_beat)
                / timeline_chord.duration_beats;
            let playhead_x = col_x + ((col_width as f32 - 2.0) * beat_progress.clamp(0.0, 1.0)) as u16;

            // Draw playhead marker at top
            buf.set_string(playhead_x, inner.y, symbols::PLAYHEAD, styles::playhead());
        }
    }

    /// Render voice leading arrows between two chord columns
    fn render_voice_leading(
        &self,
        buf: &mut Buffer,
        inner: Rect,
        from_chord: &TimelineChord,
        to_chord: &TimelineChord,
        arrow_x: u16,
    ) {
        let movements = VoiceLeading::smooth_voice_leading(&from_chord.chord, &to_chord.chord);

        for (from_note, to_note, semitones) in movements {
            // Find y positions for the notes
            let from_midi = from_note.to_midi(4); // Use octave 4 as reference
            let to_midi = to_note.to_midi(4);

            // Find actual MIDI values in our range
            let from_y = (self.midi_low..=self.midi_high)
                .filter(|&m| Note::from_midi(m) == from_note)
                .filter_map(|m| self.midi_to_y(m, inner))
                .next();

            let to_y = (self.midi_low..=self.midi_high)
                .filter(|&m| Note::from_midi(m) == to_note)
                .filter_map(|m| self.midi_to_y(m, inner))
                .next();

            if let (Some(fy), Some(ty)) = (from_y, to_y) {
                if fy > inner.y && fy < inner.y + inner.height - 1 {
                    let arrow = styles::voice_leading_arrow(semitones);
                    let style = styles::voice_leading(semitones);
                    buf.set_string(arrow_x, fy, arrow, style);
                }
            }
        }
    }
}

impl<'a> Widget for EnhancedPianoRoll<'a> {
    fn render(self, area: Rect, buf: &mut Buffer) {
        let block = Block::default()
            .title(" Timeline Piano Roll ")
            .title_style(styles::title())
            .borders(Borders::ALL)
            .border_style(styles::border());

        let inner = block.inner(area);
        block.render(area, buf);

        if inner.height < 5 || inner.width < 20 {
            buf.set_string(
                inner.x + 1,
                inner.y + 1,
                "Too small",
                Style::default().fg(Color::Red)
            );
            return;
        }

        // Reserve space for piano key labels
        let key_label_width = 5u16;
        let timeline_area = Rect {
            x: inner.x + key_label_width,
            y: inner.y,
            width: inner.width.saturating_sub(key_label_width),
            height: inner.height,
        };

        // Render piano keys on the left
        self.render_piano_keys(buf, inner);

        // Get visible chords
        let visible_chords: Vec<&TimelineChord> = self.timeline.visible_chords().collect();

        if visible_chords.is_empty() {
            buf.set_string(
                timeline_area.x + 1,
                timeline_area.y + 1,
                "No progression loaded",
                Style::default().fg(Color::DarkGray)
            );
            return;
        }

        // Calculate column width based on number of visible chords
        let num_chords = visible_chords.len().max(1);
        let col_width = (timeline_area.width / num_chords as u16).max(6);

        // Render each chord column
        for (i, chord) in visible_chords.iter().enumerate() {
            let col_x = timeline_area.x + (i as u16 * col_width);
            self.render_chord_column(buf, timeline_area, chord, col_x, col_width);
        }

        // Render voice leading arrows between columns
        if self.show_voice_leading && visible_chords.len() > 1 {
            for i in 0..visible_chords.len() - 1 {
                let arrow_x = timeline_area.x + ((i + 1) as u16 * col_width) - 2;
                self.render_voice_leading(
                    buf,
                    timeline_area,
                    visible_chords[i],
                    visible_chords[i + 1],
                    arrow_x,
                );
            }
        }
    }
}

/// Create an enhanced piano roll widget
pub fn render_enhanced_piano_roll(timeline: &TimelineState) -> EnhancedPianoRoll {
    EnhancedPianoRoll::new(timeline)
}
