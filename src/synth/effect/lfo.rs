use std::f32::consts::PI;

use crate::synth::{Effect, EffectIdx, Oscillator};

pub enum LFOLike {
    LFO(AmplitudeLFO),
    FreqLFO(FreqLFO),
    PhaseLFO(PhaseLFO),

    AmpPhase(AmpPhaseLFO),
    FreqAndAmpLFO(FreqAndAmplitudeLFO),
    FreqPhaseAmpLFO(FreqPhaseAmplitudeLFO),
    Combined(usize,Vec<Self>),
    LfoUsingDelay(LFOUsingDelay),
}

impl LFOLike {
    pub fn get_idx(&self) ->EffectIdx {
        match self {
            LFOLike::LFO(lfo) => lfo.get_idx(),
            LFOLike::LfoUsingDelay(delay) => delay.get_idx(),
            LFOLike::FreqLFO(freq_lfo) => freq_lfo.get_idx(),
            LFOLike::FreqAndAmpLFO(freq_and_amplitude_lfo) => freq_and_amplitude_lfo.get_idx(),
            LFOLike::PhaseLFO(lfo) => lfo.get_idx(),
            LFOLike::AmpPhase(lfo) => lfo.get_idx(),
            LFOLike::FreqPhaseAmpLFO(lfo) => lfo.get_idx(),
            LFOLike::Combined(idx,_) => EffectIdx(*idx),
        }
    }

    pub fn update_dry(&mut self, dry: f32) {
        match self {
            LFOLike::LFO(_) => (),
            LFOLike::LfoUsingDelay(delay) => delay.update_dry(dry),
            LFOLike::FreqLFO(_freq_lfo) => (),
            LFOLike::FreqAndAmpLFO(_freq_and_amplitude_lfo) => (),
            LFOLike::PhaseLFO(_phase_lfo) => (),
            LFOLike::AmpPhase(_amp_phase_lfo) => (),
            LFOLike::FreqPhaseAmpLFO(_freq_phase_amplitude_lfo) => (),
            LFOLike::Combined(_,_vec) => (),
        }
    }

        pub fn update_wet(&mut self, wet: f32) {
        match self {
            LFOLike::LFO(_) => (),
            LFOLike::LfoUsingDelay(delay) => delay.update_wet(wet),
            LFOLike::FreqLFO(_freq_lfo) => (),
            LFOLike::FreqAndAmpLFO(_freq_and_amplitude_lfo) => (),
            LFOLike::PhaseLFO(_phase_lfo) => (),
            LFOLike::AmpPhase(_amp_phase_lfo) => (),
            LFOLike::FreqPhaseAmpLFO(_freq_phase_amplitude_lfo) => (),
            LFOLike::Combined(_,_vec) => (),
        }
    }
    pub fn update_feedback(&mut self, feedback: f32) {
        match self {
            LFOLike::LFO(_) => (),
            LFOLike::LfoUsingDelay(delay) => delay.update_feedback(feedback),
            LFOLike::FreqLFO(_freq_lfo) => (),
            LFOLike::FreqAndAmpLFO(_freq_and_amplitude_lfo) => (),
            LFOLike::PhaseLFO(_phase_lfo) => (),
            LFOLike::AmpPhase(_amp_phase_lfo) => (),
            LFOLike::FreqPhaseAmpLFO(_freq_phase_amplitude_lfo) => (),
            LFOLike::Combined(_,_vec) => (),
        }
    }

