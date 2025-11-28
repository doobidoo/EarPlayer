pub mod backend;
pub mod ble_midi;
pub mod manager;
pub mod midi;
pub mod synth;

pub use ble_midi::{check_ble_prerequisites, BleConnectionState, BlePrerequisites};
pub use manager::{ActiveBackend, AudioManager, BleStatus, PrerequisiteStatus};
