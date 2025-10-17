use crate::synth::{Oscillator, OscillatorCtx};


pub enum MinimumOscillator {
    Minimum(Minimum),
    WeightedMinimum(WeightedMinimum),
    PowerMinimum(PowerMinimum),
    GeneralPowerMinimum(GeneralPowerMinimum),
    WeightedGeneralPowerMinimum(WeightedGeneralPowerMinimum),
    WeightedPowerMinimum(WeightedPowerMinimum),
    ExpMinimum(ExpMinimum),
    LogMinimum(LogMinimum),
}

impl Oscillator for MinimumOscillator {
    fn generate_sample(&self, ctx: OscillatorCtx, time: f32) -> f32 {
        match self {
            MinimumOscillator::Minimum(osc) => osc.generate_sample(ctx, time),
            MinimumOscillator::WeightedMinimum(osc) => osc.generate_sample(ctx, time),
            MinimumOscillator::PowerMinimum(osc) => osc.generate_sample(ctx, time),
            MinimumOscillator::GeneralPowerMinimum(osc) => osc.generate_sample(ctx, time),
            MinimumOscillator::WeightedGeneralPowerMinimum(osc) => osc.generate_sample(ctx, time),
            MinimumOscillator::WeightedPowerMinimum(osc) => osc.generate_sample(ctx, time),
            MinimumOscillator::ExpMinimum(osc) => osc.generate_sample(ctx, time),
            MinimumOscillator::LogMinimum(osc) => osc.generate_sample(ctx, time),
        }
    }
}

impl MinimumOscillator {
    pub fn update_oscillators(&mut self, oscillators: Vec<Box<dyn Oscillator>>) {
        match self {
            MinimumOscillator::Minimum(osc) => osc.oscillators = oscillators,
            MinimumOscillator::PowerMinimum(osc) => osc.oscillators = oscillators,
            MinimumOscillator::ExpMinimum(osc) => osc.oscillators = oscillators,
            MinimumOscillator::LogMinimum(osc) => osc.oscillators = oscillators,
            _ => {} // Not applicable for weighted variants
        }
    }

    pub fn update_weighted_oscillators(&mut self, oscillators: Vec<(Box<dyn Oscillator>, f32)>) {
        match self {
            MinimumOscillator::WeightedMinimum(osc) => osc.oscillators = oscillators,
            _ => {} // Not applicable for non-weighted variants
        }
    }

    pub fn update_general_power_oscillators(&mut self, oscillators: Vec<(Box<dyn Oscillator>, f32)>) {
        match self {
            MinimumOscillator::GeneralPowerMinimum(osc) => osc.oscillators = oscillators,
            _ => {} 
        }
    }

    pub fn update_weighted_general_power_oscillators(&mut self, oscillators: Vec<(f32, Box<dyn Oscillator>, f32)>) {
        match self {
            MinimumOscillator::WeightedGeneralPowerMinimum(osc) => osc.oscillators = oscillators,
            _ => {} 
        }
    }

    pub fn update_power(&mut self, power: f32) {
        match self {
            MinimumOscillator::PowerMinimum(osc) => osc.power = power,
            MinimumOscillator::WeightedPowerMinimum(osc) => osc.power = power,
            _ => {} // Not applicable for other variants
        }
    }

    pub fn update_add_oscillator(&mut self, oscillator: Box<dyn Oscillator>) {
        match self {
            MinimumOscillator::Minimum(osc) => osc.oscillators.push(oscillator),
            MinimumOscillator::PowerMinimum(osc) => osc.oscillators.push(oscillator),
            MinimumOscillator::ExpMinimum(osc) => osc.oscillators.push(oscillator),
            MinimumOscillator::LogMinimum(osc) => osc.oscillators.push(oscillator),
            _ => {} // Not applicable for weighted variants
        }
    }

    pub fn update_add_weighted_oscillator(&mut self, oscillator: Box<dyn Oscillator>, weight: f32) {
        match self {
            MinimumOscillator::WeightedMinimum(osc) => osc.oscillators.push((oscillator, weight)),
            _ => {} // Not applicable for non-weighted variants
        }
    }

