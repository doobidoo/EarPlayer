use crate::audio::{ActiveBackend, AudioManager, BleStatus};
use crate::music::{Chord, ChordScaleMatcher, Progression, ProgressionLibrary, Scale};
use std::time::Instant;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum AppMode {
    Listen,
    Practice,
    Quiz,
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
}

impl App {
    pub fn new() -> Self {
        let library = ProgressionLibrary::new();
        let genres = library.all_genres();
        let current_genre = genres.first().cloned().unwrap_or_else(|| "Jazz".to_string());

        let mut audio_manager = AudioManager::default();
        // Start BLE scanning in background
        audio_manager.start_ble_scan();

        Self {
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
        }
    }

    pub fn next_genre(&mut self) {
        let genres = self.library.all_genres();
        self.selected_genre_idx = (self.selected_genre_idx + 1) % genres.len();
        self.current_genre = genres[self.selected_genre_idx].clone();
        self.current_progression_idx = 0;
        self.current_chord_idx = 0;
        self.stop();
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
                let chord_duration_ms = (current_change.duration * beat_duration_ms as f32) as u64;

                self.current_beat =
                    (elapsed.as_millis() as f32 / beat_duration_ms as f32) % current_change.duration;

                elapsed.as_millis() >= chord_duration_ms as u128
            } else {
                false
            };

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
        let notes = chord.notes_in_range(48, 72);
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
}

impl Default for App {
    fn default() -> Self {
        Self::new()
    }
}
