use crossbeam_channel::Sender;
use egui::{ComboBox, Ui};

use crate::{select_variant, synth::{Oscillator, OscillatorControls, OscillatorCtx, SynthMessage}};

use super::{OscillatorModification, Simple};

#[derive(Debug, Clone,PartialEq, Eq, PartialOrd, Ord)]
pub struct Sine;

impl Default for Sine {
    fn default() -> Self {
        Self
    }
}

impl Oscillator for Sine {
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
        amplitude * (2.0 * std::f32::consts::PI * freq * (time + phase)).sin()
    }
}



#[derive(Debug, Clone,PartialEq, Eq, PartialOrd, Ord)]
pub struct Cosine;

impl Default for Cosine {
    fn default() -> Self {
        Self
    }
}

impl Oscillator for Cosine {
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
        amplitude * (2.0 * std::f32::consts::PI * freq * (time + phase)).cos()
    }
}

#[derive(Debug, Clone,PartialEq, Eq, PartialOrd, Ord)]
pub struct Tan;

impl Default for Tan {
    fn default() -> Self {
        Self
    }
}

impl Oscillator for Tan {
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
        amplitude * (2.0 * std::f32::consts::PI * freq * (time + phase)).tan().clamp(-1.0, 1.0)
    }
}

#[derive(Debug, Clone,PartialEq, Eq, PartialOrd, Ord)]
pub struct Cotan;

impl Default for Cotan {
    fn default() -> Self {
        Self
    }
}

impl Oscillator for Cotan {
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
        amplitude /(2.0 * std::f32::consts::PI * freq * (time + phase)).tan().clamp(-1.0, 1.0)
    }
}


#[derive(Debug, Clone,PartialEq, Eq, PartialOrd, Ord)]
pub enum Trig {
    Sine(Sine),
    Cosine(Cosine),
    Tan(Tan),
    Cotan(Cotan),
}
impl Trig {
    fn variant_name(&self) ->&str {
        match self{
            Trig::Sine(_sine) => "Sine",
            Trig::Cosine(_cosine) => "Cosine",
            Trig::Tan(_tan) => "Tan",
            Trig::Cotan(_cotan) => "Cotan",
        }
    }
}

impl Default for Trig {
    fn default() -> Self {
        Self::Sine(Sine)
    }
}

impl Oscillator for Trig {
    fn generate_sample(
        & self,
        ctx: OscillatorCtx,
        time: f32,
    ) -> f32 {
        match self {
            Trig::Sine(sine) => sine.generate_sample(ctx, time),
            Trig::Cosine(cosine) => cosine.generate_sample(ctx, time),
            Trig::Tan(tan) => tan.generate_sample(ctx, time),
            Trig::Cotan(cotan) => cotan.generate_sample(ctx, time),
        }
    }
}

impl OscillatorControls for Trig {
    fn draw_controls(&mut self, ui: &mut Ui) -> Option<super::OscillatorModification> {
        let mut modification = None;

        ui.horizontal(|ui| {
            //let old_discriminant = std::mem::discriminant(self);

            ComboBox::from_label("Type")
                .selected_text(self.variant_name())
                .show_ui(ui, |ui| {
                    let mut changed = false;

                    changed |=
                        select_variant!(ui, self, Self::Sine(Sine), "Sine");
                    changed |=
                        select_variant!(ui, self, Self::Cosine(Cosine), "Cosine");
                    changed |=
                        select_variant!(ui, self, Self::Tan(Tan), "Tan");
                    changed |=
                        select_variant!(ui, self, Self::Cotan(Cotan), "Cotan");
                    
                    
                    if changed {
                        modification = Some(OscillatorModification::Replace(Simple::Trig(self.clone())));
                    }
                });
        });
        modification
    }
}

/*
impl OscillatorControls for Trig {
    fn draw_controls(&mut self, ui: &mut Ui, sender: Sender<SynthMessage>) {
        ui.horizontal(|ui| {
            ComboBox::from_label("Trigonometric Functions")
              .selected_text(match self {
                Trig::Sine(_sine) => "Sine",
                  Trig::Cosine(_cosine) => "Cosine",
                  Trig::Tan(_tan) => "Tan",
                  Trig::Cotan(_cotan) => "Cotan",
                          })
              .show_ui(ui, |ui| {
                  if ui.selectable_value(self, Self::Sine(Sine), "Sine").changed() {
                  sender.send(SynthMessage::UpdateOscillator(Simple::Trig(self.clone()))).unwrap();
                  }
                  if ui.selectable_value(self, Self::Cosine(Cosine), "Cosine").changed() {
                      sender.send(SynthMessage::UpdateOscillator(Simple::Trig(self.clone()))).unwrap();
                  }
                  if ui.selectable_value(self, Self::Tan(Tan), "Tan").changed() {
                      sender.send(SynthMessage::UpdateOscillator(Simple::Trig(self.clone()))).unwrap();
                  }
                  if ui.selectable_value(self, Self::Cotan(Cotan), "Cotan").changed() {
                      sender.send(SynthMessage::UpdateOscillator(Simple::Trig(self.clone()))).unwrap();
                  }
                  
                });
        });

        
    }
}
*/
