use crate::synth::effect::filters::biquad::BiquadFilter;
use crate::synth::effect::filters::fir::{FIRFilter,WindowType};
use crate::synth::{Effect, EffectIdx};

pub mod biquad;
pub mod fir;

pub enum Filter{
    BiQuad(BiquadFilter),
    Fir(FIRFilter),
}

impl Effect for Filter{
    fn process(&mut self, sample: f32) -> f32 {
        self.calculate(sample)
    }
    fn get_idx(&self)->EffectIdx {
        self.get_idx()
    }

}

impl Filter{
    pub fn get_idx(& self) -> EffectIdx {
        match self{
            Self::BiQuad(biquad)=>biquad.get_idx(),
            Self::Fir(fir)=>fir.get_idx(),
        }
    }


    pub fn calculate(&mut self, sample: f32) -> f32 {
        match self{
            Self::BiQuad(biquad)=>biquad.calculate(sample),
            Self::Fir(fir)=>fir.calculate(sample),
        }
    }

    pub fn frequency_response(&self, freq: f32) -> f32 {
        match self{
            Self::BiQuad(biquad)=>biquad.frequency_response(freq),
            Self::Fir(fir)=>fir.frequency_response(freq),
        }
    }
    

    pub fn modify_one_tap(&mut self, index: usize, new_tap: f32) {
        match self {
            Filter::BiQuad(_biquad_filter) => (),
            Filter::Fir(firfilter) => firfilter.modify_one_tap(index, new_tap),
        }
    }




    
}
