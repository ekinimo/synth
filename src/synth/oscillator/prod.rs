use crate::synth::{Oscillator, OscillatorCtx};

pub enum ProductOscillator {
    Product(Prod),
    WeightedProduct(WeightedProduct),
    PowerProduct(PowerProduct),
    GeneralPowerProduct(GeneralPowerProduct),
    WeightedGeneralPowerProduct(WeightedGeneralPowerProduct),
    WeightedPowerProduct(WeightedPowerProduct),
    ExpProduct(ExpProduct),
    LogProduct(LogProduct),
}

impl Oscillator for ProductOscillator {
    fn generate_sample(&self, ctx: OscillatorCtx, time: f32) -> f32 {
        match self {
            ProductOscillator::Product(osc) => osc.generate_sample(ctx, time),
            ProductOscillator::WeightedProduct(osc) => osc.generate_sample(ctx, time),
            ProductOscillator::PowerProduct(osc) => osc.generate_sample(ctx, time),
            ProductOscillator::GeneralPowerProduct(osc) => osc.generate_sample(ctx, time),
            ProductOscillator::WeightedGeneralPowerProduct(osc) => osc.generate_sample(ctx, time),
            ProductOscillator::WeightedPowerProduct(osc) => osc.generate_sample(ctx, time),
            ProductOscillator::ExpProduct(osc) => osc.generate_sample(ctx, time),
            ProductOscillator::LogProduct(osc) => osc.generate_sample(ctx, time),
        }
    }
}

impl ProductOscillator {
    pub fn update_oscillators(&mut self, oscillators: Vec<Box<dyn Oscillator>>) {
        match self {
            ProductOscillator::Product(osc) => osc.oscillators = oscillators,
            ProductOscillator::PowerProduct(osc) => osc.oscillators = oscillators,
            ProductOscillator::ExpProduct(osc) => osc.oscillators = oscillators,
            ProductOscillator::LogProduct(osc) => osc.oscillators = oscillators,
            _ => {} // Not applicable for weighted variants
        }
    }

    pub fn update_weighted_oscillators(&mut self, oscillators: Vec<(Box<dyn Oscillator>, f32)>) {
        match self {
            ProductOscillator::WeightedProduct(osc) => osc.oscillators = oscillators,
            _ => {} // Not applicable for non-weighted variants
        }
    }

    pub fn update_general_power_oscillators(&mut self, oscillators: Vec<(Box<dyn Oscillator>, f32)>) {
        match self {
            ProductOscillator::GeneralPowerProduct(osc) => osc.oscillators = oscillators,
            _ => {}
        }
    }

    pub fn update_weighted_general_power_oscillators(&mut self, oscillators: Vec<(f32, Box<dyn Oscillator>, f32)>) {
        match self {
            ProductOscillator::WeightedGeneralPowerProduct(osc) => osc.oscillators = oscillators,
            _ => {}
        }
    }

    pub fn update_power(&mut self, power: f32) {
        match self {
            ProductOscillator::PowerProduct(osc) => osc.power = power,
            ProductOscillator::WeightedPowerProduct(osc) => osc.power = power,
            _ => {} // Not applicable for other variants
        }
    }

    pub fn update_add_oscillator(&mut self, oscillator: Box<dyn Oscillator>) {
        match self {
            ProductOscillator::Product(osc) => osc.oscillators.push(oscillator),
            ProductOscillator::PowerProduct(osc) => osc.oscillators.push(oscillator),
            ProductOscillator::ExpProduct(osc) => osc.oscillators.push(oscillator),
            ProductOscillator::LogProduct(osc) => osc.oscillators.push(oscillator),
            _ => {} // Not applicable for weighted variants
        }
    }

    pub fn update_add_weighted_oscillator(&mut self, oscillator: Box<dyn Oscillator>, weight: f32) {
        match self {
            ProductOscillator::WeightedProduct(osc) => osc.oscillators.push((oscillator, weight)),
            _ => {} // Not applicable for non-weighted variants
        }
    }

    pub fn update_remove_oscillator(&mut self, index: usize) {
        match self {
            ProductOscillator::Product(osc) => {
                if index < osc.oscillators.len() {
                    osc.oscillators.remove(index);
                }
            }
            ProductOscillator::PowerProduct(osc) => {
                if index < osc.oscillators.len() {
                    osc.oscillators.remove(index);
                }
            }
            ProductOscillator::ExpProduct(osc) => {
                if index < osc.oscillators.len() {
                    osc.oscillators.remove(index);
                }
            }
            ProductOscillator::LogProduct(osc) => {
                if index < osc.oscillators.len() {
                    osc.oscillators.remove(index);
                }
            }
            ProductOscillator::WeightedProduct(osc) => {
                if index < osc.oscillators.len() {
                    osc.oscillators.remove(index);
                }
            }
            _ => {} // Not applicable for other variants
        }
    }

    pub fn update_oscillator_at(&mut self, index: usize, oscillator: Box<dyn Oscillator>) {
        match self {
            ProductOscillator::Product(osc) => {
                if index < osc.oscillators.len() {
                    osc.oscillators[index] = oscillator;
                }
            }
            ProductOscillator::PowerProduct(osc) => {
                if index < osc.oscillators.len() {
                    osc.oscillators[index] = oscillator;
                }
            }
            ProductOscillator::ExpProduct(osc) => {
                if index < osc.oscillators.len() {
                    osc.oscillators[index] = oscillator;
                }
            }
            ProductOscillator::LogProduct(osc) => {
                if index < osc.oscillators.len() {
                    osc.oscillators[index] = oscillator;
                }
            }
            ProductOscillator::WeightedProduct(osc) => {
                if index < osc.oscillators.len() {
                    osc.oscillators[index].0 = oscillator;
                }
            }
            _ => {} // Not applicable for other variants
        }
    }

