use crate::synth::{Effect, EffectIdx};

pub struct Identitiy(usize);

impl Identitiy{
    pub fn calculate(&mut self,sample:f32)->f32{
        sample
    }
    pub fn get_idx(&self)->EffectIdx{
        EffectIdx(self.0)
    }
}

impl Effect for Identitiy{
    fn process(&mut self, sample: f32) -> f32 {
        self.calculate(sample)
    }

    fn get_idx(&self)->EffectIdx {
        self.get_idx()
    }
    
}