    pub fn update_remove_oscillator(&mut self, index: usize) {
        match self {
            MinimumOscillator::Minimum(osc) => {
                if index < osc.oscillators.len() {
                    osc.oscillators.remove(index);
                }
            }
            MinimumOscillator::PowerMinimum(osc) => {
                if index < osc.oscillators.len() {
                    osc.oscillators.remove(index);
                }
            }
            MinimumOscillator::ExpMinimum(osc) => {
                if index < osc.oscillators.len() {
                    osc.oscillators.remove(index);
                }
            }
            MinimumOscillator::LogMinimum(osc) => {
                if index < osc.oscillators.len() {
                    osc.oscillators.remove(index);
                }
            }
            MinimumOscillator::WeightedMinimum(osc) => {
                if index < osc.oscillators.len() {
                    osc.oscillators.remove(index);
                }
            }
            _ => {} // Not applicable for other variants
        }
    }

    pub fn update_oscillator_at(&mut self, index: usize, oscillator: Box<dyn Oscillator>) {
        match self {
            MinimumOscillator::Minimum(osc) => {
                if index < osc.oscillators.len() {
                    osc.oscillators[index] = oscillator;
                }
            }
            MinimumOscillator::PowerMinimum(osc) => {
                if index < osc.oscillators.len() {
                    osc.oscillators[index] = oscillator;
                }
            }
            MinimumOscillator::ExpMinimum(osc) => {
                if index < osc.oscillators.len() {
                    osc.oscillators[index] = oscillator;
                }
            }
            MinimumOscillator::LogMinimum(osc) => {
                if index < osc.oscillators.len() {
                    osc.oscillators[index] = oscillator;
                }
            }
            MinimumOscillator::WeightedMinimum(osc) => {
                if index < osc.oscillators.len() {
                    osc.oscillators[index].0 = oscillator;
                }
            }
            _ => {} // Not applicable for other variants
        }
    }

    pub fn update_weight_at(&mut self, index: usize, weight: f32) {
        match self {
            MinimumOscillator::WeightedMinimum(osc) => {
                if index < osc.oscillators.len() {
                    osc.oscillators[index].1 = weight;
                }
            }
            _ => {} // Not applicable for non-weighted variants
        }
    }
}


pub struct Minimum {
    oscillators: Vec<Box<dyn Oscillator>>,
}

impl Default for Minimum {
    fn default() -> Self {
        Self {
            oscillators: vec![],
        }
    }
}

impl Oscillator for Minimum {
    fn generate_sample(&self, ctx: OscillatorCtx, time: f32) -> f32 {
        self.oscillators
          .iter()
          .map(|osc| osc.generate_sample(ctx.clone(), time))
          .min_by(|a, b| a.partial_cmp(b).unwrap_or(std::cmp::Ordering::Equal))
          .unwrap_or(0.0)
    }
}

pub struct WeightedMinimum {
    oscillators: Vec<(Box<dyn Oscillator>, f32)>,
}

impl WeightedMinimum {
    pub fn new(oscillators: Vec<(Box<dyn Oscillator>, f32)>) -> Self {
        Self { oscillators }
    }
}

impl Oscillator for WeightedMinimum {
    fn generate_sample(&self, ctx: OscillatorCtx, time: f32) -> f32 {
        self.oscillators
          .iter()
          .map(|(osc, weight)| weight * osc.generate_sample(ctx.clone(), time))
          .min_by(|a, b| a.partial_cmp(b).unwrap_or(std::cmp::Ordering::Equal))
          .unwrap_or(0.0)
    }
}

pub struct PowerMinimum {
    oscillators: Vec<Box<dyn Oscillator>>,
    power: f32,
}

impl PowerMinimum {
    pub fn new(oscillators: Vec<Box<dyn Oscillator>>, power: f32) -> Self {
        Self { oscillators, power }
    }
}

