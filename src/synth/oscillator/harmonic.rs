use egui::{ComboBox, Slider};

use super::{trig::Trig, OscillatorModification, Simple};
use crate::synth::{Oscillator, OscillatorControls, OscillatorCtx};

#[derive(Debug, Clone, PartialEq, PartialOrd)]
pub enum HarmonicOscillators {
    Basic(BasicHarmonicOscillator),
    BasicWithPhaseShift(BasicHarmonicOscillatorWithPhaseShift),
    WithPhaseShift(HarmonicOscillatorWithPhaseShift),
    Generic(GenericHarmonicOscillator),
}
impl Default for HarmonicOscillators {
    fn default() -> Self {
        Self::Basic(BasicHarmonicOscillator::default())
    }
}

impl HarmonicOscillators {
    fn name(&self) -> String {
        match self {
            HarmonicOscillators::Basic(_) => "Basic",
            HarmonicOscillators::BasicWithPhaseShift(_) => "Basic with Phase Shift",
            HarmonicOscillators::WithPhaseShift(_) => "Fourier-like",
            HarmonicOscillators::Generic(_) => "Generic",
        }
        .to_string()
    }

    pub(crate) fn update(&mut self, modif: HarmonicModification) {
        match modif {
            HarmonicModification::BaseOsc(modification) => match self {
                HarmonicOscillators::Basic(oscillator) => oscillator.base_osc.update(*modification),
                HarmonicOscillators::BasicWithPhaseShift(oscillator) => {
                    oscillator.base_osc.update(*modification)
                }
                HarmonicOscillators::WithPhaseShift(oscillator) => {
                    oscillator.base_osc.update(*modification)
                }
                HarmonicOscillators::Generic(_) => (),
            },
            HarmonicModification::WeightModification(idx, weight) => match self {
                HarmonicOscillators::Basic(oscillator) => oscillator.weights[idx] = weight,
                _ => (),
            },
            HarmonicModification::PhasedWeightModification(idx, w, p) => match self {
                HarmonicOscillators::BasicWithPhaseShift(oscillator) => {
                    oscillator.weights[idx] = (w, p)
                }
                _ => (),
            },
            HarmonicModification::PhaseAndWeightModification(idx, w1, p1, w2, p2) => match self {
                HarmonicOscillators::WithPhaseShift(oscillator) => {
                    oscillator.weights[idx] = (w1, p1, w2, p2)
                }
                _ => (),
            },
            HarmonicModification::OscillatorPhaseAndWeightModification(
                idx,
                o1,
                w1,
                p1,
                o2,
                w2,
                p2,
            ) => match self {
                HarmonicOscillators::Generic(oscillator) => {
                    let (o_1, w_1, p_1, o_2, w_2, p_2) = oscillator.weights.get_mut(idx).unwrap();
                    match (&*o1, &*o2) {
                        (OscillatorModification::None, OscillatorModification::None) => {
                            *w_1 = w1;
                            *w_2 = w2;
                            *p_1 = p1;
                            *p_2 = p2;
                        }
                        (OscillatorModification::None, x) => {
                            o_2.update(x.clone());
                            *w_1 = w1;
                            *w_2 = w2;
                            *p_1 = p1;
                            *p_2 = p2;
                        }
                        (x, OscillatorModification::None) => {
                            o_1.update(x.clone());
                            *w_1 = w1;
                            *w_2 = w2;
                            *p_1 = p1;
                            *p_2 = p2;
                        }

                        _ => (),
                    }
                }
                _ => (),
            },
            HarmonicModification::AddWeight => match self {
                HarmonicOscillators::Basic(oscillator) => {
                    oscillator.weights.push(1.0);
                }
                HarmonicOscillators::BasicWithPhaseShift(oscillator) => {
                    oscillator.weights.push((1.0, 0.0));
                }
                HarmonicOscillators::WithPhaseShift(oscillator) => {
                    oscillator.weights.push((1.0, 0.0, 1.0, 0.0));
                }
                HarmonicOscillators::Generic(oscillator) => {
                    oscillator.weights.push((
                        Simple::Trig(Trig::default()),
                        1.0,
                        0.0,
                        Simple::Trig(Trig::default()),
                        1.0,
                        0.0,
                    ));
                }
            },
            HarmonicModification::Remove(idx) => match self {
                HarmonicOscillators::Basic(oscillator) => {
                    oscillator.weights.remove(idx);
                }
                HarmonicOscillators::BasicWithPhaseShift(oscillator) => {
                    oscillator.weights.remove(idx);
                }
                HarmonicOscillators::WithPhaseShift(oscillator) => {
                    oscillator.weights.remove(idx);
                }
                HarmonicOscillators::Generic(oscillator) => {
                    oscillator.weights.remove(idx);
                }
            },
        }
    }
}

