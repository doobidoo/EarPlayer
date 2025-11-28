//! Timeline state management for chord progression visualization

use crate::music::{Chord, ChordScaleMatcher, Progression, Scale};

/// A chord positioned on the timeline with its visual state
#[derive(Debug, Clone)]
pub struct TimelineChord {
    /// The chord to display
    pub chord: Chord,
    /// The scale associated with this chord
    pub scale: Scale,
    /// Starting beat position in the progression
    pub start_beat: f32,
    /// Duration in beats
    pub duration_beats: f32,
    /// Whether this is the currently playing chord
    pub is_current: bool,
    /// Whether this chord has already been played
    pub is_past: bool,
    /// Index in the progression
    pub index: usize,
}

/// State for the timeline visualization
#[derive(Debug, Clone)]
pub struct TimelineState {
    /// All chords in the current progression
    pub chords: Vec<TimelineChord>,
    /// Current playback beat position (global, from start of progression)
    pub current_beat: f32,
    /// How many beats are visible in the timeline view
    pub visible_beats: f32,
    /// Scroll offset for viewing different parts of the progression
    pub scroll_offset: f32,
    /// Beats per measure (typically 4)
    pub beats_per_measure: u8,
    /// Current chord index
    pub current_chord_idx: usize,
}

impl Default for TimelineState {
    fn default() -> Self {
        Self {
            chords: Vec::new(),
            current_beat: 0.0,
            visible_beats: 16.0,
            scroll_offset: 0.0,
            beats_per_measure: 4,
            current_chord_idx: 0,
        }
    }
}

impl TimelineState {
    /// Create a new empty timeline state
    pub fn new() -> Self {
        Self::default()
    }

    /// Create timeline state from a progression
    pub fn from_progression(
        progression: &Progression,
        current_chord_idx: usize,
        current_beat_in_chord: f32,
    ) -> Self {
        let mut accumulated_beat = 0.0;
        let chords: Vec<TimelineChord> = progression
            .changes
            .iter()
            .enumerate()
            .map(|(i, change)| {
                let tc = TimelineChord {
                    chord: change.chord.clone(),
                    scale: ChordScaleMatcher::get_primary_scale(&change.chord),
                    start_beat: accumulated_beat,
                    duration_beats: change.duration,
                    is_current: i == current_chord_idx,
                    is_past: i < current_chord_idx,
                    index: i,
                };
                accumulated_beat += change.duration;
                tc
            })
            .collect();

        // Calculate global beat position
        let global_beat = chords
            .get(current_chord_idx)
            .map(|c| c.start_beat + current_beat_in_chord)
            .unwrap_or(0.0);

        // Center the view on the current position
        let scroll_offset = (global_beat - 4.0).max(0.0);

        Self {
            chords,
            current_beat: global_beat,
            visible_beats: 16.0,
            scroll_offset,
            beats_per_measure: 4,
            current_chord_idx,
        }
    }

    /// Update the timeline state with new playback position
    pub fn update(&mut self, current_chord_idx: usize, current_beat_in_chord: f32) {
        self.current_chord_idx = current_chord_idx;

        // Update is_current and is_past flags
        for (i, chord) in self.chords.iter_mut().enumerate() {
            chord.is_current = i == current_chord_idx;
            chord.is_past = i < current_chord_idx;
        }

        // Update global beat position
        if let Some(current) = self.chords.get(current_chord_idx) {
            self.current_beat = current.start_beat + current_beat_in_chord;
        }

        // Auto-scroll to keep playhead visible
        self.auto_scroll();
    }

    /// Automatically scroll to keep the playhead visible
    fn auto_scroll(&mut self) {
        let margin = 2.0; // Keep 2 beats of margin on each side

        // If playhead is too far right, scroll right
        if self.current_beat > self.scroll_offset + self.visible_beats - margin {
            self.scroll_offset = self.current_beat - self.visible_beats + margin;
        }

        // If playhead is too far left, scroll left
        if self.current_beat < self.scroll_offset + margin {
            self.scroll_offset = (self.current_beat - margin).max(0.0);
        }
    }

    /// Manually scroll the timeline by a number of beats
    pub fn scroll(&mut self, beats: f32) {
        let total_beats = self.total_duration();
        self.scroll_offset = (self.scroll_offset + beats)
            .max(0.0)
            .min((total_beats - self.visible_beats).max(0.0));
    }

    /// Get the total duration of the progression in beats
    pub fn total_duration(&self) -> f32 {
        self.chords
            .last()
            .map(|c| c.start_beat + c.duration_beats)
            .unwrap_or(0.0)
    }

    /// Get chords that are visible in the current view window
    pub fn visible_chords(&self) -> impl Iterator<Item = &TimelineChord> {
        let start = self.scroll_offset;
        let end = self.scroll_offset + self.visible_beats;

        self.chords.iter().filter(move |c| {
            let chord_end = c.start_beat + c.duration_beats;
            // Chord is visible if it overlaps with the visible window
            c.start_beat < end && chord_end > start
        })
    }

    /// Get the current chord if any
    pub fn current_chord(&self) -> Option<&TimelineChord> {
        self.chords.get(self.current_chord_idx)
    }

    /// Get the next chord if any
    pub fn next_chord(&self) -> Option<&TimelineChord> {
        self.chords.get(self.current_chord_idx + 1)
    }

    /// Get the previous chord if any
    pub fn prev_chord(&self) -> Option<&TimelineChord> {
        if self.current_chord_idx > 0 {
            self.chords.get(self.current_chord_idx - 1)
        } else {
            None
        }
    }

    /// Calculate the x position (0.0 to 1.0) for a given beat in the visible window
    pub fn beat_to_position(&self, beat: f32) -> f32 {
        (beat - self.scroll_offset) / self.visible_beats
    }

    /// Check if a beat position is within the visible window
    pub fn is_beat_visible(&self, beat: f32) -> bool {
        beat >= self.scroll_offset && beat <= self.scroll_offset + self.visible_beats
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::music::{ChordQuality, Note};

    fn create_test_progression() -> Progression {
        let mut prog = Progression::new(
            "Test".to_string(),
            "Jazz".to_string(),
            Note::C,
            120.0,
        );
        prog.add_chord(Chord::new(Note::D, ChordQuality::Minor7), 4.0);
        prog.add_chord(Chord::new(Note::G, ChordQuality::Dominant7), 4.0);
        prog.add_chord(Chord::new(Note::C, ChordQuality::Major7), 4.0);
        prog
    }

    #[test]
    fn test_timeline_from_progression() {
        let prog = create_test_progression();
        let timeline = TimelineState::from_progression(&prog, 1, 2.0);

        assert_eq!(timeline.chords.len(), 3);
        assert_eq!(timeline.current_chord_idx, 1);
        assert_eq!(timeline.current_beat, 6.0); // 4.0 (first chord) + 2.0 (beat in second)
    }

    #[test]
    fn test_visible_chords() {
        let prog = create_test_progression();
        let timeline = TimelineState::from_progression(&prog, 0, 0.0);

        let visible: Vec<_> = timeline.visible_chords().collect();
        assert_eq!(visible.len(), 3); // All chords should be visible with 16 beat window
    }

    #[test]
    fn test_chord_flags() {
        let prog = create_test_progression();
        let timeline = TimelineState::from_progression(&prog, 1, 0.0);

        assert!(timeline.chords[0].is_past);
        assert!(!timeline.chords[0].is_current);

        assert!(!timeline.chords[1].is_past);
        assert!(timeline.chords[1].is_current);

        assert!(!timeline.chords[2].is_past);
        assert!(!timeline.chords[2].is_current);
    }
}
