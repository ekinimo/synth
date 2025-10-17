use std::f32::consts::PI;

use crate::synth::{Effect, EffectIdx};

pub struct FIR {
    pub idx:usize,
    taps: Vec<f32>,
    buffer: Vec<f32>,
    cursor: usize,
}
pub struct LowPass {
    pub idx:usize,
    pub cutoff: f32,
    pub filter_length: usize,
    pub window_type: WindowType,
    fir: FIR,
}

pub struct HighPass {
    pub idx:usize,
    pub cutoff: f32,
    pub filter_length: usize,
    pub window_type: WindowType,
    fir: FIR,
}

pub struct BandPass {
    pub idx:usize,
    pub low_cutoff: f32,
    pub high_cutoff: f32,
    pub filter_length: usize,
    pub window_type: WindowType,
    fir: FIR,
}

pub struct Notch {
    pub idx:usize,
    pub center_freq: f32,
    pub bandwidth: f32,
    pub filter_length: usize,
    pub window_type: WindowType,
    fir: FIR,
}

pub struct Peaking {
    pub idx:usize,
    pub center_freq: f32,
    pub bandwidth: f32,
    pub gain: f32,
    pub filter_length: usize,
    pub window_type: WindowType,
    fir: FIR,
}

pub struct LowShelf {
    pub idx:usize,
    pub cutoff: f32,
    pub gain_db: f32,
    pub filter_length: usize,
    pub window_type: WindowType,
    fir: FIR,
}

pub struct HighShelf {
    pub idx:usize,
    pub cutoff: f32,
    pub gain_db: f32,
    pub filter_length: usize,
    pub window_type: WindowType,
    fir: FIR,
}

#[derive(Clone, Copy)]
pub enum WindowType {
    Rectangular,
    Hamming,
    Hann,
    Blackman,
    Kaiser(f32), // Beta parameter
}

impl WindowType {
    pub fn value(&self, n: usize, length: usize) -> f32 {
        let n_f = n as f32;
        let length_f = length as f32;
        match self {
            WindowType::Rectangular => 1.0,
            WindowType::Hamming => 0.54 - 0.46 * (2.0 * PI * n_f / (length_f - 1.0)).cos(),
            WindowType::Hann => 0.5 * (1.0 - (2.0 * PI * n_f / (length_f - 1.0)).cos()),
            WindowType::Blackman => {
                0.42 - 0.5 * (2.0 * PI * n_f / (length_f - 1.0)).cos()
                    + 0.08 * (4.0 * PI * n_f / (length_f - 1.0)).cos()
            }
            WindowType::Kaiser(beta) => {
                let ratio = (n_f - (length_f - 1.0) / 2.0) / ((length_f - 1.0) / 2.0);
                bessel_i0(*beta * (1.0 - ratio * ratio).sqrt()) / bessel_i0(*beta)
            }
        }
    }
}

fn spectral_inversion(taps: &mut [f32]) {
    for tap in taps.iter_mut() {
        *tap = -*tap;
    }
    let middle = taps.len() / 2;
    taps[middle] += 1.0;
}

fn normalize_taps(taps: &mut [f32]) {
    let sum: f32 = taps.iter().sum();
    for tap in taps.iter_mut() {
        *tap /= sum;
    }
}

fn bessel_i0(x: f32) -> f32 {
    let mut result = 1.0;
    let x_half = x / 2.0;
    let mut factorial = 1.0;
    for i in 1..10 {
        factorial *= i as f32;
        result += (x_half.powi(i as i32) / factorial).powi(2);
    }
    result
}

impl LowPass {
    pub fn new(idx:usize, cutoff: f32, filter_length: usize, window_type: WindowType) -> Self {
        let taps = Self::design_filter(cutoff, filter_length, window_type);
        Self {idx,
            cutoff,
            filter_length,
            window_type,
            fir: FIR::new(0,taps),
        }
    }

    fn design_filter(cutoff: f32, length: usize, window: WindowType) -> Vec<f32> {
        let mut taps = vec![0.0; length];
        let omega_c = 2.0 * PI * cutoff / 44100.0;
        let middle = (length as f32 - 1.0) / 2.0;

        for n in 0..length {
            let t = n as f32 - middle;
            if t == 0.0 {
                taps[n] = omega_c / PI;
            } else {
                taps[n] = (omega_c * t).sin() / (PI * t);
            }
            taps[n] *= window.value(n, length);
        }

        normalize_taps(&mut taps);
        taps
    }
    pub fn modify_one_tap(&mut self, index: usize, new_tap: f32) {
        self.fir.modify_one_tap(index, new_tap);
    }
    pub fn frequency_response(&self, freq: f32) -> f32 {
        self.fir.frequency_response(freq)
    }
    pub fn calculate(&mut self, sample: f32) -> f32 {
        self.fir.calculate(sample)
    }
    pub fn modify_taps(&mut self, new_taps: Vec<f32>) {
        self.fir.modify_taps(new_taps);
    }
}

