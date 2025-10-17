use crate::synth::{Oscillator, OscillatorCtx};

pub enum MaximumOscillator {
    Maximum(Maximum),
    WeightedMaximum(WeightedMaximum),
    PowerMaximum(PowerMaximum),
    GeneralPowerMaximum(GeneralPowerMaximum),
    WeightedGeneralPowerMaximum(WeightedGeneralPowerMaximum),
    WeightedPowerMaximum(WeightedPowerMaximum),
    ExpMaximum(ExpMaximum),
    LogMaximum(LogMaximum),
}

impl Oscillator for MaximumOscillator {
    fn generate_sample(&self, ctx: OscillatorCtx, time: f32) -> f32 {
        match self {
            MaximumOscillator::Maximum(osc) => osc.generate_sample(ctx, time),
            MaximumOscillator::WeightedMaximum(osc) => osc.generate_sample(ctx, time),
            MaximumOscillator::PowerMaximum(osc) => osc.generate_sample(ctx, time),
            MaximumOscillator::GeneralPowerMaximum(osc) => osc.generate_sample(ctx, time),
            MaximumOscillator::WeightedGeneralPowerMaximum(osc) => osc.generate_sample(ctx, time),
            MaximumOscillator::WeightedPowerMaximum(osc) => osc.generate_sample(ctx, time),
            MaximumOscillator::ExpMaximum(osc) => osc.generate_sample(ctx, time),
            MaximumOscillator::LogMaximum(osc) => osc.generate_sample(ctx, time),
        }
    }
}

impl MaximumOscillator {
    pub fn update_oscillators(&mut self, oscillators: Vec<Box<dyn Oscillator>>) {
        match self {
            MaximumOscillator::Maximum(osc) => osc.oscillators = oscillators,
            MaximumOscillator::PowerMaximum(osc) => osc.oscillators = oscillators,
            MaximumOscillator::ExpMaximum(osc) => osc.oscillators = oscillators,
            MaximumOscillator::LogMaximum(osc) => osc.oscillators = oscillators,
            _ => {} // Not applicable for weighted variants
        }
    }

    pub fn update_weighted_oscillators(&mut self, oscillators: Vec<(Box<dyn Oscillator>, f32)>) {
        match self {
            MaximumOscillator::WeightedMaximum(osc) => osc.oscillators = oscillators,
            _ => {} // Not applicable for non-weighted variants
        }
    }

    pub fn update_general_power_oscillators(&mut self, oscillators: Vec<(Box<dyn Oscillator>, f32)>) {
        match self {
            MaximumOscillator::GeneralPowerMaximum(osc) => osc.oscillators = oscillators,
            _ => {}
        }
    }

    pub fn update_weighted_general_power_oscillators(&mut self, oscillators: Vec<(f32, Box<dyn Oscillator>, f32)>) {
        match self {
            MaximumOscillator::WeightedGeneralPowerMaximum(osc) => osc.oscillators = oscillators,
            _ => {}
        }
    }

    pub fn update_power(&mut self, power: f32) {
        match self {
            MaximumOscillator::PowerMaximum(osc) => osc.power = power,
            MaximumOscillator::WeightedPowerMaximum(osc) => osc.power = power,
            _ => {} // Not applicable for other variants
        }
    }

    pub fn update_add_oscillator(&mut self, oscillator: Box<dyn Oscillator>) {
        match self {
            MaximumOscillator::Maximum(osc) => osc.oscillators.push(oscillator),
            MaximumOscillator::PowerMaximum(osc) => osc.oscillators.push(oscillator),
            MaximumOscillator::ExpMaximum(osc) => osc.oscillators.push(oscillator),
            MaximumOscillator::LogMaximum(osc) => osc.oscillators.push(oscillator),
            _ => {} // Not applicable for weighted variants
        }
    }

    pub fn update_add_weighted_oscillator(&mut self, oscillator: Box<dyn Oscillator>, weight: f32) {
        match self {
            MaximumOscillator::WeightedMaximum(osc) => osc.oscillators.push((oscillator, weight)),
            _ => {} // Not applicable for non-weighted variants
        }
    }

