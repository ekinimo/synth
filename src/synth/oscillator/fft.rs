use crate::synth::{effect::filters::fir::WindowType, Oscillator, OscillatorCtx};
use realfft::{RealFftPlanner, num_complex::Complex};

pub enum FftMode {
    Forward,
    Inverse,
}

pub struct FftSynth {
    oscillators: Vec<Box<dyn Oscillator>>,
    fft_forward: std::sync::Arc<dyn realfft::RealToComplex<f32>>,
    fft_inverse: std::sync::Arc<dyn realfft::ComplexToReal<f32>>,
    window: WindowType,
    mode: FftMode,
}

impl FftSynth {
    pub fn new(oscillators: Vec<Box<dyn Oscillator>>, window:WindowType,fft_size: usize, mode: FftMode) -> Self {
        let mut planner = RealFftPlanner::<f32>::new();
        let fft_forward = planner.plan_fft_forward(fft_size);
        let fft_inverse = planner.plan_fft_inverse(fft_size);


        

        Self {
            oscillators,
            fft_forward,
            fft_inverse,
            window,
            mode,
        }
    }

    pub fn set_mode(&mut self, mode: FftMode) {
        self.mode = mode;
    }
}

impl Oscillator for FftSynth {
    fn generate_sample(&self, ctx: OscillatorCtx, time: f32) -> f32 {
        let mut buffer : Vec<f32> = self.oscillators.iter().map(|osc| osc.generate_sample(ctx.clone(), time)).collect();
        let mut  spectrum : Vec<Complex<f32>> = buffer.iter().map(|x| Complex::i()*x).collect();
        let mut  output   = buffer.clone();
        let len = buffer.len();
        for (i, sample) in buffer.iter_mut().enumerate() {
            *sample *= self.window.value(i, len);
        }

        // Perform FFT or IFFT based on mode
        match self.mode {
            FftMode::Forward => {
                self.fft_forward.process(&mut buffer, &mut spectrum).unwrap();
            }
            FftMode::Inverse => {
                self.fft_inverse.process(&mut spectrum, &mut output).unwrap();
            }
        }

        // Return the first sample of the output buffer
        match self.mode {
            FftMode::Forward => spectrum.iter().map(|x| x.norm_sqr()).sum::<f32>().sqrt(),
            FftMode::Inverse => output.iter().map(|x| x.abs().powi(2)).sum::<f32>().sqrt(),
        }
    }
}