impl HighPass {
    pub fn new(idx:usize, cutoff: f32, filter_length: usize, window_type: WindowType) -> Self {
        let taps = Self::design_filter(cutoff, filter_length, window_type);
        Self {
            idx,
            cutoff,
            filter_length,
            window_type,
            fir: FIR::new(0,taps),
        }
    }
    

    fn design_filter(cutoff: f32, length: usize, window: WindowType) -> Vec<f32> {
        let mut taps = LowPass::design_filter(cutoff, length, window);
        spectral_inversion(&mut taps);
        taps
    }
    pub fn modify_one_tap(&mut self, index: usize, new_tap: f32) {
        self.fir.modify_one_tap(index, new_tap);
    }
    pub fn frequency_response(&self, freq: f32) -> f32 {
        self.fir.frequency_response(freq)
    }
    pub fn calculate(&mut self, sample: f32) -> f32 {
        self.fir.calculate(sample)
    }
    pub fn modify_taps(&mut self, new_taps: Vec<f32>) {
        self.fir.modify_taps(new_taps);
    }
}

fn sinc(x: f32) -> f32 {
    if x == 0.0 {
        1.0
    } else {
        (PI * x).sin() / (PI * x)
    }
}

fn design_fir_bandpass(low: f32, high: f32, length: usize, window: WindowType) -> Vec<f32> {
    let sample_rate = 44100.0;
    let mut taps = vec![0.0; length];
    let omega_l = 2.0 * PI * low / sample_rate;
    let omega_h = 2.0 * PI * high / sample_rate;
    let middle = (length as f32 - 1.0) / 2.0;

    for n in 0..length {
        let t = n as f32 - middle;
        taps[n] = (omega_h * t).sin() / (PI * t) - (omega_l * t).sin() / (PI * t);
        taps[n] *= window.value(n, length);
    }

    normalize_taps(&mut taps);
    taps
}

impl BandPass {
    pub fn new(idx:usize, low: f32, high: f32, length: usize, window: WindowType) -> Self {
        let taps = BandPass::design_filter(low, high, length, window);
        Self {
            idx,
            low_cutoff: low,
            high_cutoff: high,
            filter_length: length,
            window_type: window,
            fir: FIR::new(0,taps),
        }
    }

    

    fn design_filter(
        lower_cutoff: f32,
        highger_cutoff: f32,
        length: usize,
        window: WindowType,
    ) -> Vec<f32> {
        design_fir_bandpass(lower_cutoff, highger_cutoff, length, window)
    }
    pub fn modify_one_tap(&mut self, index: usize, new_tap: f32) {
        self.fir.modify_one_tap(index, new_tap);
    }
    pub fn frequency_response(&self, freq: f32) -> f32 {
        self.fir.frequency_response(freq)
    }
    pub fn calculate(&mut self, sample: f32) -> f32 {
        self.fir.calculate(sample)
    }
    pub fn modify_taps(&mut self, new_taps: Vec<f32>) {
        self.fir.modify_taps(new_taps);
    }
}

impl Notch {
    pub fn new(idx:usize, center: f32, bandwidth: f32, length: usize, window: WindowType) -> Self {
        let taps = Self::design_filter(center, bandwidth, length, window);
        Self {
            idx,
            center_freq: center,
            bandwidth,
            filter_length: length,
            window_type: window,
            fir: FIR::new(0,taps),
        }
    }

    
    fn design_filter(
        center: f32,
        bandwidth: f32,
        length: usize,
        window: WindowType,
    ) -> Vec<f32> {
        let mut taps = design_fir_bandpass(
            center - bandwidth / 2.0,
            center + bandwidth / 2.0,
            length,
            window,
        );
        spectral_inversion(&mut taps);
        taps
    }
    pub fn modify_one_tap(&mut self, index: usize, new_tap: f32) {
        self.fir.modify_one_tap(index, new_tap);
    }
    pub fn frequency_response(&self, freq: f32) -> f32 {
        self.fir.frequency_response(freq)
    }
    pub fn calculate(&mut self, sample: f32) -> f32 {
        self.fir.calculate(sample)
    }
    pub fn modify_taps(&mut self, new_taps: Vec<f32>) {
        self.fir.modify_taps(new_taps);
    }
}

