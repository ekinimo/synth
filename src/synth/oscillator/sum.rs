use crate::synth::{Oscillator, OscillatorCtx};

pub enum SumOscillator {
    Sum(Sum),
    WeightedSum(WeightedSum),
    PowerSum(PowerSum),
    GeneralPowerSum(GeneralPowerSum),
    WeightedGeneralPowerSum(WeightedGeneralPowerSum),
    WeightedPowerSum(WeightedPowerSum),
    ExpSum(ExpSum),
    LogSum(LogSum),
}

impl Oscillator for SumOscillator {
    fn generate_sample(&self, ctx: OscillatorCtx, time: f32) -> f32 {
        match self {
            SumOscillator::Sum(osc) => osc.generate_sample(ctx, time),
            SumOscillator::WeightedSum(osc) => osc.generate_sample(ctx, time),
            SumOscillator::PowerSum(osc) => osc.generate_sample(ctx, time),
            SumOscillator::GeneralPowerSum(osc) => osc.generate_sample(ctx, time),
            SumOscillator::WeightedGeneralPowerSum(osc) => osc.generate_sample(ctx, time),
            SumOscillator::WeightedPowerSum(osc) => osc.generate_sample(ctx, time),
            SumOscillator::ExpSum(osc) => osc.generate_sample(ctx, time),
            SumOscillator::LogSum(osc) => osc.generate_sample(ctx, time),
        }
    }
}

impl SumOscillator {
    pub fn update_oscillators(&mut self, oscillators: Vec<Box<dyn Oscillator>>) {
        match self {
            SumOscillator::Sum(osc) => osc.oscillators = oscillators,
            SumOscillator::PowerSum(osc) => osc.oscillators = oscillators,
            SumOscillator::ExpSum(osc) => osc.oscillators = oscillators,
            SumOscillator::LogSum(osc) => osc.oscillators = oscillators,
            _ => {} // Not applicable for weighted variants
        }
    }

    pub fn update_weighted_oscillators(&mut self, oscillators: Vec<(Box<dyn Oscillator>, f32)>) {
        match self {
            SumOscillator::WeightedSum(osc) => osc.oscillators = oscillators,
            _ => {} // Not applicable for non-weighted variants
        }
    }

    pub fn update_general_power_oscillators(&mut self, oscillators: Vec<(Box<dyn Oscillator>, f32)>) {
        match self {
            SumOscillator::GeneralPowerSum(osc) => osc.oscillators = oscillators,
            _ => {}
        }
    }

    pub fn update_weighted_general_power_oscillators(&mut self, oscillators: Vec<(f32, Box<dyn Oscillator>, f32)>) {
        match self {
            SumOscillator::WeightedGeneralPowerSum(osc) => osc.oscillators = oscillators,
            _ => {}
        }
    }

    pub fn update_power(&mut self, power: f32) {
        match self {
            SumOscillator::PowerSum(osc) => osc.power = power,
            SumOscillator::WeightedPowerSum(osc) => osc.power = power,
            _ => {} // Not applicable for other variants
        }
    }

    pub fn update_add_oscillator(&mut self, oscillator: Box<dyn Oscillator>) {
        match self {
            SumOscillator::Sum(osc) => osc.oscillators.push(oscillator),
            SumOscillator::PowerSum(osc) => osc.oscillators.push(oscillator),
            SumOscillator::ExpSum(osc) => osc.oscillators.push(oscillator),
            SumOscillator::LogSum(osc) => osc.oscillators.push(oscillator),
            _ => {} // Not applicable for weighted variants
        }
    }

    pub fn update_add_weighted_oscillator(&mut self, oscillator: Box<dyn Oscillator>, weight: f32) {
        match self {
            SumOscillator::WeightedSum(osc) => osc.oscillators.push((oscillator, weight)),
            _ => {} // Not applicable for non-weighted variants
        }
    }

    pub fn update_remove_oscillator(&mut self, index: usize) {
        match self {
            SumOscillator::Sum(osc) => {
                if index < osc.oscillators.len() {
                    osc.oscillators.remove(index);
                }
            }
            SumOscillator::PowerSum(osc) => {
                if index < osc.oscillators.len() {
                    osc.oscillators.remove(index);
                }
            }
            SumOscillator::ExpSum(osc) => {
                if index < osc.oscillators.len() {
                    osc.oscillators.remove(index);
                }
            }
            SumOscillator::LogSum(osc) => {
                if index < osc.oscillators.len() {
                    osc.oscillators.remove(index);
                }
            }
            SumOscillator::WeightedSum(osc) => {
                if index < osc.oscillators.len() {
                    osc.oscillators.remove(index);
                }
            }
            _ => {} // Not applicable for other variants
        }
    }

    pub fn update_oscillator_at(&mut self, index: usize, oscillator: Box<dyn Oscillator>) {
        match self {
            SumOscillator::Sum(osc) => {
                if index < osc.oscillators.len() {
                    osc.oscillators[index] = oscillator;
                }
            }
            SumOscillator::PowerSum(osc) => {
                if index < osc.oscillators.len() {
                    osc.oscillators[index] = oscillator;
                }
            }
            SumOscillator::ExpSum(osc) => {
                if index < osc.oscillators.len() {
                    osc.oscillators[index] = oscillator;
                }
            }
            SumOscillator::LogSum(osc) => {
                if index < osc.oscillators.len() {
                    osc.oscillators[index] = oscillator;
                }
            }
            SumOscillator::WeightedSum(osc) => {
                if index < osc.oscillators.len() {
                    osc.oscillators[index].0 = oscillator;
                }
            }
            _ => {} // Not applicable for other variants
        }
    }

