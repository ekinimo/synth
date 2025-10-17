use std::f32::consts::PI;

use crate::synth::{Effect, EffectIdx};

pub struct Biquad {
    pub idx:usize,
    pub b0: f32,
    pub b1: f32,
    pub b2: f32,
    pub a1: f32,
    pub a2: f32,
    x1: f32,
    x2: f32,
    y1: f32,
    y2: f32,
}

pub struct LowPass {
    pub idx:usize,
    pub freq: f32,
    pub q: f32,

    filter: Biquad,
}

pub struct HighPass {
    pub idx:usize,
    pub freq: f32,
    pub q: f32,

    filter: Biquad,
}

pub struct BandPass {
    pub idx:usize,
    pub freq: f32,
    pub q: f32,

    filter: Biquad,
}

pub struct Notch {
    pub idx:usize,
    pub freq: f32,
    pub q: f32,
    pub gain: f32,

    filter: Biquad,
}

pub struct Peaking {
    pub idx:usize,
    pub freq: f32,
    pub q: f32,
    pub gain: f32,

    filter: Biquad,
}

pub struct LowShelf {
    pub idx:usize,
    pub freq: f32,
    pub q: f32,
    pub gain: f32,
    pub s: f32,

    filter: Biquad,
}

pub struct HighShelf {
    pub idx:usize,
    pub freq: f32,
    pub q: f32,
    pub gain: f32,
    pub s: f32,

    filter: Biquad,
}

impl LowPass {
    pub fn get_idx(&self)->EffectIdx{
        EffectIdx(self.idx)
    }
    pub fn frequency_response(&self, freq: f32) -> f32 {
        self.filter.frequency_response(freq)
    }

    pub fn calculate(&mut self, sample: f32) -> f32 {
        self.filter.calculate(sample)
    }

    
    pub fn modify(&mut self, freq: f32, q: f32) {
        let omega = 2.0 * PI * freq / 44100.0;
        let sn = omega.sin();
        let cs = omega.cos();
        let alpha = sn / (2.0 * q);

        let a0 = 1.0 + alpha;

        let b0 = (1.0 - cs) / 2.0 / a0;
        let b1 = (1.0 - cs) / a0;
        let b2 = (1.0 - cs) / 2.0 / a0;
        let a1 = (-2.0 * cs) / a0;
        let a2 = (1.0 - alpha) / a0;

        self.freq = freq;
        self.q = q;
        self.filter.modify(b0, b1, b2, a1, a2);
    }

    pub fn new(idx:usize,freq: f32, q: f32) -> Self {
        let omega = 2.0 * PI * freq / 44100.0;
        let sn = omega.sin();
        let cs = omega.cos();
        let alpha = sn / (2.0 * q);

        let a0 = 1.0 + alpha;

        let b0 = (1.0 - cs) / 2.0 / a0;
        let b1 = (1.0 - cs) / a0;
        let b2 = (1.0 - cs) / 2.0 / a0;
        let a1 = (-2.0 * cs) / a0;
        let a2 = (1.0 - alpha) / a0;
        let x1 = 0.0;
        let x2 = 0.0;
        let y1 = 0.0;
        let y2 = 0.0;
        Self {
            idx,
            freq,
            q,
            filter: Biquad {
                idx:0,
                b0,
                b1,
                b2,
                a1,
                a2,
                x1,
                x2,
                y1,
                y2,
            },
        }
    }

    pub fn set_filter(&mut self, filter: Biquad) {
        self.filter = filter;
    }
}

impl HighPass {
    pub fn get_idx(&self)->EffectIdx{
        EffectIdx(self.idx)
    }
    pub fn frequency_response(&self, freq: f32) -> f32 {
        self.filter.frequency_response(freq)
    }

