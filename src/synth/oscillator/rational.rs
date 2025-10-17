use crate::synth::{Oscillator, OscillatorCtx};
use super::phasor::Phasor;


pub struct Rational {
    numerator: Vec<f32>,
    denominator: Vec<f32>,
    ramp: Box<dyn Oscillator>,
}

impl Default for Rational {
    fn default() -> Self {
        Self {
            numerator: vec![0.0, 1.0],
            denominator: vec![1.0],
            ramp: Box::new(Phasor::default()),
        }
    }
}

impl Oscillator for Rational {
    fn generate_sample(&self, ctx: OscillatorCtx, time: f32) -> f32 {
        let amp = ctx.amplitude;
        let x = self.ramp.generate_sample(ctx, time);
        let num = self.numerator.iter()
            .enumerate()
            .fold(0.0, |acc, (i, &c)| acc + c * x.powi(i as i32));
        let den = self.denominator.iter()
            .enumerate()
            .fold(0.0, |acc, (i, &c)| acc + c * x.powi(i as i32));
        amp * num / den
    }
}
