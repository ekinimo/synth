use crate::synth::{Oscillator, OscillatorCtx};
use super::phasor::Phasor;


pub struct Gaussian {
    ramp: Box<dyn Oscillator>,
    sigma: f32,
}

impl Default for Gaussian {
    fn default() -> Self {
        Self {
            ramp: Box::new(Phasor::default()),
            sigma: 0.4,
        }
    }
}

impl Oscillator for Gaussian {
    fn generate_sample(&self, ctx: OscillatorCtx, time: f32) -> f32 {
        let amp = ctx.amplitude;
        let x = 2.0 * self.ramp.generate_sample(ctx, time) - 1.0; // Map to [-1, 1]
        amp * (-x.powi(2) / (2.0 * self.sigma.powi(2))).exp()
    }
}