    pub fn update_should_offset(&mut self, should_offset: bool) {
        match self {
            LFOLike::LFO(lfo) => lfo.update_should_offset(should_offset),
            LFOLike::LfoUsingDelay(delay) => delay.update_should_offset(should_offset),
            LFOLike::FreqLFO(_lfo) => (),
            LFOLike::FreqAndAmpLFO(lfo) => lfo.update_should_offset(should_offset),
            LFOLike::PhaseLFO(_phase_lfo) => (),
            LFOLike::AmpPhase(amp_phase_lfo) => amp_phase_lfo.update_should_offset(should_offset),
            LFOLike::FreqPhaseAmpLFO(freq_phase_amplitude_lfo) => freq_phase_amplitude_lfo.update_should_offset(should_offset),
            LFOLike::Combined(_,_vec) => (),
        }
    }
    pub fn update_div_instead_of_mul(&mut self, flag: bool) {
        match self {
            LFOLike::LFO(lfo) => lfo.update_div_instead_of_mul(flag),
            LFOLike::LfoUsingDelay(delay) => delay.update_div_instead_of_mul(flag),
            LFOLike::FreqLFO(_lfo) => (),
            LFOLike::FreqAndAmpLFO(lfo) => lfo.update_div_instead_of_mul(flag),
            LFOLike::PhaseLFO(_phase_lfo) => (),
            LFOLike::AmpPhase(amp_phase_lfo) => amp_phase_lfo.update_div_instead_of_mul(flag),
            LFOLike::FreqPhaseAmpLFO(freq_phase_amplitude_lfo) => freq_phase_amplitude_lfo.update_div_instead_of_mul(flag),
            LFOLike::Combined(_,_vec) => (),
        }
    }
    pub fn update_oscillator(&mut self, osc: impl Oscillator + 'static) {
        match self {
            LFOLike::LFO(lfo) => lfo.update_oscillator(osc),
            LFOLike::LfoUsingDelay(_delay) => (),
            LFOLike::FreqLFO(_freq_lfo) => (),
            LFOLike::FreqAndAmpLFO(lfo) => lfo.update_oscillator(osc),
            LFOLike::PhaseLFO(phase_lfo) => phase_lfo.update_oscillator(osc),
            LFOLike::AmpPhase(amp_phase_lfo) => amp_phase_lfo.update_oscillator(osc),
            LFOLike::FreqPhaseAmpLFO(freq_phase_amplitude_lfo) => freq_phase_amplitude_lfo.update_oscillator(osc),
            LFOLike::Combined(_,_vec) => (),
        }
    }
    pub fn update_first_oscillator(&mut self, osc: impl Oscillator + 'static) {
        match self {
            LFOLike::LFO(lfo) => lfo.update_oscillator(osc),
            LFOLike::LfoUsingDelay(_delay) => (),
            LFOLike::FreqLFO(_freq_lfo) => (),
            LFOLike::FreqAndAmpLFO(lfo) => lfo.update_first_oscillator(osc),
            LFOLike::PhaseLFO(_phase_lfo) => (),
            LFOLike::AmpPhase(_lfo) => (),
            LFOLike::FreqPhaseAmpLFO(lfo) => lfo.update_first_oscillator(osc),
            LFOLike::Combined(_,_vec) => (),
        }
    }
    pub fn update_second_oscillator(&mut self, osc: impl Oscillator + 'static) {
        match self {
            LFOLike::LFO(lfo) => lfo.update_oscillator(osc),
            LFOLike::LfoUsingDelay(_delay) => (),
            LFOLike::FreqLFO(_freq_lfo) => (),
            LFOLike::FreqAndAmpLFO(lfo) => lfo.update_second_oscillator(osc),
            LFOLike::AmpPhase(_lfo) => (),
            LFOLike::FreqPhaseAmpLFO(lfo) => lfo.update_second_oscillator(osc),
            LFOLike::Combined(_,_vec) => (),
            LFOLike::PhaseLFO(_phase_lfo) => (),
        }
    }
    pub fn update_freq(&mut self, freq: f32) {
        match self {
            LFOLike::LFO(lfo) => lfo.update_freq(freq),
            LFOLike::LfoUsingDelay(_delay) => (),
            LFOLike::FreqLFO(freq_lfo) => freq_lfo.update_freq(freq),
            LFOLike::FreqAndAmpLFO(freq_and_amplitude_lfo) => freq_and_amplitude_lfo.update_freq(freq),
            LFOLike::PhaseLFO(lfo) => lfo.update_freq(freq),
            LFOLike::AmpPhase(lfo) => lfo.update_freq(freq),
            LFOLike::FreqPhaseAmpLFO(lfo) => lfo.update_freq(freq),
            LFOLike::Combined(_,_vec) => (),
        }
    }
    pub fn update_freq_freq(&mut self, freq: f32) {
        match self {
            LFOLike::LFO(_lfo) => (),
            LFOLike::LfoUsingDelay(_delay) => (),
            LFOLike::FreqLFO(_freq_lfo) => (),
            LFOLike::FreqAndAmpLFO(freq_and_amplitude_lfo) => freq_and_amplitude_lfo.update_freq_freq(freq),
            LFOLike::PhaseLFO(_) => (),
            LFOLike::AmpPhase(_amp_phase_lfo) => (),
            LFOLike::FreqPhaseAmpLFO(lfo) => lfo.update_freq_freq(freq),
            LFOLike::Combined(_,_vec) => (),
        }
    }

