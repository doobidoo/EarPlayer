use crate::audio::{ActiveBackend, AudioManager, BleStatus};
use crate::music::{Chord, ChordScaleMatcher, Progression, ProgressionLibrary, Scale, VoicingType};
use super::lego_mode::LegoModeState;
use super::timeline::TimelineState;
use std::time::Instant;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum AppMode {
    Listen,
    Practice,
    Quiz,
    /// LEGO Bricks: Listen to bricks with annotations
    LegoListen,
    /// LEGO Bricks: Quiz - identify which brick is playing
    LegoQuiz,
}

pub struct App {
    pub mode: AppMode,
    pub audio_manager: AudioManager,
    pub library: ProgressionLibrary,
    pub current_genre: String,
    pub current_progression_idx: usize,
    pub current_chord_idx: usize,
    pub is_playing: bool,
    pub show_help: bool,
    pub show_scales: bool,
    pub show_voice_leading: bool,
    pub tempo: f32,
    pub last_chord_change: Option<Instant>,
    pub chord_duration_ms: u64,
    pub current_beat: f32,
    pub selected_genre_idx: usize,
    /// Timeline state for piano roll visualization
    pub timeline_state: TimelineState,
    /// LEGO Bricks training mode state
    pub lego_state: LegoModeState,
    /// Current voicing type for chord playback
    pub current_voicing: VoicingType,
    /// Swing enabled for jazz feel
    pub swing_enabled: bool,
    /// Swing ratio: 0.5 = straight, 0.58 = light swing, 0.67 = hard swing
    pub swing_ratio: f32,
}

impl App {
    pub fn new() -> Self {
        let library = ProgressionLibrary::new();
        let genres = library.all_genres();
        let current_genre = genres.first().cloned().unwrap_or_else(|| "Jazz".to_string());

        let mut audio_manager = AudioManager::default();
        // Start BLE scanning in background
        audio_manager.start_ble_scan();

        let mut app = Self {
            mode: AppMode::Listen,
            audio_manager,
            library,
            current_genre,
            current_progression_idx: 0,
            current_chord_idx: 0,
            is_playing: false,
            show_help: false,
            show_scales: true,
            show_voice_leading: true,
            tempo: 120.0,
            last_chord_change: None,
            chord_duration_ms: 2000,
            current_beat: 0.0,
            selected_genre_idx: 0,
            timeline_state: TimelineState::new(),
            lego_state: LegoModeState::new(),
            current_voicing: VoicingType::Full,
            swing_enabled: false,
            swing_ratio: 0.5, // Straight timing by default
        };
        app.refresh_timeline();
        app
    }

    /// Refresh the timeline state from the current progression
    pub fn refresh_timeline(&mut self) {
        if let Some(prog) = self.current_progression() {
            self.timeline_state = TimelineState::from_progression(
                prog,
                self.current_chord_idx,
                self.current_beat,
            );
        }
    }

    pub fn current_progression(&self) -> Option<&Progression> {
        self.library
            .get_by_genre(&self.current_genre)
            .and_then(|progs| progs.get(self.current_progression_idx))
    }

    pub fn current_chord(&self) -> Option<&Chord> {
        self.current_progression()
            .and_then(|prog| prog.changes.get(self.current_chord_idx))
            .map(|change| &change.chord)
    }

    pub fn current_scale(&self) -> Option<Scale> {
        self.current_chord()
            .map(|chord| ChordScaleMatcher::get_primary_scale(chord))
    }

    pub fn next_progression(&mut self) {
        if let Some(progs) = self.library.get_by_genre(&self.current_genre) {
            self.current_progression_idx = (self.current_progression_idx + 1) % progs.len();
            self.current_chord_idx = 0;
            self.stop();
            self.refresh_timeline();
        }
    }

    pub fn prev_progression(&mut self) {
        if let Some(progs) = self.library.get_by_genre(&self.current_genre) {
            if self.current_progression_idx == 0 {
                self.current_progression_idx = progs.len() - 1;
            } else {
                self.current_progression_idx -= 1;
            }
            self.current_chord_idx = 0;
            self.stop();
            self.refresh_timeline();
        }
    }

    pub fn next_genre(&mut self) {
        let genres = self.library.all_genres();
        self.selected_genre_idx = (self.selected_genre_idx + 1) % genres.len();
        self.current_genre = genres[self.selected_genre_idx].clone();
        self.current_progression_idx = 0;
        self.current_chord_idx = 0;
        self.stop();
        self.refresh_timeline();
    }

    pub fn prev_genre(&mut self) {
        let genres = self.library.all_genres();
        if self.selected_genre_idx == 0 {
            self.selected_genre_idx = genres.len() - 1;
        } else {
            self.selected_genre_idx -= 1;
        }
        self.current_genre = genres[self.selected_genre_idx].clone();
        self.current_progression_idx = 0;
        self.current_chord_idx = 0;
        self.stop();
        self.refresh_timeline();
    }

    pub fn play(&mut self) {
        self.is_playing = true;
        self.last_chord_change = Some(Instant::now());

        if let Some(chord) = self.current_chord().cloned() {
            self.play_current_chord(&chord);
        }
    }

    pub fn stop(&mut self) {
        self.is_playing = false;
        self.last_chord_change = None;
        self.current_beat = 0.0;
        self.audio_manager.stop_all();
    }