impl Oscillator for HarmonicOscillators {
    fn generate_sample(&self, ctx: OscillatorCtx, time: f32) -> f32 {
        match self {
            HarmonicOscillators::Basic(osc) => osc.generate_sample(ctx, time),
            HarmonicOscillators::BasicWithPhaseShift(osc) => osc.generate_sample(ctx, time),
            HarmonicOscillators::WithPhaseShift(osc) => osc.generate_sample(ctx, time),
            HarmonicOscillators::Generic(osc) => osc.generate_sample(ctx, time),
        }
    }
}

impl OscillatorControls for HarmonicOscillators {
    fn draw_controls(&mut self, ui: &mut egui::Ui) -> Option<OscillatorModification> {
        let mut modification = None;
        ComboBox::from_label("Oscillator Type")
            .selected_text(self.name())
            .show_ui(ui, |ui| {
                let simples = [
                    HarmonicOscillators::Basic(BasicHarmonicOscillator::default()),
                    HarmonicOscillators::BasicWithPhaseShift(BasicHarmonicOscillatorWithPhaseShift::default()),
                    HarmonicOscillators::WithPhaseShift(HarmonicOscillatorWithPhaseShift::default()),
                    HarmonicOscillators::Generic(GenericHarmonicOscillator::default()),
                ];
                for simple in simples.iter() {
                    if ui
                        .radio_value(self, simple.clone(), simple.name())
                        .changed()
                    {
                        modification = Some(OscillatorModification::Replace(Simple::Harmonic(self.clone()) ))
                    }
                }
            });
        let inner_modification = match self {
            HarmonicOscillators::Basic(osc) => osc.draw_controls(ui),
            HarmonicOscillators::BasicWithPhaseShift(osc) => osc.draw_controls(ui),
            HarmonicOscillators::WithPhaseShift(osc) => osc.draw_controls(ui),
            HarmonicOscillators::Generic(osc) => osc.draw_controls(ui),
        };
        modification.or(inner_modification)
    }
}

#[derive(Debug, Clone, PartialEq, PartialOrd)]
pub struct HarmonicOscillator {
    base_osc: Box<Simple>,    // Base oscillator (e.g., sine, phasor, etc.)
    weights: Vec<(f32, f32)>, // (weight for base oscillator, weight for 90-degree offset)
    sampling_rate: f32,
}

impl HarmonicOscillator {
    pub fn new(base_osc: Box<Simple>, weights: Vec<(f32, f32)>, sampling_rate: f32) -> Self {
        Self {
            base_osc,
            weights,
            sampling_rate,
        }
    }
}

impl Oscillator for HarmonicOscillator {
    fn generate_sample(&self, ctx: OscillatorCtx, time: f32) -> f32 {
        let mut sample_x = 0.0;
        let mut sample_y = 0.0;
        let mut weights_x = 0.0;
        let mut weights_y = 0.0;
        for (i, &(weight_base, weight_offset)) in self.weights.iter().enumerate() {
            let harmonic_freq = (i + 1) as f32 * ctx.freq;
            if harmonic_freq > self.sampling_rate as f32 / 2.0 {
                break;
            }

            // Base oscillator at harmonic frequency
            let base_ctx = OscillatorCtx {
                freq: harmonic_freq,
                amplitude: ctx.amplitude,
                phase: 0.0,
                sampling_rate: ctx.sampling_rate,
            };
            let base_sample = self.base_osc.generate_sample(base_ctx, time);

            let offset_time = time + 0.25 / harmonic_freq; // 90-degree phase shift
            let offset_sample = self.base_osc.generate_sample(base_ctx, offset_time);

            sample_x += weight_base * base_sample;
            sample_y += weight_offset * offset_sample;
            weights_x += weight_base;
            weights_y += weight_offset;
        }

        sample_x / (2.0 * weights_x) + sample_y / (2.0 * weights_y)
    }
}
#[derive(Debug, Clone, PartialEq, PartialOrd)]
pub struct BasicHarmonicOscillator {
    base_osc: Box<Simple>,
    weights: Vec<f32>,
    sampling_rate: f32,
}

