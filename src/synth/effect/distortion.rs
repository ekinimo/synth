use crate::synth::{Effect, EffectIdx};

impl Distortion{
    pub fn calculate(&mut self,sample:f32)->f32{
        match self {
            Distortion::HardClipping(hard_clipping) => hard_clipping.calculate(sample),
            Distortion::SoftClipping(soft_clipping) => soft_clipping.calculate(sample),
            Distortion::ExponentialSoftClipping(exponential_soft_clipping) => exponential_soft_clipping.calculate(sample),
            Distortion::FullWaveRectifier(full_wave_rectifier) => full_wave_rectifier.calculate(sample),
            Distortion::TanhSaturator(tanh_saturator) => tanh_saturator.calculate(sample),
            Distortion::SoftMaxSaturator(soft_max_saturator) => soft_max_saturator.calculate(sample),
            Distortion::SoftAbsoluteSaturator(soft_absolute_saturator) => soft_absolute_saturator.calculate(sample),
            Distortion::SoftQuadraticSaturator(soft_quadratic_saturator) => soft_quadratic_saturator.calculate(sample),
        }
    }
    pub fn get_idx(&self)->EffectIdx{
        match self {
            Distortion::HardClipping(hard_clipping) => hard_clipping.get_idx(),
            Distortion::SoftClipping(soft_clipping) => soft_clipping.get_idx(),
            Distortion::ExponentialSoftClipping(exponential_soft_clipping) => exponential_soft_clipping.get_idx(),
            Distortion::FullWaveRectifier(full_wave_rectifier) => full_wave_rectifier.get_idx(),
            Distortion::TanhSaturator(tanh_saturator) => tanh_saturator.get_idx(),
            Distortion::SoftMaxSaturator(soft_max_saturator) => soft_max_saturator.get_idx(),
            Distortion::SoftAbsoluteSaturator(soft_absolute_saturator) => soft_absolute_saturator.get_idx(),
            Distortion::SoftQuadraticSaturator(soft_quadratic_saturator) => soft_quadratic_saturator.get_idx(),
        }
    }

}

impl Effect for Distortion{
    fn process(&mut self, sample: f32) -> f32 {
        self.calculate(sample)
    }

    fn get_idx(&self)->EffectIdx {
        self.get_idx()
    }

}


pub struct HardClipping{
    idx:usize,
    gain:f32,
    threshold:f32,
}
pub struct SoftClipping{
    idx:usize,
    gain:f32,
    low_threshold:f32,
    high_threshold:f32,
}
pub struct ExponentialSoftClipping{
    idx:usize,
    gain:f32,
}
pub struct FullWaveRectifier{
    idx:usize,
    gain:f32,
}
pub struct HalfWaveRectifier{
    idx:usize,
    gain:f32,
}
pub struct TanhSaturator{
    idx:usize,
    gain:f32,
}
pub struct SoftMaxSaturator{
    idx:usize,
    gain:f32,
}
pub struct SoftAbsoluteSaturator{
    idx:usize,
    gain:f32,
    pow:usize,
}
pub struct SoftQuadraticSaturator{
    idx:usize,
    gain:f32,
    pow:usize,
}


pub enum Distortion{
    HardClipping(HardClipping),
    SoftClipping(SoftClipping),
    ExponentialSoftClipping(ExponentialSoftClipping),
    FullWaveRectifier(FullWaveRectifier),
    TanhSaturator(TanhSaturator),
    SoftMaxSaturator(SoftMaxSaturator),
    SoftAbsoluteSaturator(SoftAbsoluteSaturator),
    SoftQuadraticSaturator(SoftQuadraticSaturator)
}


impl Effect for HardClipping {
    fn process(&mut self, sample: f32) -> f32 {
        self.calculate(sample)
    }
    fn get_idx(&self)->EffectIdx {
        self.get_idx()
    }

}

impl Effect for SoftClipping {
    fn process(&mut self, sample: f32) -> f32 {
        self.calculate(sample)
    }
    fn get_idx(&self)->EffectIdx {
        self.get_idx()
    }

}

impl Effect for ExponentialSoftClipping {
    fn process(&mut self, sample: f32) -> f32 {
        self.calculate(sample)
    }
    fn get_idx(&self)->EffectIdx {
        self.get_idx()
    }

}

impl Effect for FullWaveRectifier {
    fn process(&mut self, sample: f32) -> f32 {
        self.calculate(sample)
    }
    fn get_idx(&self)->EffectIdx {
        self.get_idx()
    }

}

impl Effect for HalfWaveRectifier {
    fn process(&mut self, sample: f32) -> f32 {
        self.calculate(sample)
    }
    fn get_idx(&self)->EffectIdx {
        self.get_idx()
    }

}

impl Effect for TanhSaturator{
    fn process(&mut self, sample: f32) -> f32 {
        self.calculate(sample)
    }
    fn get_idx(&self)->EffectIdx {
        self.get_idx()
    }

}
impl Effect for SoftAbsoluteSaturator{
    fn process(&mut self, sample: f32) -> f32 {
        self.calculate(sample)
    }
    fn get_idx(&self)->EffectIdx {
        self.get_idx()
    }

}
impl Effect for SoftMaxSaturator{
    fn process(&mut self, sample: f32) -> f32 {
        self.calculate(sample)
    }
    fn get_idx(&self)->EffectIdx {
        self.get_idx()
    }

}
impl Effect for SoftQuadraticSaturator{
    fn process(&mut self, sample: f32) -> f32 {
        self.calculate(sample)
    }
    fn get_idx(&self)->EffectIdx {
        self.get_idx()
    }

}

