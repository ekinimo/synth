use egui::Ui;
use oscillator::phasor;

use crate::synth::{self, oscillator, Oscillator, OscillatorControls, OscillatorCtx};
use super::{phasor::Phasor, OscillatorModification, Simple};

#[derive(Debug, Clone,PartialEq, PartialOrd)]
pub struct Circle {
    ramp: Box<Simple>//Box<dyn Oscillator>,
}
impl Circle {
    pub(crate) fn update(&mut self, modif: Box<OscillatorModification>) {
        self.ramp.update(*modif);
    }
}

impl Default for Circle {
    fn default() -> Self {
        Self {
            ramp: Box::new(Simple::Phasor(Phasor::default()))//Box::new(Phasor::default()),
        }
    }
}

impl Oscillator for Circle {
    fn generate_sample(&self, ctx: OscillatorCtx, time: f32) -> f32 {
        let amp = ctx.amplitude;
        let x = 2.0 * self.ramp.generate_sample(ctx, time) - 1.0; // Map to [-1, 1]
        let ret = (1.0 - x.powi(2)).sqrt();
        let non_nan = if ret.is_nan(){ 0.0} else{ret};
        non_nan*amp
    }
}

impl OscillatorControls for Circle {
    fn draw_controls(&mut self, ui: &mut Ui) -> Option<OscillatorModification> {
        let mut  ret = None;
        ui.label("Circle Settings:");
        ui.vertical(|ui| {
            ret = self.ramp.draw_controls(ui)
                .map(|m| {
                    OscillatorModification::CircleMod(Box::new(m))
                });
        });
        ret
    }
}
