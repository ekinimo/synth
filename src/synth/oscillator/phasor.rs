use crossbeam_channel::Sender;
use egui::Ui;

use crate::synth::{Oscillator, OscillatorControls, OscillatorCtx, SynthMessage};

use super::{OscillatorModification, Simple};

#[derive(Debug, Clone,Eq,PartialEq, PartialOrd, Ord)]
pub struct Phasor;

impl Default for Phasor {
    fn default() -> Self { Self }
}

impl Oscillator for Phasor {
    fn generate_sample(&self, OscillatorCtx { freq, phase,amplitude, .. }: OscillatorCtx, time: f32) -> f32 {
        2.0*amplitude*(time * freq + phase).fract() - amplitude 
    }
}
impl OscillatorControls for Phasor {
    fn draw_controls(&mut self, ui: &mut Ui) -> Option<OscillatorModification> {
        None
    }
}