impl Default for BasicHarmonicOscillator {
    fn default() -> Self {
        let x = Simple::Trig(Trig::default());
        BasicHarmonicOscillator::new(x, (0..4).map(|x| 2.0_f32.powi(-x)).collect(), 44100.0)
    }
}

impl Default for GenericHarmonicOscillator {
    fn default() -> Self {
        let osc = Simple::Trig(Trig::default());
        let weights = (0..4)
            .map(|x| 2.0_f32.powi(-x))
            .map(|x| (osc.clone(), x, 0.0, osc.clone(), x, 0.0))
            .collect();
        Self::new(weights, 44100.0)
    }
}
impl Default for BasicHarmonicOscillatorWithPhaseShift {
    fn default() -> Self {
        let x = Simple::Trig(Trig::default());
        let weights = (0..4).map(|x| 2.0_f32.powi(-x)).map(|x| (x, 0.0)).collect();
        Self::new(Box::new(x), weights, 44100.0)
    }
}
impl Default for HarmonicOscillatorWithPhaseShift {
    fn default() -> Self {
        let x = Simple::Trig(Trig::default());
        let weights = (0..4)
            .map(|x| 2.0_f32.powi(-x))
            .map(|x| (x, 0.0, x, 0.0))
            .collect();
        Self::new(Box::new(x), weights, 44100.0)
    }
}

impl BasicHarmonicOscillator {
    pub fn new(base_osc: Simple, weights: Vec<f32>, sampling_rate: f32) -> Self {
        Self {
            base_osc: Box::new(base_osc),
            weights,
            sampling_rate,
        }
    }
}

impl Oscillator for BasicHarmonicOscillator {
    fn generate_sample(&self, ctx: OscillatorCtx, time: f32) -> f32 {
        let mut sample_x = 0.0;
        let mut weights_x = 0.0;
        for (i, &weight_base) in self.weights.iter().enumerate() {
            let harmonic_freq = (i + 1) as f32 * ctx.freq;
            if harmonic_freq > self.sampling_rate as f32 / 2.0 {
                break;
            }

            let base_ctx = OscillatorCtx {
                freq: harmonic_freq,
                amplitude: ctx.amplitude,
                phase: 0.0,
                sampling_rate: ctx.sampling_rate,
            };
            let base_sample = self.base_osc.generate_sample(base_ctx, time);

            sample_x += weight_base * base_sample;
            weights_x += weight_base;
        }

        sample_x / (2.0 * weights_x)
    }
}

#[derive(Debug, Clone, PartialEq, PartialOrd)]
pub struct BasicHarmonicOscillatorWithPhaseShift {
    base_osc: Box<Simple>,
    weights: Vec<(f32, f32)>,
    sampling_rate: f32,
}

impl BasicHarmonicOscillatorWithPhaseShift {
    pub fn new(base_osc: Box<Simple>, weights: Vec<(f32, f32)>, sampling_rate: f32) -> Self {
        Self {
            base_osc,
            weights,
            sampling_rate,
        }
    }
}