    pub fn update_remove_oscillator(&mut self, index: usize) {
        match self {
            MaximumOscillator::Maximum(osc) => {
                if index < osc.oscillators.len() {
                    osc.oscillators.remove(index);
                }
            }
            MaximumOscillator::PowerMaximum(osc) => {
                if index < osc.oscillators.len() {
                    osc.oscillators.remove(index);
                }
            }
            MaximumOscillator::ExpMaximum(osc) => {
                if index < osc.oscillators.len() {
                    osc.oscillators.remove(index);
                }
            }
            MaximumOscillator::LogMaximum(osc) => {
                if index < osc.oscillators.len() {
                    osc.oscillators.remove(index);
                }
            }
            MaximumOscillator::WeightedMaximum(osc) => {
                if index < osc.oscillators.len() {
                    osc.oscillators.remove(index);
                }
            }
            _ => {} // Not applicable for other variants
        }
    }

    pub fn update_oscillator_at(&mut self, index: usize, oscillator: Box<dyn Oscillator>) {
        match self {
            MaximumOscillator::Maximum(osc) => {
                if index < osc.oscillators.len() {
                    osc.oscillators[index] = oscillator;
                }
            }
            MaximumOscillator::PowerMaximum(osc) => {
                if index < osc.oscillators.len() {
                    osc.oscillators[index] = oscillator;
                }
            }
            MaximumOscillator::ExpMaximum(osc) => {
                if index < osc.oscillators.len() {
                    osc.oscillators[index] = oscillator;
                }
            }
            MaximumOscillator::LogMaximum(osc) => {
                if index < osc.oscillators.len() {
                    osc.oscillators[index] = oscillator;
                }
            }
            MaximumOscillator::WeightedMaximum(osc) => {
                if index < osc.oscillators.len() {
                    osc.oscillators[index].0 = oscillator;
                }
            }
            _ => {} // Not applicable for other variants
        }
    }

    pub fn update_weight_at(&mut self, index: usize, weight: f32) {
        match self {
            MaximumOscillator::WeightedMaximum(osc) => {
                if index < osc.oscillators.len() {
                    osc.oscillators[index].1 = weight;
                }
            }
            _ => {} // Not applicable for non-weighted variants
        }
    }
}

pub struct Maximum {
    oscillators: Vec<Box<dyn Oscillator>>,
}

impl Default for Maximum {
    fn default() -> Self {
        Self {
            oscillators: vec![],
        }
    }
}

impl Oscillator for Maximum {
    fn generate_sample(&self, ctx: OscillatorCtx, time: f32) -> f32 {
        self.oscillators
          .iter()
          .map(|osc| osc.generate_sample(ctx.clone(), time))
          .max_by(|a, b| a.partial_cmp(b).unwrap_or(std::cmp::Ordering::Equal))
          .unwrap_or(0.0)
    }
}

pub struct WeightedMaximum {
    oscillators: Vec<(Box<dyn Oscillator>, f32)>,
}

impl WeightedMaximum {
    pub fn new(oscillators: Vec<(Box<dyn Oscillator>, f32)>) -> Self {
        Self { oscillators }
    }
}

impl Oscillator for WeightedMaximum {
    fn generate_sample(&self, ctx: OscillatorCtx, time: f32) -> f32 {
        self.oscillators
          .iter()
          .map(|(osc, weight)| weight * osc.generate_sample(ctx.clone(), time))
          .max_by(|a, b| a.partial_cmp(b).unwrap_or(std::cmp::Ordering::Equal))
          .unwrap_or(0.0)
    }
}

pub struct PowerMaximum {
    oscillators: Vec<Box<dyn Oscillator>>,
    power: f32,
}

impl PowerMaximum {
    pub fn new(oscillators: Vec<Box<dyn Oscillator>>, power: f32) -> Self {
        Self { oscillators, power }
    }
}

