use std::{collections::HashMap, ops::Mul, u64};
use crate::synth::{Oscillator, OscillatorCtx};


pub enum RoundingOscillator {
    Round(Round),
    Floor(Floor),
    Ceil(Ceil),
}

impl Oscillator for RoundingOscillator {
    fn generate_sample(&self, ctx: OscillatorCtx, time: f32) -> f32 {
        match self {
            RoundingOscillator::Round(osc) => osc.generate_sample(ctx, time),
            RoundingOscillator::Floor(osc) => osc.generate_sample(ctx, time),
            RoundingOscillator::Ceil(osc) => osc.generate_sample(ctx, time),
        }
    }
}


pub struct Round {
    oscillator: Box<dyn Oscillator>,
}

impl Round {
    pub fn new(oscillator: Box<dyn Oscillator>) -> Self {
        Self { oscillator }
    }
}

impl Oscillator for Round {
    fn generate_sample(&self, ctx: OscillatorCtx, time: f32) -> f32 {
        self.oscillator.generate_sample(ctx, time).round()
    }
}

pub struct Floor {
    oscillator: Box<dyn Oscillator>,
}

impl Floor {
    pub fn new(oscillator: Box<dyn Oscillator>) -> Self {
        Self { oscillator }
    }
}

impl Oscillator for Floor {
    fn generate_sample(&self, ctx: OscillatorCtx, time: f32) -> f32 {
        self.oscillator.generate_sample(ctx, time).floor()
    }
}

pub struct Ceil {
    oscillator: Box<dyn Oscillator>,
}

impl Ceil {
    pub fn new(oscillator: Box<dyn Oscillator>) -> Self {
        Self { oscillator }
    }
}

impl Oscillator for Ceil {
    fn generate_sample(&self, ctx: OscillatorCtx, time: f32) -> f32 {
        self.oscillator.generate_sample(ctx, time).ceil()
    }
}
