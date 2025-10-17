use crate::synth::{Effect, EffectIdx, Oscillator, OscillatorCtx, OscillatorIdx};

pub enum DelayLike {
    Delay(Delay),
    MultiDelay(MultiTapDelay),
    Chorus(Chorus),
    Vibrato(Vibrato),
    Flanger(Flanger),
    Doppler(Doppler),
}

impl Effect for DelayLike {
    fn process(&mut self, sample: f32) -> f32 {
        self.calculate(sample)
    }

    fn get_idx(&self)->EffectIdx {
        self.get_idx()
    }
    

}

impl DelayLike {


    pub fn calculate(&mut self, sample: f32) -> f32 {
        match self {
            DelayLike::Delay(delay) => delay.calculate(sample),
            DelayLike::MultiDelay(multi_tap_delay) => multi_tap_delay.calculate(sample),
            DelayLike::Chorus(chorus) => chorus.calculate(sample),
            DelayLike::Vibrato(vibrato) => vibrato.calculate(sample),
            DelayLike::Flanger(flanger) => flanger.calculate(sample),
            DelayLike::Doppler(doppler) => doppler.calculate(sample),
        }
    }

    pub fn get_idx(&self)->EffectIdx{
        match self {

        DelayLike::Delay(delay) => delay.get_idx(),
        DelayLike::MultiDelay(multi_tap_delay) => multi_tap_delay.get_idx(),
        DelayLike::Chorus(chorus) => chorus.get_idx(),
        DelayLike::Vibrato(vibrato) => vibrato.get_idx(),
        DelayLike::Flanger(flanger) => flanger.get_idx(),
        DelayLike::Doppler(doppler) => doppler.get_idx(),
    }
    }
}

pub struct Doppler {
    pub idx:usize,
    pub delta: f32,
    sampling_rate: usize,
    buffer: Box<[f32]>,
    read: f32,
    write: usize,
}
pub struct Chorus {
    idx:usize,
    sample_rate: usize,
    flangers: Vec<ChorusElem>, // TODO make ChorusElement an Effect 
}
// TODO
// Do a version with size change / interpolation
pub struct Delay {
    //
    pub idx:usize,
    pub dry: f32,
    pub wet: f32,
    pub feedback: f32,
    sampling_rate: usize,
    buffer: Box<[f32]>,
    read: usize,
    write: usize,
}

pub struct MultiTapDelay {
    pub idx:usize,
    delays: Vec<Delay>,
    dry: f32,
    wet: f32,
    sampling_rate: usize,
}
pub struct Vibrato {
    pub idx:usize,
    pub freq: f32,
    pub sweep_width: f32, //Width of the LFO in samples
    pub osc: Box<dyn Oscillator>,

    buffer: Box<[f32]>,
    write: usize,
    phase: f32,
    sample_rate: usize,
}

pub struct Flanger {
    pub idx:usize,
    pub freq: f32,
    pub sweep_width: f32, //Width of the LFO in samples
    pub depth: f32,
    pub feedback: f32,
    pub osc: Box<dyn Oscillator>,

    buffer: Box<[f32]>,
    write: usize,
    phase: f32,
    sample_rate: usize,
}

impl Doppler {
    pub fn new(idx:usize,sampling_rate: usize, delta: f32, delay: f32) -> Self {
        let len = 2 * (sampling_rate as f32 * delay).ceil() as usize;

        let mut vec = Vec::with_capacity(len);
        vec.fill(0.0);
        let buffer = vec.into_boxed_slice();

        let read = 0.0;
        let write = len / 2;
        Self {
            idx,
            delta,
            sampling_rate,
            buffer,
            read,
            write,
        }
    }

    pub fn calculate(&mut self, sample: f32) -> f32 {
        let len = self.buffer.len() as f32;
        let sample_before = self.read.floor() % len;
        let sample_after = (sample_before + 1.0) % len;
        let fract = self.read - sample_before;
        let out = (1.0 - fract) * self.buffer[sample_before as usize]
            + fract * self.buffer[sample_after as usize];
        self.buffer[self.write] = sample;
        self.write += 1;
        self.write %= self.buffer.len();
        self.read += 1.0 - self.delta;
        if self.read >= len {
            self.read -= len
        }
        out
    }

    pub fn get_idx(&self)->EffectIdx{
        EffectIdx(self.idx)
    }
}
impl Effect for Doppler {
    fn process(&mut self, sample: f32) -> f32 {
        self.calculate(sample)
    }

    fn get_idx(&self)->crate::synth::EffectIdx {
        todo!()
    }
}

impl Delay {
    pub fn new(idx:usize,sampling_rate: usize, dry: f32, wet: f32, feedback: f32, delay: f32) -> Self {
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
        }
    }
    pub fn get_idx(&self)->EffectIdx{
        EffectIdx(self.idx)
    }
    
    pub fn calculate(&mut self, sample: f32) -> f32 {
        let out = self.dry * sample + self.wet * self.buffer[self.read];
        self.buffer[self.write] = sample + self.buffer[self.read] * self.feedback;
        self.write += 1;
        self.read += 1;
        self.write %= self.buffer.len();
        self.read %= self.buffer.len();
        out
    }
}