impl Oscillator for PowerMinimum {
    fn generate_sample(&self, ctx: OscillatorCtx, time: f32) -> f32 {
        self.oscillators
          .iter()
          .map(|osc| osc.generate_sample(ctx.clone(), time).powf(self.power))
          .min_by(|a, b| a.partial_cmp(b).unwrap_or(std::cmp::Ordering::Equal))
          .unwrap_or(0.0)
    }
}

pub struct GeneralPowerMinimum {
    oscillators: Vec<(Box<dyn Oscillator>, f32)>,
}

impl GeneralPowerMinimum {
    pub fn new(oscillators: Vec<(Box<dyn Oscillator>, f32)>) -> Self {
        Self { oscillators }
    }
}

impl Oscillator for GeneralPowerMinimum {
    fn generate_sample(&self, ctx: OscillatorCtx, time: f32) -> f32 {
        self.oscillators
          .iter()
          .map(|(osc, pow)| osc.generate_sample(ctx.clone(), time).powf(*pow))
          .min_by(|a, b| a.partial_cmp(b).unwrap_or(std::cmp::Ordering::Equal))
          .unwrap_or(0.0)
    }
}

pub struct WeightedGeneralPowerMinimum {
    oscillators: Vec<(f32, Box<dyn Oscillator>, f32)>,
}

impl WeightedGeneralPowerMinimum {
    pub fn new(oscillators: Vec<(f32, Box<dyn Oscillator>, f32)>) -> Self {
        Self { oscillators }
    }
}

impl Oscillator for WeightedGeneralPowerMinimum {
    fn generate_sample(&self, ctx: OscillatorCtx, time: f32) -> f32 {
        self.oscillators
          .iter()
          .map(|(weight, osc, pow)| weight * osc.generate_sample(ctx.clone(), time).powf(*pow))
          .min_by(|a, b| a.partial_cmp(b).unwrap_or(std::cmp::Ordering::Equal))
          .unwrap_or(0.0)
    }
}

pub struct WeightedPowerMinimum {
    oscillators: Vec<(Box<dyn Oscillator>, f32)>,
    power: f32,
}

impl WeightedPowerMinimum {
    pub fn new(oscillators: Vec<(Box<dyn Oscillator>, f32)>, power: f32) -> Self {
        Self {
            oscillators,
            power,
        }
    }
}

impl Oscillator for WeightedPowerMinimum {
    fn generate_sample(&self, ctx: OscillatorCtx, time: f32) -> f32 {
        self.oscillators
          .iter()
          .map(|(osc, weight)| weight * osc.generate_sample(ctx.clone(), time).powf(self.power))
          .min_by(|a, b| a.partial_cmp(b).unwrap_or(std::cmp::Ordering::Equal))
          .unwrap_or(0.0)
    }
}

pub struct ExpMinimum {
    oscillators: Vec<Box<dyn Oscillator>>,
}

impl ExpMinimum {
    pub fn new(oscillators: Vec<Box<dyn Oscillator>>) -> Self {
        Self { oscillators }
    }
}

impl Oscillator for ExpMinimum {
    fn generate_sample(&self, ctx: OscillatorCtx, time: f32) -> f32 {
        self.oscillators
          .iter()
          .map(|osc| osc.generate_sample(ctx.clone(), time).exp())
          .min_by(|a, b| a.partial_cmp(b).unwrap_or(std::cmp::Ordering::Equal))
          .unwrap_or(0.0)
    }
}

pub struct LogMinimum {
    oscillators: Vec<Box<dyn Oscillator>>,
}

impl LogMinimum {
    pub fn new(oscillators: Vec<Box<dyn Oscillator>>) -> Self {
        Self { oscillators }
    }
}

impl Oscillator for LogMinimum {
    fn generate_sample(&self, ctx: OscillatorCtx, time: f32) -> f32 {
        self.oscillators
          .iter()
          .map(|osc| osc.generate_sample(ctx.clone(), time).ln())
          .min_by(|a, b| a.partial_cmp(b).unwrap_or(std::cmp::Ordering::Equal))
          .unwrap_or(0.0)
    }
}
