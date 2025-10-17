use std::{collections::HashMap, ops::Mul, u64};


use crate::synth::{Oscillator, OscillatorCtx};

pub struct Mode {
    oscillators: Vec<Box<dyn Oscillator>>,
}

impl Mode {
    pub fn new(oscillators: Vec<Box<dyn Oscillator>>) -> Self {
        Self { oscillators }
    }
}

impl Oscillator for Mode {
    fn generate_sample(&self, ctx: OscillatorCtx, time: f32) -> f32 {
        let  samples: Vec<u64> = self
          .oscillators
          .iter()
          .map(|osc| osc.generate_sample(ctx.clone(), time))
            .map(|x| (x as f64) .mul((u32::MAX - 3) as f64 ))
            .map(|x| x as u64)
          .collect();
        let mut counts = HashMap::new();
        for num in samples {
            *counts.entry(num).or_insert(0) += 1;
        }

        let mut mode = 0;
        let mut max_count = 0;
        for (&num, &count) in &counts {
            if count > max_count {
                mode = num;
                max_count = count;
            }
        }
        (u64::MAX as f64 /mode as f64) as f32
    }
}
