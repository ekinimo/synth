use egui::{ScrollArea, Slider, Ui};

use crate::synth::{Oscillator, OscillatorControls, OscillatorCtx};
use std::{f32, ops::Mul};

use super::{
    trig::{self, Sine},
    OscillatorModification, Simple,
};

#[derive(Debug, Clone, Copy, PartialEq, PartialOrd)]
pub enum Transform {
    Identity,
    Power(f32),
    Exp,
    Log,
}

impl Transform {
    fn apply(&self, value: f32) -> f32 {
        match self {
            Transform::Identity => value,
            Transform::Power(p) => value.powf(*p),
            Transform::Exp => value.exp(),
            Transform::Log => value.ln(),
        }
    }

    fn inverse_apply(&self, value: f32) -> f32 {
        match self {
            Transform::Identity => value,
            Transform::Power(p) => value.abs().powf(1.0 / p),
            Transform::Exp => value.ln(),
            Transform::Log => value.exp(),
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub enum Combinator {
    Sum,
    Product,
    Maximum,
    Minimum,
    Mean,
}

impl Combinator {
    fn combine<I>(self, iter: I, transform: Transform) -> f32
    where
        I: Iterator<Item = (f32, f32, f32)>,
    {
        match self {
            Combinator::Sum => iter.map(|(w, v, p)| v.powf(p).mul(w)).sum(),
            Combinator::Product => iter.map(|(w, v, p)| v.powf(p).mul(w)).product(),
            Combinator::Maximum => iter
                .map(|(w, v, p)| v.powf(p).mul(w))
                .fold(f32::NEG_INFINITY, f32::max),
            Combinator::Minimum => iter
                .map(|(w, v, p)| v.powf(p).mul(w))
                .fold(f32::INFINITY, f32::min),
            Combinator::Mean => {
                let mut sum = 0.0;
                let mut count = 0.0;
                for (w, value, p) in iter {
                    sum += w * value.powf(p);
                    count += w;
                }
                if count > 0.0 {
                    transform.inverse_apply(sum / count)
                } else {
                    transform.inverse_apply(0.0)
                }
            }
        }
    }
}

#[derive(Debug, Clone, PartialEq, PartialOrd)]
pub struct CombinedOscillator {
    oscillators: Vec<(f32, Simple, f32)>,
    transform: Transform,
    combinator: Combinator,
}

impl Default for CombinedOscillator {
    fn default() -> Self {
        Self {
            oscillators: vec![(1.0, Simple::Trig(trig::Trig::Sine(Sine)), 1.0)],
            transform: Transform::Identity,
            combinator: Combinator::Mean,
        }
    }
}

impl CombinedOscillator {
    pub fn new(
        oscillators: Vec<(f32, Simple, f32)>,
        transform: Transform,
        combinator: Combinator,
    ) -> Self {
        Self {
            oscillators,
            transform,
            combinator,
        }
    }

    pub(crate) fn update_combinator(&mut self, modif: Combinator) {
        self.combinator = modif;
    }

    pub(crate) fn update_transform(&mut self, modif: Transform) {
        self.transform = modif;
    }

    pub(crate) fn add_oscillator(&mut self, modif: CombinedOscillatorModification) {
        let CombinedOscillatorModification {
            weight,
            power,
            oscillator,
        } = modif;
        self.oscillators.push((weight, oscillator, power));
    }

    pub(crate) fn remove_oscillator(&mut self, modif: usize) {
        self.oscillators.remove(modif);
    }

    pub(crate) fn modify_oscillator(&mut self, index: usize, modif: Box<OscillatorModification>) {
        self.oscillators[index].1.update(*modif);
    }
    pub(crate) fn modify_oscillator_weight(&mut self, index: usize, weight: f32) {
        self.oscillators[index].0 = weight;
    }
    pub(crate) fn modify_oscillator_power(&mut self, index: usize, power: f32) {
        self.oscillators[index].2 = power;
    }
}

impl Oscillator for CombinedOscillator {
    fn generate_sample(&self, ctx: OscillatorCtx, time: f32) -> f32 {
        self.combinator.combine(
            self.oscillators.iter().map(|(weight, osc, pow)| {
                (
                    *weight,
                    self.transform.apply(osc.generate_sample(ctx.clone(), time)),
                    *pow,
                )
            }),
            self.transform,
        )
    }
}
/*

pub trait OscillatorExt: Oscillator + Sized {
    fn weighted(self, weight: f32) -> (f32, Box<dyn Oscillator>, Transform) {
        (weight, Box::new(self), Transform::Identity)
    }

    fn transformed(self, transform: Transform) -> (f32, Box<dyn Oscillator>, Transform) {
        (1.0, Box::new(self), transform)
    }

    fn weighted_transformed(
        self,
        weight: f32,
        transform: Transform,
    ) -> (f32, Box<dyn Oscillator>, Transform) {
        (weight, Box::new(self), transform)
    }
}

impl<T: Oscillator + Sized> OscillatorExt for T {}

// Compatibility layer for the old types
pub enum SumOscillator {
    Sum(CombinedOscillator),
    WeightedSum(CombinedOscillator),
    PowerSum(CombinedOscillator),
    GeneralPowerSum(CombinedOscillator),
    WeightedGeneralPowerSum(CombinedOscillator),
    WeightedPowerSum(CombinedOscillator),
    ExpSum(CombinedOscillator),
    LogSum(CombinedOscillator),
}

impl Oscillator for SumOscillator {
    fn generate_sample(&self, ctx: OscillatorCtx, time: f32) -> f32 {
        use SumOscillator::*;
        match self {
            Sum(osc) => osc.generate_sample(ctx, time),
            WeightedSum(osc) => osc.generate_sample(ctx, time),
            PowerSum(osc) => osc.generate_sample(ctx, time),
            GeneralPowerSum(osc) => osc.generate_sample(ctx, time),
            WeightedGeneralPowerSum(osc) => osc.generate_sample(ctx, time),
            WeightedPowerSum(osc) => osc.generate_sample(ctx, time),
            ExpSum(osc) => osc.generate_sample(ctx, time),
            LogSum(osc) => osc.generate_sample(ctx, time),
        }
    }
}
*/

#[derive(Debug, Clone,PartialEq, PartialOrd)]
pub struct CombinedOscillatorModification {
    pub weight: f32,
    pub power: f32,
    pub oscillator: Simple,
}

impl OscillatorControls for CombinedOscillator {
    fn draw_controls(&mut self, ui: &mut Ui) -> Option<OscillatorModification> {
        let mut modification = None;

        ui.vertical(|ui| {
            // Combinator selection
            ui.group(|ui| {
                ui.horizontal(|ui| {
                    ui.label("Combinator:");
                    let combinators = [
                        Combinator::Sum,
                        Combinator::Product,
                        Combinator::Maximum,
                        Combinator::Minimum,
                        Combinator::Mean,
                    ];

                    for combo in combinators.iter() {
                        if ui
                            .radio_value(&mut self.combinator, *combo, format!("{:?}", combo))
                            .changed()
                        {
                            modification = Some(OscillatorModification::BulkCombinator(*combo));
                        }
                    }
                });

                // Transform selection
                ui.horizontal(|ui| {
                    ui.label("Transform:");
                    if ui
                        .radio_value(&mut self.transform, Transform::Identity, "Identity")
                        .changed()
                    {
                        modification =
                            Some(OscillatorModification::BulkTransform(Transform::Identity));
                    }
                    if ui
                        .radio_value(&mut self.transform, Transform::Exp, "Exp")
                        .changed()
                    {
                        modification = Some(OscillatorModification::BulkTransform(Transform::Exp));
                    }
                    if ui
                        .radio_value(&mut self.transform, Transform::Log, "Log")
                        .changed()
                    {
                        modification = Some(OscillatorModification::BulkTransform(Transform::Log));
                    }

                    if let Transform::Power(ref mut power) = self.transform {
                        if ui
                            .add(Slider::new(power, 0.1..=10.0).text("Power"))
                            .changed()
                        {
                            modification = Some(OscillatorModification::BulkTransform(
                                Transform::Power(*power),
                            ));
                        }
                    }
                });

                // Oscillator list and controls
                ui.separator();
                ui.label("Oscillators:");
                let len = self.oscillators.len();
                ui.horizontal(|ui| {
                    ui.group(|ui| {
                        for (index, (weight, osc, power)) in self.oscillators.iter_mut().enumerate()
                        {
                            // Create a unique ID for this oscillator's group
                            ui.push_id(index, |ui| {
                                //ui.horizontal(|ui| {
                                    ui.vertical(|ui| {
                                        ui.label(format!("Osc {}:", index));
                                        // Weight control
                                        if ui
                                            .add(Slider::new(weight, 0.0..=2.0).text("Weight"))
                                            .changed()
                                        {
                                            modification = Some(
                                            OscillatorModification::BulkModifyOscillatorWeight {
                                                index,
                                                weight: *weight,
                                            },
                                        );
                                        }

                                        // Create a child UI with its own ID scope for the oscillator controls
                                        if let Some(mod_result) = osc.draw_controls(ui) {
                                            modification = Some(
                                                OscillatorModification::BulkModifyOscillator {
                                                    index,
                                                    modifier: Box::new(mod_result),
                                                },
                                            );
                                        }
                                  
                                        // Power control
                                        if ui
                                            .add(Slider::new(power, 0.1..=10.0).text("Power"))
                                            .changed()
                                        {
                                            modification = Some(
                                                OscillatorModification::BulkModifyOscillatorPower {
                                                    index,
                                                    power: *power,
                                                },
                                            );
                                        }

                                        if ui.button("Remove").clicked() {
                                            modification = Some(
                                                OscillatorModification::BulkRemoveOscillator(index),
                                            );
                                        }
                                    });
                                //});
                                if index  != len - 1 {
                                    ui.separator();
                                }
                            });
                        }
                    });
                });

                // Add new oscillator button
                if ui.button("Add Oscillator").clicked() {
                    modification = Some(OscillatorModification::BulkAddOscillator(
                        CombinedOscillatorModification {
                            weight: 1.0,
                            power: 1.0,
                            oscillator: Simple::Trig(super::trig::Trig::Sine(Sine)),
                        },
                    ));
                }
            });
        });

        modification
    }
    /*    fn draw_controls(&mut self, ui: &mut Ui) -> Option<OscillatorModification> {
        let mut modification = None;

        ui.vertical(|ui| {
            // Combinator selection
            ui.horizontal(|ui| {
                ui.label("Combinator:");
                let combinators = [
                    Combinator::Sum,
                    Combinator::Product,
                    Combinator::Maximum,
                    Combinator::Minimum,
                    Combinator::Mean,
                ];

                for combo in combinators.iter() {
                    if ui
                        .radio_value(&mut self.combinator, *combo, format!("{:?}", combo))
                        .changed()
                    {
                        modification = Some(OscillatorModification::BulkCombinator(*combo));
                    }
                }
            });

            // Transform selection
            ui.horizontal(|ui| {
                ui.label("Transform:");
                if ui
                    .radio_value(&mut self.transform, Transform::Identity, "Identity")
                    .changed()
                {
                    modification = Some(OscillatorModification::BulkTransform(Transform::Identity));
                }
                if ui
                    .radio_value(&mut self.transform, Transform::Exp, "Exp")
                    .changed()
                {
                    modification = Some(OscillatorModification::BulkTransform(Transform::Exp));
                }
                if ui
                    .radio_value(&mut self.transform, Transform::Log, "Log")
                    .changed()
                {
                    modification = Some(OscillatorModification::BulkTransform(Transform::Log));
                }

                // Power transform with slider
                if let Transform::Power(ref mut power) = self.transform {
                    if ui
                        .add(Slider::new(power, 0.1..=10.0).text("Power"))
                        .changed()
                    {
                        modification = Some(OscillatorModification::BulkTransform(
                            Transform::Power(*power),
                        ));
                    }
                }
            });

            // Oscillator list and controls
            ui.separator();
            ui.label("Oscillators:");

            ui.group(|ui| {
            for (index, (weight, osc, power)) in self.oscillators.iter_mut().enumerate() {
                ui.horizontal(|ui| {
                    let mut changed = false;
                    ui.label(format!("Osc {}:", index));
                    changed |= ui
                        .add(Slider::new(weight, 0.0..=2.0).text("Weight"))
                        .changed();
                    if changed {
                        modification = Some(OscillatorModification::BulkModifyOscillatorWeight {
                            index,
                            weight: *weight,
                        });
                    }

                    ui.vertical(|ui| {
                        modification = osc.draw_controls(ui).map(|modifier| {
                            OscillatorModification::BulkModifyOscillator {
                                index,
                                modifier: Box::new(modifier),
                            }
                        });
                    });
                    let mut changed = false;
                    changed |= ui
                        .add(Slider::new(power, 0.1..=10.0).text("Power"))
                        .changed();
                    if changed {
                        modification = Some(OscillatorModification::BulkModifyOscillatorPower {
                            index,
                            power: *power,
                        });
                    }

                    if ui.button("Remove").clicked() {
                        modification = Some(OscillatorModification::BulkRemoveOscillator(index));
                    }
                });
            }
            });

            // Add new oscillator button
            if ui.button("Add Oscillator").clicked() {
                modification = Some(OscillatorModification::BulkAddOscillator(
                    CombinedOscillatorModification {
                        weight: 1.0,
                        power: 1.0,
                        oscillator: Simple::Trig(super::trig::Trig::Sine(Sine)),
                    },
                ));
            }
        });

        modification
    }*/
}