impl Peaking {
    pub fn new(idx:usize, 
        center: f32,
        bandwidth: f32,
        gain: f32,
        length: usize,
        window: WindowType,
    ) -> Self {
        let taps = Self::design_filter(center, bandwidth, gain, length, window);
        Self {
            idx,
            center_freq: center,
            bandwidth,
            gain,
            filter_length: length,
            window_type: window,
            fir: FIR::new(0,taps),
        }
    }

    pub fn modify(
        &mut self,
        center_freq: f32,
        bandwidth: f32,
        gain: f32,
        filter_length: usize,
        window: WindowType,
    ) {
        self.center_freq = center_freq;
        self.bandwidth = bandwidth;
        self.gain = gain;
        self.filter_length = filter_length;
        self.window_type = window;
        self.fir.modify_taps(Self::design_filter(
            self.center_freq,
            self.bandwidth,
            self.gain,
            self.filter_length,
            self.window_type,
        ));
    }

        fn design_filter(
        center: f32,
        bandwidth: f32,
        gain: f32,
        length: usize,
        window: WindowType,
    ) -> Vec<f32> {
        let mut taps = design_fir_bandpass(
            center - bandwidth / 2.0,
            center + bandwidth / 2.0,
            length,
            WindowType::Hann,
        );

        let gain = 10.0f32.powf(gain / 20.0);
        for tap in &mut taps {
            *tap *= gain;
        }
        taps
    }
    pub fn modify_one_tap(&mut self, index: usize, new_tap: f32) {
        self.fir.modify_one_tap(index, new_tap);
    }
    pub fn frequency_response(&self, freq: f32) -> f32 {
        self.fir.frequency_response(freq)
    }
    pub fn calculate(&mut self, sample: f32) -> f32 {
        self.fir.calculate(sample)
    }
    pub fn modify_taps(&mut self, new_taps: Vec<f32>) {
        self.fir.modify_taps(new_taps);
    }
}

impl LowShelf {
    pub fn new(idx:usize, cutoff: f32, gain_db: f32, length: usize, window: WindowType) -> Self {
        let taps = Self::design_filter(cutoff, gain_db, length, window);
        Self {
            idx,
            cutoff,
            gain_db,
            filter_length: length,
            window_type: window,
            fir: FIR::new(0,taps),
        }
    }

    pub fn modify(
        &mut self,
        cutoff: f32,
        gain: f32,
        new_length: usize,
        window: WindowType,
    ) {
        self.cutoff = cutoff;
        self.gain_db = gain;
        self.filter_length = new_length;
        self.window_type = window;
        self.fir.modify_taps(Self::design_filter(
            self.cutoff,
            self.gain_db,
            self.filter_length,
            self.window_type,
        ));
    }

    fn design_filter(
        cutoff: f32,
        gain: f32,
        length: usize,
        window: WindowType,
    ) -> Vec<f32> {
        let mut taps = vec![0.0; length];
        let omega_c = 2.0 * PI * cutoff / 44100.0;
        let middle = (length as f32 - 1.0) / 2.0;
        let gain = 10.0f32.powf(gain / 20.0);

        for n in 0..length {
            let t = n as f32 - middle;
            if t == 0.0 {
                taps[n] = (gain - 1.0) * omega_c / (2.0 * PI) + 1.0;
            } else {
                taps[n] = (gain - 1.0) * (omega_c * t).sin() / (PI * t);
            }
            let m = t.abs();
            let l = if m > 0.0 && m < 0.0001 { 0.0 } else { 1.0 }; // Add delta function
            taps[n] += sinc(t) * l;
        }

        normalize_taps(&mut taps);
        taps
    }
    pub fn modify_taps(&mut self, new_taps: Vec<f32>) {
        self.fir.modify_taps(new_taps);
    }
    pub fn modify_one_tap(&mut self, index: usize, new_tap: f32) {
        self.fir.modify_one_tap(index, new_tap);
    }
    pub fn frequency_response(&self, freq: f32) -> f32 {
        self.fir.frequency_response(freq)
    }
    pub fn calculate(&mut self, sample: f32) -> f32 {
        self.fir.calculate(sample)
    }
    pub fn get_idx(&self)->EffectIdx{
        EffectIdx(self.idx)
    }
}

