use std::ops::{Add, Mul};

use egui::{ComboBox, DragValue, Ui};

use super::{phasor::Phasor, OscillatorModification, Simple};
use crate::synth::{Oscillator, OscillatorControls, OscillatorCtx};

#[derive(Clone,Debug,PartialEq,  PartialOrd)]
pub struct AddMulPow {
    ramp: Box<Simple>,//Box<dyn Oscillator>,
    add: f32,
    mul: f32,
    pow: f32,
}
impl AddMulPow {
    pub(crate) fn update(&mut self, modifier: AddMulPowModification) {
        match modifier{
            AddMulPowModification::Add(val) => {self.add = val;},
            AddMulPowModification::Mul(val) => {self.mul = val;},
            AddMulPowModification::Pow(val) => {self.pow = val;},
            AddMulPowModification::RampModification(val) => {
                self.ramp.update(*val);
            },
        }
    }
}

impl Default for AddMulPow {
    fn default() -> Self {
        Self {
            ramp: Box::new(Simple::Phasor(Phasor::default())),
            pow: 1.0,
            mul: 1.0,
            add: 0.0,
        }
    }
}


impl Oscillator for AddMulPow {
    fn generate_sample(&self, ctx: OscillatorCtx, time: f32) -> f32 {
        //let amp = ctx.amplitude;
        let phase = self.ramp.generate_sample(ctx, time)  ;
        
        let ret = phase.powf(self.pow).mul(self.mul).add(self.add);
        let non_nan = if ret.is_nan(){ 0.0} else{ret};
        non_nan
    }
}
#[derive(Debug, Clone,PartialEq, PartialOrd)]
pub enum AddMulPowModification{
    Add(f32),
    Mul(f32),
    Pow(f32),
    RampModification(Box<OscillatorModification>)
}


impl OscillatorControls for AddMulPow {
    fn draw_controls(&mut self, ui: &mut Ui) -> Option<OscillatorModification> {
        let mut modification = None;

        ui.horizontal(|ui| {
            ui.label("Add:");
            if ui.add(DragValue::new(&mut self.add).speed(0.1)).changed() {
                dbg!("i changed");
                modification = Some(AddMulPowModification::Add(self.add));
            }

            ui.label("Mul:");
            if ui.add(DragValue::new(&mut self.mul).speed(0.1)).changed() {
                modification = Some(AddMulPowModification::Mul(self.mul));
            }

            ui.label("Pow:");
            if ui.add(DragValue::new(&mut self.pow).speed(0.1)).changed() {
                modification = Some(AddMulPowModification::Pow(self.pow));
            }
        });

        let mut inner_mod = None;
        ui.vertical(|ui| {
            inner_mod = self.ramp.draw_controls(ui)
                .map(|m| {
                    AddMulPowModification::RampModification(Box::new(m))
                });
        });
        

        modification.or(inner_mod).map(|m| OscillatorModification::AddMulPowMod(m))
    }
}
