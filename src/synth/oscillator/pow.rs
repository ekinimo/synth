use super::phasor::Phasor;
use crate::synth::{Oscillator, OscillatorCtx};


pub struct Pow {
    a: Box<dyn Oscillator>,
    b: Box<dyn Oscillator>,
}



impl Oscillator for Pow {
    fn generate_sample(&self, ctx: OscillatorCtx, time: f32) -> f32 {
        let amp = ctx.amplitude;
        let phase1 = self.a.generate_sample(ctx, time) + amp ;
        let phase2 = self.b.generate_sample(ctx, time)  ;
        
        phase1.powf(phase2)
    }
}