    pub fn update_freq_delta(&mut self, freq: f32) {
        match self {
            LFOLike::LFO(_lfo) => (),
            LFOLike::LfoUsingDelay(_delay) => (),
            LFOLike::FreqLFO(freq_lfo) => freq_lfo.update_freq_delta(freq),
            LFOLike::FreqAndAmpLFO(freq_and_amplitude_lfo) => freq_and_amplitude_lfo.update_freq_delta(freq),
            LFOLike::PhaseLFO(_) => (),
            LFOLike::AmpPhase(_amp_phase_lfo) => (),
            LFOLike::FreqPhaseAmpLFO(lfo) => lfo.update_freq_freq(freq),
            LFOLike::Combined(_,_vec) => (),
        }
    }

    
    pub fn update_tree_nth_wet(&mut self,idx:&[usize],wet: f32) {
        let mut s = self;
        for id in idx{
            match s {
                LFOLike::Combined(_,vec) => {
                    s = &mut vec[*id];
                },
                _ => return
            }
        }
        s.update_wet(wet);
    }

    pub fn add_lfo(&mut self,lfo:Self){
        match self {
            Self::Combined(_,vec)=>{vec.push(lfo);},
            _ => ()
        }
    }
    pub fn update_tree_nth_feedback(&mut self,idx:&[usize], feedback: f32) {
        let mut s = self;
        for id in idx{
            match s {
                LFOLike::Combined(_,vec) => {
                    s = &mut vec[*id];
                },
                _ => return
            }
        }
        s.update_feedback(feedback);
    }
    pub fn update_tree_nth_add_lfo(&mut self,idx:&[usize], lfo: Self) {
        let mut s = self;
        for id in idx{
            match s {
                LFOLike::Combined(_,vec) => {
                    s = &mut vec[*id];
                },
                _ => return
            }
        }
        s.add_lfo(lfo);
    }
    pub fn update_tree_nth_dry(&mut self,idx:&[usize], dry: f32) {
        let mut s = self;
        for id in idx{
            match s {
                LFOLike::Combined(_,vec) => {
                    s = &mut vec[*id];
                },
                _ => return
            }
        }
        s.update_dry(dry);
    }
    pub fn update_tree_nth_freq_freq(&mut self,idx:&[usize], freq: f32){
        let mut s = self;
        for id in idx{
            match s {
                LFOLike::Combined(_,vec) => {
                    s = &mut vec[*id];
                },
                _ => return
            }
        }
        s.update_freq_freq(freq);
    }
    pub fn update_tree_nth_freq_delta(&mut self,idx:&[usize], freq: f32){
        let mut s = self;
        for id in idx{
            match s {
                LFOLike::Combined(_,vec) => {
                    s = &mut vec[*id];
                },
                _ => return
            }
        }
        s.update_freq_delta(freq);
    }
    pub fn update_tree_nth_freq(&mut self,idx:&[usize], freq: f32){
        let mut s = self;
        for id in idx{
            match s {
                LFOLike::Combined(_,vec) => {
                    s = &mut vec[*id];
                },
                _ => return
            }
        }
        s.update_freq_delta(freq);
    }
    pub fn update_tree_nth_second_oscillator(&mut self,idx:&[usize], osc: impl Oscillator + 'static){
        let mut s = self;
        for id in idx{
            match s {
                LFOLike::Combined(_,vec) => {
                    s = &mut vec[*id];
                },
                _ => return
            }
        }
        s.update_second_oscillator(osc);
    }
    pub fn update_tree_nth_first_oscillator(&mut self,idx:&[usize], osc: impl Oscillator + 'static){
        let mut s = self;
        for id in idx{
            match s {
                LFOLike::Combined(_,vec) => {
                    s = &mut vec[*id];
                },
                _ => return
            }
        }
        s.update_first_oscillator(osc);
    }
    pub fn update_tree_nth_oscillator(&mut self,idx:&[usize], osc: impl Oscillator + 'static){
        let mut s = self;
        for id in idx{
            match s {
                LFOLike::Combined(_,vec) => {
                    s = &mut vec[*id];
                },
                _ => return
            }
        }
        s.update_oscillator(osc);
    }
    pub fn update_tree_nth_div_instead_of_mul(&mut self,idx:&[usize], flag: bool) {
        let mut s = self;
        for id in idx{
            match s {
                LFOLike::Combined(_,vec) => {
                    s = &mut vec[*id];
                },
                _ => return
            }
        }
        s.update_div_instead_of_mul(flag);
    }
    pub fn update_tree_nth_should_offset(&mut self,idx:&[usize], should_offset: bool) {
        let mut s = self;
        for id in idx{
            match s {
                LFOLike::Combined(_,vec) => {
                    s = &mut vec[*id];
                },
                _ => return
            }
        }
        s.update_should_offset(should_offset);
    }