    pub fn calculate(&mut self, sample: f32) -> f32 {
        self.filter.calculate(sample)
    }

    
    pub fn modify(&mut self, freq: f32, q: f32) {
        let omega = 2.0 * PI * freq / 44100.0;
        let sn = omega.sin();
        let cs = omega.cos();
        let alpha = sn / (2.0 * q);

        let a0 = 1.0 + alpha;

        let b0 = (1.0 + cs) / 2.0 / a0;
        let b1 = -(1.0 + cs) / a0;
        let b2 = (1.0 + cs) / 2.0 / a0;
        let a1 = (-2.0 * cs) / a0;
        let a2 = (1.0 - alpha) / a0;

        self.freq = freq;
        self.q = q;
        self.filter.modify(b0, b1, b2, a1, a2);
    }
    pub fn new(idx:usize,freq: f32, q: f32) -> Self {
        let omega = 2.0 * PI * freq / 44100.0;
        let sn = omega.sin();
        let cs = omega.cos();
        let alpha = sn / (2.0 * q);

        let a0 = 1.0 + alpha;

        let b0 = (1.0 + cs) / 2.0 / a0;
        let b1 = -(1.0 + cs) / a0;
        let b2 = (1.0 + cs) / 2.0 / a0;
        let a1 = (-2.0 * cs) / a0;
        let a2 = (1.0 - alpha) / a0;
        let filter = Biquad::new(0,b0, b1, b2, a1, a2);

        Self { idx,freq, q, filter }
    }
}

impl BandPass {
    pub fn get_idx(&self)->EffectIdx{
        EffectIdx(self.idx)
    }
    pub fn frequency_response(&self, freq: f32) -> f32 {
        self.filter.frequency_response(freq)
    }

    pub fn calculate(&mut self, sample: f32) -> f32 {
        self.filter.calculate(sample)
    }
    
    pub fn modify(&mut self, freq: f32, q: f32) {
        let omega = 2.0 * PI * freq / 44100.0;
        let sn = omega.sin();
        let cs = omega.cos();
        let alpha = sn / (2.0 * q);

        let a0 = 1.0 + alpha;

        let b0 = (q * alpha) / a0;
        let b1 = 0.0 / a0;
        let b2 = (-q * alpha) / a0;
        let a1 = (-2.0 * cs) / a0;
        let a2 = (1.0 - alpha) / a0;

        self.freq = freq;
        self.q = q;
        self.filter.modify(b0, b1, b2, a1, a2);
    }
    pub fn new(idx:usize, freq: f32, q: f32) -> Self {
        let omega = 2.0 * PI * freq / 44100.0;
        let sn = omega.sin();
        let cs = omega.cos();
        let alpha = sn / (2.0 * q);

        let a0 = 1.0 + alpha;

        let b0 = (q * alpha) / a0;
        let b1 = 0.0 / a0;
        let b2 = (-q * alpha) / a0;
        let a1 = (-2.0 * cs) / a0;
        let a2 = (1.0 - alpha) / a0;
        let filter = Biquad::new(0,b0, b1, b2, a1, a2);

        Self { idx,freq, q, filter }
    }
}

impl Notch {
    pub fn frequency_response(&self, freq: f32) -> f32 {
        self.filter.frequency_response(freq)
    }

    pub fn calculate(&mut self, sample: f32) -> f32 {
        self.filter.calculate(sample)
    }
    pub fn get_idx(&self)->EffectIdx{
        EffectIdx(self.idx)
    }
    

    pub fn modify(&mut self, freq: f32, q: f32, gain: f32) {
        let a = 10.0f32.powf(gain / 40.0);

        let omega = 2.0 * PI * freq / 44100.0;
        let sn = omega.sin();
        let cs = omega.cos();
        let alpha = sn / (2.0 * q);
        let a0 = 1.0 + alpha / a;

        let b0 = (1.0 + alpha * a) / a0;
        let b1 = (-2.0 * cs) / a0;
        let b2 = (1.0 - alpha * a) / a0;
        let a1 = (-2.0 * cs) / a0;
        let a2 = (1.0 - alpha / a) / a0;

        self.freq = freq;
        self.q = q;
        self.gain = gain;
        self.filter.modify(b0, b1, b2, a1, a2);
    }

