use super::phasor::Phasor;
use crate::synth::{Oscillator, OscillatorCtx};

pub enum TerracedModifier{
    
}

pub struct Terraced {
    ramp: Box<dyn Oscillator>,
    levels: Vec<f32>,
}

impl Default for Terraced {
    fn default() -> Self {
        Self {
            ramp: Box::new(Phasor::default()),
            levels: vec![-1.0, 1.0],
        }
    }
}


impl Oscillator for Terraced {
    fn generate_sample(&self, ctx: OscillatorCtx, time: f32) -> f32 {
        let amp = ctx.amplitude;
        let phase = self.ramp.generate_sample(ctx, time);
        let index = (phase * self.levels.len() as f32).floor() as usize % self.levels.len();
        amp * self.levels[index]
    }
}