    pub fn calculate(&mut self, sample: f32) -> f32 {
        match self {
            LFOLike::LFO(lfo) => lfo.calculate(sample),
            LFOLike::LfoUsingDelay(delay) => delay.calculate(sample),
            LFOLike::FreqLFO(freq_lfo) => freq_lfo.calculate(sample),
            LFOLike::FreqAndAmpLFO(freq_and_amplitude_lfo) => freq_and_amplitude_lfo.calculate(sample),
            LFOLike::PhaseLFO(lfo) => lfo.calculate(sample),
            LFOLike::AmpPhase(lfo) => lfo.calculate(sample),
            LFOLike::FreqPhaseAmpLFO(lfo) => lfo.calculate(sample),
            LFOLike::Combined(_,vec) => vec.iter_mut().fold(sample, |new_sample,lfo| lfo.calculate(new_sample)),
        }
    }
}

impl Effect for LFOLike {
    fn process(&mut self, sample: f32) -> f32 {
        self.calculate(sample)
    }

    fn get_idx(&self)->EffectIdx {
        self.get_idx()
    }

    
}

pub struct AmplitudeLFO {
    pub idx:usize,
    pub freq: f32,
    pub osc: Box<dyn Oscillator>,
    pub sample_rate: usize,
    pub should_offset: bool,
    pub div_instead_of_mul: bool,
    phase: f32,
}

impl Effect for AmplitudeLFO {
    fn process(&mut self, sample: f32) -> f32 {
        self.calculate(sample)
    }
    fn get_idx(&self)->EffectIdx {
        self.get_idx()
    }

}

impl AmplitudeLFO {
    pub fn get_idx(&self)->EffectIdx{
        EffectIdx(self.idx)
    }
    pub fn update_oscillator(&mut self, osc: impl Oscillator + 'static) {
        self.osc = Box::new(osc);
    }
    pub fn update_freq(&mut self, freq: f32) {
        self.freq = freq;
    }
    pub fn update_should_offset(&mut self, should_offset: bool) {
        self.should_offset = should_offset;
    }
    pub fn update_div_instead_of_mul(&mut self, flag: bool) {
        self.div_instead_of_mul = flag;
    }

    pub fn calculate(&mut self, sample: f32) -> f32 {
        let inverse_sample_rate = 1.0 / self.sample_rate as f32;
        let ctx = crate::synth::OscillatorCtx {
            amplitude: 0.5,
            freq: 1.0,
            phase: 0.0,
            sampling_rate: self.sample_rate,
        };

        let offset = if self.should_offset { 1.0 } else { 0.0 };
        let carrier = self.osc.generate_sample(ctx, self.phase) + offset;

        self.phase += self.freq * inverse_sample_rate;
        self.phase %= 1.0;
        if self.div_instead_of_mul {
            sample / (carrier + 1.0).exp()
        } else {
            carrier * sample
        }
    }
}

pub struct FreqLFO {
    pub idx:usize,
    pub freq_delta: f32,
    pub freq: f32,
    pub osc1: Box<dyn Oscillator>,
    pub osc2: Box<dyn Oscillator>,
    pub sample_rate: usize,
    phase: f32,
}

