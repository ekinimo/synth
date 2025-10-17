pub mod delay;
pub mod distortion;
pub mod filters;
pub mod gain;
pub mod id;
pub mod lfo;
pub mod mux;
pub mod waveshaping;
use delay::{ChorusElem, Delay, DelayLike};
use distortion::Distortion;
use filters::Filter;
use gain::Gain;
use id::Identitiy;
use lfo::LFOLike;
use mux::{Mux, Reducer};
use waveshaping::{WaveFormers, WaveShaping};

use crate::synth::Effect;

pub struct EffectStack {
    pub stack: Vec<Box<dyn Effect>>,
}

impl EffectStack {
    pub fn calculate(&mut self, sample: f32) -> f32 {
        self.stack
            .iter_mut()
            .fold(sample, |acc, effect| effect.process(acc))
    }
}
use crate::synth::effect::filters::fir::WindowType;

use super::{EffectIdx, Oscillator};

pub enum AllEffects {
    Filters(Filter),
    Delay(DelayLike),
    Distortion(Distortion),
    Waveshaping(WaveShaping),
    Lfo(LFOLike),
    Mux(Mux),
    Id(Identitiy),
    Gain(Gain),
}

impl Effect for AllEffects{
    fn process(&mut self, sample: f32) -> f32 {
        self.calculate(sample)
    }

    fn get_idx(&self)->EffectIdx {
        self.get_idx()
    }
    
}

impl AllEffects {
    pub fn calculate(&mut self, sample: f32) -> f32 {
        match self {
            AllEffects::Filters(filter) => filter.calculate(sample),
            AllEffects::Delay(delay) => delay.calculate(sample),
            AllEffects::Distortion(distortion) => distortion.calculate(sample),
            AllEffects::Waveshaping(wave_shaping) => wave_shaping.calculate(sample),
            AllEffects::Lfo(lfolike) => lfolike.calculate(sample),
            AllEffects::Mux(mux) => mux.calculate(sample),
            AllEffects::Id(identitiy) => identitiy.calculate(sample),
            AllEffects::Gain(gain) => gain.calculate(sample),
        }
    }

    pub fn get_idx(&self)->EffectIdx{
        match self {
            AllEffects::Filters(filter) => filter.get_idx(),
            AllEffects::Delay(delay) => delay.get_idx(),
            AllEffects::Distortion(distortion) => distortion.get_idx(),
            AllEffects::Waveshaping(wave_shaping) => wave_shaping.get_idx(),
            AllEffects::Lfo(lfolike) => lfolike.get_idx(),
            AllEffects::Mux(mux) => mux.get_idx(),
            AllEffects::Id(identitiy) => identitiy.get_idx(),
            AllEffects::Gain(gain) => gain.get_idx(),
        }
    }
}