    pub fn new(idx:usize, freq: f32, q: f32, gain: f32) -> Self {
        let a = 10.0f32.powf(gain / 40.0);

        let omega = 2.0 * PI * freq / 44100.0;
        let sn = omega.sin();
        let cs = omega.cos();
        let alpha = sn / (2.0 * q);
        let a0 = 1.0 + alpha / a;

        let b0 = (1.0 + alpha * a) / a0;
        let b1 = (-2.0 * cs) / a0;
        let b2 = (1.0 - alpha * a) / a0;
        let a1 = (-2.0 * cs) / a0;
        let a2 = (1.0 - alpha / a) / a0;
        let filter = Biquad::new(0,b0, b1, b2, a1, a2);

        Self {
            idx,
            freq,
            q,
            gain,
            filter,
        }
    }
}

impl Peaking {
    pub fn get_idx(&self)->EffectIdx{
        EffectIdx(self.idx)
    }
    pub fn frequency_response(&self, freq: f32) -> f32 {
        self.filter.frequency_response(freq)
    }

    pub fn calculate(&mut self, sample: f32) -> f32 {
        self.filter.calculate(sample)
    }
    

    pub fn modify(&mut self, freq: f32, q: f32, gain: f32) {
        let a = 10.0f32.powf(gain / 40.0);
        let omega = 2.0 * PI * freq / 44100.0;
        let sn = omega.sin();
        let cs = omega.cos();
        let alpha = sn / (2.0 * q);

        let a0 = 1.0 + alpha / a;
        let b0 = (1.0 + alpha * a) / a0;
        let b1 = (-2.0 * cs) / a0;
        let b2 = (1.0 - alpha * a) / a0;
        let a1 = (-2.0 * cs) / a0;
        let a2 = (1.0 - alpha / a) / a0;

        self.freq = freq;
        self.q = q;
        self.gain = gain;
        self.filter.modify(b0, b1, b2, a1, a2);
    }

    pub fn new(idx:usize, freq: f32, q: f32, gain: f32) -> Self {
        let a = 10.0f32.powf(gain / 40.0);
        let omega = 2.0 * PI * freq / 44100.0;
        let sn = omega.sin();
        let cs = omega.cos();
        let alpha = sn / (2.0 * q);

        let a0 = 1.0 + alpha / a;

        let b0 = (1.0 + alpha * a) / a0;
        let b1 = (-2.0 * cs) / a0;
        let b2 = (1.0 - alpha * a) / a0;
        let a1 = (-2.0 * cs) / a0;
        let a2 = (1.0 - alpha / a) / a0;
        let filter = Biquad::new(0,b0, b1, b2, a1, a2);

        Self {
            idx,
            freq,
            q,
            gain,
            filter,
        }
    }
}

impl LowShelf {
    pub fn frequency_response(&self, freq: f32) -> f32 {
        self.filter.frequency_response(freq)
    }

    pub fn calculate(&mut self, sample: f32) -> f32 {
        self.filter.calculate(sample)
    }
    pub fn get_idx(&self)->EffectIdx{
        EffectIdx(self.idx)
    }

    pub fn modify(&mut self, freq: f32, q: f32, gain: f32, s: f32) {
        let a = 10.0f32.powf(gain / 40.0);

        let omega = 2.0 * PI * freq / 44100.0;
        let sn = omega.sin();
        let cs = omega.cos();
        let alpha = sn / 2.0 * ((a + 1.0 / a) * (1.0 / s - 1.0) + 2.0).sqrt();
        let beta = 2.0 * (a).sqrt() * alpha;
        let a0 = (a + 1.0) + (a - 1.0) * cs + beta;

        let b0 = (a * ((a + 1.0) - (a - 1.0) * cs + beta)) / a0;
        let b1 = (2.0 * a * ((a - 1.0) - (a + 1.0) * cs)) / a0;
        let b2 = (a * ((a + 1.0) - (a - 1.0) * cs - beta)) / a0;
        let a1 = (-2.0 * ((a - 1.0) + (a + 1.0) * cs)) / a0;
        let a2 = ((a + 1.0) + (a - 1.0) * cs - beta) / a0;

        self.freq = freq;
        self.q = q;
        self.gain = gain;
        self.s = s;
        self.filter.modify(b0, b1, b2, a1, a2);
    }

