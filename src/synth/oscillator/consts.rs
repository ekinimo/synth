use crate::synth::{Oscillator, OscillatorCtx};

pub enum ConstModifier{}

pub enum ContextualOscillator {
    Constant(Constant),
    Contexted(Contexted),
}

impl ContextualOscillator {
    pub fn update_osc(&mut self, osc: Box<dyn Oscillator>) {
        match self {
            ContextualOscillator::Constant(c) => c.osc = osc,
            ContextualOscillator::Contexted(c) => c.osc = osc,
        }
    }

    pub fn update_ctx(&mut self, ctx: OscillatorCtx) {
        match self {
            ContextualOscillator::Constant(c) => c.ctx = ctx,
            ContextualOscillator::Contexted(c) => c.ctx = ctx,
        }
    }

    pub fn update_time(&mut self, current_time: f32) {
        match self {
            ContextualOscillator::Contexted(c) => c.current_time = current_time,
            _ => {}
        }
    }

    pub fn update_phase_offset(&mut self, current_phase_offset: f32) {
        match self {
            ContextualOscillator::Contexted(c) => c.current_phase_offset = current_phase_offset,
            _ => {}
        }
    }

    pub fn update_amp_offset(&mut self, current_amp_offset: f32) {
        match self {
            ContextualOscillator::Contexted(c) => c.current_amp_offset = current_amp_offset,
            _ => {}
        }
    }

    pub fn update_freq_offset(&mut self, current_freq_offset: f32) {
        match self {
            ContextualOscillator::Contexted(c) => c.current_freq_offset = current_freq_offset,
            _ => {}
        }
    }
}

impl Oscillator for ContextualOscillator {
    fn generate_sample(&self, ctx: OscillatorCtx, time: f32) -> f32 {
        match self {
            ContextualOscillator::Constant(osc) => osc.generate_sample(ctx, time),
            ContextualOscillator::Contexted(osc) => osc.generate_sample(ctx, time),
        }
    }
}


pub struct Constant {
    osc: Box<dyn Oscillator>,
    ctx: OscillatorCtx,
}

impl Constant {
    pub fn new(osc: Box<dyn Oscillator>, ctx: OscillatorCtx) -> Self {
        Self { osc, ctx }
    }

    pub fn update_osc(&mut self, osc: Box<dyn Oscillator>) {
        self.osc = osc;
    }

    pub fn update_ctx(&mut self, ctx: OscillatorCtx) {
        self.ctx = ctx;
    }
}

impl Oscillator for Constant {
    fn generate_sample(&self, _ctx: OscillatorCtx, time: f32) -> f32 {
        self.osc.generate_sample(self.ctx.clone(), time) 
    }
}

pub struct Contexted {
    osc: Box<dyn Oscillator>,
    ctx: OscillatorCtx,
    current_time: f32,
    current_phase_offset: f32,
    current_amp_offset: f32,
    current_freq_offset: f32,
}

impl Contexted {
    pub fn new(
        osc: Box<dyn Oscillator>,
        ctx: OscillatorCtx,
        current_time: f32,
        current_phase_offset: f32,
        current_amp_offset: f32,
        current_freq_offset: f32,
    ) -> Self {
        Self {
            osc,
            ctx,
            current_time,
            current_phase_offset,
            current_amp_offset,
            current_freq_offset,
        }
    }
    pub fn update_osc(&mut self, osc: Box<dyn Oscillator>) {
        self.osc = osc;
    }

    pub fn update_ctx(&mut self, ctx: OscillatorCtx) {
        self.ctx = ctx;
    }

    pub fn update_current_time(&mut self, current_time: f32) {
        self.current_time = current_time;
    }

    pub fn update_current_phase_offset(&mut self, current_phase_offset: f32) {
        self.current_phase_offset = current_phase_offset;
    }

    pub fn update_current_amp_offset(&mut self, current_amp_offset: f32) {
        self.current_amp_offset = current_amp_offset;
    }

    pub fn update_current_freq_offset(&mut self, current_freq_offset: f32) {
        self.current_freq_offset = current_freq_offset;
    }
}

impl Oscillator for Contexted {
    fn generate_sample(&self, _ctx: OscillatorCtx, _time: f32) -> f32 {
        
        let mut ctx = self.ctx.clone();
        ctx.freq += self.current_freq_offset;
        ctx.phase += self.current_phase_offset;
        ctx.amplitude += self.current_amp_offset;
        self.osc.generate_sample(ctx, self.current_time)
    }
}