impl TanhSaturator{
    pub fn new(idx:usize,gain:f32)->Self{
        Self { idx,gain }
    }

    pub fn get_idx(&self)->EffectIdx{
        EffectIdx(self.idx)
    }
    pub fn calculate(&mut self,mut sample:f32)->f32{
        let gain = 10.0_f32.powf( self.gain / 20.0);
        sample *=gain ;
        sample.tanh()
    }

}

impl SoftMaxSaturator{
    pub fn new(idx:usize,gain:f32)->Self{
        Self { idx,gain }
    }

    pub fn get_idx(&self)->EffectIdx{
        EffectIdx(self.idx)
    }
    
    pub fn calculate(&mut self,mut sample:f32)->f32{
        let gain = 10.0_f32.powf( self.gain / 20.0);
        sample *= -gain ;
        2.0/ (sample.exp()+1.0) - 1.0
    }

}
impl SoftAbsoluteSaturator{
    pub fn new(idx:usize,gain:f32,pow:usize)->Self{
        Self { idx,gain,pow }
    }
    pub fn get_idx(&self)->EffectIdx{
        EffectIdx(self.idx)
    }
    
    pub fn calculate(&mut self,mut sample:f32)->f32{
        let gain = 10.0_f32.powf( self.gain / 20.0);
        sample *= -gain ;
        sample.signum()/ (1.0+sample.abs().powi(- (self.pow as i32))) 
    }

}

impl SoftQuadraticSaturator{
    pub fn new(idx:usize,gain:f32,pow:usize)->Self{
        Self { idx,gain,pow }
    }

    pub fn get_idx(&self)->EffectIdx{
        EffectIdx(self.idx)
    }
    
    pub fn calculate(&mut self,mut sample:f32)->f32{
        let gain = 10.0_f32.powf( self.gain / 20.0);
        sample *= -gain ;
        sample.signum()/ (1.0+sample.powi(- (2*self.pow as i32)))
    }

}



impl HardClipping{
    pub fn new(idx:usize,gain:f32,threshold:f32)->Self{
        Self { idx,gain ,threshold}
    }
    pub fn get_idx(&self)->EffectIdx{
        EffectIdx(self.idx)
    }

    pub fn calculate(&mut self,mut sample:f32)->f32{
        let gain = 10.0_f32.powf( self.gain / 20.0);
        sample *=gain ;

        if sample> self.threshold{
            self.threshold
        }else if sample < -self.threshold{
            -self.threshold
        }
        else{
            sample
        }
    }

}

impl SoftClipping{
    pub fn new(idx:usize,gain:f32,low_threshold:f32,high_threshold:f32)->Self{
        Self { idx,gain ,low_threshold,high_threshold}
    }
    pub fn get_idx(&self)->EffectIdx{
        EffectIdx(self.idx)
    }
    pub fn calculate(&mut self,mut sample:f32)->f32{
        let gain = 10.0_f32.powf( self.gain / 20.0);
        sample *=gain ;

        if sample> self.high_threshold{
            1.0
        }else if sample > self.low_threshold{
            (3.0 - (2.0 - 3.0 * sample) * (2.0 - 3.0 * sample) )/3.0
        }else if sample < -self.high_threshold{
            -1.0
        }else if sample > self.low_threshold{
            -(3.0 - (2.0 + 3.0 * sample) * (2.0 + 3.0 * sample) )/3.0
        }
        else{
            2.0*sample
        }
    }
}

impl ExponentialSoftClipping{
    pub fn new(idx:usize,gain:f32)->Self{
        Self { idx,gain }
    }
    pub fn get_idx(&self)->EffectIdx{
        EffectIdx(self.idx)
    }
    pub fn calculate(&mut self,mut sample:f32)->f32{
        let gain = 10.0_f32.powf( self.gain / 20.0);
        sample *=gain ;

        if sample > 0.0{
            1.0 - (-sample).exp()
        } else{
            sample.exp_m1()
        }
    }
}

impl FullWaveRectifier{
    pub fn new(idx:usize,gain:f32)->Self{
        Self { idx,gain }
    }
    pub fn get_idx(&self)->EffectIdx{
        EffectIdx(self.idx)
    }
    pub fn calculate(&mut self,mut sample:f32)->f32{
        let gain = 10.0_f32.powf( self.gain / 20.0);
        sample *=gain ;

        sample.abs()
    }
}
impl HalfWaveRectifier{
    pub fn new(idx:usize,gain:f32,)->Self{
        Self { idx,gain }
    }
    pub fn get_idx(&self)->EffectIdx{
        EffectIdx(self.idx)
    }
    pub fn calculate(&mut self,mut sample:f32)->f32{
        let gain = 10.0_f32.powf( self.gain / 20.0);
        sample *=gain ;

        if sample > 0.0{
            sample
        } else{
            0.0
        }
    }
}