    pub fn new( idx:usize,freq: f32, q: f32, gain: f32, s: f32) -> Self {
        let a = 10.0f32.powf(gain / 40.0);

        let omega = 2.0 * PI * freq / 44100.0;
        let sn = omega.sin();
        let cs = omega.cos();
        let alpha = sn / 2.0 * ((a + 1.0 / a) * (1.0 / s - 1.0) + 2.0).sqrt();
        let beta = 2.0 * (a).sqrt() * alpha;
        let a0 = (a + 1.0) + (a - 1.0) * cs + beta;

        let b0 = (a * ((a + 1.0) - (a - 1.0) * cs + beta)) / a0;
        let b1 = (2.0 * a * ((a - 1.0) - (a + 1.0) * cs)) / a0;
        let b2 = (a * ((a + 1.0) - (a - 1.0) * cs - beta)) / a0;
        let a1 = (-2.0 * ((a - 1.0) + (a + 1.0) * cs)) / a0;
        let a2 = ((a + 1.0) + (a - 1.0) * cs - beta) / a0;
        let filter = Biquad::new(0,b0, b1, b2, a1, a2);

        Self {
            idx,
            freq,
            q,
            gain,
            s,
            filter,
        }
    }

    pub fn set_filter(&mut self, filter: Biquad) {
        self.filter = filter;
    }
}

impl HighShelf {
    pub fn frequency_response(&self, freq: f32) -> f32 {
        self.filter.frequency_response(freq)
    }

    pub fn calculate(&mut self, sample: f32) -> f32 {
        self.filter.calculate(sample)
    }
    pub fn get_idx(&self)->EffectIdx{
        EffectIdx(self.idx)
    }

    pub fn modify(&mut self, freq: f32, q: f32, gain: f32, s: f32) {
        let a = 10.0f32.powf(gain / 40.0);

        let omega = 2.0 * PI * freq / 44100.0;
        let sn = omega.sin();
        let cs = omega.cos();
        let alpha = sn / 2.0 * ((a + 1.0 / a) * (1.0 / s - 1.0) + 2.0).sqrt();
        let beta = 2.0 * (a).sqrt() * alpha;
        let a0 = (a + 1.0) + (a - 1.0) * cs + beta;

        let b0 = (a * ((a + 1.0) + (a - 1.0) * cs + beta)) / a0;
        let b1 = (-2.0 * a * ((a - 1.0) + (a + 1.0) * cs)) / a0;
        let b2 = (a * ((a + 1.0) + (a - 1.0) * cs - beta)) / a0;
        let a1 = (2.0 * ((a - 1.0) - (a + 1.0) * cs)) / a0;
        let a2 = ((a + 1.0) - (a - 1.0) * cs - beta) / a0;

        self.freq = freq;
        self.q = q;
        self.gain = gain;
        self.s = s;
        self.filter.modify(b0, b1, b2, a1, a2);
    }

    pub fn new(idx:usize,freq: f32, q: f32, gain: f32, s: f32) -> Self {
        let a = 10.0f32.powf(gain / 40.0);

        let omega = 2.0 * PI * freq / 44100.0;
        let sn = omega.sin();
        let cs = omega.cos();
        let alpha = sn / 2.0 * ((a + 1.0 / a) * (1.0 / s - 1.0) + 2.0).sqrt();
        let beta = 2.0 * (a).sqrt() * alpha;
        let a0 = (a + 1.0) + (a - 1.0) * cs + beta;

        let b0 = (a * ((a + 1.0) + (a - 1.0) * cs + beta)) / a0;
        let b1 = (-2.0 * a * ((a - 1.0) + (a + 1.0) * cs)) / a0;
        let b2 = (a * ((a + 1.0) + (a - 1.0) * cs - beta)) / a0;
        let a1 = (2.0 * ((a - 1.0) - (a + 1.0) * cs)) / a0;
        let a2 = ((a + 1.0) - (a - 1.0) * cs - beta) / a0;
        let filter = Biquad::new(0,b0, b1, b2, a1, a2);
        Self {
            idx,
            freq,
            q,
            gain,
            s,
            filter,
        }
    }
}

