use crate::synth::{Oscillator, OscillatorCtx};
use super::phasor::Phasor;


pub struct Polynomial {
    coefficients: Vec<f32>,
    ramp: Box<dyn Oscillator>,
}

impl Default for Polynomial {
    fn default() -> Self {
        Self {
            coefficients: vec![0.0, 1.0], // Linear by default
            ramp: Box::new(Phasor::default()),
        }
    }
}

impl Oscillator for Polynomial {
    fn generate_sample(&self, ctx: OscillatorCtx, time: f32) -> f32 {
        let amp = ctx.amplitude;
        let x = self.ramp.generate_sample(ctx, time);
        let val = self.coefficients.iter()
            .enumerate()
            .fold(0.0, |acc, (i, &c)| acc + c * x.powi(i as i32));
        amp * ctx.amplitude * val
    }
}