    pub fn update_weight_at(&mut self, index: usize, weight: f32) {
        match self {
            SumOscillator::WeightedSum(osc) => {
                if index < osc.oscillators.len() {
                    osc.oscillators[index].1 = weight;
                }
            }
            _ => {} // Not applicable for non-weighted variants
        }
    }
}

pub struct Sum {
    oscillators: Vec<Box<dyn Oscillator>>,
}

impl Default for Sum {
    fn default() -> Self {
        Self {
            oscillators: vec![],
        }
    }
}

impl Oscillator for Sum {
    fn generate_sample(&self, ctx: OscillatorCtx, time: f32) -> f32 {
        self.oscillators.iter()
            .map(|osc| osc.generate_sample(ctx.clone(), time))
            .sum()
    }
}



pub struct WeightedSum {
    oscillators: Vec<(Box<dyn Oscillator>, f32)>,
}

impl WeightedSum {
    pub fn new(oscillators: Vec<(Box<dyn Oscillator>, f32)>) -> Self {
        Self { oscillators }
    }
}

impl Oscillator for WeightedSum {
    fn generate_sample(&self, ctx: OscillatorCtx, time: f32) -> f32 {
        self.oscillators
            .iter()
            .map(|(osc, weight)| weight * osc.generate_sample(ctx.clone(), time))
            .sum()
    }
}


pub struct PowerSum {
    oscillators: Vec<Box<dyn Oscillator>>,
    power: f32,
}

impl PowerSum {
    pub fn new(oscillators: Vec<Box<dyn Oscillator>>, power: f32) -> Self {
        Self { oscillators, power }
    }
}

impl Oscillator for PowerSum {
    fn generate_sample(&self, ctx: OscillatorCtx, time: f32) -> f32 {
        self.oscillators
            .iter()
            .map(|osc| osc.generate_sample(ctx.clone(), time).powf(self.power))
            .sum()
    }
}

pub struct GeneralPowerSum {
    oscillators: Vec<(Box<dyn Oscillator>,f32)>,
}

impl GeneralPowerSum {
    pub fn new(oscillators: Vec<(Box<dyn Oscillator>,f32)>) -> Self {
        Self { oscillators }
    }
}

impl Oscillator for GeneralPowerSum {
    fn generate_sample(&self, ctx: OscillatorCtx, time: f32) -> f32 {
        self.oscillators
            .iter()
            .map(|(osc,pow)| osc.generate_sample(ctx.clone(), time).powf(*pow))
            .sum()
    }
}

pub struct WeightedGeneralPowerSum {
    oscillators: Vec<(f32,Box<dyn Oscillator>,f32)>,
}

impl WeightedGeneralPowerSum {
    pub fn new(oscillators: Vec<(f32,Box<dyn Oscillator>,f32)>) -> Self {
        Self { oscillators }
    }
}

impl Oscillator for WeightedGeneralPowerSum {
    fn generate_sample(&self, ctx: OscillatorCtx, time: f32) -> f32 {
        self.oscillators
            .iter()
            .map(|(weight,osc,pow)| weight*osc.generate_sample(ctx.clone(), time).powf(*pow))
            .sum()
    }
}


pub struct WeightedPowerSum {
    oscillators: Vec<(Box<dyn Oscillator>, f32)>,
    power: f32,
}

impl WeightedPowerSum {
    pub fn new(oscillators: Vec<(Box<dyn Oscillator>, f32)>, power: f32) -> Self {
        Self {
            oscillators,
            power,
        }
    }
}

impl Oscillator for WeightedPowerSum {
    fn generate_sample(&self, ctx: OscillatorCtx, time: f32) -> f32 {
        self.oscillators
            .iter()
            .map(|(osc, weight)| weight * osc.generate_sample(ctx.clone(), time).powf(self.power))
            .sum()
    }
}



pub struct ExpSum {
    oscillators: Vec<Box<dyn Oscillator>>,
}

impl ExpSum {
    pub fn new(oscillators: Vec<Box<dyn Oscillator>>) -> Self {
        Self { oscillators }
    }
}

impl Oscillator for ExpSum {
    fn generate_sample(&self, ctx: OscillatorCtx, time: f32) -> f32 {
        self.oscillators
            .iter()
            .map(|osc| osc.generate_sample(ctx.clone(), time).exp())
            .sum()
    }
}

pub struct LogSum {
    oscillators: Vec<Box<dyn Oscillator>>,
}

impl LogSum {
    pub fn new(oscillators: Vec<Box<dyn Oscillator>>) -> Self {
        Self { oscillators }
    }
}

impl Oscillator for LogSum {
    fn generate_sample(&self, ctx: OscillatorCtx, time: f32) -> f32 {
        self.oscillators
            .iter()
            .map(|osc| osc.generate_sample(ctx.clone(), time).ln())
            .sum()
    }
}

