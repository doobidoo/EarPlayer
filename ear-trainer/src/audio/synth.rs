use super::backend::AudioBackend;
use anyhow::Result;
use rodio::{OutputStream, Sink, Source};
use std::sync::{Arc, Mutex};
use std::time::Duration;

pub struct SynthBackend {
    _stream: OutputStream,
    sink: Arc<Mutex<Sink>>,
    active_notes: Arc<Mutex<Vec<u8>>>,
}

impl SynthBackend {
    pub fn new() -> Result<Self> {
        let (stream, stream_handle) = OutputStream::try_default()?;
        let sink = Sink::try_new(&stream_handle)?;

        Ok(Self {
            _stream: stream,
            sink: Arc::new(Mutex::new(sink)),
            active_notes: Arc::new(Mutex::new(Vec::new())),
        })
    }

    fn midi_to_frequency(note: u8) -> f32 {
        440.0 * 2f32.powf((note as f32 - 69.0) / 12.0)
    }
}

impl AudioBackend for SynthBackend {
    fn play_note(&mut self, note: u8, velocity: u8) -> Result<()> {
        let frequency = Self::midi_to_frequency(note);
        let amplitude = (velocity as f32 / 127.0) * 0.2;

        let sink = self.sink.lock().unwrap();

        let source = SineWave::new(frequency)
            .amplify(amplitude)
            .fade_in(Duration::from_millis(10))
            .take_duration(Duration::from_secs(10));

        sink.append(source);

        self.active_notes.lock().unwrap().push(note);

        Ok(())
    }

    fn stop_note(&mut self, note: u8) -> Result<()> {
        let mut active = self.active_notes.lock().unwrap();
        active.retain(|&n| n != note);
        Ok(())
    }

    fn play_chord(&mut self, notes: &[u8], velocity: u8) -> Result<()> {
        for &note in notes {
            self.play_note(note, velocity)?;
        }
        Ok(())
    }

    fn stop_all(&mut self) -> Result<()> {
        let sink = self.sink.lock().unwrap();
        sink.stop();
        self.active_notes.lock().unwrap().clear();
        Ok(())
    }

    fn name(&self) -> &'static str {
        "Synthesis"
    }
}

impl Default for SynthBackend {
    fn default() -> Self {
        Self::new().expect("Failed to create synthesis backend")
    }
}

struct SineWave {
    frequency: f32,
    sample_rate: u32,
    current_sample: u64,
}

impl SineWave {
    fn new(frequency: f32) -> Self {
        Self {
            frequency,
            sample_rate: 48000,
            current_sample: 0,
        }
    }
}

impl Iterator for SineWave {
    type Item = f32;

    fn next(&mut self) -> Option<Self::Item> {
        let value = (self.current_sample as f32 * self.frequency * 2.0 * std::f32::consts::PI
            / self.sample_rate as f32)
            .sin();

        self.current_sample = self.current_sample.wrapping_add(1);

        Some(value)
    }
}

impl Source for SineWave {
    fn current_frame_len(&self) -> Option<usize> {
        None
    }

    fn channels(&self) -> u16 {
        1
    }

    fn sample_rate(&self) -> u32 {
        self.sample_rate
    }

    fn total_duration(&self) -> Option<Duration> {
        None
    }
}