impl Effect for Delay {
    fn process(&mut self, sample: f32) -> f32 {
        self.calculate(sample)
    }
    fn get_idx(&self)->EffectIdx{
        EffectIdx(self.idx)
    }
}

impl MultiTapDelay {
    pub fn new(idx:usize,sampling_rate: usize, dry: f32, wet: f32) -> Self {
        let delays = vec![];
        Self {
            idx,
            delays,
            dry,
            wet,
            sampling_rate,
        }
    }

    
    pub fn add(&mut self, delay: Delay) {
        self.delays.push(delay);
    }

    pub fn get_idx(&self)->EffectIdx{
        EffectIdx(self.idx)
    }

    pub fn calculate(&mut self, sample: f32) -> f32 {
        let n = self
            .delays
            .iter_mut()
            .map(|x| x.calculate(sample))
            .sum::<f32>()
            / (self.delays.len() as f32);
        let out = self.dry * sample + self.wet * n;
        out
    }
}

impl Effect for MultiTapDelay {
    fn process(&mut self, sample: f32) -> f32 {
        self.calculate(sample)
    }

    fn get_idx(&self)->EffectIdx {
        self.get_idx()
    }
    
}

impl Vibrato {
    pub fn new(
        idx:usize,
        sample_rate: usize,
        freq: f32,
        sweep_width: f32,
        osc: impl Oscillator + 'static,
        delay: f32,
    ) -> Self {
        let len = 2 * (sample_rate as f32 * delay).ceil() as usize;

        let mut vec = Vec::with_capacity(len);
        vec.fill(0.0);
        let buffer = vec.into_boxed_slice();

        let phase = 0.0;
        let write = len / 2;
        Self {
            idx,
            freq,
            sweep_width,
            osc: Box::new(osc),
            buffer,
            write,
            phase,
            sample_rate,
        }
    }

    pub fn get_idx(&self)->EffectIdx{
        EffectIdx(self.idx)
    }

    pub fn calculate(&mut self, sample: f32) -> f32 {
        let sample_rate = self.sample_rate as f32;
        let inverse_sample_rate = 1.0 / sample_rate;
        let write = self.write as f32;
        let len = self.buffer.len() as f32;

        let ctx = OscillatorCtx {
            amplitude: 0.5,
            freq: 1.0,
            phase: 0.0,
            sampling_rate: self.sample_rate,
        };
        let curr_delay = self.sweep_width * (0.5 + self.osc.generate_sample(ctx, self.phase));
        let read = (write - (curr_delay * sample_rate) + len - 3.0) % (len);

        let fract = read - read.floor();
        let prev_sample = (read.floor() % len) as usize;
        let next_sample = (prev_sample + 1) % self.buffer.len();
        let interp_sample =
            fract * self.buffer[next_sample] + (1.0 - fract) * self.buffer[prev_sample];
        self.buffer[self.write] = sample;
        self.write += 1;
        self.write %= self.buffer.len();

        self.phase += self.freq * inverse_sample_rate;
        self.phase %= 1.0;
        interp_sample
    }
}

impl Effect for Vibrato {
    fn process(&mut self, sample: f32) -> f32 {
        self.calculate(sample)
    }

    fn get_idx(&self)->EffectIdx {
        self.get_idx()
    }

}

impl Flanger {
    pub fn new(
        idx:usize,
        sample_rate: usize,
        freq: f32,
        sweep_width: f32,
        depth: f32,
        feedback: f32,
        osc: impl Oscillator + 'static,
        delay: f32,
    ) -> Self {
        let len = 2 * (sample_rate as f32 * delay).ceil() as usize;

        let mut vec = Vec::with_capacity(len);
        vec.fill(0.0);
        let buffer = vec.into_boxed_slice();

        let phase = 0.0;
        let write = len / 2;
        Self {
            idx,
            freq,
            sweep_width,
            depth,
            osc: Box::new(osc),
            buffer,
            write,
            phase,
            sample_rate,
            feedback,
        }
    }

    pub fn get_idx(&self)->EffectIdx{
        EffectIdx(self.idx)
    }

    pub fn calculate(&mut self, sample: f32) -> f32 {
        let sample_rate = self.sample_rate as f32;
        let inverse_sample_rate = 1.0 / sample_rate;
        let write = self.write as f32;
        let len = self.buffer.len() as f32;

        let ctx = OscillatorCtx {
            amplitude: 0.5,
            freq: 1.0,
            phase: 0.0,
            sampling_rate: self.sample_rate,
        };
        let curr_delay = self.sweep_width * (0.5 + self.osc.generate_sample(ctx, self.phase));
        let read = (write - (curr_delay * sample_rate) + len - 3.0) % (len);

        let fract = read - read.floor();
        let prev_sample = (read.floor() % len) as usize;
        let next_sample = (prev_sample + 1) % self.buffer.len();
        let interp_sample =
            fract * self.buffer[next_sample] + (1.0 - fract) * self.buffer[prev_sample];
        self.buffer[self.write] = sample + interp_sample * self.feedback;
        self.write += 1;
        self.write %= self.buffer.len();

        self.phase += self.freq * inverse_sample_rate;
        self.phase %= 1.0;
        sample + interp_sample * self.depth
    }
}

