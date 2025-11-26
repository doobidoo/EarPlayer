use anyhow::Result;

#[derive(Debug, Clone, Copy)]
pub struct AudioEvent {
    pub note: u8,
    pub velocity: u8,
    pub duration_ms: u64,
}

impl AudioEvent {
    pub fn new(note: u8, velocity: u8, duration_ms: u64) -> Self {
        Self {
            note,
            velocity,
            duration_ms,
        }
    }
}

pub trait AudioBackend {
    fn play_note(&mut self, note: u8, velocity: u8) -> Result<()>;
    fn stop_note(&mut self, note: u8) -> Result<()>;
    fn play_chord(&mut self, notes: &[u8], velocity: u8) -> Result<()>;
    fn stop_all(&mut self) -> Result<()>;
    fn name(&self) -> &'static str;
}
