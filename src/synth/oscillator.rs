use addmlpow::{AddMulPow, AddMulPowModification};
use bulk::{Combinator, CombinedOscillator, CombinedOscillatorModification, Transform};
use circle::Circle;
use compose::{ComposeModifier, ComposedOscillator};
use conditional::ConditionalOscillator;
use consts::ContextualOscillator;
use crossbeam_channel::Sender;
use egui::{CollapsingHeader, ComboBox, ScrollArea, Ui};
use egui_plot::{Line, Plot, PlotPoints};
use exp::Exponential;
use fft::FftSynth;
use gaussian::Gaussian;
use harmonic::{HarmonicModification, HarmonicOscillators};
use log::Logarithmic;
use max::MaximumOscillator;
use mean::MeanOscillator;

use median::Median;
use min::MinimumOscillator;
use mode::Mode;
use morph::{Morph, MorphModification};
use phasor::Phasor;
use polynomial::Polynomial;
use pow::Pow;
use prod::ProductOscillator;
use rational::Rational;
use round::RoundingOscillator;
use square::Square;
use sum::SumOscillator;
use terraced::Terraced;
use triangle::{ModifiableTriangle, Triangle};
use trig::Trig;
use wavebank::Wavetables;

use super::{Oscillator, OscillatorControls, OscillatorCtx, OscillatorModifiers, SynthMessage};

pub mod addmlpow;
pub mod bulk;
pub mod circle;
pub mod compose;
pub mod conditional;
pub mod consts;
pub mod exp;
pub mod fft;
pub mod gaussian;
pub mod harmonic;
pub mod log;
pub mod median;
pub mod mode;
pub mod morph;
pub mod phasor;
pub mod polynomial;
pub mod pow;
pub mod prod;
pub mod rational;
pub mod round;
pub mod square;
pub mod terraced;
pub mod triangle;
pub mod trig;
pub mod wavebank;

//unneeded
pub mod max;
pub mod mean;
pub mod min;
pub mod sum;

#[derive(Debug, Clone,PartialEq, PartialOrd)]
pub enum OscillatorModification {
    AddModifier(OscillatorModifiers),
    None,
    Replace(Simple),
    SquareDutyCycle(f32),
    TriangleDutyCycle(f32),
    AddMulPowMod(AddMulPowModification),
    CircleMod(Box<OscillatorModification>),
    Morph(MorphModification),
    Harmonic(HarmonicModification),
    Compose(ComposeModifier)     ,
    //Conditional(modif)) ,
    //Const(modif))       ,
    //Terraced(modif))    ,
    //Op(modif))          ,

    BulkCombinator(Combinator),
    BulkTransform(Transform),
    BulkAddOscillator(CombinedOscillatorModification),
    BulkRemoveOscillator(usize), // index to remove
    BulkModifyOscillator {
        index: usize,
        modifier: Box<OscillatorModification>,
    },
    BulkModifyOscillatorWeight {
        index: usize,
        weight: f32,
    },
    BulkModifyOscillatorPower {
        index: usize,
        power: f32,
    },
}

impl Default for Simple{
    fn default() -> Self {
        Self::Trig(Trig::default())
    }
}

#[derive(Debug, Clone, PartialEq, PartialOrd)]
pub enum Simple {
    Trig(Trig),
    Square(Square),
    Triangle(Triangle),
    Addmlpow(AddMulPow),
    Phasor(Phasor),
    Circle(Circle),
    Bulk(CombinedOscillator),
    Morph(Morph),
    Harmonic(HarmonicOscillators) ,
    Compose(ComposedOscillator)  ,
    //Conditional(cond)  ,
    //Op(cond)           ,
    //Const(cond)        ,
    //Terraced(cond)     ,

}

pub enum BasicOp {
    Pow(Pow),
    Gaussian(Gaussian),
    Exp(Exponential),
    Log(Logarithmic),
    Polynomial(Polynomial),
    Rational(Rational),
    Round(RoundingOscillator),
}

