use std::f32::consts::PI;

use egui::{ComboBox, ScrollArea, Slider};
use egui_plot::{Arrows, Line, Plot, PlotPoints, Points};

use crate::synth::{Oscillator, OscillatorControls, OscillatorCtx};

use super::{
    phasor::Phasor,
    triangle::{ModifiableTriangle, Triangle},
    trig::{Sine, Trig},
    OscillatorModification, Simple,
};

#[derive(Debug, Clone,PartialEq, PartialOrd)]
pub enum MorphModification {
    Waveform1(Box<OscillatorModification>),
    Waveform2(Box<OscillatorModification>),
    Strategy(MorphStrategy),
    Value(f32),
}

#[derive(Debug, Clone, Copy, PartialEq, PartialOrd)]
pub enum MorphStrategy {
    Linear,
    Geometric,
    Cubic,
    Exp,
    Harmonic,

    Power(f32),
    GeometricPower(f32),
    ExpPow(f32),
    Trigonometric(f32),

    Rational(f32, f32),
}

impl MorphStrategy {
    pub fn should_draw_slider(&self) -> bool {
        match self {
            Self::Rational(_, _)
            | Self::Linear
            | Self::Geometric
            | Self::Cubic
            | Self::Exp
            | Self::Harmonic => false,
            _ => true,
        }
    }
    pub fn should_draw_two_slider(&self) -> bool {
        match self {
            Self::Rational(_, _) => true,
            _ => false,
        }
    }
    pub fn interpolate(&self, a: f32, b: f32, t: f32) -> f32 {
        match self {
            MorphStrategy::Linear => a * (1.0 - t) + b * (t),
            MorphStrategy::Geometric => a.powf(1.0 - t) * b.powf(t),
            MorphStrategy::Cubic => {
                a * (1.0 - t).powi(3)
                    + b * 3.0 * (1.0 - t).powi(2) * t
                    + a * 3.0 * (1.0 - t) * t.powi(2)
                    + b * t.powi(3)
            }
            MorphStrategy::Exp => (a.ln() * (1.0 - t) + b.ln() * (t)).exp(),
            MorphStrategy::Harmonic => 1.0 / ((1.0 - t) / a + t / b),
            MorphStrategy::Power(n) => a * (1.0 - t.powf(*n)) + b * (t.powf(*n)),
            MorphStrategy::GeometricPower(n) => a.powf(1.0 - t.powf(*n)) * b.powf(t.powf(*n)),
            MorphStrategy::ExpPow(n) => (a.ln() * (1.0 - t.powf(*n)) + b.ln() * (t.powf(*n))).exp(),
            MorphStrategy::Trigonometric(n) => (a.powf(*n) * (t * PI / 2.0).cos().powf(2.0)
                + b.powf(*n) * (t * PI / 2.0).sin().powf(2.0))
            .powf(1.0 / n),
            MorphStrategy::Rational(n, k) => {
                (a * (1.0 - t.powf(*n)) + b * (t.powf(*n)))
                    / (1.0 + *k * t.powf(*n) * (1.0 - t).powf(*n))
            }
        }
    }

    fn strategy_name(&self) -> String {
        match self {
            MorphStrategy::Linear => "Linear".to_string(),
            MorphStrategy::Geometric => "Geometric".to_string(),
            MorphStrategy::Cubic => "Cubic".to_string(),
            MorphStrategy::Exp => "Exponential".to_string(),
            MorphStrategy::Harmonic => "Harmonic".to_string(),
            MorphStrategy::Power(_n) => "Power".to_string(),
            MorphStrategy::GeometricPower(_n) => "Geometric Power".to_string(),
            MorphStrategy::ExpPow(_n) => "Exponential Power".to_string(),
            MorphStrategy::Trigonometric(_n) => "Trigonometric".to_string(),
            MorphStrategy::Rational(_n, _k) => "Rational".to_string(),
        }
    }
}