impl Biquad {

    pub fn get_idx(&self)->EffectIdx{
        EffectIdx(self.idx)
    }
    pub fn frequency_response(&self, freq: f32) -> f32 {
        let omega = 2.0 * PI * freq / 44100.0;
        let z_re = omega.cos();
        let z_im = omega.sin();

        let numerator = self.b0 + self.b1 * z_re + self.b2 * (z_re * z_re - z_im * z_im);
        let denominator = 1.0 + self.a1 * z_re + self.a2 * (z_re * z_re - z_im * z_im);

        (numerator * numerator + (self.b1 * z_im + 2.0 * self.b2 * z_re * z_im).powi(2))
            .sqrt()
            / (denominator * denominator
                + (self.a1 * z_im + 2.0 * self.a2 * z_re * z_im).powi(2))
            .sqrt()
    }
    pub fn modify(&mut self, b0: f32, b1: f32, b2: f32, a1: f32, a2: f32) {
        self.b0 = b0;
        self.b1 = b1;
        self.b2 = b2;
        self.a1 = a1;
        self.a2 = a2;
    }
    pub fn new(idx:usize,b0: f32, b1: f32, b2: f32, a1: f32, a2: f32) -> Self {
        Biquad {
            idx,
            b0,
            b1,
            b2,
            a1,
            a2,
            x1: 0.0,
            x2: 0.0,
            y1: 0.0,
            y2: 0.0,
        }
    }

    pub fn calculate(&mut self, sample: f32) -> f32 {
        let output = self.b0 * sample + self.b1 * self.x1 + self.b2 * self.x2
            - self.a1 * self.y1
            - self.a2 * self.y2;

        self.x2 = self.x1;
        self.x1 = sample;
        self.y2 = self.y1;
        self.y1 = output;

        output
    }
}

impl Effect for Biquad {
    fn process(&mut self, sample: f32) -> f32 {
        self.calculate(sample)
    }
    fn get_idx(&self)->EffectIdx {
        self.get_idx()
    }

}
impl Effect for LowPass {
    fn process(&mut self, sample: f32) -> f32 {
        self.calculate(sample)
    }
    fn get_idx(&self)->EffectIdx {
        self.get_idx()
    }

}

impl Effect for HighPass {
    fn process(&mut self, sample: f32) -> f32 {
        self.calculate(sample)
    }
    fn get_idx(&self)->EffectIdx {
        self.get_idx()
    }

}
impl Effect for BandPass {
    fn process(&mut self, sample: f32) -> f32 {
        self.calculate(sample)
    }
    fn get_idx(&self)->EffectIdx {
        self.get_idx()
    }

}
impl Effect for Notch {
    fn process(&mut self, sample: f32) -> f32 {
        self.calculate(sample)
    }
    fn get_idx(&self)->EffectIdx {
        self.get_idx()
    }

}

impl Effect for Peaking {
    fn process(&mut self, sample: f32) -> f32 {
        self.calculate(sample)
    }
    fn get_idx(&self)->EffectIdx {
        self.get_idx()
    }

}
impl Effect for LowShelf {
    fn process(&mut self, sample: f32) -> f32 {
        self.calculate(sample)
    }
    fn get_idx(&self)->EffectIdx {
        self.get_idx()
    }

}
impl Effect for HighShelf {
    fn process(&mut self, sample: f32) -> f32 {
        self.calculate(sample)
    }
    fn get_idx(&self)->EffectIdx {
        self.get_idx()
    }

}

pub enum BiquadFilter {
    Biquad(Biquad),
    LowPass(LowPass),
    HighPass(HighPass),
    BandPass(BandPass),
    Notch(Notch),
    Peaking(Peaking),
    LowShelf(LowShelf),
    HighShelf(HighShelf),
}

