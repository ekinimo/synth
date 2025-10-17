use crate::synth::{Oscillator, OscillatorCtx};

pub enum MeanOscillator {
    Arithmetic(ArithmeticMean),
    WeightedArithmetic(WeightedArithmeticMean),
    Geometric(GeometricMean),
    WeightedGeometric(WeightedGeometricMean),
    Harmonic(HarmonicMean),
    WeightedHarmonic(WeightedHarmonicMean),
    Power(PowerMean),
    WeightedPower(WeightedPowerMean),
    F(FMean),
    WeightedF(WeightedFMean),
}

impl Oscillator for MeanOscillator {
    fn generate_sample(&self, ctx: OscillatorCtx, time: f32) -> f32 {
        match self {
            MeanOscillator::Arithmetic(osc) => osc.generate_sample(ctx, time),
            MeanOscillator::WeightedArithmetic(osc) => osc.generate_sample(ctx, time),
            MeanOscillator::Geometric(osc) => osc.generate_sample(ctx, time),
            MeanOscillator::WeightedGeometric(osc) => osc.generate_sample(ctx, time),
            MeanOscillator::Harmonic(osc) => osc.generate_sample(ctx, time),
            MeanOscillator::WeightedHarmonic(osc) => osc.generate_sample(ctx, time),
            MeanOscillator::Power(osc) => osc.generate_sample(ctx, time),
            MeanOscillator::WeightedPower(osc) => osc.generate_sample(ctx, time),
            MeanOscillator::F(osc) => osc.generate_sample(ctx, time),
            MeanOscillator::WeightedF(osc) => osc.generate_sample(ctx, time),
        }
    }
}
impl MeanOscillator {
    pub fn update_oscillator_at(&mut self, index: usize, oscillator: Box<dyn Oscillator>) {
        match self {
            MeanOscillator::Arithmetic(osc) => {
                if index < osc.oscillators.len() {
                    osc.oscillators[index] = oscillator;
                }
            }
            MeanOscillator::Geometric(osc) => {
                if index < osc.oscillators.len() {
                    osc.oscillators[index] = oscillator;
                }
            }
            MeanOscillator::Harmonic(osc) => {
                if index < osc.oscillators.len() {
                    osc.oscillators[index] = oscillator;
                }
            }
            MeanOscillator::WeightedArithmetic(osc) => {
                if index < osc.oscillators.len() {
                    osc.oscillators[index].0 = oscillator;
                }
            }
            MeanOscillator::WeightedGeometric(osc) => {
                if index < osc.oscillators.len() {
                    osc.oscillators[index].0 = oscillator;
                }
            }
            MeanOscillator::WeightedHarmonic(osc) => {
                if index < osc.oscillators.len() {
                    osc.oscillators[index].0 = oscillator;
                }
            }
            _ => {} // Not applicable for PowerMean and FMean variants
        }
    }

    pub fn update_weight_at(&mut self, index: usize, weight: f32) {
        match self {
            MeanOscillator::WeightedArithmetic(osc) => {
                if index < osc.oscillators.len() {
                    osc.oscillators[index].1 = weight;
                }
            }
            MeanOscillator::WeightedGeometric(osc) => {
                if index < osc.oscillators.len() {
                    osc.oscillators[index].1 = weight;
                }
            }
            MeanOscillator::WeightedHarmonic(osc) => {
                if index < osc.oscillators.len() {
                    osc.oscillators[index].1 = weight;
                }
            }
            _ => {} // Not applicable for non-weighted variants
        }
    }

    pub fn update_oscillators(&mut self, oscillators: Vec<Box<dyn Oscillator>>) {
        match self {
            MeanOscillator::Arithmetic(osc) => osc.oscillators = oscillators,
            MeanOscillator::Geometric(osc) => osc.oscillators = oscillators,
            MeanOscillator::Harmonic(osc) => osc.oscillators = oscillators,
            _ => {} // Not applicable for weighted variants
        }
    }

    pub fn update_weighted_oscillators(&mut self, oscillators: Vec<(Box<dyn Oscillator>, f32)>) {
        match self {
            MeanOscillator::WeightedArithmetic(osc) => osc.oscillators = oscillators,
            MeanOscillator::WeightedGeometric(osc) => osc.oscillators = oscillators,
            MeanOscillator::WeightedHarmonic(osc) => osc.oscillators = oscillators,
            _ => {} // Not applicable for non-weighted variants
        }
    }

    pub fn update_power(&mut self, power: f32) {
        match self {
            MeanOscillator::Power(osc) => osc.power = power,
            MeanOscillator::WeightedPower(osc) => osc.power = power,
            _ => {} // Not applicable for other variants
        }
    }