#[derive(Debug, Clone, PartialEq, PartialOrd)]
pub struct Morph {
    waveforms: Box<(Simple, Simple)>,
    strategy: MorphStrategy,
    morph: f32, // Parameter to interpolate between waveforms [0, 1]
}

impl Default for Morph {
    fn default() -> Self {
        Self {
            waveforms: Box::new((
                Simple::Trig(Trig::Sine(Sine::default())),
                Simple::Triangle(Triangle::Triangle(ModifiableTriangle::default())),
            )),
            morph: 0.5,
            strategy: MorphStrategy::Linear,
        }
    }
}

impl Morph {
    pub fn new(waveforms: Box<(Simple, Simple)>) -> Self {
        Self {
            waveforms,
            morph: 0.5,
            strategy: MorphStrategy::Linear,
        }
    }

    pub(crate) fn update(&mut self, modif: MorphModification) {
        match modif {
            MorphModification::Waveform1(oscillator_modification) => {
                self.waveforms.0.update(*oscillator_modification)
            }
            MorphModification::Waveform2(oscillator_modification) => {
                self.waveforms.1.update(*oscillator_modification)
            }
            MorphModification::Strategy(morph_strategy) => self.strategy = morph_strategy,
            MorphModification::Value(val) => self.morph = val,
        }
    }
}

impl Oscillator for Morph {
    fn generate_sample(&self, ctx: OscillatorCtx, time: f32) -> f32 {
        let amp = ctx.amplitude;
        let (osc1, osc2) = &*self.waveforms;

        let sample1 = osc1.generate_sample(ctx.clone(), time) + amp + 1.0;
        let sample2 = osc2.generate_sample(ctx, time) + amp + 1.0;

        self.strategy.interpolate(sample1, sample2, self.morph) - amp - 1.0
    }
}

