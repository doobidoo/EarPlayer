use super::backend::AudioBackend;
use super::ble_midi::{BleConnectionState, BleEvent, BleMidiBackend};
use super::midi::MidiBackend;
use super::synth::SynthBackend;
use anyhow::Result;
use serde::{Deserialize, Serialize};
use std::fs;
use std::path::PathBuf;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum ActiveBackend {
    Midi,
    Synth,
    BleMidi,
}

impl ActiveBackend {
    pub fn next(self) -> Self {
        match self {
            ActiveBackend::Midi => ActiveBackend::Synth,
            ActiveBackend::Synth => ActiveBackend::BleMidi,
            ActiveBackend::BleMidi => ActiveBackend::Midi,
        }
    }

    pub fn display_name(&self) -> &'static str {
        match self {
            ActiveBackend::Midi => "MIDI",
            ActiveBackend::Synth => "Synth",
            ActiveBackend::BleMidi => "BLE MIDI",
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct KnownDevice {
    pub address: String,
    pub name: String,
    pub last_connected: u64,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct AudioConfig {
    pub known_ble_devices: Vec<KnownDevice>,
    pub preferred_ble_device: Option<String>,
    pub last_backend: Option<ActiveBackend>,
}

impl AudioConfig {
    fn config_path() -> Option<PathBuf> {
        dirs::config_dir().map(|p| p.join("ear-trainer").join("audio.json"))
    }

    pub fn load() -> Self {
        Self::config_path()
            .and_then(|path| fs::read_to_string(path).ok())
            .and_then(|contents| serde_json::from_str(&contents).ok())
            .unwrap_or_default()
    }

    pub fn save(&self) {
        if let Some(path) = Self::config_path() {
            if let Some(parent) = path.parent() {
                let _ = fs::create_dir_all(parent);
            }
            if let Ok(json) = serde_json::to_string_pretty(self) {
                let _ = fs::write(path, json);
            }
        }
    }

    pub fn add_known_device(&mut self, address: String, name: String) {
        let now = std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .map(|d| d.as_secs())
            .unwrap_or(0);

        // Update existing or add new
        if let Some(device) = self.known_ble_devices.iter_mut().find(|d| d.address == address) {
            device.name = name;
            device.last_connected = now;
        } else {
            self.known_ble_devices.push(KnownDevice {
                address: address.clone(),
                name,
                last_connected: now,
            });
        }

        // Set as preferred
        self.preferred_ble_device = Some(address);
        self.save();
    }
}

#[derive(Debug, Clone)]
pub struct PrerequisiteStatus {
    pub name: String,
    pub passed: bool,
    pub message: String,
}

#[derive(Debug, Clone)]
pub struct BleStatus {
    pub state: BleConnectionState,
    pub device_name: Option<String>,
    pub last_error: Option<String>,
    pub prerequisites: Vec<PrerequisiteStatus>,
}

impl Default for BleStatus {
    fn default() -> Self {
        Self {
            state: BleConnectionState::Disconnected,
            device_name: None,
            last_error: None,
            prerequisites: Vec::new(),
        }
    }
}

pub struct AudioManager {
    midi_backend: MidiBackend,
    synth_backend: SynthBackend,
    ble_midi_backend: BleMidiBackend,
    active_backend: ActiveBackend,
    config: AudioConfig,
    ble_status: BleStatus,
    auto_scan_started: bool,
}

impl AudioManager {
    pub fn new() -> Result<Self> {
        let config = AudioConfig::load();

        let mut ble_midi_backend = BleMidiBackend::new()?;

        // Set preferred device from config
        if let Some(ref addr) = config.preferred_ble_device {
            ble_midi_backend.set_preferred_device(Some(addr.clone()));
        }

        let active_backend = config.last_backend.unwrap_or(ActiveBackend::Synth);

        Ok(Self {
            midi_backend: MidiBackend::new()?,
            synth_backend: SynthBackend::new()?,
            ble_midi_backend,
            active_backend,
            config,
            ble_status: BleStatus::default(),
            auto_scan_started: false,
        })
    }

    pub fn start_ble_scan(&mut self) {
        if !self.auto_scan_started {
            self.ble_midi_backend.start_scan();
            self.auto_scan_started = true;
        }
    }

    pub fn active_backend(&self) -> ActiveBackend {
        self.active_backend
    }

    pub fn set_active_backend(&mut self, backend: ActiveBackend) {
        self.stop_all();
        self.active_backend = backend;
        self.config.last_backend = Some(backend);
        self.config.save();
    }

    pub fn toggle_backend(&mut self) {
        self.set_active_backend(self.active_backend.next());
    }

    pub fn ble_status(&self) -> &BleStatus {
        &self.ble_status
    }

    pub fn poll_ble_events(&mut self) {
        let events = self.ble_midi_backend.poll_events();

        for event in events {
            match event {
                BleEvent::StateChanged(state) => {
                    self.ble_status.state = state;
                }
                BleEvent::Connected(name) => {
                    self.ble_status.device_name = Some(name.clone());
                    self.ble_status.last_error = None;
                    // Clear prerequisites on successful connection
                    self.ble_status.prerequisites.clear();

                    // Save to known devices
                    // Note: We'd need the address here, but for now just save by name
                    // In a real implementation, we'd pass the address with the event
                }
                BleEvent::DeviceDiscovered(info) => {
                    // Could track discovered devices for UI
                    if self.ble_status.state == BleConnectionState::Connected {
                        self.config.add_known_device(info.address, info.name);
                    }
                }
                BleEvent::Disconnected => {
                    // Trigger reconnect if we were connected
                    if self.ble_status.state == BleConnectionState::Connected {
                        self.ble_midi_backend.start_reconnect();
                    }
                }
                BleEvent::Error(msg) => {
                    self.ble_status.last_error = Some(msg);
                }
                BleEvent::PrerequisiteCheck(name, passed, message) => {
                    // Update or add prerequisite status
                    if let Some(prereq) = self.ble_status.prerequisites.iter_mut().find(|p| p.name == name) {
                        prereq.passed = passed;
                        prereq.message = message;
                    } else {
                        self.ble_status.prerequisites.push(PrerequisiteStatus {
                            name,
                            passed,
                            message,
                        });
                    }
                }
            }
        }

        // Update status from backend
        self.ble_status.state = self.ble_midi_backend.get_state();
        if let Some(name) = self.ble_midi_backend.get_connected_device_name() {
            self.ble_status.device_name = Some(name);
        }
    }

    pub fn force_ble_rescan(&mut self) {
        self.ble_midi_backend.disconnect();
        self.auto_scan_started = false;
        self.ble_midi_backend.start_scan();
        self.auto_scan_started = true;
    }

    pub fn play_note(&mut self, note: u8, velocity: u8) -> Result<()> {
        match self.active_backend {
            ActiveBackend::Midi => self.midi_backend.play_note(note, velocity),
            ActiveBackend::Synth => self.synth_backend.play_note(note, velocity),
            ActiveBackend::BleMidi => self.ble_midi_backend.play_note(note, velocity),
        }
    }

    pub fn stop_note(&mut self, note: u8) -> Result<()> {
        match self.active_backend {
            ActiveBackend::Midi => self.midi_backend.stop_note(note),
            ActiveBackend::Synth => self.synth_backend.stop_note(note),
            ActiveBackend::BleMidi => self.ble_midi_backend.stop_note(note),
        }
    }

    pub fn play_chord(&mut self, notes: &[u8], velocity: u8) -> Result<()> {
        match self.active_backend {
            ActiveBackend::Midi => self.midi_backend.play_chord(notes, velocity),
            ActiveBackend::Synth => self.synth_backend.play_chord(notes, velocity),
            ActiveBackend::BleMidi => self.ble_midi_backend.play_chord(notes, velocity),
        }
    }

    pub fn stop_all(&mut self) {
        let _ = self.midi_backend.stop_all();
        let _ = self.synth_backend.stop_all();
        let _ = self.ble_midi_backend.stop_all();
    }

    pub fn backend_name(&self) -> &'static str {
        self.active_backend.display_name()
    }

    pub fn get_status_line(&self) -> String {
        match self.active_backend {
            ActiveBackend::Midi => "MIDI".to_string(),
            ActiveBackend::Synth => "Synth".to_string(),
            ActiveBackend::BleMidi => {
                let state_str = match &self.ble_status.state {
                    BleConnectionState::Disconnected => "Disconnected",
                    BleConnectionState::Scanning => "Scanning...",
                    BleConnectionState::Connecting => "Connecting...",
                    BleConnectionState::Connected => "Connected",
                    BleConnectionState::Reconnecting(n) => {
                        return format!("BLE MIDI | Reconnecting ({}/5)...", n);
                    }
                };

                if let Some(ref name) = self.ble_status.device_name {
                    if self.ble_status.state == BleConnectionState::Connected {
                        format!("BLE MIDI | {} [{}]", name, state_str)
                    } else {
                        format!("BLE MIDI | {}", state_str)
                    }
                } else {
                    format!("BLE MIDI | {}", state_str)
                }
            }
        }
    }
}

impl Default for AudioManager {
    fn default() -> Self {
        match Self::new() {
            Ok(manager) => manager,
            Err(e) => {
                eprintln!("Warning: Failed to create audio manager: {}. Using fallback.", e);
                // Create a minimal fallback with synth only
                let config = AudioConfig::load();
                Self {
                    midi_backend: MidiBackend::default(),
                    synth_backend: SynthBackend::new().expect("Synth backend required"),
                    ble_midi_backend: BleMidiBackend::new().unwrap_or_else(|_| {
                        // Create a dummy BLE backend that won't do anything
                        eprintln!("BLE MIDI unavailable");
                        BleMidiBackend::new_dummy()
                    }),
                    active_backend: ActiveBackend::Synth,
                    config,
                    ble_status: BleStatus::default(),
                    auto_scan_started: false,
                }
            }
        }
    }
}
