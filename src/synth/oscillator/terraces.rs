use crate::synth::{ Oscillator, OscillatorCtx};

#[derive(Clone,Copy,Debug,PartialEq)]
pub struct TerracedLeftSawTooth{
    pub levels:u32,
}

impl Default for TerracedLeftSawTooth{
    fn default() -> Self {
        Self { levels: 3 }
    }
}
#[derive(Clone,Copy,Debug,PartialEq)]
pub struct TerracedRightSawTooth{
    pub levels:u32,
}
impl Default for TerracedRightSawTooth{
    fn default() -> Self {
        Self { levels: 3 }
    }
}

#[derive(Clone,Copy,Debug,PartialEq)]
pub struct TerracedTriangle{
    pub levels:u32,
}
impl Default for TerracedTriangle{
    fn default() -> Self {
        Self { levels: 3 }
    }
}




impl Oscillator for TerracedLeftSawTooth {
    fn generate_sample(
        &self,
        OscillatorCtx {
            amplitude,
            freq,
            phase,
            ..
        }: OscillatorCtx,
        time: f32,
    ) -> f32 {
        let base =  freq *(time+phase);
        let levels = self.levels as f32;
        let upper = base.floor()*levels + base.ceil()*levels;
        let lower = (base*levels).floor() + (base*levels).ceil();
        let norm = (upper - lower) / (levels-1.0);
        amplitude * norm
    }
}
impl Oscillator for TerracedRightSawTooth {
    fn generate_sample(
        &self,
        OscillatorCtx {
            amplitude,
            freq,
            phase,
            ..
        }: OscillatorCtx,
        time: f32,
    ) -> f32 {
        let base =  freq *(time+phase);
        let levels = self.levels as f32;
        let upper = base.floor()*levels + base.ceil()*levels;
        let lower = (base*levels).floor() + (base*levels).ceil();
        let normalized = -(upper - lower) / (levels-1.0);
        amplitude*normalized

    }
}
impl Oscillator for TerracedTriangle {
    fn generate_sample(
        &self,
        OscillatorCtx {
            amplitude,
            freq,
            phase,
            ..
        }: OscillatorCtx,
        time: f32,
    ) -> f32 {
        let base = freq*(time+phase);
        let sq_lower = (base+0.5).floor();
        let sq_upper = base.ceil();
        let sq= 2.0*(sq_lower - sq_upper) -1.0;

        let levels = self.levels as f32;
        let saw_upper = base.floor()*levels + base.ceil()*levels;
        let saw_lower = (base*levels).floor() + (base*levels).ceil();
        let saw = -(saw_upper - saw_lower) / (levels-1.0);
        let left = (sq+1.0)/2.0 * saw;
        let right = (-sq+1.0)/2.0 * -saw;

        let normalized = 2.0*(left + right) -1.0;

        amplitude * normalized
    }
}

#[derive(Clone,Copy,Debug,PartialEq)]
pub enum Terraced{
    RightSaw(TerracedRightSawTooth),
    LeftSaw(TerracedLeftSawTooth),
    Triangle(TerracedTriangle),
}

impl Default for Terraced{
    fn default() -> Self {
        Self::RightSaw(TerracedRightSawTooth::default())
    }
}