impl Oscillator for BasicHarmonicOscillatorWithPhaseShift {
    fn generate_sample(&self, ctx: OscillatorCtx, time: f32) -> f32 {
        let mut sample_x = 0.0;
        let mut weights_x = 0.0;
        for (i, (weight_base, phase)) in self.weights.iter().enumerate() {
            let harmonic_freq = (i + 1) as f32 * ctx.freq;
            if harmonic_freq > self.sampling_rate as f32 / 2.0 {
                break;
            }

            let base_ctx = OscillatorCtx {
                freq: harmonic_freq,
                amplitude: ctx.amplitude,
                phase: *phase,
                sampling_rate: ctx.sampling_rate,
            };
            let base_sample = self.base_osc.generate_sample(base_ctx, time);

            sample_x += weight_base * base_sample;
            weights_x += weight_base;
        }

        sample_x / (2.0 * weights_x)
    }
}
#[derive(Debug, Clone, PartialEq, PartialOrd)]
pub struct HarmonicOscillatorWithPhaseShift {
    base_osc: Box<Simple>, // Base oscillator (e.g., sine, phasor, etc.)
    weights: Vec<(f32, f32, f32, f32)>, // weight for base oscillator,
    // phase_offset for base_osc,
    // weight for 90-degree offsetted base,
    // phase_offset for weight for 90-degree offset)
    sampling_rate: f32,
}

impl HarmonicOscillatorWithPhaseShift {
    pub fn new(
        base_osc: Box<Simple>,
        weights: Vec<(f32, f32, f32, f32)>,
        sampling_rate: f32,
    ) -> Self {
        Self {
            base_osc,
            weights,
            sampling_rate,
        }
    }
}

impl Oscillator for HarmonicOscillatorWithPhaseShift {
    fn generate_sample(&self, ctx: OscillatorCtx, time: f32) -> f32 {
        let mut sample_x = 0.0;
        let mut sample_y = 0.0;
        let mut weights_x = 0.0;
        let mut weights_y = 0.0;
        for (i, &(weight_base, phase_base, weight_offset, phase_offset)) in
            self.weights.iter().enumerate()
        {
            let harmonic_freq = (i + 1) as f32 * ctx.freq;
            if harmonic_freq > self.sampling_rate as f32 / 2.0 {
                break;
            }

            // Base oscillator at harmonic frequency with phase offset
            let base_ctx = OscillatorCtx {
                freq: harmonic_freq,
                amplitude: ctx.amplitude,
                phase: phase_base,
                sampling_rate: ctx.sampling_rate,
            };
            let base_sample = self.base_osc.generate_sample(base_ctx, time);

            // 90-degree offset oscillator with phase offset
            let offset_ctx = OscillatorCtx {
                freq: harmonic_freq,
                amplitude: ctx.amplitude,
                phase: phase_offset,
                sampling_rate: ctx.sampling_rate,
            };
            let offset_time = time + 0.25 / harmonic_freq; // 90-degree phase shift
            let offset_sample = self.base_osc.generate_sample(offset_ctx, offset_time);

            sample_x += weight_base * base_sample;
            sample_y += weight_offset * offset_sample;
            weights_x += weight_base;
            weights_y += weight_offset;
        }

        sample_x / (2.0 * weights_x) + sample_y / (2.0 * weights_y)
    }
}

#[derive(Debug, Clone, PartialEq, PartialOrd)]
pub struct GenericHarmonicOscillator {
    weights: Vec<(Simple, f32, f32, Simple, f32, f32)>, // weight for base oscillator,
    // phase_offset for base_osc,
    // weight for 90-degree offsetted
    // phase_offset for weight for 90-degree offset)
    sampling_rate: f32,
}

impl GenericHarmonicOscillator {
    pub fn new(weights: Vec<(Simple, f32, f32, Simple, f32, f32)>, sampling_rate: f32) -> Self {
        Self {
            weights,
            sampling_rate,
        }
    }
}

impl Oscillator for GenericHarmonicOscillator {
    fn generate_sample(&self, ctx: OscillatorCtx, time: f32) -> f32 {
        let mut sample_x = 0.0;
        let mut sample_y = 0.0;
        let mut weights_x = 0.0;
        let mut weights_y = 0.0;
        for (
            i,
            &(ref base_osc, weight_base, phase_base, ref offset_osc, weight_offset, phase_offset),
        ) in self.weights.iter().enumerate()
        {
            let harmonic_freq = (i + 1) as f32 * ctx.freq;
            if harmonic_freq > self.sampling_rate as f32 / 2.0 {
                break;
            }

            // Base oscillator at harmonic frequency with phase offset
            let base_ctx = OscillatorCtx {
                freq: harmonic_freq,
                amplitude: ctx.amplitude,
                phase: phase_base,
                sampling_rate: ctx.sampling_rate,
            };
            let base_sample = base_osc.generate_sample(base_ctx, time);

            // Offset oscillator with phase offset
            let offset_ctx = OscillatorCtx {
                freq: harmonic_freq,
                amplitude: ctx.amplitude,
                phase: phase_offset,
                sampling_rate: ctx.sampling_rate,
            };
            let offset_sample = offset_osc.generate_sample(offset_ctx, time);

            sample_x += weight_base * base_sample;
            sample_y += weight_offset * offset_sample;
            weights_x += weight_base;
            weights_y += weight_offset;
        }

        sample_x / (2.0 * weights_x) + sample_y / (2.0 * weights_y)
    }
}