pub enum BulkOp {
    Sum(SumOscillator), //done
    Prod(ProductOscillator), //done
    Max(MaximumOscillator), //done
    Min(MinimumOscillator), //done
    Mean(MeanOscillator), //done
    Median(Median),
    Mode(Mode),
    Fft(FftSynth),
}
pub enum Op {
    Basic(BasicOp),
    Bulk(BulkOp), //done   
    Harmonic(HarmonicOscillators),
    Morph(Morph), //done
    Conditional(ConditionalOscillator),
    Compose(ComposedOscillator),
}

enum AllOscillators {
    Terraced(Terraced),
    Consts(ContextualOscillator),
    Wavebank(Wavetables),
    Op(Op),
    Simple(Simple),
}

#[macro_export]
macro_rules! select_variant {
    ($ui:ident, $self:ident, $variant:expr, $name:literal) => {{
        let temp = $variant;
        let is_selected = std::mem::discriminant(&temp) == std::mem::discriminant($self);
        let mut changed = false;

        if $ui.selectable_label(is_selected, $name).clicked() {
            *$self = temp;
            changed = true;
        }

        changed
    }};
}

impl Simple {
    
    pub(crate) fn update(&mut self, part: OscillatorModification) {
        use OscillatorModification::*;

        dbg!(&part);
        match (self, part) {
            (x, Replace(y)) => {
                *x = y;
            }
            (Self::Square(Square { duty_cycle }), SquareDutyCycle(new)) => *duty_cycle = new,
            (
                Self::Triangle(Triangle::Triangle(ModifiableTriangle { duty_cycle })),
                TriangleDutyCycle(new),
            ) => *duty_cycle = new,
            (Self::Addmlpow(part), AddMulPowMod(modifier)) => part.update(dbg!(modifier)),
            (Self::Circle(circ), CircleMod(modif)) => circ.update(modif),
            (Self::Morph(morph), Morph(modif)) => morph.update(modif),
            (Self::Harmonic(harmonic), Harmonic(modif)) => harmonic.update(modif),
            (Self::Compose(harmonic), Compose(modif)) => harmonic.update(modif),
            //(Self::Conditional(cond), Conditional(modif)) => cond.update(modif),
            //(Self::Op(cond), Op(modif)) => cond.update(modif),
            //(Self::Const(cond), Const(modif)) => cond.update(modif),
            //(Self::Terraced(cond), Terraced(modif)) => cond.update(modif),

            (Self::Bulk(bulk), BulkCombinator(modif)) => bulk.update_combinator(modif),
            (Self::Bulk(bulk), BulkTransform(modif)) => bulk.update_transform(modif),
            (Self::Bulk(bulk), BulkAddOscillator(modif)) => bulk.add_oscillator(modif),
            (Self::Bulk(bulk), BulkRemoveOscillator(modif)) => bulk.remove_oscillator(modif),
            (Self::Bulk(bulk), BulkModifyOscillatorPower { index, power }) => {
                bulk.modify_oscillator_power(index, power)
            }
            (Self::Bulk(bulk), BulkModifyOscillatorWeight { index, weight }) => {
                bulk.modify_oscillator_weight(index, weight)
            }
            (Self::Bulk(bulk), BulkModifyOscillator { index, modifier }) => {
                bulk.modify_oscillator(index, modifier)
            }

            _ => (),
        }
    }

    fn name(&self) -> String {
        match self {
            Simple::Trig(_) => "Trigonometric",
            Simple::Square(_) => "Square",
            Simple::Triangle(_) => "Triangle",
            Simple::Phasor(_) => "Phasor",
            Simple::Circle(_) => "Circle",
            Simple::Addmlpow(_) => "Add/Mul/Pow",
            Simple::Morph(_) => "Interpolation",
            Simple::Bulk(_) => "Bulk Operations (Sum/Product/etc.)",
            Simple::Harmonic(_) => "Harmonic",
            Simple::Compose(_) => "Compose",
            //Simple::Conditional(cond) => "Conditional",
            //Simple::Op(cond) => "Operation",
            //Simple::Const(cond) => "Constants",
            //Simple::Terraced(cond) => "Terraced",

        }
        .to_string()
    }
}