impl Oscillator for PowerMaximum {
    fn generate_sample(&self, ctx: OscillatorCtx, time: f32) -> f32 {
        self.oscillators
          .iter()
          .map(|osc| osc.generate_sample(ctx.clone(), time).powf(self.power))
          .max_by(|a, b| a.partial_cmp(b).unwrap_or(std::cmp::Ordering::Equal))
          .unwrap_or(0.0)
    }
}

pub struct GeneralPowerMaximum {
    oscillators: Vec<(Box<dyn Oscillator>, f32)>,
}

impl GeneralPowerMaximum {
    pub fn new(oscillators: Vec<(Box<dyn Oscillator>, f32)>) -> Self {
        Self { oscillators }
    }
}

impl Oscillator for GeneralPowerMaximum {
    fn generate_sample(&self, ctx: OscillatorCtx, time: f32) -> f32 {
        self.oscillators
          .iter()
          .map(|(osc, pow)| osc.generate_sample(ctx.clone(), time).powf(*pow))
          .max_by(|a, b| a.partial_cmp(b).unwrap_or(std::cmp::Ordering::Equal))
          .unwrap_or(0.0)
    }
}

pub struct WeightedGeneralPowerMaximum {
    oscillators: Vec<(f32, Box<dyn Oscillator>, f32)>,
}

impl WeightedGeneralPowerMaximum {
    pub fn new(oscillators: Vec<(f32, Box<dyn Oscillator>, f32)>) -> Self {
        Self { oscillators }
    }
}

impl Oscillator for WeightedGeneralPowerMaximum {
    fn generate_sample(&self, ctx: OscillatorCtx, time: f32) -> f32 {
        self.oscillators
          .iter()
          .map(|(weight, osc, pow)| weight * osc.generate_sample(ctx.clone(), time).powf(*pow))
          .max_by(|a, b| a.partial_cmp(b).unwrap_or(std::cmp::Ordering::Equal))
          .unwrap_or(0.0)
    }
}

pub struct WeightedPowerMaximum {
    oscillators: Vec<(Box<dyn Oscillator>, f32)>,
    power: f32,
}

impl WeightedPowerMaximum {
    pub fn new(oscillators: Vec<(Box<dyn Oscillator>, f32)>, power: f32) -> Self {
        Self {
            oscillators,
            power,
        }
    }
}

impl Oscillator for WeightedPowerMaximum {
    fn generate_sample(&self, ctx: OscillatorCtx, time: f32) -> f32 {
        self.oscillators
          .iter()
          .map(|(osc, weight)| weight * osc.generate_sample(ctx.clone(), time).powf(self.power))
          .max_by(|a, b| a.partial_cmp(b).unwrap_or(std::cmp::Ordering::Equal))
          .unwrap_or(0.0)
    }
}

pub struct ExpMaximum {
    oscillators: Vec<Box<dyn Oscillator>>,
}

impl ExpMaximum {
    pub fn new(oscillators: Vec<Box<dyn Oscillator>>) -> Self {
        Self { oscillators }
    }
}

impl Oscillator for ExpMaximum {
    fn generate_sample(&self, ctx: OscillatorCtx, time: f32) -> f32 {
        self.oscillators
          .iter()
          .map(|osc| osc.generate_sample(ctx.clone(), time).exp())
          .max_by(|a, b| a.partial_cmp(b).unwrap_or(std::cmp::Ordering::Equal))
          .unwrap_or(0.0)
    }
}

pub struct LogMaximum {
    oscillators: Vec<Box<dyn Oscillator>>,
}

impl LogMaximum {
    pub fn new(oscillators: Vec<Box<dyn Oscillator>>) -> Self {
        Self { oscillators }
    }
}

impl Oscillator for LogMaximum {
    fn generate_sample(&self, ctx: OscillatorCtx, time: f32) -> f32 {
        self.oscillators
          .iter()
          .map(|osc| osc.generate_sample(ctx.clone(), time).ln())
          .max_by(|a, b| a.partial_cmp(b).unwrap_or(std::cmp::Ordering::Equal))
          .unwrap_or(0.0)
    }
}