impl OscillatorControls for GenericHarmonicOscillator {
    fn draw_controls(&mut self, ui: &mut egui::Ui) -> Option<super::OscillatorModification> {
        let mut modification = None;
        let mut osc1_modification = None;
        let mut osc2_modification = None;
        let mut add_weight = None;
        let len = self.weights.len();
        ui.horizontal(|ui| {
            ui.vertical(|ui| {
                ui.group(|ui| {
                    for (
                        index,
                        (osc1, weight_base, phase_base, osc2, weight_offset, phase_offset),
                    ) in self.weights.iter_mut().enumerate()
                    {
                        ui.push_id(index, |ui| {
                            ui.horizontal(|ui| {
                                if ui.button("Remove").clicked(){
                                    modification = Some(
                                        OscillatorModification::Harmonic(
                                            HarmonicModification::Remove(index)
                                        ));
                                }
                                ui.vertical(|ui| {
                                    ui.push_id(-1, |ui| {  osc1_modification = osc1.draw_controls(ui).map(|x| {
                                        OscillatorModification::Harmonic(
                                            HarmonicModification::OscillatorPhaseAndWeightModification(
                                                index,
                                                Box::new(x),
                                                *weight_base,
                                                *phase_base,
                                                Box::new(OscillatorModification::None),
                                                *weight_offset,
                                                *phase_offset,
                                            ),
                                        )
                                    });
                                    });
                                    if ui
                                        .add(Slider::new(weight_base, 0.0..=1.0).text("Weight 1"))
                                        .changed()
                                    {
                                        modification = Some(
                                            OscillatorModification::Harmonic(
                                                HarmonicModification::OscillatorPhaseAndWeightModification(
                                                    index,
                                                    Box::new(OscillatorModification::None),
                                                    *weight_base,
                                                    *phase_base,
                                                    Box::new(OscillatorModification::None),
                                                    *weight_offset,
                                                    *phase_offset,
                                                ),
                                            )
                                        );
                                    }
                                    if ui
                                        .add(Slider::new(phase_base, 0.0..=1.0).text("Phase 1"))
                                        .changed()
                                    {
                                        modification = Some(
                                            OscillatorModification::Harmonic(
                                                HarmonicModification::OscillatorPhaseAndWeightModification(
                                                    index,
                                                    Box::new(OscillatorModification::None),
                                                    *weight_base,
                                                    *phase_base,
                                                    Box::new(OscillatorModification::None),
                                                    *weight_offset,
                                                    *phase_offset,
                                                ),
                                            )
                                        );
                                    }
                                });
                                ui.vertical(|ui| {
                                    ui.push_id(-2, |ui| { osc2_modification = osc2.draw_controls(ui).map(|x| {
                                        OscillatorModification::Harmonic(
                                            HarmonicModification::OscillatorPhaseAndWeightModification(
                                                index,
                                                Box::new(OscillatorModification::None),
                                                *weight_base,
                                                *phase_base,
                                                Box::new(x),
                                                *weight_offset,
                                                *phase_offset,
                                            ),
                                        )
                                    });
                                    });
                                    if ui
                                        .add(Slider::new(weight_offset, 0.0..=1.0).text("Weight 2"))
                                        .changed()
                                    {
                                        modification = Some(
                                            OscillatorModification::Harmonic(
                                                HarmonicModification::OscillatorPhaseAndWeightModification(
                                                    index,
                                                    Box::new(OscillatorModification::None),
                                                    *weight_base,
                                                    *phase_base,
                                                    Box::new(OscillatorModification::None),
                                                    *weight_offset,
                                                    *phase_offset,
                                                ),
                                            )
                                        );
                                    }
                                    if ui
                                        .add(Slider::new(phase_offset, 0.0..=1.0).text("Phase 2"))
                                        .changed()
                                    {
                                        modification = Some(
                                            OscillatorModification::Harmonic(
                                                HarmonicModification::OscillatorPhaseAndWeightModification(
                                                    index,
                                                    Box::new(OscillatorModification::None),
                                                    *weight_base,
                                                    *phase_base,
                                                    Box::new(OscillatorModification::None),
                                                    *weight_offset,
                                                    *phase_offset,
                                                ),
                                            )
                                        );
                                    }
                                });
                            });
                            if index != len - 1 {
                                ui.separator();
                            }
                        });
                    }
                });
            });
            if ui.button("Add").clicked() {
                add_weight = Some(
                    OscillatorModification::Harmonic(
                        HarmonicModification::AddWeight
                    )
                );
            }
        });

        modification
            .or(osc1_modification.or(osc2_modification))
            .or(add_weight)
    }
}

