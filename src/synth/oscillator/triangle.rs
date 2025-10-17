use std::ops::{ Div, Mul, Sub};
use crossbeam_channel::Sender;
use egui::{ComboBox, Id, Slider, Ui};
use egui_plot::{Plot, PlotPoints};

use crate::{select_variant, synth::{Modifiers, Oscillator, OscillatorControls, OscillatorCtx, Synth, SynthMessage}};

use super::{phasor::Phasor, OscillatorModification, Simple};

#[derive(Debug, Clone,PartialEq,  PartialOrd )]
pub struct ModifiableTriangle { pub duty_cycle: f32 }

#[derive(Debug,Clone,PartialEq, PartialOrd)]
pub struct TriangleDutyCycler{
    freq:f32,
    time:f32,
    osc:Simple
}


impl OscillatorControls for ModifiableTriangle{
    fn draw_controls(&mut self, ui: &mut Ui) -> Option<super::OscillatorModification> {
        let mut modification = None;
        ui.horizontal(|ui| {
            if ui.add(Slider::new(&mut self.duty_cycle, 0.0..=1.0).text("Duty Cycle")).changed() {
                modification = Some(OscillatorModification::TriangleDutyCycle(self.duty_cycle));
            }
        });
        modification

    }
}


impl   Modifiers for TriangleDutyCycler{
    /*fn update( self )-> (Self,Box< dyn for<'a> FnMut(&'a mut Synth) > ) where Self:Sized{
        let me = Self{
            freq:*&self.freq,
            time:(&self.time + 1.0/44100.0),
            osc:Simple::Phasor(Phasor)
        };
        let f = Box::new(move |synth:&mut Synth|{
            let params = OscillatorCtx{
                amplitude: 0.35, freq: self.freq, phase: 0.0, sampling_rate: 44100
            };
            let duty_cycle = self.osc.generate_sample(params, self.time)  + 0.5;
            synth.oscillator.update_duty_cycle(duty_cycle);
        });
        (me,f)
    }*/
    fn update(&mut self ) -> OscillatorModification {
        let params = OscillatorCtx{
            amplitude: 0.35, freq: self.freq, phase: 0.0, sampling_rate: 44100
        };
        let duty_cycle = self.osc.generate_sample(params, self.time)  + 0.5;
        self.time += 1.0/44100.0;
        self.time %=2.0;
        OscillatorModification::TriangleDutyCycle(duty_cycle)
    }
}


impl Default for ModifiableTriangle {
    fn default() -> Self {
        Self { duty_cycle: 0.5 }
    }
}

impl Oscillator for ModifiableTriangle {
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

        let amplitude : f64 = amplitude.into();
        let freq : f64 = freq.into();
        let phase :f64 = phase.into();
        let time :f64 = time.into();
        let dc :f64 = self.duty_cycle.into();

        fn h1(f: f64, t: f64, d: f64) -> f64 {
            let var = f * t + d;
            let floored = var.floor();
            floored
                .mul(floored.sub(1.0))
                .div(2.0)
                + floored.mul(var.fract())
        }

        fn h2(f: f64, t: f64) -> f64 {
            let var = f * t;
            let ceiled = var.ceil();
            ceiled
                .mul(ceiled.sub(1.0))
                .div(2.0) + ceiled.mul(var.fract())
                
        }

        let coeff = -2.0 / (dc - dc * dc );
        let ret = (coeff*(h2(freq, time + phase+0.25)
                 - h1(freq, time + phase+0.25, dc)
                - (1.0 - dc) * freq * (time + phase + 0.25))
          + 1.0).clamp(-1.0, 1.0)*amplitude;

        ret as f32
    }
}


#[derive(Clone, Copy, Debug, PartialEq,Default,Eq, PartialOrd, Ord)]
pub struct LeftSawtooth;
#[derive(Clone, Copy, Debug, PartialEq,Default,Eq, PartialOrd, Ord)]
pub struct RightSawtooth;

impl Oscillator for LeftSawtooth {
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
        let omega = freq * (time + phase);
        let norm = 2.0 * (omega - omega.floor()) - 1.0;
        amplitude * norm
    }
}

