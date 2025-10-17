//TODO abstract away modifiers into its own trait
//  struct Transient{
//    start_time: Option<Instant>,
//    release_time: Option<Instant>,
//    pub is_released: bool,
//  }
//
//

use crossbeam_channel::Sender;
use egui_plot::{Line, Plot};
use crate::synth::SynthMessage;
use egui::{Ui};

use super::{AdsrControls};
//use std::f32::consts::E; // Import E for exponential functions

pub enum Interpolation {
    Linear,
    Power(usize),
    Log,
    Exp,
    Tanh,
    // Spline, // You'll need a spline library for this
}

pub enum Perturbator {
    None,
    Sine(f32), // Frequency of the sine wave
}

pub struct ADSR {
    sample_rate: u32,
    attack_time: f32,
    decay_time: f32,
    sustain_level: f32,
    release_time: f32,
    attack_interpolation: Interpolation,
    decay_interpolation: Interpolation,
    sustain_perturbator: Perturbator,
}

impl ADSR {
    pub fn new(sample_rate: u32) -> Self {
        ADSR {
            sample_rate,
            attack_time: 0.2,
            decay_time: 0.1,
            sustain_level: 0.7,
            release_time: 0.5,
            attack_interpolation: Interpolation::Linear,
            decay_interpolation: Interpolation::Linear,
            sustain_perturbator: Perturbator::None,
        }
    }

    // note_on is tick 0
    pub fn get_amplitude(&self, tick: u32) -> f32 {
        let time = tick as f32 / self.sample_rate as f32;

        if time < self.attack_time {
            // Attack phase
            match self.attack_interpolation {
                Interpolation::Linear => time / self.attack_time,
                Interpolation::Power(exponent) => (time / self.attack_time).powf(exponent as f32),
                Interpolation::Log => (time / self.attack_time).log10(), // Use log10 for a smoother curve
                Interpolation::Exp => (time / self.attack_time).exp(), // Use exp for exponential growth
                Interpolation::Tanh => (time / self.attack_time * 5.0).tanh(), // Use tanh for a sigmoid-like curve
                //Interpolation::Spline => {
                //    // Implement spline interpolation here
                //    0.0
                //},
            }
        } else if time < self.attack_time + self.decay_time {
            // Decay phase
            let decay_time = time - self.attack_time;
            let decay_amount = 1.0 - self.sustain_level;
            match self.decay_interpolation {
                Interpolation::Linear => 1.0 - (decay_time / self.decay_time) * decay_amount,
                Interpolation::Power(exponent) => 1.0 - (decay_time / self.decay_time).powf(exponent as f32) * decay_amount,
                Interpolation::Log => 1.0 - (decay_time / self.decay_time).log10() * decay_amount,
                Interpolation::Exp => 1.0 - (decay_time / self.decay_time).exp() * decay_amount,
                Interpolation::Tanh => 1.0 - (decay_time / self.decay_time * 5.0).tanh() * decay_amount,
                //Interpolation::Spline => {
                //    // Implement spline interpolation here
                //    0.0
                //},
            }
        } else {
            // Sustain phase
            match self.sustain_perturbator {
                Perturbator::None => self.sustain_level,
                Perturbator::Sine(freq) => {
                    self.sustain_level + (time * freq * 2.0 * std::f32::consts::PI).sin() * 0.1
                }
            }
        };
        1.0
        
    }

    // note off is tick 0
    pub fn get_amplitude_after_release(&self, tick: u32) -> f32 {
        let time = tick as f32 / self.sample_rate as f32;
        let remaining_sustain = self.sustain_level - (time / self.release_time) * self.sustain_level;
        remaining_sustain.max(0.0)
    }

    pub fn should_release(&self, tick: u32) -> bool {
        let time = tick as f32 / self.sample_rate as f32;
        time > self.release_time
    }

    pub fn draw_controls(&mut self, ui: &mut Ui, sender: Sender<SynthMessage>) {
        ui.heading("ADSR");

        // Create a plot for the ADSR envelope
        let plot = Plot::new("adsr_plot")
          .view_aspect(2.0)
            .height(60.0)
            .width(60.0)
          .show_axes([false, false]);

        plot.show(ui, |plot_ui| {
            let points: egui_plot::PlotPoints = [
             //,
                [self.attack_time.into(), 1.0 as f64],
                [<f32 as Into<f64>>::into(self.attack_time) + <f32 as Into<f64>>::into(self.decay_time), self.sustain_level.into()],
                [4.0, self.sustain_level.into()], // Placeholder for sustain phase end
                [4.0  + <f32 as Into<f64>>::into(self.release_time), 0.0],
            ]
          .into_iter()
          //.map(|&p| Value::new(p, p))
          .collect();
            let line = Line::new(points);
            plot_ui.line(line);

            // Add draggable points for each ADSR stage
            //... (Implementation for draggable points)
        });

        // Sliders for ADSR parameters
        ui.horizontal(|ui| {
            ui.label("Attack:");
            ui.add(egui::DragValue::new(&mut self.attack_time).speed(0.01));
        });
        //... (Sliders for decay, sustain, release)

        // Comboboxes for interpolation and perturbator options
        //... (Implementation for comboboxes)
    }
}


impl AdsrControls for ADSR {
    fn draw_controls(&mut self, ui: &mut Ui, sender: Sender<SynthMessage>) {
        self.draw_controls(ui, sender);
    }
}