impl Effect for BiquadFilter {
    fn process(&mut self, sample: f32) -> f32 {
        match self {
            BiquadFilter::Biquad(biquad) => biquad.calculate(sample),
            BiquadFilter::LowPass(low_pass) => low_pass.calculate(sample),
            BiquadFilter::HighPass(high_pass) => high_pass.calculate(sample),
            BiquadFilter::BandPass(band_pass) => band_pass.calculate(sample),
            BiquadFilter::Notch(notch) => notch.calculate(sample),
            BiquadFilter::Peaking(peaking) => peaking.calculate(sample),
            BiquadFilter::LowShelf(low_shelf) => low_shelf.calculate(sample),
            BiquadFilter::HighShelf(high_shelf) => high_shelf.calculate(sample),
        }
    }
    fn get_idx(&self)->EffectIdx {
        self.get_idx()
    }
}

impl BiquadFilter {
    pub fn calculate(&mut self, sample: f32) -> f32 {
        match self {
            BiquadFilter::Biquad(biquad) => biquad.calculate(sample),
            BiquadFilter::LowPass(low_pass) => low_pass.calculate(sample),
            BiquadFilter::HighPass(high_pass) => high_pass.calculate(sample),
            BiquadFilter::BandPass(band_pass) => band_pass.calculate(sample),
            BiquadFilter::Notch(notch) => notch.calculate(sample),
            BiquadFilter::Peaking(peaking) => peaking.calculate(sample),
            BiquadFilter::LowShelf(low_shelf) => low_shelf.calculate(sample),
            BiquadFilter::HighShelf(high_shelf) => high_shelf.calculate(sample),
        }
    }

    pub fn get_idx(&self) -> EffectIdx {
        match self {
            BiquadFilter::Biquad(biquad) => biquad.get_idx(),
            BiquadFilter::LowPass(low_pass) => low_pass.get_idx(),
            BiquadFilter::HighPass(high_pass) => high_pass.get_idx(),
            BiquadFilter::BandPass(band_pass) => band_pass.get_idx(),
            BiquadFilter::Notch(notch) => notch.get_idx(),
            BiquadFilter::Peaking(peaking) => peaking.get_idx(),
            BiquadFilter::LowShelf(low_shelf) => low_shelf.get_idx(),
            BiquadFilter::HighShelf(high_shelf) => high_shelf.get_idx(),
        }
    }


    pub fn frequency_response(&self, freq: f32) -> f32 {
        match self {
            BiquadFilter::Biquad(biquad) => biquad.frequency_response(freq),
            BiquadFilter::LowPass(low_pass) => low_pass.frequency_response(freq),
            BiquadFilter::HighPass(high_pass) => high_pass.frequency_response(freq),
            BiquadFilter::BandPass(band_pass) => band_pass.frequency_response(freq),
            BiquadFilter::Notch(notch) => notch.frequency_response(freq),
            BiquadFilter::Peaking(peaking) => peaking.frequency_response(freq),
            BiquadFilter::LowShelf(low_shelf) => low_shelf.frequency_response(freq),
            BiquadFilter::HighShelf(high_shelf) => high_shelf.frequency_response(freq),
        }
    }
    pub fn modify(&mut self, freq: f32, q: f32, gain: f32, s: f32) {
        match self {
            BiquadFilter::Biquad(_biquad) => (),
            BiquadFilter::LowPass(low_pass) => low_pass.modify(freq, q),
            BiquadFilter::HighPass(high_pass) => high_pass.modify(freq, q),
            BiquadFilter::BandPass(band_pass) => band_pass.modify(freq, q),
            BiquadFilter::Notch(notch) => notch.modify(freq, q, gain),
            BiquadFilter::Peaking(peaking) => peaking.modify(freq, q, gain),
            BiquadFilter::LowShelf(low_shelf) => low_shelf.modify(freq, q, gain, s),
            BiquadFilter::HighShelf(high_shelf) => high_shelf.modify(freq, q, gain, s),
        }
    }
    
}
