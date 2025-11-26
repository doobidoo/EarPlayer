use super::backend::AudioBackend;
use anyhow::{Context, Result};
use midir::{MidiOutput, MidiOutputConnection};
use std::sync::{Arc, Mutex};

pub struct MidiBackend {
    connection: Arc<Mutex<Option<MidiOutputConnection>>>,
    channel: u8,
}

impl MidiBackend {
    pub fn new() -> Result<Self> {
        let midi_out = MidiOutput::new("Ear Trainer MIDI Out")
            .context("Failed to create MIDI output")?;

        let ports = midi_out.ports();

        let connection = if !ports.is_empty() {
            let port = &ports[0];
            let port_name = midi_out.port_name(port).unwrap_or_else(|_| "Unknown".to_string());
            println!("Connecting to MIDI port: {}", port_name);

            match midi_out.connect(port, "ear-trainer") {
                Ok(conn) => Some(conn),
                Err(e) => {
                    eprintln!("Failed to connect to MIDI port: {}", e);
                    None
                }
            }
        } else {
            eprintln!("No MIDI ports available. MIDI output will be disabled.");
            None
        };

        Ok(Self {
            connection: Arc::new(Mutex::new(connection)),
            channel: 0,
        })
    }

    fn send_message(&mut self, message: &[u8]) -> Result<()> {
        if let Some(conn) = self.connection.lock().unwrap().as_mut() {
            conn.send(message).context("Failed to send MIDI message")?;
        }
        Ok(())
    }
}

impl AudioBackend for MidiBackend {
    fn play_note(&mut self, note: u8, velocity: u8) -> Result<()> {
        let msg = [0x90 | self.channel, note, velocity];
        self.send_message(&msg)
    }

    fn stop_note(&mut self, note: u8) -> Result<()> {
        let msg = [0x80 | self.channel, note, 0];
        self.send_message(&msg)
    }

    fn play_chord(&mut self, notes: &[u8], velocity: u8) -> Result<()> {
        for &note in notes {
            self.play_note(note, velocity)?;
        }
        Ok(())
    }

    fn stop_all(&mut self) -> Result<()> {
        for note in 0..128 {
            let msg = [0x80 | self.channel, note, 0];
            let _ = self.send_message(&msg);
        }
        Ok(())
    }

    fn name(&self) -> &'static str {
        "MIDI"
    }
}

impl Default for MidiBackend {
    fn default() -> Self {
        Self::new().unwrap_or_else(|_| Self {
            connection: Arc::new(Mutex::new(None)),
            channel: 0,
        })
    }
}