    pub fn toggle_play(&mut self) {
        if self.is_playing {
            self.stop();
        } else {
            self.play();
        }
    }

    pub fn toggle_audio_mode(&mut self) {
        self.stop();
        self.audio_manager.toggle_backend();
    }

    pub fn force_ble_rescan(&mut self) {
        self.audio_manager.force_ble_rescan();
    }

    pub fn active_backend(&self) -> ActiveBackend {
        self.audio_manager.active_backend()
    }

    pub fn ble_status(&self) -> &BleStatus {
        self.audio_manager.ble_status()
    }

    pub fn audio_status_line(&self) -> String {
        self.audio_manager.get_status_line()
    }

    pub fn update(&mut self) {
        // Poll BLE events
        self.audio_manager.poll_ble_events();

        if !self.is_playing {
            return;
        }

        if let Some(last_change) = self.last_chord_change {
            let elapsed = last_change.elapsed();

            let should_change = if let Some(prog) = self.current_progression() {
                let current_change = &prog.changes[self.current_chord_idx];
                let beat_duration_ms = (60000.0 / prog.tempo) as u64;

                // Apply swing timing if enabled
                // Swing affects pairs of eighth notes: first is longer, second is shorter
                // For chord changes, we apply swing to odd/even chord indices
                let swing_factor = if self.swing_enabled {
                    if self.current_chord_idx % 2 == 0 {
                        // Even index (1st, 3rd, etc.) - longer duration
                        self.swing_ratio * 2.0
                    } else {
                        // Odd index (2nd, 4th, etc.) - shorter duration
                        (1.0 - self.swing_ratio) * 2.0
                    }
                } else {
                    1.0
                };

                let chord_duration_ms =
                    (current_change.duration * beat_duration_ms as f32 * swing_factor) as u64;

                self.current_beat =
                    (elapsed.as_millis() as f32 / beat_duration_ms as f32) % current_change.duration;

                elapsed.as_millis() >= chord_duration_ms as u128
            } else {
                false
            };

            // Update timeline state with current playback position
            self.timeline_state.update(self.current_chord_idx, self.current_beat);

            if should_change {
                self.next_chord();
            }
        }
    }

    fn next_chord(&mut self) {
        self.audio_manager.stop_all();

        let num_changes = self.current_progression().map(|p| p.changes.len()).unwrap_or(1);
        self.current_chord_idx = (self.current_chord_idx + 1) % num_changes;
        self.last_chord_change = Some(Instant::now());

        if let Some(chord) = self.current_chord().cloned() {
            self.play_current_chord(&chord);
        }
    }

    fn play_current_chord(&mut self, chord: &Chord) {
        // Use voicing system to get properly voiced notes
        let voiced = self.current_voicing.voice_chord(
            chord,
            2,        // Bass octave (C2 = MIDI 36)
            4,        // Voicing octave (C4 = MIDI 60)
            (36, 84), // Range: C2 to C6
        );

        // Combine bass and voicing notes
        let mut notes = voiced.all_notes();
        notes.sort();
        notes.dedup();

        let _ = self.audio_manager.play_chord(&notes, 80);
    }

    pub fn increase_tempo(&mut self) {
        if let Some(prog) = self.current_progression() {
            self.tempo = (prog.tempo + 5.0).min(300.0);
        }
    }

    pub fn decrease_tempo(&mut self) {
        if let Some(prog) = self.current_progression() {
            self.tempo = (prog.tempo - 5.0).max(40.0);
        }
    }

    /// Cycle to the next voicing type
    pub fn cycle_voicing(&mut self) {
        self.current_voicing = self.current_voicing.next();
    }

    /// Toggle swing feel
    pub fn toggle_swing(&mut self) {
        self.swing_enabled = !self.swing_enabled;
    }

    /// Cycle swing ratio: 0.5 (straight) -> 0.58 (light) -> 0.67 (hard) -> 0.5
    pub fn cycle_swing_ratio(&mut self) {
        self.swing_ratio = match self.swing_ratio {
            r if r < 0.55 => 0.58,
            r if r < 0.63 => 0.67,
            _ => 0.5,
        };
    }

    /// Enter LEGO Listen mode
    pub fn enter_lego_listen(&mut self) {
        self.stop();
        self.mode = AppMode::LegoListen;
        // Set first brick if not set
        if self.lego_state.current_brick_name.is_none() {
            let bricks = self.lego_state.brick_library.for_difficulty(self.lego_state.difficulty);
            if let Some(brick) = bricks.first() {
                self.lego_state.current_brick_name = Some(brick.name.clone());
            }
        }
    }

    /// Enter LEGO Quiz mode
    pub fn enter_lego_quiz(&mut self) {
        self.stop();
        self.mode = AppMode::LegoQuiz;
        self.lego_state.generate_quiz();
    }

    /// Submit quiz answer (1-4)
    pub fn submit_quiz_answer(&mut self, answer: usize) {
        if self.mode == AppMode::LegoQuiz {
            self.lego_state.submit_answer(answer);
        }
    }

    /// Check if in a LEGO mode
    pub fn is_lego_mode(&self) -> bool {
        matches!(self.mode, AppMode::LegoListen | AppMode::LegoQuiz)
    }
}

impl Default for App {
    fn default() -> Self {
        Self::new()
    }
}