impl HighShelf {
    pub fn new(idx:usize, cutoff: f32, gain_db: f32, length: usize, window: WindowType) -> Self {
        let taps = Self::design_filter(cutoff, gain_db, length, window);
        Self {
            idx,
            cutoff,
            gain_db,
            filter_length: length,
            window_type: window,
            fir: FIR::new(0,taps),
        }
    }
    pub fn modify(
        &mut self,
        cutoff: f32,
        gain: f32,
        new_length: usize,
        window: WindowType,
    ) {
        self.cutoff = cutoff;
        self.gain_db = gain;
        self.filter_length = new_length;
        self.window_type = window;
        self.fir.modify_taps(Self::design_filter(
            self.cutoff,
            self.gain_db,
            self.filter_length,
            self.window_type,
        ));
    }
    fn design_filter(
        cutoff: f32,
        gain: f32,
        length: usize,
        window: WindowType,
    ) -> Vec<f32> {
        let mut taps = vec![0.0; length];
        let omega_c = 2.0 * PI * cutoff / 44100.0;
        let middle = (length as f32 - 1.0) / 2.0;
        let gain = 10.0f32.powf(-gain / 20.0);

        for n in 0..length {
            let t = n as f32 - middle;
            if t == 0.0 {
                taps[n] = (gain - 1.0) * omega_c / (2.0 * PI) + 1.0;
            } else {
                taps[n] = (gain - 1.0) * (omega_c * t).sin() / (PI * t);
            }
            let m = t.abs();
            let l = if m > 0.0 && m < 0.0001 { 0.0 } else { 1.0 }; // Add delta function
            taps[n] += sinc(t) * l;
        }

        normalize_taps(&mut taps);
        spectral_inversion(&mut taps);
        taps
    }

    pub fn modify_taps(&mut self, new_taps: Vec<f32>) {
        self.fir.modify_taps(new_taps);
    }
    pub fn modify_one_tap(&mut self, index: usize, new_tap: f32) {
        self.fir.modify_one_tap(index, new_tap);
    }
    pub fn frequency_response(&self, freq: f32) -> f32 {
        self.fir.frequency_response(freq)
    }
    pub fn calculate(&mut self, sample: f32) -> f32 {
        self.fir.calculate(sample)
    }
    pub fn get_idx(&self)->EffectIdx{
        EffectIdx(self.idx)
    }
}

impl FIR {
    pub fn new(idx:usize, taps: Vec<f32>) -> Self {
        let buffer = vec![0.0; taps.len()];
        FIR {
            idx,
            taps,
            buffer,
            cursor: 0,
        }
    }
    pub fn get_idx(&self)->EffectIdx{
        EffectIdx(self.idx)
    }
    pub fn calculate(&mut self, sample: f32) -> f32 {
        self.buffer[self.cursor] = sample;
        self.cursor = (self.cursor + 1) % self.buffer.len();

        let mut output = 0.0;
        for i in 0..self.taps.len() {
            let index = (self.cursor + self.buffer.len() - 1 - i) % self.buffer.len();
            output += self.taps[i] * self.buffer[index];
        }
        output
    }

    pub fn frequency_response(&self, freq: f32) -> f32 {
        let omega = 2.0 * std::f32::consts::PI * freq / 44100.0;
        let mut real = 0.0;
        let mut imag = 0.0;

        for (i, &tap) in self.taps.iter().enumerate() {
            let angle = omega * i as f32;
            real += tap * angle.cos();
            imag -= tap * angle.sin(); // e^(-jω)
        }

        (real.powi(2) + imag.powi(2)).sqrt()
    }

    pub fn modify_taps(&mut self, new_taps: Vec<f32>) {
        if new_taps.len() != self.taps.len() {
            self.buffer.resize(new_taps.len(), 0.0);
            self.buffer.fill(0.0);
            self.cursor = 0;
        }
        self.taps = new_taps;
    }

    pub fn modify_one_tap(&mut self, index: usize, new_tap: f32) {
        self.taps[index] = new_tap;
    }
}