impl Oscillator for RightSawtooth {
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
        let omega = freq * (time + phase);
        let norm =  1.0- (2.0*(omega - omega.floor())-1.0);
        amplitude * norm
    }
}

#[derive(Clone,Debug,PartialEq,  PartialOrd)]
pub enum Triangle{
    Triangle(ModifiableTriangle),
    LeftSaw(LeftSawtooth),
    RightSaw(RightSawtooth)
}
impl Triangle {
    fn variant_name(&self) ->&str {
        match self{
            Triangle::Triangle(_) => "Triangle",
            Triangle::LeftSaw(_) => "Left Sawtoth",
            Triangle::RightSaw(_) => "Right Sawtooth",
        }
    }
}

impl Default for Triangle {
    fn default() -> Self {
        Self::Triangle(ModifiableTriangle::default())
    }
}


impl Oscillator for Triangle {
    fn generate_sample(&self, ctx: OscillatorCtx, phase: f32) -> f32 {
        match self {
            Triangle::Triangle(triangle) => triangle.generate_sample(ctx, phase),
            Triangle::LeftSaw(left_sawtooth) => left_sawtooth.generate_sample(ctx, phase),
            Triangle::RightSaw(right_sawtooth) => right_sawtooth.generate_sample(ctx, phase),
        }
    }
}


impl OscillatorControls for Triangle{
    fn draw_controls(&mut self, ui: &mut Ui) -> Option<super::OscillatorModification> {
        let mut modification = None;

        ui.horizontal(|ui| {
            let old_discriminant = std::mem::discriminant(self);

            ComboBox::from_label("Triangle Type")
                .selected_text(self.variant_name())
                .show_ui(ui, |ui| {
                    let mut changed = false;

                    changed |=
                        select_variant!(ui, self, Self::Triangle(ModifiableTriangle{duty_cycle:0.5}), "Triangle");
                    changed |=
                        select_variant!(ui, self, Self::LeftSaw(LeftSawtooth), "Left Sawtooth");
                    changed |=
                        select_variant!(ui, self, Self::RightSaw(RightSawtooth), "Right Sawtooth");
                    // ... other variants

                    if changed {
                        modification = Some(OscillatorModification::Replace(Simple::Triangle(self.clone())));
                    }
                });
        });

        let inner_mod = match self {
            Triangle::Triangle(modifiable_triangle) => modifiable_triangle.draw_controls(ui),
            Triangle::LeftSaw(_left_sawtooth) => None,
            Triangle::RightSaw(_right_sawtooth) => None,
        };

        modification.or(inner_mod)
    }
}



/*impl OscillatorControls for Triangle {
    fn draw_controls(&mut self, ui: &mut Ui, sender: Sender<SynthMessage>) {
        ui.horizontal(|ui| {
            ComboBox::from_label("Triangle Type")
              .selected_text(match self {
                Triangle::Triangle(_) => "Triangle",
                Triangle::LeftSaw(_) => "Left Sawtooth",
                  Triangle::RightSaw(_) => "Right Sawtooth",
                          })
              .show_ui(ui, |ui| {
                    if ui.selectable_value(self, Self::Triangle(ModifiableTriangle::default()), "Triangle").changed() {
                        sender.send(SynthMessage::UpdateOscillator(Simple::Triangle(self.clone()))).unwrap();
                    }
                    if ui.selectable_value(self, Self::LeftSaw(LeftSawtooth), "Left Sawtooth").changed() {
                        sender.send(SynthMessage::UpdateOscillator(Simple::Triangle(self.clone()))).unwrap();
                    }
                  if ui.selectable_value(self, Self::RightSaw(RightSawtooth), "Right Sawtooth").changed() {
                      sender.send(SynthMessage::UpdateOscillator(Simple::Triangle(self.clone()))).unwrap();
                    }
                  
                });
        });

        ui.horizontal(|ui| {
            match self {
                Triangle::Triangle(modifiable_triangle) => modifiable_triangle.draw_controls(ui, sender.clone()),
                Triangle::LeftSaw(_) => (),
                Triangle::RightSaw(_) => (),
            }
        });
    }
}
*/