impl Effect for FreqLFO {
    fn process(&mut self, sample: f32) -> f32 {
        self.calculate(sample)
    }
    fn get_idx(&self)->EffectIdx {
        self.get_idx()
    }

}

impl FreqLFO {
    pub fn get_idx(&self)->EffectIdx{
        EffectIdx(self.idx)
    }
    pub fn update_first_oscillator(&mut self, osc: impl Oscillator + 'static) {
        self.osc1 = Box::new(osc);
    }
    pub fn update_second_oscillator(&mut self, osc: impl Oscillator + 'static) {
        self.osc2 = Box::new(osc);
    }
    pub fn update_freq(&mut self, freq: f32) {
        self.freq = freq;
    }
    pub fn update_freq_delta(&mut self, freq: f32) {
        self.freq_delta = freq;
    }

    pub fn calculate(&mut self, sample: f32) -> f32 {
        let inverse_sample_rate = 1.0 / self.sample_rate as f32;

        let ctx1 = crate::synth::OscillatorCtx {
            amplitude: self.freq_delta / self.freq,
            freq: self.freq,
            phase: 0.0,
            sampling_rate: self.sample_rate,
        };

        let o1 = self.osc1.generate_sample(ctx1, self.phase);

        let ctx2 = crate::synth::OscillatorCtx {
            amplitude: 1.0,
            freq: sample,
            phase: o1 / (2.0 * std::f32::consts::PI * sample),
            sampling_rate: self.sample_rate,
        };

        let result = self.osc2.generate_sample(ctx2, self.phase);

        self.phase += inverse_sample_rate;
        self.phase %= 1.0;
        result
    }
}


pub struct PhaseLFO {
    pub idx:usize,
    pub freq: f32,
    pub osc: Box<dyn Oscillator>,
    pub sample_rate: usize,
    phase: f32,
}

impl Effect for PhaseLFO {
    fn process(&mut self, sample: f32) -> f32 {
        self.calculate(sample)
    }
    fn get_idx(&self)->EffectIdx {
        self.get_idx()
    }

}

impl PhaseLFO {
    pub fn get_idx(&self)->EffectIdx{
        EffectIdx(self.idx)
    }
    pub fn update_oscillator(&mut self, osc: impl Oscillator + 'static) {
        self.osc = Box::new(osc);
    }
    pub fn update_freq(&mut self, freq: f32) {
        self.freq = freq;
    }
    
    pub fn calculate(&mut self, sample: f32) -> f32 {
        let inverse_sample_rate = 1.0 / self.sample_rate as f32;

        let ctx1 = crate::synth::OscillatorCtx {
            amplitude:  1.0,
            freq: self.freq,
            phase: sample,
            sampling_rate: self.sample_rate,
        };

        let result = self.osc.generate_sample(ctx1, self.phase);

        
        self.phase += inverse_sample_rate;
        self.phase %= 1.0;
        result
    }
}
impl Effect for AmpPhaseLFO {
    fn process(&mut self, sample: f32) -> f32 {
        self.calculate(sample)
    }
    fn get_idx(&self)->EffectIdx {
        self.get_idx()
    }

}

pub struct AmpPhaseLFO {
    pub idx : usize,
    pub freq: f32,
    pub osc: Box<dyn Oscillator>,
    pub sample_rate: usize,
    pub should_offset: bool,
    pub div_instead_of_mul: bool,

    phase: f32,
    
}

impl AmpPhaseLFO {
    pub fn get_idx(&self)->EffectIdx{
        EffectIdx(self.idx)
    }
    pub fn update_oscillator(&mut self, osc: impl Oscillator + 'static) {
        self.osc = Box::new(osc);
    }
    pub fn update_freq(&mut self, freq: f32) {
        self.freq = freq;
    }
    pub fn update_should_offset(&mut self, should_offset: bool) {
        self.should_offset = should_offset;
    }
    pub fn update_div_instead_of_mul(&mut self, flag: bool) {
        self.div_instead_of_mul = flag;
    }

