use super::phasor::Phasor;
use crate::synth::{Oscillator, OscillatorCtx};

pub struct Exponential {
    ramp: Box<dyn Oscillator>,
    base: f32,
}

impl Default for Exponential {
    fn default() -> Self {
        Self {
            ramp: Box::new(Phasor::default()),
            base: std::f32::consts::E,
        }
    }
}

impl Oscillator for Exponential {
    fn generate_sample(&self, ctx: OscillatorCtx, time: f32) -> f32 {
        let amp = ctx.amplitude;
        let x = self.ramp.generate_sample(ctx, time);
        amp * self.base.powf(x)
    }
}