#[derive(Debug, Clone,PartialEq, PartialOrd)]
pub enum HarmonicModification {
    BaseOsc(Box<OscillatorModification>),
    WeightModification(usize, f32),
    PhasedWeightModification(usize, f32, f32),
    PhaseAndWeightModification(usize, f32, f32, f32, f32),
    OscillatorPhaseAndWeightModification(
        usize,
        Box<OscillatorModification>,
        f32,
        f32,
        Box<OscillatorModification>,
        f32,
        f32,
    ),
    AddWeight,
    Remove(usize),
}

impl OscillatorControls for BasicHarmonicOscillator {
    fn draw_controls(&mut self, ui: &mut egui::Ui) -> Option<super::OscillatorModification> {
        let mut modification = None;
        let mut inner_modification = None;
        let mut add_weight = None;
        let len = self.weights.len();
        ui.horizontal(|ui| {
            inner_modification = self
                .base_osc
                .draw_controls(ui)
                .map(Box::new)
                .map(HarmonicModification::BaseOsc)
                .map(OscillatorModification::Harmonic);
            ui.vertical(|ui| {
                
                ui.group(|ui| {
                    for (index, value) in self.weights.iter_mut().enumerate() {
                        ui.push_id(index, |ui| {
                            if ui.button("Remove").clicked(){
                                modification = Some(
                                    OscillatorModification::Harmonic(
                                        HarmonicModification::Remove(index)
                                    ));
                            }
                            if ui
                                .add(Slider::new(value, 0.0..=1.0).text("Weight"))
                                .changed()
                            {
                                modification = Some(OscillatorModification::Harmonic(
                                    HarmonicModification::WeightModification(index, *value),
                                ));
                            }
                            if index != len - 1 {
                                ui.separator();
                            }
                        });
                    }
                });
            });
            if ui.button("Add").clicked() {
                add_weight = Some(OscillatorModification::Harmonic(
                    HarmonicModification::AddWeight,
                ))
            }
        });

        inner_modification.or(modification)
    }
}
impl OscillatorControls for BasicHarmonicOscillatorWithPhaseShift {
    fn draw_controls(&mut self, ui: &mut egui::Ui) -> Option<super::OscillatorModification> {
        let mut modification = None;
        let mut inner_modification = None;
        let mut add_weight = None;
        let len = self.weights.len();
        ui.horizontal(|ui| {
            inner_modification = self
                .base_osc
                .draw_controls(ui)
                .map(Box::new)
                .map(HarmonicModification::BaseOsc)
                .map(OscillatorModification::Harmonic);
            ui.vertical(|ui| {
                ui.group(|ui| {
                    for (index, (weight_base, phase_base)) in self.weights.iter_mut().enumerate() {
                        ui.push_id(index, |ui| {
                            ui.horizontal(|ui| {
                                if ui.button("Remove").clicked(){
                                    modification = Some(
                                        OscillatorModification::Harmonic(
                                            HarmonicModification::Remove(index)
                                        ));
                                }
                                ui.vertical(|ui| {
                                    if ui
                                        .add(Slider::new(weight_base, 0.0..=1.0).text("Weight"))
                                        .changed()
                                    {
                                        modification = Some(OscillatorModification::Harmonic(
                                            HarmonicModification::PhasedWeightModification(
                                                index,
                                                *weight_base,
                                                *phase_base,
                                            ),
                                        ));
                                    }
                                    if ui
                                        .add(Slider::new(phase_base, 0.0..=1.0).text("Phase"))
                                        .changed()
                                    {
                                        modification = Some(OscillatorModification::Harmonic(
                                            HarmonicModification::PhasedWeightModification(
                                                index,
                                                *weight_base,
                                                *phase_base,
                                            ),
                                        ));
                                    }
                                });
                            });
                            if index != len - 1 {
                                ui.separator();
                            }
                        });
                    }
                });
            });
            if ui.button("Add").clicked() {
                add_weight = Some(OscillatorModification::Harmonic(
                    HarmonicModification::AddWeight,
                ))
            }
        });

        inner_modification.or(modification).or(add_weight)
    }
}
impl OscillatorControls for HarmonicOscillatorWithPhaseShift {
    fn draw_controls(&mut self, ui: &mut egui::Ui) -> Option<super::OscillatorModification> {
        let mut modification = None;
        let mut inner_modification = None;
        let mut add_weight = None;
        let len = self.weights.len();
        ui.horizontal(|ui| {
            inner_modification = self
                .base_osc
                .draw_controls(ui)
                .map(Box::new)
                .map(HarmonicModification::BaseOsc)
                .map(OscillatorModification::Harmonic);
            ui.vertical(|ui| {
                ui.group(|ui| {
                    for (index, (weight_base, phase_base, weight_offset, phase_offset)) in
                        self.weights.iter_mut().enumerate()
                    {
                        ui.push_id(index, |ui| {
                            ui.horizontal(|ui| {
                                if ui.button("Remove").clicked(){
                                    modification = Some(
                                        OscillatorModification::Harmonic(
                                            HarmonicModification::Remove(index)
                                        ));
                                }
                                ui.vertical(|ui| {
                                    if ui
                                        .add(Slider::new(weight_base, 0.0..=1.0).text("Weight 1"))
                                        .changed()
                                    {
                                        modification = Some(OscillatorModification::Harmonic(
                                            HarmonicModification::PhaseAndWeightModification(
                                                index,
                                                *weight_base,
                                                *phase_base,
                                                *weight_offset,
                                                *phase_offset,
                                            ),
                                        ));
                                    }
                                    if ui
                                        .add(Slider::new(phase_base, 0.0..=1.0).text("Phase 1"))
                                        .changed()
                                    {
                                        modification = Some(OscillatorModification::Harmonic(
                                            HarmonicModification::PhaseAndWeightModification(
                                                index,
                                                *weight_base,
                                                *phase_base,
                                                *weight_offset,
                                                *phase_offset,
                                            ),
                                        ));
                                    }
                                });
                                ui.vertical(|ui| {
                                    if ui
                                        .add(Slider::new(weight_offset, 0.0..=1.0).text("Weight 2"))
                                        .changed()
                                    {
                                        modification = Some(OscillatorModification::Harmonic(
                                            HarmonicModification::PhaseAndWeightModification(
                                                index,
                                                *weight_base,
                                                *phase_base,
                                                *weight_offset,
                                                *phase_offset,
                                            ),
                                        ));
                                    }
                                    if ui
                                        .add(Slider::new(phase_offset, 0.0..=1.0).text("Phase 2"))
                                        .changed()
                                    {
                                        modification = Some(OscillatorModification::Harmonic(
                                            HarmonicModification::PhaseAndWeightModification(
                                                index,
                                                *weight_base,
                                                *phase_base,
                                                *weight_offset,
                                                *phase_offset,
                                            ),
                                        ));
                                    }
                                });
                            });
                            if index != len - 1 {
                                ui.separator();
                            }
                        });
                    }
                });
            });
            if ui.button("Add").clicked() {
                add_weight = Some(OscillatorModification::Harmonic(
                    HarmonicModification::AddWeight,
                ))
            }
        });

        inner_modification.or(modification).or(add_weight)
    }
}