    pub fn calculate(&mut self, sample: f32) -> f32 {
        let inverse_sample_rate = 1.0 / self.sample_rate as f32;
        let ctx = crate::synth::OscillatorCtx {
            amplitude: 0.5,
            freq: 1.0,
            phase: sample,
            sampling_rate: self.sample_rate,
        };

        let offset = if self.should_offset { 1.0 } else { 0.0 };
        let carrier = self.osc.generate_sample(ctx, self.phase) + offset;

        self.phase += self.freq * inverse_sample_rate;
        self.phase %= 1.0;
        if self.div_instead_of_mul {
            sample / (carrier + 1.0).exp()
        } else {
            carrier * sample
        }
    }

}
impl Effect for FreqAndAmplitudeLFO {
    fn process(&mut self, sample: f32) -> f32 {
        self.calculate(sample)
    }
    fn get_idx(&self)->EffectIdx {
        self.get_idx()
    }

}

pub struct FreqAndAmplitudeLFO {
    pub idx : usize,
    pub freq_delta: f32,
    pub freq_freq: f32,
    pub amp_freq: f32,
    pub freq_osc1: Box<dyn Oscillator>,
    pub freq_osc2: Box<dyn Oscillator>,
    pub amp_osc: Box<dyn Oscillator>,
    pub sample_rate: usize,
    pub should_offset: bool,
    pub div_instead_of_mul: bool,

    freq_phase: f32,
    amp_phase: f32,
}

impl FreqAndAmplitudeLFO {
    pub fn get_idx(&self)->EffectIdx{
        EffectIdx(self.idx)
    }
    pub fn update_first_oscillator(&mut self, osc: impl Oscillator + 'static) {
        self.freq_osc1 = Box::new(osc);
    }
    pub fn update_second_oscillator(&mut self, osc: impl Oscillator + 'static) {
        self.freq_osc2 = Box::new(osc);
    }
    pub fn update_oscillator(&mut self, osc: impl Oscillator + 'static) {
        self.amp_osc = Box::new(osc);
    }
    pub fn update_freq_freq(&mut self, freq: f32) {
        self.freq_freq = freq;
    }
    pub fn update_freq_delta(&mut self, freq: f32) {
        self.freq_delta = freq;
    }
    pub fn update_freq(&mut self, freq: f32) {
        self.amp_freq = freq;
    }
    pub fn update_should_offset(&mut self, should_offset: bool) {
        self.should_offset = should_offset;
    }
    pub fn update_div_instead_of_mul(&mut self, flag: bool) {
        self.div_instead_of_mul = flag;
    }

    pub fn calculate(&mut self, sample: f32) -> f32 {
        let inverse_sample_rate = 1.0 / self.sample_rate as f32;

        let ctx1 = crate::synth::OscillatorCtx {
            amplitude: self.freq_delta / self.freq_freq,
            freq: self.freq_freq,
            phase: 0.0,
            sampling_rate: self.sample_rate,
        };

        let o1 = self.freq_osc1.generate_sample(ctx1, self.freq_phase);

        let ctx2 = crate::synth::OscillatorCtx {
            amplitude: 1.0,
            freq: sample,
            phase: o1 / (2.0 * std::f32::consts::PI * sample),
            sampling_rate: self.sample_rate,
        };

        let result = self.freq_osc2.generate_sample(ctx2, self.freq_phase);

        let ctx = crate::synth::OscillatorCtx {
            amplitude: 0.5,
            freq: 1.0,
            phase: 0.0,
            sampling_rate: self.sample_rate,
        };

        let offset = if self.should_offset { 1.0 } else { 0.0 };
        let carrier = self.amp_osc.generate_sample(ctx, self.amp_phase) + offset;

        self.freq_phase += inverse_sample_rate;
        self.freq_phase %= 1.0;
        self.amp_phase += self.amp_freq * inverse_sample_rate;
        self.amp_phase %= 1.0;

        if self.div_instead_of_mul {
            result / (carrier + 1.0).exp()
        } else {
            carrier * result
        }
    }
}
impl Effect for FreqAndPhaseLFO {
    fn process(&mut self, sample: f32) -> f32 {
        self.calculate(sample)
    }
    fn get_idx(&self)->EffectIdx {
        self.get_idx()
    }

}

pub struct FreqAndPhaseLFO {
    pub idx : usize,
    pub freq_delta: f32,
    pub freq_freq: f32,
    pub phase_freq: f32,
    pub freq_osc1: Box<dyn Oscillator>,
    pub freq_osc2: Box<dyn Oscillator>,
    pub phase_osc: Box<dyn Oscillator>,
    pub sample_rate: usize,
    pub should_offset: bool,
    pub div_instead_of_mul: bool,

