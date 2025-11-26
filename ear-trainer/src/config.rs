use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Config {
    pub default_tempo: f32,
    pub default_velocity: u8,
    pub midi_channel: u8,
    pub min_note: u8,
    pub max_note: u8,
}

impl Default for Config {
    fn default() -> Self {
        Self {
            default_tempo: 120.0,
            default_velocity: 80,
            midi_channel: 0,
            min_note: 48,  // C3
            max_note: 84,  // C6
        }
    }
}

impl Config {
    pub fn load() -> Self {
        Self::default()
    }

    pub fn save(&self) -> anyhow::Result<()> {
        Ok(())
    }
}
