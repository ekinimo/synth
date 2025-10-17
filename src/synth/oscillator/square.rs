use crossbeam_channel::Sender;
use egui::{util::IdTypeMap, Id, Slider, Ui};

use crate::synth::{ModifierIdx, Modifiers, Oscillator, OscillatorControls, OscillatorCtx, Synth, SynthMessage};

use super::{phasor::Phasor, trig::Sine, OscillatorModification, Simple};


#[derive(Debug, Clone,PartialEq,  PartialOrd)]
pub struct Square { pub duty_cycle: f32 }

impl Square {
    

    pub fn update_duty_cycle(&mut self, duty_cycle: f32) {
        self.duty_cycle = duty_cycle;
    }
}

        impl Default for Square {
    fn default() -> Self {
        Self { duty_cycle: 0.5 }
    }
}

impl Oscillator for Square {
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
        let base = freq * (time + phase);
        let lower = (base + self.duty_cycle).floor();
        let upper = base.ceil();
        let normalized = 2.0 * (lower - upper) +1.0;
        amplitude * normalized
    }
}

#[derive(Debug,Clone,PartialEq, PartialOrd)]
pub struct SquareDutyCycler{
    freq:f32,
    time:f32,
    osc:Simple
}


impl   Modifiers for SquareDutyCycler{
    fn update(&mut self ) -> OscillatorModification {
        let params = OscillatorCtx{
            amplitude: 0.35, freq: 1.0, phase: 0.0, sampling_rate: 44100
        };
        let duty_cycle = self.osc.generate_sample(params, self.time)  + 0.5;
        self.time += self.freq/44100.0;
        if self.time > 1.0{
            self.time *=1.0;
        }
        OscillatorModification::SquareDutyCycle(duty_cycle)
    }
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
}


impl OscillatorControls for Square{
    fn draw_controls(&mut self, ui: &mut Ui) -> Option<super::OscillatorModification> {
        let mut modification = None;
        ui.horizontal(|ui| {
            if ui.add(Slider::new(&mut self.duty_cycle, 0.0..=1.0).text("Duty Cycle")).changed() {
                modification = Some(OscillatorModification::SquareDutyCycle(self.duty_cycle));
            }

            let mut modifier_enabled = ui.data_mut(|data | *data.get_persisted_mut_or_default(Id::new("square_duty_cycle")) );

            if ui.checkbox(&mut modifier_enabled, "Enable Modifier").changed() {
                if modifier_enabled {
                    modification = Some(OscillatorModification::AddModifier(crate::synth::OscillatorModifiers::SquareDutyCycler(SquareDutyCycler{
                        freq: 2.0,
                        time: -1.0,
                        osc: Simple::Phasor(Phasor),
                    })));
                    /*Osc::UpdateModifier(crate::synth::OscillatorModifiers::SquareDutyCycler(SquareDutyCycler{
                        freq: 2.0,
                        time: 0.0,
                        osc: Simple::Phasor(Phasor),
                    }))).unwrap();*/
                } else {
                    // Send a message to disable the modifier
                    //sender.send(SynthMessage::DisableModifier).unwrap();
                }
            }

            // Store the updated value back in egui's memory

            ui.data_mut(|x| x.insert_persisted(Id::new("square_duty_cycle"), modifier_enabled) )
        });
        modification

    }
}

/*impl OscillatorControls for Square {
    fn draw_controls(&mut self, ui: &mut Ui, sender: Sender<SynthMessage>) {
        ui.horizontal(|ui| {
            if ui.add(Slider::new(&mut self.duty_cycle, 0.0..=1.0).text("Duty Cycle")).changed() {
                sender.send(SynthMessage::UpdateOscillator(Simple::Square(self.clone()))).unwrap();
            }
        });

        
        let mut modifier_enabled = ui.data_mut(|data | *data.get_persisted_mut_or_default(Id::new("square_duty_cycle")) );

        if ui.checkbox(&mut modifier_enabled, "Enable Modifier").changed() {
            if modifier_enabled {
                // Create and send the modifier function
                //let modifier_fn = create_modifier_function(); // Replace with your logic
                sender.send(SynthMessage::UpdateModifier(crate::synth::AllModifiers::SquareDutyCycler(SquareDutyCycler{
                    freq: 2.0,
                    time: 0.0,
                    osc: Simple::Phasor(Phasor),
                }))).unwrap();
            } else {
                // Send a message to disable the modifier
                //sender.send(SynthMessage::DisableModifier).unwrap();
            }
        }

        // Store the updated value back in egui's memory

        ui.data_mut(|x| x.insert_persisted(Id::new("square_duty_cycle"), modifier_enabled) )
        
    }
}

*/