pub struct ChorusElem {
    
    pub freq: f32,
    pub sweep_width: f32, //Width of the LFO in samples
    pub depth: f32,
    pub feedback: f32,
    pub osc: Box<dyn Oscillator>,

    buffer: Box<[f32]>,
    write: usize,
    phase: f32,
}

impl ChorusElem {
    pub fn new(
        freq: f32,
        sweep_width: f32,
        depth: f32,
        feedback: f32,
        osc: impl Oscillator + 'static,
        delay: f32,
        sample_rate: usize,
    ) -> Self {
        let len = 2 * (sample_rate as f32 * delay).ceil() as usize;

        let mut vec = Vec::with_capacity(len);
        vec.fill(0.0);
        let buffer = vec.into_boxed_slice();

        let phase = 0.0;
        let write = len / 2;
        Self {
            freq,
            sweep_width,
            depth,
            osc: Box::new(osc),
            buffer,
            write,
            phase,
            feedback,
        }
    }
    pub fn update_freq(&mut self, freq: f32) {
        self.freq = freq;
    }
    pub fn update_sweep_width(&mut self, sweep_width: f32) {
        self.sweep_width = sweep_width
    }
    pub fn update_depth(&mut self, depth: f32) {
        self.depth = depth;
    }
    pub fn update_feedback(&mut self, feedback: f32) {
        self.feedback = feedback
    }

    pub fn update_oscillator(&mut self, osc: impl Oscillator + 'static) {
        self.osc = Box::new(osc)
    }

    pub fn modify(
        &mut self,
        freq: f32,
        sweep_width: f32,
        depth: f32,
        feedback: f32,
        osc: impl Oscillator + 'static,
    ) {
        self.freq = freq;
        self.sweep_width = sweep_width;
        self.depth = depth;
        self.feedback = feedback;
        self.osc = Box::new(osc);
    }

    pub fn calculate(&mut self, sampling_rate: usize, sample: f32) -> f32 {
        let sample_rate = sampling_rate as f32;
        let inverse_sample_rate = 1.0 / sample_rate;
        let write = self.write as f32;
        let len = self.buffer.len() as f32;

        let ctx = OscillatorCtx {
            amplitude: 0.5,
            freq: 1.0,
            phase: 0.0,
            sampling_rate: sampling_rate,
        };
        let curr_delay = self.sweep_width * (0.5 + self.osc.generate_sample(ctx, self.phase));
        let read = (write - (curr_delay * sample_rate) + len - 3.0) % (len);

        let fract = read - read.floor();
        let prev_sample = (read.floor() % len) as usize;
        let next_sample = (prev_sample + 1) % self.buffer.len();
        let interp_sample =
            fract * self.buffer[next_sample] + (1.0 - fract) * self.buffer[prev_sample];
        self.buffer[self.write] = sample + interp_sample * self.feedback;
        self.write += 1;
        self.write %= self.buffer.len();

        self.phase += self.freq * inverse_sample_rate;
        self.phase %= 1.0;

        interp_sample * self.depth
    }
}

impl Chorus {
    pub fn new(idx:usize,sample_rate: usize) -> Self {
        Self {
            idx,
            sample_rate,
            flangers: vec![],
        }
    }

    pub fn add(&mut self, elem: ChorusElem) {
        self.flangers.push(elem);
    }

    pub fn update_nth_freq(&mut self, idx: usize, freq: f32) {
        self.flangers[idx].update_freq(freq);
    }
    pub fn update_nth_sweep_width(&mut self, idx: usize, sweep_width: f32) {
        self.flangers[idx].update_sweep_width(sweep_width);
    }
    pub fn update_nth_depth(&mut self, idx: usize, depth: f32) {
        self.flangers[idx].update_depth(depth);
    }
    pub fn update_nth_feedback(&mut self, idx: usize, feedback: f32) {
        self.flangers[idx].update_feedback(feedback);
    }

    pub fn update_nth_oscillator(&mut self, idx: usize, osc: impl Oscillator + 'static) {
        self.flangers[idx].update_oscillator(osc);
    }

    pub fn modify_nth(
        &mut self,
        idx: usize,
        freq: f32,
        sweep_width: f32,
        depth: f32,
        feedback: f32,
        osc: impl Oscillator + 'static,
    ) {
        self.flangers[idx].modify(freq, sweep_width, depth, feedback, osc);
    }

    pub fn get_idx(&self)->EffectIdx{
        EffectIdx(self.idx)
    }

    pub fn calculate(&mut self, sample: f32) -> f32 {
        let sample_rate = self.sample_rate;
        let len = (self.flangers.len() + 1) as f32;
        (sample
            + self
                .flangers
                .iter_mut()
                .map(|x| x.calculate(sample_rate, sample))
                .sum::<f32>())
            / len
    }
}

impl Effect for Chorus {
    
    fn process(&mut self, sample: f32) -> f32 {
        self.calculate(sample)
    }

    fn get_idx(&self)->EffectIdx {
        self.get_idx()
    }

}