    freq_phase: f32,
    phase_phase: f32,
}

impl FreqAndPhaseLFO {
    pub fn get_idx(&self)->EffectIdx{
        EffectIdx(self.idx)
    }
    pub fn update_first_oscillator(&mut self, osc: impl Oscillator + 'static) {
        self.freq_osc1 = Box::new(osc);
    }
    pub fn update_second_oscillator(&mut self, osc: impl Oscillator + 'static) {
        self.freq_osc2 = Box::new(osc);
    }
    pub fn update_oscillator(&mut self, osc: impl Oscillator + 'static) {
        self.phase_osc = Box::new(osc);
    }
    pub fn update_freq_freq(&mut self, freq: f32) {
        self.freq_freq = freq;
    }
    pub fn update_freq_delta(&mut self, freq: f32) {
        self.freq_delta = freq;
    }
    pub fn update_freq(&mut self, freq: f32) {
        self.phase_freq = freq;
    }
    pub fn update_should_offset(&mut self, should_offset: bool) {
        self.should_offset = should_offset;
    }
    pub fn update_div_instead_of_mul(&mut self, flag: bool) {
        self.div_instead_of_mul = flag;
    }

    pub fn calculate(&mut self, sample: f32) -> f32 {
        let inverse_sample_rate = 1.0 / self.sample_rate as f32;

        let ctx1 = crate::synth::OscillatorCtx {
            amplitude: self.freq_delta / self.freq_freq,
            freq: self.freq_freq,
            phase: 0.0,
            sampling_rate: self.sample_rate,
        };

        let o1 = self.freq_osc1.generate_sample(ctx1, self.freq_phase);

        let ctx2 = crate::synth::OscillatorCtx {
            amplitude: 1.0,
            freq: sample,
            phase: o1 / (2.0 * std::f32::consts::PI * sample),
            sampling_rate: self.sample_rate,
        };

        let result = self.freq_osc2.generate_sample(ctx2, self.freq_phase);

        let ctx = crate::synth::OscillatorCtx {
            amplitude: 1.0,
            freq: 1.0,
            phase: sample,
            sampling_rate: self.sample_rate,
        };

        let offset = if self.should_offset { 1.0 } else { 0.0 };
        let carrier = self.phase_osc.generate_sample(ctx, self.phase_phase) + offset;

        self.freq_phase += inverse_sample_rate;
        self.freq_phase %= 1.0;
        self.phase_phase += self.phase_freq * inverse_sample_rate;
        self.phase_phase %= 1.0;

        if self.div_instead_of_mul {
            result / (carrier + 1.0).exp()
        } else {
            carrier * result
        }
    }
}

impl Effect for FreqPhaseAmplitudeLFO {
    fn process(&mut self, sample: f32) -> f32 {
        self.calculate(sample)
    }
    fn get_idx(&self)->EffectIdx {
        self.get_idx()
    }

}

pub struct FreqPhaseAmplitudeLFO {
    pub idx:usize,
    pub freq_delta: f32,
    pub freq_freq: f32,
    pub amp_freq: f32,
    pub freq_osc1: Box<dyn Oscillator>,
    pub freq_osc2: Box<dyn Oscillator>,
    pub amp_osc: Box<dyn Oscillator>,
    pub sample_rate: usize,
    pub should_offset: bool,
    pub div_instead_of_mul: bool,

    freq_phase: f32,
    amp_phase: f32,
}

impl FreqPhaseAmplitudeLFO {
    pub fn get_idx(&self)->EffectIdx{
        EffectIdx(self.idx)
    }
    pub fn update_first_oscillator(&mut self, osc: impl Oscillator + 'static) {
        self.freq_osc1 = Box::new(osc);
    }
    pub fn update_second_oscillator(&mut self, osc: impl Oscillator + 'static) {
        self.freq_osc2 = Box::new(osc);
    }
    pub fn update_oscillator(&mut self, osc: impl Oscillator + 'static) {
        self.amp_osc = Box::new(osc);
    }
    pub fn update_freq_freq(&mut self, freq: f32) {
        self.freq_freq = freq;
    }
    pub fn update_freq_delta(&mut self, freq: f32) {
        self.freq_delta = freq;
    }
    pub fn update_freq(&mut self, freq: f32) {
        self.amp_freq = freq;
    }
    pub fn update_should_offset(&mut self, should_offset: bool) {
        self.should_offset = should_offset;
    }
    pub fn update_div_instead_of_mul(&mut self, flag: bool) {
        self.div_instead_of_mul = flag;
    }

