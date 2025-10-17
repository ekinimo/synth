use std::ops::Mul;

pub enum WaveShaping {
    WaveFormer(WaveFormers),
    BitCrusher(BitCrusher),
}

impl WaveShaping {

    pub fn get_idx(& self)->EffectIdx {
        match self {
            Self::WaveFormer(form) => form.get_idx(),
            Self::BitCrusher(crusher) => crusher.get_idx(),
        }
    }
    pub fn calculate(&mut self, sample: f32) -> f32 {
        match self {
            Self::WaveFormer(former) => former.calculate(sample),
            Self::BitCrusher(crusher) => crusher.calculate(sample),
        }
    }
    pub fn update_depth(&mut self, bit_depth: u8) {
        match self {
            Self::WaveFormer(_) => (),
            Self::BitCrusher(crusher) => crusher.update_depth(bit_depth),
        }
    }

    pub fn update_factor(&mut self, factor: u32) {
        match self {
            Self::WaveFormer(_) => (),
            Self::BitCrusher(crusher) => crusher.update_factor(factor),
        }
    }
    pub fn update_polynomial(&mut self, factor: Vec<f32>) {
        match self {
            Self::WaveFormer(former) => former.update_polynomial(factor),
            Self::BitCrusher(_) => (),
        }
    }
    pub fn update_polynomial_coeff(&mut self, idx: usize, factor: f32) {
        match self {
            Self::WaveFormer(former) => former.update_polynomial_coeff(idx, factor),
            Self::BitCrusher(_) => (),
        }
    }
    pub fn add_polynomial_coeff(&mut self, factor: f32) {
        match self {
            Self::WaveFormer(former) => former.add_polynomial_coeff(factor),
            Self::BitCrusher(_) => (),
        }
    }
    pub fn update_coef(&mut self, factor: f32) {
        match self {
            Self::WaveFormer(former) => former.update_coef(factor),
            Self::BitCrusher(_) => (),
        }
    }
    pub fn add_waveform(&mut self, form: WaveFormers) {
        match self {
            Self::WaveFormer(former) => former.add_waveform(form),
            Self::BitCrusher(_) => (),
        }
    }
    pub fn update_first_waveform(&mut self, form: WaveFormers) {
        match self {
            Self::WaveFormer(former) => former.update_first_waveform(form),
            Self::BitCrusher(_) => (),
        }
    }
    pub fn update_second_waveform(&mut self, form: WaveFormers) {
        match self {
            Self::WaveFormer(former) => former.update_second_waveform(form),
            Self::BitCrusher(_) => (),
        }
    }
    pub fn update_nth_waveform(&mut self, idx: usize, form: WaveFormers) {
        match self {
            Self::WaveFormer(former) => former.update_nth_waveform(idx, form),
            Self::BitCrusher(_) => (),
        }
    }
}
impl Effect for WaveShaping {
    fn process(&mut self, sample: f32) -> f32 {
        self.calculate(sample)
    }

    fn get_idx(&self)->EffectIdx {
        self.get_idx()
    }

}

pub struct WaveFormers{
    idx:usize,
    fun:Funs
}

pub enum Funs {
    Mul(f32),
    Power(f32),
    Polynomial(Vec<f32>),

    Sqrt(f32),
    Exp(f32),
    Log(f32),
    Sin(f32),
    Cos(f32),
    Tan(f32),
    Sinh(f32),
    Cosh(f32),
    Tanh(f32),
    ASin(f32),
    ACos(f32),
    ATan(f32),
    ASinh(f32),
    ACosh(f32),
    ATanh(f32),

    Composite(Vec<WaveFormers>),
    Sum(Vec<WaveFormers>),
    Max(Vec<WaveFormers>),
    Min(Vec<WaveFormers>),
    Mean(Vec<WaveFormers>),
    Median(Vec<WaveFormers>),
    Product(Vec<WaveFormers>),
    PowerOfTowWaves(Box<(WaveFormers, WaveFormers)>),
    Division(Box<(WaveFormers, WaveFormers)>),
}
use std::ops::Add;