impl OscillatorControls for Simple {
    fn draw_controls(&mut self, ui: &mut Ui) -> Option<OscillatorModification> {
        let name = self.name();
        let mut modification = None;

        CollapsingHeader::new(name)
            .default_open(true)
            .show_unindented(ui, |ui| {
                ui.group(|ui| {
                    ui.horizontal(|ui| {
                        ui.vertical(|ui| {
                            ui.horizontal(|ui| {
                                ComboBox::from_label("Oscillator Type")
                                    .selected_text(self.name())
                                    .show_ui(ui, |ui| {
                                        let simples = [
                                            Simple::Trig(Trig::default()),
                                            Simple::Square(Square::default()),
                                            Simple::Triangle(Triangle::default()),
                                            Simple::Phasor(Phasor::default()),
                                            Simple::Morph(Morph::default()),
                                            Simple::Circle(Circle::default()),
                                            Simple::Addmlpow(AddMulPow::default()),
                                            Simple::Bulk(CombinedOscillator::default()),
                                            Simple::Harmonic(HarmonicOscillators::default()),
                                            Simple::Compose(ComposedOscillator::default()) ,
                                            //Simple::Conditional(cond) ,
                                            //Simple::Op(cond) ,
                                            //Simple::Const(cond) ,
                                            //Simple::Terraced(cond) ,
                                        ];
                                        for simple in simples.iter() {
                                            if ui
                                                .radio_value(self, simple.clone(), simple.name())
                                                .changed()
                                            {
                                                modification = Some(
                                                    OscillatorModification::Replace(self.clone()),
                                                )
                                            }
                                        }
                                    });
                            });

                            let inner_mod = match self {
                                Simple::Circle(c) => c.draw_controls(ui),
                                Simple::Trig(trig) => trig.draw_controls(ui),
                                Simple::Square(square) => square.draw_controls(ui),
                                Simple::Triangle(triangle) => triangle.draw_controls(ui),
                                Simple::Addmlpow(add_mul_pow) => add_mul_pow.draw_controls(ui),
                                Simple::Phasor(phasor) => phasor.draw_controls(ui),
                                Simple::Bulk(bulk) => bulk.draw_controls(ui),
                                Simple::Morph(morph) => morph.draw_controls(ui),
                                Simple::Harmonic(harm) => harm.draw_controls(ui),
                                Simple::Compose(harmonic) => harmonic.draw_controls(ui),
                                //Simple::Conditional(cond) => cond.draw_controls(ui),
                                //Simple::Op(cond) => cond.draw_controls(ui),
                                //Simple::Const(cond) => cond.draw_controls(ui),
                                //Simple::Terraced(cond) => cond.draw_controls(ui),
                            };
                            modification = modification.clone().or(inner_mod);
                        });
                        ui.separator();
                        ui.vertical(|ui| {
                            ui.label("Waveform");
                            let points = PlotPoints::from_iter((0..(44100)).map(|i| {
                                let x = i as f32 / 44100.0;
                                let y = self.generate_sample(
                                    OscillatorCtx {
                                        amplitude: 1.0,
                                        freq: 2.0, // Example frequency
                                        phase: 0.0,
                                        sampling_rate: (44100),
                                    },
                                    x,
                                );
                                [x as f64, y as f64]
                            }));

                            Plot::new("waveform")
                                .view_aspect(0.5)
                                .height(100.0)
                                .width(100.0)
                                .show(ui, |plot_ui| plot_ui.line(Line::new(points)));
                        });
                    });
                });
            });

        // Combine modifications
        modification
    }
}