    pub fn calculate(&mut self, sample: f32) -> f32 {
        let inverse_sample_rate = 1.0 / self.sample_rate as f32;

        let ctx1 = crate::synth::OscillatorCtx {
            amplitude: self.freq_delta / self.freq_freq,
            freq: self.freq_freq,
            phase: sample/(2.0 * std::f32::consts::PI*self.freq_freq),
            sampling_rate: self.sample_rate,
        };

        let o1 = self.freq_osc1.generate_sample(ctx1, self.freq_phase);

        let ctx2 = crate::synth::OscillatorCtx {
            amplitude: 1.0,
            freq: sample,
            phase: (o1+sample) / (2.0 * std::f32::consts::PI * sample),
            sampling_rate: self.sample_rate,
        };

        let result = self.freq_osc2.generate_sample(ctx2, self.freq_phase);

        let ctx = crate::synth::OscillatorCtx {
            amplitude: 1.0,
            freq: 1.0,
            phase: sample,
            sampling_rate: self.sample_rate,
        };

        let offset = if self.should_offset { 1.0 } else { 0.0 };
        let carrier = self.amp_osc.generate_sample(ctx, self.amp_phase) + offset;

        self.freq_phase += inverse_sample_rate;
        self.freq_phase %= 1.0;
        self.amp_phase += self.amp_freq * inverse_sample_rate;
        self.amp_phase %= 1.0;

        if self.div_instead_of_mul {
            result / (carrier + 1.0).exp()
        } else {
            carrier * result
        }
    }
}





pub struct LFOUsingDelay {
    pub idx:usize,
    pub dry: f32,
    pub wet: f32,
    pub feedback: f32,
    pub should_offset: bool,
    pub div_instead_of_mul: bool,

    sampling_rate: usize,
    buffer: Box<[f32]>,
    read: usize,
    write: usize,
}

impl LFOUsingDelay {
    pub fn new(
        idx:usize,
        sampling_rate: usize,
        dry: f32,
        wet: f32,
        feedback: f32,
        delay: f32,
        should_offset: bool,
        div_instead_of_mul: bool,
    ) -> Self {
        let len = 2 * (sampling_rate as f32 * delay).ceil() as usize;

        let mut vec = Vec::with_capacity(len);
        vec.fill(0.0);
        let buffer = vec.into_boxed_slice();

        let read = 0;
        let write = len / 2;
        Self {
            idx,
            dry,
            wet,
            feedback,
            sampling_rate,
            buffer,
            write,
            read,
            should_offset,
            div_instead_of_mul,
        }
    }

    pub fn update_dry(&mut self, dry: f32) {
        self.dry = dry;
    }
    pub fn update_wet(&mut self, wet: f32) {
        self.wet = wet;
    }
    pub fn update_feedback(&mut self, feedback: f32) {
        self.feedback = feedback;
    }

    pub fn update_should_offset(&mut self, should_offset: bool) {
        self.should_offset = should_offset;
    }
    pub fn update_div_instead_of_mul(&mut self, flag: bool) {
        self.div_instead_of_mul = flag;
    }

    pub fn calculate(&mut self, sample: f32) -> f32 {
        let out = self.dry * sample + self.wet * self.buffer[self.read];
        self.buffer[self.write] = sample + self.buffer[self.read] * self.feedback;
        self.write += 1;
        self.read += 1;
        self.write %= self.buffer.len();
        self.read %= self.buffer.len();
        let offset = if self.should_offset { 1.0 } else { 0.0 };
        if self.div_instead_of_mul {
            sample / (out + offset + 1.0).exp()
        } else {
            sample * (out + offset)
        }
    }
    pub fn get_idx(&self)->EffectIdx{
        EffectIdx(self.idx)
    }
}