impl Effect for FIR {
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

pub enum FIRFilter {
    FIR(FIR),
    LowPass(LowPass),
    HighPass(HighPass),
    BandPass(BandPass),
    Notch(Notch),
    Peaking(Peaking),
    LowShelf(LowShelf),
    HighShelf(HighShelf),
}

impl Effect for FIRFilter {
    fn process(&mut self, sample: f32) -> f32 {
        match self {
            FIRFilter::FIR(fir) => fir.calculate(sample),
            FIRFilter::LowPass(low_pass) => low_pass.calculate(sample),
            FIRFilter::HighPass(high_pass) => high_pass.calculate(sample),
            FIRFilter::BandPass(band_pass) => band_pass.calculate(sample),
            FIRFilter::Notch(notch) => notch.calculate(sample),
            FIRFilter::Peaking(peaking) => peaking.calculate(sample),
            FIRFilter::LowShelf(low_shelf) => low_shelf.calculate(sample),
            FIRFilter::HighShelf(high_shelf) => high_shelf.calculate(sample),
        }
    }
    fn get_idx(&self)->EffectIdx {
        self.get_idx()
    }

}

impl FIRFilter {
    pub fn calculate(&mut self, sample: f32) -> f32 {
        match self {
            FIRFilter::FIR(fir) => fir.calculate(sample),
            FIRFilter::LowPass(low_pass) => low_pass.calculate(sample),
            FIRFilter::HighPass(high_pass) => high_pass.calculate(sample),
            FIRFilter::BandPass(band_pass) => band_pass.calculate(sample),
            FIRFilter::Notch(notch) => notch.calculate(sample),
            FIRFilter::Peaking(peaking) => peaking.calculate(sample),
            FIRFilter::LowShelf(low_shelf) => low_shelf.calculate(sample),
            FIRFilter::HighShelf(high_shelf) => high_shelf.calculate(sample),
        }
    }
    pub fn frequency_response(&self, freq: f32) -> f32 {
        match self {
            FIRFilter::FIR(fir) => fir.frequency_response(freq),
            FIRFilter::LowPass(low_pass) => low_pass.frequency_response(freq),
            FIRFilter::HighPass(high_pass) => high_pass.frequency_response(freq),
            FIRFilter::BandPass(band_pass) => band_pass.frequency_response(freq),
            FIRFilter::Notch(notch) => notch.frequency_response(freq),
            FIRFilter::Peaking(peaking) => peaking.frequency_response(freq),
            FIRFilter::LowShelf(low_shelf) => low_shelf.frequency_response(freq),
            FIRFilter::HighShelf(high_shelf) => high_shelf.frequency_response(freq),
        }
    }

    pub fn modify_one_tap(&mut self, index: usize, new_tap: f32) {
        match self {
            FIRFilter::FIR(fir) => fir.modify_one_tap(index, new_tap),
            FIRFilter::LowPass(low_pass) => low_pass.modify_one_tap(index, new_tap),
            FIRFilter::HighPass(high_pass) => high_pass.modify_one_tap(index, new_tap),
            FIRFilter::BandPass(band_pass) => band_pass.modify_one_tap(index, new_tap),
            FIRFilter::Notch(notch) => notch.modify_one_tap(index, new_tap),
            FIRFilter::Peaking(peaking) => peaking.modify_one_tap(index, new_tap),
            FIRFilter::LowShelf(low_shelf) => low_shelf.modify_one_tap(index, new_tap),
            FIRFilter::HighShelf(high_shelf) => high_shelf.modify_one_tap(index, new_tap),
        }
    }
    pub fn modify_taps(&mut self, new_tap: Vec<f32>) {
        match self {
            FIRFilter::FIR(fir) => fir.modify_taps(new_tap),
            FIRFilter::LowPass(low_pass) => low_pass.modify_taps(new_tap),
            FIRFilter::HighPass(high_pass) => high_pass.modify_taps( new_tap),
            FIRFilter::BandPass(band_pass) => band_pass.modify_taps( new_tap),
            FIRFilter::Notch(notch) => notch.modify_taps( new_tap),
            FIRFilter::Peaking(peaking) => peaking.modify_taps( new_tap),
            FIRFilter::LowShelf(low_shelf) => low_shelf.modify_taps( new_tap),
            FIRFilter::HighShelf(high_shelf) => high_shelf.modify_taps( new_tap),
        }
    }

    pub fn get_idx(& self) -> EffectIdx {
        match self {
            FIRFilter::FIR(fir) => fir.get_idx(),
            FIRFilter::LowPass(low_pass) => low_pass.get_idx(),
            FIRFilter::HighPass(high_pass) => high_pass.get_idx(),
            FIRFilter::BandPass(band_pass) => band_pass.get_idx(),
            FIRFilter::Notch(notch) => notch.get_idx(),
            FIRFilter::Peaking(peaking) => peaking.get_idx(),
            FIRFilter::LowShelf(low_shelf) => low_shelf.get_idx(),
            FIRFilter::HighShelf(high_shelf) => high_shelf.get_idx(),
        }
    }
    
}