/*impl OscillatorControls for Simple {
    fn draw_controls(&mut self, ui: &mut Ui, sender: Sender<SynthMessage>) {
        ui.horizontal(|ui| {
            ComboBox::from_label("Oscillator Type")
                .selected_text(match self {
                    Simple::Trig(_) => "Trigonometric",
                    Simple::Square(_) => "Square",
                    Simple::Triangle(_) => "Triangle",
                    Simple::Phasor(_) => "Phasor",
                    Simple::Circle(_) => "Circle",
                    Simple::Addmlpow(_) => "Add/Mul/Pow",
                })
                .show_ui(ui, |ui| {
                    if ui
                        .selectable_value(self, Simple::Trig(Trig::default()), "Trigonometric")
                        .changed()
                    {
                        sender
                            .send(SynthMessage::UpdateOscillator(self.clone()))
                            .unwrap();
                    }
                    if ui
                        .selectable_value(self, Simple::Square(Square::default()), "Square")
                        .changed()
                    {
                        sender
                            .send(SynthMessage::UpdateOscillator(self.clone()))
                            .unwrap();
                    }
                    if ui
                        .selectable_value(self, Simple::Triangle(Triangle::default()), "Triangle")
                        .changed()
                    {
                        sender
                            .send(SynthMessage::UpdateOscillator(self.clone()))
                            .unwrap();
                    }
                    if ui
                        .selectable_value(self, Simple::Phasor(Phasor::default()), "Phasor")
                        .changed()
                    {
                        sender
                            .send(SynthMessage::UpdateOscillator(self.clone()))
                            .unwrap();
                    }
                    if ui
                        .selectable_value(self, Simple::Circle(Circle::default()), "Circle")
                        .changed()
                    {
                        sender
                            .send(SynthMessage::UpdateOscillator(self.clone()))
                            .unwrap();
                    }

                    if ui
                        .selectable_value(
                            self,
                            Simple::Addmlpow(AddMulPow::default()),
                            "Add/Mul/Pow",
                        )
                        .changed()
                    {
                        sender
                            .send(SynthMessage::UpdateOscillator(self.clone()))
                            .unwrap();
                    }
                });
        });

        ui.horizontal(|ui| {
            match self {
                Simple::Trig(_) => {
                    // TODO: Add controls for Trig variant
                }
                Simple::Square(square) => {
                    square.draw_controls(ui, sender.clone());
                }
                Simple::Triangle(_) => {
                    // TODO: Add controls for Triangle variant
                }
                Simple::Phasor(_) => {
                    // TODO: Add controls for Phasor variant
                }
                Simple::Circle(_) => {
                    // TODO: Add controls for Circle variant
                }
                Simple::Addmlpow(_) => {
                    // TODO: Add controls for Addmlpow variant
                }
            }
        });

        let points = PlotPoints::from_iter((0..(44100)).map(|i| {
            let x = i as f32 / 44100.0;
            let y = self.generate_sample(
                OscillatorCtx {
                    amplitude: 1.0,
                    freq: 2.0, // Example frequency
                    phase: 0.0,
                    sampling_rate: (44100),
                },
                x,
            );
            [x as f64, y as f64]
        }));

        Plot::new("waveform")
            .view_aspect(0.5)
            .height(100.0)
            .width(100.0)
            .show(ui, |plot_ui| plot_ui.line(Line::new(points)));
    }
}
*/
impl Simple {
    pub fn update_duty_cycle(&mut self, duty_cycle: f32) {
        match self {
            Simple::Triangle(Triangle::Triangle(triangle)) => (),
            Simple::Square(square) => square.update_duty_cycle(duty_cycle),
            _ => (),
        }
    }
}