use crate::synth::Effect;
use crate::synth::EffectIdx;
impl WaveFormers {
    pub fn get_idx(&self)->EffectIdx{
        EffectIdx(self.idx)
    }
    pub fn update_polynomial(&mut self, factor: Vec<f32>) {
        match self {
            Self{fun:Funs::Polynomial(vec),..}
                => {
                *vec = factor;
            }
            _ => {}
        }
    }
    pub fn update_polynomial_coeff(&mut self, idx: usize, factor: f32) {
        match self {
            Self{fun:Funs::Polynomial(vec) ,..}=> {
                vec[idx] = factor;
            }
            _ => {}
        }
    }
    pub fn add_polynomial_coeff(&mut self, factor: f32) {
        match self {
            WaveFormers{fun:Funs::Polynomial(vec),..} => {
                vec.push(factor);
            }
            _ => {}
        }
    }
    pub fn update_coef(&mut self, factor: f32) {
        match self {
            WaveFormers{fun:Funs::Mul(x),..}
            | WaveFormers{fun:Funs::Power(x),..}
            | WaveFormers{fun:Funs::Sqrt(x),..}
            | WaveFormers{fun:Funs::Exp(x),..}
            | WaveFormers{fun:Funs::Log(x),..}
            | WaveFormers{fun:Funs::Sin(x),..}
            | WaveFormers{fun:Funs::Cos(x),..}
            | WaveFormers{fun:Funs::Tan(x),..}
            | WaveFormers{fun:Funs::Sinh(x),..}
            | WaveFormers{fun:Funs::Cosh(x),..}
            | WaveFormers{fun:Funs::Tanh(x),..}
            | WaveFormers{fun:Funs::ASin(x),..}
            | WaveFormers{fun:Funs::ACos(x),..}
            | WaveFormers{fun:Funs::ATan(x),..}
            | WaveFormers{fun:Funs::ASinh(x),..}
            | WaveFormers{fun:Funs::ACosh(x),..}
            | WaveFormers{fun:Funs::ATanh(x),..} => {
                *x = factor;
            }
            _ => (),
        }
    }
    pub fn add_waveform(&mut self, form: WaveFormers) {
        match self {
            WaveFormers{fun:Funs::Composite(vec),..}
            | WaveFormers{fun:Funs::Sum(vec),..}
            | WaveFormers{fun:Funs::Max(vec),..}
            | WaveFormers{fun:Funs::Min(vec),..}
            | WaveFormers{fun:Funs::Mean(vec),..}
            | WaveFormers{fun:Funs::Median(vec),..}
            | WaveFormers{fun:Funs::Product(vec),..}
                => vec.push(form),
            _ => (),
        }
    }
    pub fn update_first_waveform(&mut self, form: WaveFormers) {
        match self {
            WaveFormers{fun:Funs::PowerOfTowWaves(x),..}
            | WaveFormers{fun:Funs::Division(x),..}
                => {
                let (ref mut a, ref mut b) = &mut **x;
                *a = form;
            }
            _ => (),
        }
    }
    pub fn update_second_waveform(&mut self, form: WaveFormers) {
        match self {
            WaveFormers{fun:Funs::PowerOfTowWaves(x),..}
            | WaveFormers{fun:Funs::Division(x),..}
                => {
                let (ref mut a, ref mut b) = &mut **x;
                *b = form;
            }
            _ => (),
        }
    }
    pub fn update_nth_waveform(&mut self, idx: usize, form: WaveFormers) {
        match self {
            WaveFormers{fun:Funs::Composite(vec),..}
            | WaveFormers{fun:Funs::Sum(vec),..}
            | WaveFormers{fun:Funs::Max(vec),..}
            | WaveFormers{fun:Funs::Min(vec),..}
            | WaveFormers{fun:Funs::Mean(vec),..}
            | WaveFormers{fun:Funs::Median(vec),..}
            | WaveFormers{fun:Funs::Product(vec),..} => {
                vec[idx] = form;
            }
            _ => (),
        }
    }