    pub fn update_weight_at(&mut self, index: usize, weight: f32) {
        match self {
            ProductOscillator::WeightedProduct(osc) => {
                if index < osc.oscillators.len() {
                    osc.oscillators[index].1 = weight;
                }
            }
            _ => {} // Not applicable for non-weighted variants
        }
    }
}

pub struct Prod {
    oscillators: Vec<Box<dyn Oscillator>>,
}

impl Default for Prod {
    fn default() -> Self {
        Self {
            oscillators: vec![],
        }
    }
}

impl Oscillator for Prod {
    fn generate_sample(&self, ctx: OscillatorCtx, time: f32) -> f32 {
        self.oscillators.iter()
            .map(|osc| osc.generate_sample(ctx.clone(), time))
            .product()
    }
}


pub struct WeightedProduct {
    oscillators: Vec<(Box<dyn Oscillator>, f32)>,
}

impl WeightedProduct {
    pub fn new(oscillators: Vec<(Box<dyn Oscillator>, f32)>) -> Self {
        Self { oscillators }
    }
}

impl Oscillator for WeightedProduct {
    fn generate_sample(&self, ctx: OscillatorCtx, time: f32) -> f32 {
        self.oscillators
          .iter()
          .map(|(osc, weight)| weight * osc.generate_sample(ctx.clone(), time))
          .product()
    }
}

pub struct PowerProduct {
    oscillators: Vec<Box<dyn Oscillator>>,
    power: f32,
}

impl PowerProduct {
    pub fn new(oscillators: Vec<Box<dyn Oscillator>>, power: f32) -> Self {
        Self { oscillators, power }
    }
}

impl Oscillator for PowerProduct {
    fn generate_sample(&self, ctx: OscillatorCtx, time: f32) -> f32 {
        self.oscillators
          .iter()
          .map(|osc| osc.generate_sample(ctx.clone(), time).powf(self.power))
          .product()
    }
}

pub struct GeneralPowerProduct {
    oscillators: Vec<(Box<dyn Oscillator>, f32)>,
}

impl GeneralPowerProduct {
    pub fn new(oscillators: Vec<(Box<dyn Oscillator>, f32)>) -> Self {
        Self { oscillators }
    }
}

impl Oscillator for GeneralPowerProduct {
    fn generate_sample(&self, ctx: OscillatorCtx, time: f32) -> f32 {
        self.oscillators
          .iter()
          .map(|(osc, pow)| osc.generate_sample(ctx.clone(), time).powf(*pow))
          .product()
    }
}

pub struct WeightedGeneralPowerProduct {
    oscillators: Vec<(f32, Box<dyn Oscillator>, f32)>,
}

impl WeightedGeneralPowerProduct {
    pub fn new(oscillators: Vec<(f32, Box<dyn Oscillator>, f32)>) -> Self {
        Self { oscillators }
    }
}

impl Oscillator for WeightedGeneralPowerProduct {
    fn generate_sample(&self, ctx: OscillatorCtx, time: f32) -> f32 {
        self.oscillators
          .iter()
          .map(|(weight, osc, pow)| weight * osc.generate_sample(ctx.clone(), time).powf(*pow))
          .product()
    }
}

pub struct WeightedPowerProduct {
    oscillators: Vec<(Box<dyn Oscillator>, f32)>,
    power: f32,
}

impl WeightedPowerProduct {
    pub fn new(oscillators: Vec<(Box<dyn Oscillator>, f32)>, power: f32) -> Self {
        Self {
            oscillators,
            power,
        }
    }
}

impl Oscillator for WeightedPowerProduct {
    fn generate_sample(&self, ctx: OscillatorCtx, time: f32) -> f32 {
        self.oscillators
          .iter()
          .map(|(osc, weight)| weight * osc.generate_sample(ctx.clone(), time).powf(self.power))
          .product()
    }
}

pub struct ExpProduct {
    oscillators: Vec<Box<dyn Oscillator>>,
}

impl ExpProduct {
    pub fn new(oscillators: Vec<Box<dyn Oscillator>>) -> Self {
        Self { oscillators }
    }
}

impl Oscillator for ExpProduct {
    fn generate_sample(&self, ctx: OscillatorCtx, time: f32) -> f32 {
        self.oscillators
          .iter()
          .map(|osc| osc.generate_sample(ctx.clone(), time).exp())
          .product()
    }
}

pub struct LogProduct {
    oscillators: Vec<Box<dyn Oscillator>>,
}

impl LogProduct {
    pub fn new(oscillators: Vec<Box<dyn Oscillator>>) -> Self {
        Self { oscillators }
    }
}

impl Oscillator for LogProduct {
    fn generate_sample(&self, ctx: OscillatorCtx, time: f32) -> f32 {
        self.oscillators
          .iter()
          .map(|osc| osc.generate_sample(ctx.clone(), time).ln())
          .product()
    }
}