    pub fn update_f(&mut self, f: fn(f32) -> f32, f_inv: fn(f32) -> f32) {
        match self {
            MeanOscillator::F(osc) => {
                osc.f = f;
                osc.f_inv = f_inv;
            }
            MeanOscillator::WeightedF(osc) => {
                osc.f = f;
                osc.f_inv = f_inv;
            }
            _ => {} // Not applicable for other variants
        }
    }
    pub fn add_oscillator(&mut self, oscillator: Box<dyn Oscillator>) {
        match self {
            MeanOscillator::Arithmetic(osc) => osc.oscillators.push(oscillator),
            MeanOscillator::Geometric(osc) => osc.oscillators.push(oscillator),
            MeanOscillator::Harmonic(osc) => osc.oscillators.push(oscillator),
            _ => {} // Not applicable for weighted variants
        }
    }

    pub fn add_weighted_oscillator(&mut self, oscillator: Box<dyn Oscillator>, weight: f32) {
        match self {
            MeanOscillator::WeightedArithmetic(osc) => osc.oscillators.push((oscillator, weight)),
            MeanOscillator::WeightedGeometric(osc) => osc.oscillators.push((oscillator, weight)),
            MeanOscillator::WeightedHarmonic(osc) => osc.oscillators.push((oscillator, weight)),
            _ => {} // Not applicable for non-weighted variants
        }
    }

    pub fn remove_oscillator(&mut self, index: usize) {
        match self {
            MeanOscillator::Arithmetic(osc) => {
                if index < osc.oscillators.len() {
                    osc.oscillators.remove(index);
                }
            }
            MeanOscillator::Geometric(osc) => {
                if index < osc.oscillators.len() {
                    osc.oscillators.remove(index);
                }
            }
            MeanOscillator::Harmonic(osc) => {
                if index < osc.oscillators.len() {
                    osc.oscillators.remove(index);
                }
            }
            MeanOscillator::WeightedArithmetic(osc) => {
                if index < osc.oscillators.len() {
                    osc.oscillators.remove(index);
                }
            }
            MeanOscillator::WeightedGeometric(osc) => {
                if index < osc.oscillators.len() {
                    osc.oscillators.remove(index);
                }
            }
            MeanOscillator::WeightedHarmonic(osc) => {
                if index < osc.oscillators.len() {
                    osc.oscillators.remove(index);
                }
            }
            _ => {} // Not applicable for PowerMean and FMean variants
        }
    }
}
pub struct ArithmeticMean {
    oscillators: Vec<Box<dyn Oscillator>>,
}

impl Default for ArithmeticMean {
    fn default() -> Self {
        Self {
            oscillators: vec![],
        }
    }
}

impl Oscillator for ArithmeticMean {
    fn generate_sample(&self, ctx: OscillatorCtx, time: f32) -> f32 {
        let sum: f32 = self
            .oscillators
            .iter()
            .map(|osc| osc.generate_sample(ctx.clone(), time))
            .sum();
        sum / self.oscillators.len() as f32
    }
}

pub struct WeightedArithmeticMean {
    oscillators: Vec<(Box<dyn Oscillator>, f32)>,
}

impl WeightedArithmeticMean {
    pub fn new(oscillators: Vec<(Box<dyn Oscillator>, f32)>) -> Self {
        Self { oscillators }
    }
}

impl Oscillator for WeightedArithmeticMean {
    fn generate_sample(&self, ctx: OscillatorCtx, time: f32) -> f32 {
        let (sum, total_weight) =
            self.oscillators
                .iter()
                .fold((0.0, 0.0), |(acc_sum, acc_weight), (osc, weight)| {
                    (
                        acc_sum + weight * osc.generate_sample(ctx.clone(), time),
                        acc_weight + weight,
                    )
                });
        sum / total_weight
    }
}

pub struct GeometricMean {
    oscillators: Vec<Box<dyn Oscillator>>,
}

impl GeometricMean {
    pub fn new(oscillators: Vec<Box<dyn Oscillator>>) -> Self {
        Self { oscillators }
    }
}

impl Oscillator for GeometricMean {
    fn generate_sample(&self, ctx: OscillatorCtx, time: f32) -> f32 {
        let product: f32 = self
            .oscillators
            .iter()
            .map(|osc| osc.generate_sample(ctx.clone(), time))
            .product();
        product.powf(1.0 / self.oscillators.len() as f32)
    }
}

pub struct WeightedGeometricMean {
    oscillators: Vec<(Box<dyn Oscillator>, f32)>,
}

impl WeightedGeometricMean {
    pub fn new(oscillators: Vec<(Box<dyn Oscillator>, f32)>) -> Self {
        Self { oscillators }
    }
}

impl Oscillator for WeightedGeometricMean {
    fn generate_sample(&self, ctx: OscillatorCtx, time: f32) -> f32 {
        let (product, total_weight) =
            self.oscillators
                .iter()
                .fold((1.0, 0.0), |(acc_product, acc_weight), (osc, weight)| {
                    (
                        acc_product * osc.generate_sample(ctx.clone(), time).powf(*weight),
                        acc_weight + weight,
                    )
                });
        product.powf(1.0 / total_weight)
    }
}

