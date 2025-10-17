/*use crate::synth::{Synth,Oscillator};
pub trait Parameter<Osc>: Send + Sync  {
    fn update_param(&self, osc: &mut Osc) ;
}





#[derive(Debug,Clone, Copy)]
pub struct Volume(pub f32);
#[derive(Debug,Clone, Copy)]
pub struct LowestFreq(pub f32);
#[derive(Debug,Clone, Copy)]
pub struct OctaveScale(pub f32);
#[derive(Debug,Clone, Copy)]
pub struct NumOfNotes(pub u8);
#[derive(Clone, Copy)]
pub struct SetOsc<T: Oscillator + Clone>(pub T);

//pub struct PushEffect;
// pub struct RemoveEffect(usize);
// pub struct SwapEffect(usize, usize);
//pub struct SetOscParam<T: Sized + Parameter<Box<dyn >>>(pub T );



impl <T:Sized+Oscillator+Clone+'static> Parameter<Synth> for SetOsc<T> {
fn update_param(&self, osc: &mut Synth) {
    osc.oscillator = Box::new(self.0.clone()) as Box<dyn Oscillator>
}
}

impl Parameter<Synth> for Volume {
fn update_param(&self, osc: &mut Synth) {
    let Volume(num) = self;
    osc.volume = *num;
}
}


impl Parameter<Synth> for OctaveScale {
fn update_param(&self, osc: &mut Synth) {
    let OctaveScale(num) = self;
    osc.octave_scale = *num;
}
}

impl Parameter<Synth> for LowestFreq {
fn update_param(&self, osc: &mut Synth) {
    let LowestFreq(num) = self;
    osc.lowest_freq = *num;
}
}


impl Parameter<Synth> for NumOfNotes {
    fn update_param(&self, osc: &mut Synth) {
        let NumOfNotes(num) = self;
        osc.num_of_notes = *num;
    }
}
*/