impl Oscillator for Simple {
    fn generate_sample(&self, ctx: super::OscillatorCtx, phase: f32) -> f32 {
        match self {
            Self::Trig(trig) => trig.generate_sample(ctx, phase),
            Self::Square(square) => square.generate_sample(ctx, phase),
            Self::Triangle(triangle) => triangle.generate_sample(ctx, phase),
            Self::Phasor(phasor) => phasor.generate_sample(ctx, phase),
            Self::Circle(circle) => circle.generate_sample(ctx, phase),
            Self::Addmlpow(add_mul_pow) => add_mul_pow.generate_sample(ctx, phase),
            Self::Bulk(bulk) => bulk.generate_sample(ctx, phase),
            Simple::Morph(morph) => morph.generate_sample(ctx, phase),
            Simple::Harmonic(harm) => harm.generate_sample(ctx, phase),
            Simple::Compose(harmonic) => harmonic.generate_sample(ctx, phase),
            //Simple::Conditional(cond) => cond.generate_sample(ctx, phase),
            //Simple::Op(cond) => cond.generate_sample(ctx, phase),
            //Simple::Const(cond) => cond.generate_sample(ctx, phase),
            //Simple::Terraced(cond) => cond.generate_sample(ctx, phase),

        }
    }
}

impl Oscillator for BasicOp {
    fn generate_sample(&self, ctx: super::OscillatorCtx, phase: f32) -> f32 {
        match self {
            BasicOp::Pow(pow) => pow.generate_sample(ctx, phase),
            BasicOp::Gaussian(gaussian) => gaussian.generate_sample(ctx, phase),
            BasicOp::Exp(exponential) => exponential.generate_sample(ctx, phase),
            BasicOp::Log(logarithmic) => logarithmic.generate_sample(ctx, phase),
            BasicOp::Polynomial(polynomial) => polynomial.generate_sample(ctx, phase),
            BasicOp::Rational(rational) => rational.generate_sample(ctx, phase),
            BasicOp::Round(rounding_oscillator) => rounding_oscillator.generate_sample(ctx, phase),
        }
    }
}

impl Oscillator for BulkOp {
    fn generate_sample(&self, ctx: super::OscillatorCtx, phase: f32) -> f32 {
        match self {
            BulkOp::Sum(sum_oscillator) => sum_oscillator.generate_sample(ctx, phase),
            BulkOp::Prod(product_oscillator) => product_oscillator.generate_sample(ctx, phase),
            BulkOp::Max(maximum_oscillator) => maximum_oscillator.generate_sample(ctx, phase),
            BulkOp::Min(minimum_oscillator) => minimum_oscillator.generate_sample(ctx, phase),
            BulkOp::Mean(mean_oscillator) => mean_oscillator.generate_sample(ctx, phase),
            BulkOp::Median(median_like) => median_like.generate_sample(ctx, phase),
            BulkOp::Mode(mode) => mode.generate_sample(ctx, phase),
            BulkOp::Fft(fft_synth) => fft_synth.generate_sample(ctx, phase),
        }
    }
}

impl Oscillator for Op {
    fn generate_sample(&self, ctx: super::OscillatorCtx, phase: f32) -> f32 {
        match self {
            Op::Basic(basic) => basic.generate_sample(ctx, phase),
            Op::Bulk(bulk) => bulk.generate_sample(ctx, phase),
            Op::Compose(composed_oscillator) => composed_oscillator.generate_sample(ctx, phase),
            Op::Harmonic(harmonic_oscillators) => harmonic_oscillators.generate_sample(ctx, phase),
            Op::Morph(morph) => morph.generate_sample(ctx, phase),
            Op::Conditional(conditional_oscillator) => {
                conditional_oscillator.generate_sample(ctx, phase)
            }
        }
    }
}

impl Oscillator for AllOscillators {
    fn generate_sample(&self, ctx: super::OscillatorCtx, phase: f32) -> f32 {
        match self {
            AllOscillators::Terraced(terraced) => terraced.generate_sample(ctx, phase),
            AllOscillators::Consts(contextual_oscillator) => {
                contextual_oscillator.generate_sample(ctx, phase)
            }
            AllOscillators::Wavebank(wavetables) => wavetables.generate_sample(ctx, phase),
            AllOscillators::Op(op) => op.generate_sample(ctx, phase),
            AllOscillators::Simple(simple) => simple.generate_sample(ctx, phase),
        }
    }
}
