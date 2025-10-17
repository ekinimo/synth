use crate::synth::{Oscillator, OscillatorCtx};

use super::phasor::Phasor;

pub enum Wavetables{
    Simple(Wavetable),
    Multi(MultibandWavetable)
}

impl Oscillator for Wavetables{
    fn generate_sample(&self, ctx: OscillatorCtx, phase: f32) -> f32 {
        match self{
            Wavetables::Simple(wavetable) => wavetable.generate_sample(ctx, phase),
            Wavetables::Multi(multiband_wavetable) => multiband_wavetable.generate_sample(ctx, phase),
        }
    }
    }

pub struct Wavetable {
    buffer: Vec<f32>,
    ramp: Box<dyn Oscillator>,
}

impl Wavetable {
    pub fn new(buffer: Vec<f32>) -> Self {
        Self {
            buffer,
            ramp: Box::new(Phasor::default()),
        }
    }

    // Cubic interpolation helper
    fn cubic_interpolate(&self, frac: f32, y0: f32, y1: f32, y2: f32, y3: f32) -> f32 {
        let a = -0.5 * y0 + 1.5 * y1 - 1.5 * y2 + 0.5 * y3;
        let b = y0 - 2.5 * y1 + 2.0 * y2 - 0.5 * y3;
        let c = -0.5 * y0 + 0.5 * y2;
        let d = y1;

        ((a * frac + b) * frac + c) * frac + d
    }

    // Get sample at position with cubic interpolation
    fn get_interpolated(&self, position: f32) -> f32 {
        let size = self.buffer.len() as f32;
        let scaled_pos = position * size;
        let index = scaled_pos.floor() as usize;
        let frac = scaled_pos.fract();

        // Get 4 adjacent samples for cubic interpolation
        let y0 = self.buffer[(index + self.buffer.len() - 1) % self.buffer.len()];
        let y1 = self.buffer[index];
        let y2 = self.buffer[(index + 1) % self.buffer.len()];
        let y3 = self.buffer[(index + 2) % self.buffer.len()];

        self.cubic_interpolate(frac, y0, y1, y2, y3)
    }

    // Resample the buffer to a new size using cubic interpolation
    pub fn resample(&mut self, new_size: usize) {
        if new_size == self.buffer.len() || new_size == 0 {
            return;
        }

        let mut resampled = Vec::with_capacity(new_size);
        let scale = self.buffer.len() as f32 / new_size as f32;

        for i in 0..new_size {
            let position = (i as f32 * scale) / self.buffer.len() as f32;
            resampled.push(self.get_interpolated(position));
        }

        self.buffer = resampled;
    }

    // Update the buffer with new data and optionally resample
    pub fn update_buffer(&mut self, new_buffer: Vec<f32>, target_size: Option<usize>) {
        self.buffer = new_buffer;
        if let Some(size) = target_size {
            self.resample(size);
        }
    }

    // Get the current buffer
    pub fn buffer(&self) -> &[f32] {
        &self.buffer
    }

    // Get the buffer size
    pub fn len(&self) -> usize {
        self.buffer.len()
    }

    pub fn is_empty(&self) -> bool {
        self.buffer.is_empty()
    }
}


impl Oscillator for Wavetable {
    fn generate_sample(&self, ctx: OscillatorCtx, time: f32) -> f32 {
        let amp = ctx.amplitude;
        if self.buffer.is_empty() {
            return 0.0;
        }

        let position = self.ramp.generate_sample(ctx, time) + amp ;
        amp * self.get_interpolated(position)
    }
}


pub struct MultibandWavetable {
    // Vector of wavetables for different frequency bands
    bands: Vec<Vec<f32>>,
    // Frequencies corresponding to each band
    frequencies: Vec<f32>,
    ramp: Box<dyn Oscillator>,
}

impl MultibandWavetable {
    pub fn new(bands: Vec<Vec<f32>>, frequencies: Vec<f32>) -> Self {
        assert_eq!(bands.len(), frequencies.len(), "Number of bands must match number of frequencies");
        assert!(!bands.is_empty(), "Must provide at least one band");
        
        // Ensure all bands have the same length
        let band_size = bands[0].len();
        assert!(bands.iter().all(|b| b.len() == band_size), "All bands must have the same length");

        Self {
            bands,
            frequencies,
            ramp: Box::new(Phasor::default()),
        }
    }

    fn cubic_interpolate(&self, frac: f32, y0: f32, y1: f32, y2: f32, y3: f32) -> f32 {
        let a = -0.5 * y0 + 1.5 * y1 - 1.5 * y2 + 0.5 * y3;
        let b = y0 - 2.5 * y1 + 2.0 * y2 - 0.5 * y3;
        let c = -0.5 * y0 + 0.5 * y2;
        let d = y1;

        ((a * frac + b) * frac + c) * frac + d
    }

    fn get_band_sample(&self, band_idx: usize, position: f32) -> f32 {
        let size = self.bands[band_idx].len() as f32;
        let scaled_pos = position * size;
        let index = scaled_pos.floor() as usize;
        let frac = scaled_pos.fract();

        let band = &self.bands[band_idx];

        let y0 = band[(index + band.len() - 1) % band.len()];
        let y1 = band[index];
        let y2 = band[(index + 1) % band.len()];
        let y3 = band[(index + 2) % band.len()];

        self.cubic_interpolate(frac, y0, y1, y2, y3)
    }

    
    fn get_interpolated_sample(&self, position: f32, frequency: f32) -> f32 {
        let mut lower_idx = 0;
        let mut upper_idx = 0;
        for i in 0..self.frequencies.len() {
            if frequency <= self.frequencies[i] {
                if i == 0 {
                    return self.get_band_sample(0, position);
                }
                upper_idx = i;
                lower_idx = i - 1;
                break;
            }
        }

        if frequency > *self.frequencies.last().unwrap() {
            return self.get_band_sample(self.bands.len() - 1, position);
        }

        let lower_freq = self.frequencies[lower_idx];
        let upper_freq = self.frequencies[upper_idx];
        let frac = (frequency - lower_freq) / (upper_freq - lower_freq);

        let lower_sample = self.get_band_sample(lower_idx, position);
        let upper_sample = self.get_band_sample(upper_idx, position);

        lower_sample * (1.0 - frac) + upper_sample * frac
    }

    pub fn resample_all(&mut self, new_size: usize) {
        if new_size == 0 || (self.bands.len() > 0 && new_size == self.bands[0].len()) {
            return;
        }

        let mut resampled_bands = Vec::with_capacity(self.bands.len());

        for band in &self.bands {
            let mut resampled = Vec::with_capacity(new_size);
            let scale = band.len() as f32 / new_size as f32;

            for i in 0..new_size {
                let position = i as f32 / new_size as f32;
                let size = band.len() as f32;
                let scaled_pos = position * size;
                let index = scaled_pos.floor() as usize;
                let frac = scaled_pos.fract();

                let y0 = band[(index + band.len() - 1) % band.len()];
                let y1 = band[index];
                let y2 = band[(index + 1) % band.len()];
                let y3 = band[(index + 2) % band.len()];

                resampled.push(self.cubic_interpolate(frac, y0, y1, y2, y3));
            }

            resampled_bands.push(resampled);
        }

        self.bands = resampled_bands;
    }
}


impl Oscillator for MultibandWavetable {
    fn generate_sample(&self, ctx: OscillatorCtx, time: f32) -> f32 {
        if self.bands.is_empty() {
            return 0.0;
        }

        let position = self.ramp.generate_sample(ctx.clone(), time);
        ctx.amplitude * self.get_interpolated_sample(position, ctx.freq)
    }
}