pub struct HarmonicMean {
    oscillators: Vec<Box<dyn Oscillator>>,
}

impl HarmonicMean {
    pub fn new(oscillators: Vec<Box<dyn Oscillator>>) -> Self {
        Self { oscillators }
    }
}

impl Oscillator for HarmonicMean {
    fn generate_sample(&self, ctx: OscillatorCtx, time: f32) -> f32 {
        let sum_of_inverses: f32 = self
            .oscillators
            .iter()
            .map(|osc| 1.0 / osc.generate_sample(ctx.clone(), time))
            .sum();
        self.oscillators.len() as f32 / sum_of_inverses
    }
}

pub struct WeightedHarmonicMean {
    oscillators: Vec<(Box<dyn Oscillator>, f32)>,
}

impl WeightedHarmonicMean {
    pub fn new(oscillators: Vec<(Box<dyn Oscillator>, f32)>) -> Self {
        Self { oscillators }
    }
}

impl Oscillator for WeightedHarmonicMean {
    fn generate_sample(&self, ctx: OscillatorCtx, time: f32) -> f32 {
        let (sum_of_weighted_inverses, total_weight) =
            self.oscillators
                .iter()
                .fold((0.0, 0.0), |(acc_sum, acc_weight), (osc, weight)| {
                    (
                        acc_sum + weight / osc.generate_sample(ctx.clone(), time),
                        acc_weight + weight,
                    )
                });
        total_weight / sum_of_weighted_inverses
    }
}

pub struct PowerMean {
    oscillators: Vec<Box<dyn Oscillator>>,
    power: f32,
}

impl PowerMean {
    pub fn new(oscillators: Vec<Box<dyn Oscillator>>, power: f32) -> Self {
        Self { oscillators, power }
    }
}

impl Oscillator for PowerMean {
    fn generate_sample(&self, ctx: OscillatorCtx, time: f32) -> f32 {
        let sum: f32 = self
            .oscillators
            .iter()
            .map(|osc| osc.generate_sample(ctx.clone(), time).powf(self.power))
            .sum();
        (sum / self.oscillators.len() as f32).powf(1.0 / self.power)
    }
}

pub struct WeightedPowerMean {
    oscillators: Vec<(Box<dyn Oscillator>, f32)>,
    power: f32,
}

impl WeightedPowerMean {
    pub fn new(oscillators: Vec<(Box<dyn Oscillator>, f32)>, power: f32) -> Self {
        Self { oscillators, power }
    }
}

impl Oscillator for WeightedPowerMean {
    fn generate_sample(&self, ctx: OscillatorCtx, time: f32) -> f32 {
        let (sum, total_weight) =
            self.oscillators
                .iter()
                .fold((0.0, 0.0), |(acc_sum, acc_weight), (osc, weight)| {
                    (
                        acc_sum + weight * osc.generate_sample(ctx.clone(), time).powf(self.power),
                        acc_weight + weight,
                    )
                });
        (sum / total_weight).powf(1.0 / self.power)
    }
}

pub struct FMean {
    oscillators: Vec<Box<dyn Oscillator>>,
    f: fn(f32) -> f32,
    f_inv: fn(f32) -> f32,
}

impl FMean {
    pub fn new(
        oscillators: Vec<Box<dyn Oscillator>>,
        f: fn(f32) -> f32,
        f_inv: fn(f32) -> f32,
    ) -> Self {
        Self {
            oscillators,
            f,
            f_inv,
        }
    }
}

impl Oscillator for FMean {
    fn generate_sample(&self, ctx: OscillatorCtx, time: f32) -> f32 {
        let sum: f32 = self
            .oscillators
            .iter()
            .map(|osc| (self.f)(osc.generate_sample(ctx.clone(), time)))
            .sum();
        (self.f_inv)(sum / self.oscillators.len() as f32)
    }
}

pub struct WeightedFMean {
    oscillators: Vec<(Box<dyn Oscillator>, f32)>,
    f: fn(f32) -> f32,
    f_inv: fn(f32) -> f32,
}

impl WeightedFMean {
    pub fn new(
        oscillators: Vec<(Box<dyn Oscillator>, f32)>,
        f: fn(f32) -> f32,
        f_inv: fn(f32) -> f32,
    ) -> Self {
        Self {
            oscillators,
            f,
            f_inv,
        }
    }
}

impl Oscillator for WeightedFMean {
    fn generate_sample(&self, ctx: OscillatorCtx, time: f32) -> f32 {
        let (sum, total_weight) =
            self.oscillators
                .iter()
                .fold((0.0, 0.0), |(acc_sum, acc_weight), (osc, weight)| {
                    (
                        acc_sum + weight * (self.f)(osc.generate_sample(ctx.clone(), time)),
                        acc_weight + weight,
                    )
                });
        (self.f_inv)(sum / total_weight)
    }
}
