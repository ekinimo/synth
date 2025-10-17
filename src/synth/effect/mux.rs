use std::ops::Div;

use crate::synth::{Effect, EffectIdx};

use super::EffectStack;

#[derive(Clone)]
pub enum Reducer {
    Sum,
    Mul,
    Max,
    Median,
    Min,

    PowerMean(f32),
    ArithmeticMean,
    GeometricMean,
    HarmonicMean,
}

pub struct Mux {
    idx:usize,
    channels: Vec<(f32 /*weight*/, f32 /*pow*/, EffectStack)>,
    reducer: Reducer,
}

impl Mux {

    pub fn new(idx:usize)->Self{
        Self { idx,channels: vec![], reducer: Reducer::Sum }
    }
    pub fn add_channel(&mut self,weight:f32,pow:f32){
        self.channels.push((weight,pow,EffectStack{stack:vec![]}));
    }
    pub fn add_effect(&mut self,channel:usize,eff:impl Effect  +'static){
        self.channels[channel].2.stack.push(Box::new(eff))
    }
    pub fn remove_channel(&mut self,idx:usize){
        self.channels.remove(idx);
    }
    pub fn remove_effect(&mut self,channel:usize,idx:usize){
        self.channels[channel].2.stack.remove(idx);
    }

    pub fn update_weight(&mut self,channel:usize,val:f32){
        self.channels[channel].0 = val;
    }
    pub fn update_pow(&mut self,channel:usize,val:f32){
        self.channels[channel].1 = val;
    }
    pub fn update_effect(&mut self,channel:usize,idx:usize,val:impl Effect  +'static){
        self.channels[channel].2.stack[idx] = Box::new(val);
    }
    pub fn update_reducer(&mut self, red:Reducer){
        self.reducer = red;
    }

    pub fn calculate(&mut self, sample: f32) -> f32 {
        if self.channels.is_empty(){return sample}
        let weights: f32 = self.channels.iter().map(|(w, _, _)| w).sum();
        //let pows : f32 = self.channels.iter().map(|(_,pow,_) w|).sum();

        let samples = self
            .channels
            .iter_mut()
            .map(|(weight, pow, channel)| *weight * channel.calculate(sample).powf(*pow));
        match self.reducer {
            Reducer::Sum => samples.fold(0.0, |x, y| x + y),
            Reducer::Mul => samples.fold(1.0, |x, y| x * y),
            Reducer::Max => samples.fold(f32::MIN, |x, y| x.max(y)),
            Reducer::Min => samples.fold(f32::MAX, |x, y| x.min(y)),

            Reducer::Median => {
                let mut vals: Box<[_]> = samples.collect();
                vals.sort_by(|a, b| a.partial_cmp(b).unwrap_or(std::cmp::Ordering::Equal));
                vals[vals.len() / 2]
            }
            Reducer::PowerMean(pow) => samples
                .fold(0.0, |x, y| x + y.powf(pow))
                .div(weights)
                .powf(1.0 / pow),
            Reducer::ArithmeticMean => samples.fold(0.0, |x, y| x + y).div(weights),
            Reducer::GeometricMean => samples.fold(0.0, |x, y| x + y).ln().div(weights).exp(),
            Reducer::HarmonicMean => 1.0 / samples.fold(0.0, |x, y| x + 1.0 / y).ln().div(weights),
        }
    }
}

impl Effect for Mux{
    fn process(&mut self, sample: f32) -> f32 {
        self.calculate(sample)
    }

    fn get_idx(&self)->EffectIdx {
        EffectIdx(self.idx)
    }

}
