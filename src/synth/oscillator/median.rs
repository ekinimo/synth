use crate::synth::{Oscillator, OscillatorCtx};

pub struct Median {
    oscillators: Vec<Box<dyn Oscillator>>,
}

impl Default for Median {
    fn default() -> Self {
        Self {
            oscillators: vec![],
        }
    }
}

impl Oscillator for Median {
    fn generate_sample(&self, ctx: OscillatorCtx, time: f32) -> f32 {
        let mut samples: Vec<f32> = self
          .oscillators
          .iter()
          .map(|osc| osc.generate_sample(ctx.clone(), time))
          .collect();
        samples.sort_by(|a, b| a.partial_cmp(b).unwrap_or(std::cmp::Ordering::Equal));

        let len = samples.len();
        if len == 0 {
            0.0
        } else if len % 2 == 0 {
            (samples[len / 2 - 1] + samples[len / 2]) / 2.0
        } else {
            samples[len / 2]
        }
    }
}