impl OscillatorControls for Morph {
    fn draw_controls(&mut self, ui: &mut egui::Ui) -> Option<super::OscillatorModification> {
        let mut modification = None;
        let mut osc1_modif = None;
        let mut osc2_modif = None;

        ui.vertical(|ui| {
            // Morph slider
            ui.horizontal(|ui| {
                ui.label("Morph:");
                if ui
                    .add(Slider::new(&mut self.morph, 0.0..=1.0).text("Morph"))
                    .changed()
                {
                    dbg!("morph should change");
                    modification = Some(OscillatorModification::Morph(MorphModification::Value(
                        self.morph,
                    )));
                }
            });

            // Morph Strategy Selection
            ui.horizontal(|ui| {
                ui.label("Interpolation Strategy:");
                ComboBox::from_label("Strategy")
                    .selected_text(self.strategy.strategy_name())
                    .show_ui(ui, |ui| {
                        let strategies = [
                            MorphStrategy::Linear,
                            MorphStrategy::Geometric,
                            MorphStrategy::Cubic,
                            MorphStrategy::Exp,
                            MorphStrategy::Harmonic,
                            MorphStrategy::Power(1.0),
                            MorphStrategy::GeometricPower(1.0),
                            MorphStrategy::ExpPow(1.0),
                            MorphStrategy::Trigonometric(1.0),
                            MorphStrategy::Rational(1.0, 1.0),
                        ];

                        for strategy in strategies.iter() {
                            if ui
                                .radio_value(
                                    &mut self.strategy,
                                    *strategy,
                                    strategy.strategy_name(),
                                )
                                .changed()
                            {
                                self.strategy = *strategy;
                                modification = Some(OscillatorModification::Morph(
                                    MorphModification::Strategy(*strategy),
                                ));
                            }
                        }
                    });
                let plot_points = PlotPoints::from_iter((0..(1000)).map(|i| {
                    let x = i as f32 / 1000.0;
                    let y = self.strategy.interpolate(1.0, 2.0, x) - 1.0;
                    [x as f64, y as f64]
                }));
                let marker_points_x = PlotPoints::from_iter((0..(1000)).map(|i| {
                    let x = i as f64 / 1000.0;
                    let y_max = self.strategy.interpolate(1.0, 2.0, self.morph) as f64 - 1.0;
                    let y = y_max * x;
                    [self.morph as f64, y]
                }));
                let marker_points_y = PlotPoints::from_iter((0..(1000)).map(|i| {
                    let t = i as f64 / 1000.0;
                    let y = self.strategy.interpolate(1.0, 2.0, self.morph) as f64 - 1.0;
                    let x = t * self.morph as f64;
                    [x, y]
                }));

                Plot::new("Interpolation function")
                    .height(100.0)
                    .width(100.0)
                    .show(ui, |plot_ui| {
                        plot_ui.line(Line::new(plot_points));
                        plot_ui.line(Line::new(marker_points_x));
                        plot_ui.line(Line::new(marker_points_y));
                    });
            });

            //let mut strategy = ;
            // Parameterized strategy controls
            //ui.horizontal(|ui| {});
            match &mut self.strategy {
                MorphStrategy::Power(n) => {
                    ui.horizontal(|ui| {
                        ui.label("Parameter:");
                        if ui.add(Slider::new(n, 0.1..=10.0).text("n")).changed() {
                            let x = MorphStrategy::Power(*n);
                            modification =
                                Some(OscillatorModification::Morph(MorphModification::Strategy(x)));
                        }
                    });
                }
                MorphStrategy::GeometricPower(n) => {
                    ui.horizontal(|ui| {
                        ui.label("Parameter:");
                        if ui.add(Slider::new(n, 0.1..=10.0).text("n")).changed() {
                            let x = MorphStrategy::GeometricPower(*n);
                            modification =
                                Some(OscillatorModification::Morph(MorphModification::Strategy(x)));
                        }
                    });
                }
                MorphStrategy::ExpPow(n) => {
                    ui.horizontal(|ui| {
                        ui.label("Parameter:");
                        if ui.add(Slider::new(n, 0.1..=10.0).text("n")).changed() {
                            let x = MorphStrategy::ExpPow(*n);
                            modification =
                                Some(OscillatorModification::Morph(MorphModification::Strategy(x)));
                        }
                    });
                }
                MorphStrategy::Trigonometric(n) => {
                    ui.horizontal(|ui| {
                        ui.label("Parameter:");
                        if ui.add(Slider::new(n, 0.1..=10.0).text("n")).changed() {
                            let x = MorphStrategy::Trigonometric(*n);
                            modification =
                                Some(OscillatorModification::Morph(MorphModification::Strategy(x)));
                        }
                    });
                }
                MorphStrategy::Rational(n, k) => {
                    ui.horizontal(|ui| {
                        ui.label("Parameters:");
                        ui.horizontal(|ui| {
                            ui.label("n:");
                            if ui.add(Slider::new(n, 0.1..=10.0)).changed() {
                                let x = MorphStrategy::Rational(*n, *k);
                                modification = Some(OscillatorModification::Morph(
                                    MorphModification::Strategy(x),
                                ));
                            }
                            ui.label("k:");
                            let x = MorphStrategy::Rational(*n, *k);
                            if ui.add(Slider::new(k, -10.0..=10.0)).changed() {
                                modification = Some(OscillatorModification::Morph(
                                    MorphModification::Strategy(x),
                                ));
                            }
                        });
                    });
                }
                _ => {}
            }

            ui.separator();
            ui.label("Oscillators:");
            ui.group(|ui| {
                ui.horizontal(|ui| {
                    ui.push_id(0, |ui| {
                        ui.label("First Oscillator:");

                        osc1_modif = self.waveforms.0.draw_controls(ui).map(|m| {
                            OscillatorModification::Morph(MorphModification::Waveform1(Box::new(m)))
                        });
                    });
                    ui.separator();
                    ui.push_id(1, |ui| {
                        ui.label("Second Oscillator:");

                        osc2_modif = self.waveforms.1.draw_controls(ui).map(|m| {
                            OscillatorModification::Morph(MorphModification::Waveform2(Box::new(m)))
                        });
                    });
                });
            });
        });
        let osc_modif = osc1_modif.or(osc2_modif);
        osc_modif.or(modification)
    }
}