    fn calculate(&mut self, sample: f32) -> f32 {
        match self {
            WaveFormers{fun:Funs::Mul(k),..}
                => sample * *k,
            WaveFormers{fun:Funs::Power(k),..}
                => sample.powf(*k),
            WaveFormers{fun:Funs::Sqrt(k),..}
                => (sample * *k).abs().sqrt(),
            WaveFormers{fun:Funs::Exp(k),..}
                => (sample * *k).exp(),
            WaveFormers{fun:Funs::Log(k),..}
                => sample.abs().add(0.01).log(*k),
            WaveFormers{fun:Funs::Sin(k),..}
                => (sample * *k).sin(),
            WaveFormers{fun:Funs::Cos(k),..}
                => (sample * *k).cos(),
            WaveFormers{fun:Funs::Tan(k),..}
                => (sample * *k).tan(),
            WaveFormers{fun:Funs::Sinh(k),..}
                => (sample * *k).sinh(),
            WaveFormers{fun:Funs::Cosh(k),..}
                => (sample * *k).cosh(),
            WaveFormers{fun:Funs::Tanh(k),..}
                => (sample * *k).tanh(),
            WaveFormers{fun:Funs::ASin(k),..}
                => (sample * *k).clamp(-1.0, 1.0).asin(),
            WaveFormers{fun:Funs::ACos(k),..}
                => (sample * *k).clamp(-1.0, 1.0).acos(),
            WaveFormers{fun:Funs::ATan(k),..}
                => (sample * *k).atan(),
            WaveFormers{fun:Funs::ASinh(k),..}
                => (sample * *k).asinh(),
            WaveFormers{fun:Funs::ACosh(k),..}
                => (sample * *k).acosh(),
            WaveFormers{fun:Funs::ATanh(k),..}
                => (sample * *k).atanh(),
            WaveFormers{fun:Funs::Polynomial(vec),..}
                => vec
                .iter()
                .enumerate()
                .map(|(pow, coeff)| sample.mul(coeff).powi(pow as i32))
                .sum(),
            WaveFormers{fun:Funs::Composite(vec),..}
                => vec
                .iter_mut()
                .fold(sample, |acc, former| former.calculate(sample)),
            WaveFormers{fun:Funs::Sum(vec),..}
                => vec
                .iter_mut()
                .fold(0.0, |acc, former| acc + former.calculate(sample)),
            WaveFormers{fun:Funs::Max(vec),..}
                => vec
                .iter_mut()
                .fold(f32::MIN, |acc, former| former.calculate(sample).max(acc)),
            WaveFormers{fun:Funs::Min(vec),..}
                => vec
                .iter_mut()
                .fold(f32::MAX, |acc, former| former.calculate(sample).min(acc)),
            WaveFormers{fun:Funs::Mean(vec),..}
                => {
                vec.iter_mut()
                    .fold(0.0, |acc, former| acc + former.calculate(sample))
                    / (vec.len() as f32)
            }
            WaveFormers{fun:Funs::Median(vec),..}
                => {
                let mut vals: Box<[_]> = vec
                    .iter_mut()
                    .map(|former| former.calculate(sample))
                    .collect();
                vals.sort_by(|a, b| a.partial_cmp(b).unwrap_or(std::cmp::Ordering::Equal));
                vals[vals.len() / 2]
            }
            WaveFormers{fun:Funs::Product(vec),..}
                => vec
                .iter_mut()
                .fold(1.0, |acc, former| acc * former.calculate(sample)),

            WaveFormers{fun:Funs::PowerOfTowWaves(boxed),..}
                => {
                let (ref mut a, ref mut b) = &mut **boxed;
                a.calculate(sample).powf(b.calculate(sample))
            }
            WaveFormers{fun:Funs::Division(boxed),..}
                => {
                let (ref mut a, ref mut b) = &mut **boxed;
                a.calculate(sample) / b.calculate(sample)
            }
        }
    }
}

pub struct BitCrusher {
    idx:usize,
    bit_depth: u8,
    downsample_factor: u32,
    downsample_counter: u32,
    held_sample: f32,
}

impl BitCrusher {
    pub fn get_idx(&self)->EffectIdx{
        EffectIdx(self.idx)
    }
    pub fn new(idx:usize,bit_depth: u8, downsample_factor: u32) -> Self {
        let bit_depth = bit_depth.clamp(1, 16);
        let downsample_factor = downsample_factor.clamp(1, 1024);
        Self {
            idx,
            bit_depth,
            downsample_factor,
            downsample_counter: 0,
            held_sample: 0.0,
        }
    }

    pub fn calculate(&mut self, sample: f32) -> f32 {
        if self.downsample_counter == 0 {
            let quantized = self.bit_reduce(sample);
            self.held_sample = quantized;
            self.downsample_counter = self.downsample_factor;
        }
        self.downsample_counter -= 1;
        self.held_sample
    }

    fn bit_reduce(&self, sample: f32) -> f32 {
        if self.bit_depth == 1 {
            return if sample >= 0.0 { 1.0 } else { -1.0 };
        }

        if self.bit_depth >= 16 {
            return sample;
        }

        let max_val = (1 << (self.bit_depth - 1)) as f32;
        let scaled = sample * max_val;
        let rounded = scaled.round();
        let clamped = rounded.clamp(-max_val, max_val - 1.0);
        clamped / max_val
    }

    pub fn update_depth(&mut self, bit_depth: u8) {
        self.bit_depth = bit_depth.clamp(1, 16);
    }

    pub fn update_factor(&mut self, factor: u32) {
        self.downsample_factor = factor.clamp(1, 1024);
    }

    pub fn modify(&mut self, bit_depth: u8, factor: u32) {
        self.bit_depth = bit_depth.clamp(1, 16);
        self.downsample_factor = factor.clamp(1, 1024);
    }
}

impl Effect for BitCrusher {
    fn process(&mut self, sample: f32) -> f32 {
        self.calculate(sample)
    }
    fn get_idx(&self)->EffectIdx{
        EffectIdx(self.idx)
    }
}

impl Effect for WaveFormers {
    fn process(&mut self, sample: f32) -> f32 {
        self.calculate(sample)
    }
    fn get_idx(&self)->EffectIdx{
        EffectIdx(self.idx)
    }
}
