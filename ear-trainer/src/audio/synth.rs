use super::backend::AudioBackend;
use anyhow::Result;
use rodio::{OutputStream, OutputStreamHandle, Sink, Source};
use std::sync::{Arc, Mutex};
use std::time::Duration;

const SAMPLE_RATE: u32 = 48000;

pub struct SynthBackend {
    _stream: OutputStream,
    stream_handle: OutputStreamHandle,
    sinks: Arc<Mutex<Vec<Sink>>>,
    active_notes: Arc<Mutex<Vec<u8>>>,
}

impl SynthBackend {
    pub fn new() -> Result<Self> {
        let (stream, stream_handle) = OutputStream::try_default()?;

        Ok(Self {
            _stream: stream,
            stream_handle,
            sinks: Arc::new(Mutex::new(Vec::new())),
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
        // Reduce amplitude to prevent clipping when multiple notes play
        let amplitude = (velocity as f32 / 127.0) * 0.15;

        // Create a new sink for each note so we can stop them independently
        let sink = Sink::try_new(&self.stream_handle)?;

        // Use piano-like tone with harmonics and proper envelope
        let source = PianoTone::new(frequency, amplitude, 3000);

        sink.append(source);

        self.sinks.lock().unwrap().push(sink);
        self.active_notes.lock().unwrap().push(note);

        Ok(())
    }

    fn stop_note(&mut self, note: u8) -> Result<()> {
        let mut active = self.active_notes.lock().unwrap();
        active.retain(|&n| n != note);
        Ok(())
    }

    fn play_chord(&mut self, notes: &[u8], velocity: u8) -> Result<()> {
        // Clean up finished sinks before adding new ones to prevent accumulation
        {
            let mut sinks = self.sinks.lock().unwrap();
            sinks.retain(|sink| !sink.empty());
        }

        for &note in notes {
            self.play_note(note, velocity)?;
        }
        Ok(())
    }

    fn stop_all(&mut self) -> Result<()> {
        // Stop and clear all sinks
        let mut sinks = self.sinks.lock().unwrap();
        for sink in sinks.drain(..) {
            sink.stop();
        }
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

/// Piano-like tone with harmonics and ADSR envelope
/// This creates a richer, more musical sound than a pure sine wave
struct PianoTone {
    frequency: f32,
    amplitude: f32,
    sample_rate: u32,
    current_sample: u64,
    total_samples: u64,
    // ADSR envelope parameters (in samples)
    attack_samples: u64,
    decay_samples: u64,
    sustain_level: f32,
    release_samples: u64,
    // Phase accumulators for each harmonic (prevents clicks)
    phases: [f64; 4],
}

impl PianoTone {
    fn new(frequency: f32, amplitude: f32, duration_ms: u64) -> Self {
        let total_samples = (SAMPLE_RATE as u64 * duration_ms) / 1000;

        Self {
            frequency,
            amplitude,
            sample_rate: SAMPLE_RATE,
            current_sample: 0,
            total_samples,
            // Quick attack, medium decay, sustain at 60%, longer release
            attack_samples: (SAMPLE_RATE as f32 * 0.01) as u64,   // 10ms attack
            decay_samples: (SAMPLE_RATE as f32 * 0.15) as u64,    // 150ms decay
            sustain_level: 0.6,
            release_samples: (SAMPLE_RATE as f32 * 0.3) as u64,   // 300ms release
            phases: [0.0; 4],
        }
    }

    /// Calculate ADSR envelope value at current sample
    fn envelope(&self) -> f32 {
        let sample = self.current_sample;
        let release_start = self.total_samples.saturating_sub(self.release_samples);

        if sample < self.attack_samples {
            // Attack phase: linear ramp up
            sample as f32 / self.attack_samples as f32
        } else if sample < self.attack_samples + self.decay_samples {
            // Decay phase: exponential decay to sustain level
            let decay_pos = (sample - self.attack_samples) as f32 / self.decay_samples as f32;
            1.0 - (1.0 - self.sustain_level) * decay_pos
        } else if sample < release_start {
            // Sustain phase
            self.sustain_level
        } else {
            // Release phase: exponential decay to zero
            let release_pos = (sample - release_start) as f32 / self.release_samples as f32;
            self.sustain_level * (1.0 - release_pos).max(0.0)
        }
    }
}

impl Iterator for PianoTone {
    type Item = f32;

    fn next(&mut self) -> Option<Self::Item> {
        if self.current_sample >= self.total_samples {
            return None;
        }

        // Phase increment for fundamental frequency
        let phase_inc = (self.frequency as f64 * 2.0 * std::f64::consts::PI) / self.sample_rate as f64;

        // Generate tone with harmonics for richer sound
        // Piano-like harmonic series with decreasing amplitudes
        let harmonic_weights = [1.0f64, 0.5, 0.25, 0.125];

        let mut sample = 0.0f64;
        for (i, &weight) in harmonic_weights.iter().enumerate() {
            let harmonic = (i + 1) as f64;
            // Accumulate phase to prevent discontinuities
            self.phases[i] += phase_inc * harmonic;
            // Keep phase in reasonable range to prevent precision loss
            if self.phases[i] > 2.0 * std::f64::consts::PI {
                self.phases[i] -= 2.0 * std::f64::consts::PI;
            }
            sample += self.phases[i].sin() * weight;
        }

        // Normalize by sum of weights
        sample /= harmonic_weights.iter().sum::<f64>();

        // Apply envelope and amplitude
        let envelope = self.envelope();
        let value = (sample as f32 * self.amplitude * envelope).clamp(-1.0, 1.0);

        self.current_sample += 1;

        Some(value)
    }
}

impl Source for PianoTone {
    fn current_frame_len(&self) -> Option<usize> {
        Some((self.total_samples - self.current_sample) as usize)
    }

    fn channels(&self) -> u16 {
        1
    }

    fn sample_rate(&self) -> u32 {
        self.sample_rate
    }

    fn total_duration(&self) -> Option<Duration> {
        Some(Duration::from_millis(
            (self.total_samples * 1000) / self.sample_rate as u64,
        ))
    }
}
