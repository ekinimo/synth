use crate::synth::{Effect, EffectIdx};

pub struct Gain{
    idx:usize,
    gain:f32
}

impl Gain{
    pub fn new(idx:usize,gain:f32)->Self{
        Self { idx,gain }
    }

    pub fn get_idx(&self)->EffectIdx{
        EffectIdx(self.idx)
    }
    pub fn calculate(&mut self,sample:f32)->f32{
        self.gain * sample
    }

    pub fn update_gain(&mut self,gain:f32){
        self.gain = gain;
    }
}

impl Effect for Gain{
    fn process(&mut self, sample: f32) -> f32 {
        self.calculate(sample)
    }
    fn get_idx(&self)->EffectIdx {
        self.get_idx()
    }

}
