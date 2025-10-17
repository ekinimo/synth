use egui::{ComboBox, Id, Slider};

use crate::synth::{Oscillator, OscillatorControls, OscillatorCtx};

use super::{OscillatorModification, Simple};

#[derive(Debug, Clone, Copy, PartialEq, PartialOrd)]
pub enum ModType {
    Amplitude,
    Frequency,
    Phase,
}

#[derive(Debug, Clone, Copy, PartialEq, PartialOrd)]
pub enum ParamType {
    Osc,
    Amplitude,
    Frequency,
    Phase,
}

#[derive(Debug, Clone, PartialEq, PartialOrd)]
pub enum ComposeModifier {
    Osc1(Box<OscillatorModification>),
    Osc2(Box<OscillatorModification>),
    Osc3(Box<OscillatorModification>),
    Osc4(Box<OscillatorModification>),
    Params(ParamType, Option<(f32, OscillatorCtx)>),
    EnableMod(ModType, bool),
}

#[derive(Debug, Clone, PartialEq, PartialOrd)]
pub enum ComposedOscillator {
    Amplitude(ComposeAmplitude),
    Freq(ComposeFreq),
    Phase(ComposePhase),
    All(ComposeAll),
}

impl Default for ComposedOscillator {
    fn default() -> Self {
        Self::Amplitude(ComposeAmplitude::default())
    }
}

impl OscillatorControls for ComposedOscillator {
    fn draw_controls(&mut self, ui: &mut egui::Ui) -> Option<OscillatorModification> {
        let mut modification = None;
        ComboBox::from_label("Oscillator Type")
            .selected_text(self.name())
            .show_ui(ui, |ui| {
                let simples = [
                    ComposedOscillator::Amplitude(ComposeAmplitude::default()),
                    ComposedOscillator::Freq(ComposeFreq::default()),
                    ComposedOscillator::Phase(ComposePhase::default()),
                    ComposedOscillator::All(ComposeAll::default()),
                ];
                for simple in simples.iter() {
                    if ui
                        .radio_value(self, simple.clone(), simple.name())
                        .changed()
                    {
                        modification = Some(OscillatorModification::Replace(Simple::Compose(
                            self.clone(),
                        )))
                    }
                }
            });
        let inner_modification = match self {
            ComposedOscillator::Amplitude(compose) => compose.draw_controls(ui),
            ComposedOscillator::Freq(compose) => compose.draw_controls(ui),
            ComposedOscillator::Phase(compose) => compose.draw_controls(ui),
            ComposedOscillator::All(compose) => compose.draw_controls(ui),
        };
        modification.or(inner_modification)
    }
}

impl ComposedOscillator {
    fn name(&self) -> String {
        match self {
            ComposedOscillator::Amplitude(_compose_amplitude) => "Amplitude",
            ComposedOscillator::Freq(_compose_freq) => "Frequency",
            ComposedOscillator::Phase(_compose_phase) => "Phase",
            ComposedOscillator::All(_compose_all) => "Combined",
        }
        .to_string()
    }

    pub fn update(&mut self, modification: ComposeModifier) {
        match (self, modification) {
            (ComposedOscillator::All(compose), ComposeModifier::Osc1(m)) => {
                compose.osc.update(*m);
            }
            (ComposedOscillator::All(compose), ComposeModifier::Osc2(m)) => {
                if let Some(amp_mod) = compose.amplitude_mod.as_mut() {
                    amp_mod.update(*m);
                }
            }
            (ComposedOscillator::All(compose), ComposeModifier::Osc3(m)) => {
                if let Some(freq_mod) = compose.freq_mod.as_mut() {
                    freq_mod.update(*m);
                }
            }
            (ComposedOscillator::All(compose), ComposeModifier::Osc4(m)) => {
                if let Some(phase_mod) = compose.phase_mod.as_mut() {
                    phase_mod.update(*m);
                }
            }
            (ComposedOscillator::All(compose), ComposeModifier::Params(typ, m)) => match typ {
                ParamType::Osc => {
                    compose.osc_params = m;
                }
                ParamType::Amplitude => {
                    compose.amp_params = m;
                }
                ParamType::Frequency => {
                    compose.freq_params = m;
                }
                ParamType::Phase => {
                    compose.phase_params = m;
                }
            },

            (ComposedOscillator::All(osc), ComposeModifier::EnableMod(mod_type, enabled)) => {
                match mod_type {
                    ModType::Amplitude => {
                        if enabled && osc.amplitude_mod.is_none() {
                            osc.amplitude_mod = Some(Box::new(Simple::default()));
                        } else if !enabled {
                            osc.amplitude_mod = None;
                        }
                    }
                    ModType::Frequency => {
                        if enabled && osc.freq_mod.is_none() {
                            osc.freq_mod = Some(Box::new(Simple::default()));
                        } else if !enabled {
                            osc.freq_mod = None;
                        }
                    }
                    ModType::Phase => {
                        if enabled && osc.phase_mod.is_none() {
                            osc.phase_mod = Some(Box::new(Simple::default()));
                        } else if !enabled {
                            osc.phase_mod = None;
                        }
                    }
                }
            }

            _ => {}
        }
    }
}
impl Oscillator for ComposedOscillator {
    fn generate_sample(&self, ctx: OscillatorCtx, time: f32) -> f32 {
        match self {
            ComposedOscillator::Amplitude(osc) => osc.generate_sample(ctx, time),
            ComposedOscillator::Freq(osc) => osc.generate_sample(ctx, time),
            ComposedOscillator::Phase(osc) => osc.generate_sample(ctx, time),
            ComposedOscillator::All(osc) => osc.generate_sample(ctx, time),
        }
    }
}

impl ComposedOscillator {}

#[derive(Default, Debug, Clone, PartialEq, PartialOrd)]
pub struct ComposeAmplitude {
    osc: Box<Simple>,
    amplitude_mod: Box<Simple>,
    osc_params: Option<(f32, OscillatorCtx)>,
    amp_params: Option<(f32, OscillatorCtx)>,
}

impl ComposeAmplitude {
    pub fn new(
        osc: Box<Simple>,
        amplitude_mod: Box<Simple>,
        osc_params: Option<(f32, OscillatorCtx)>,
        amp_params: Option<(f32, OscillatorCtx)>,
    ) -> Self {
        Self {
            osc,
            amplitude_mod,
            osc_params,
            amp_params,
        }
    }
}

impl Oscillator for ComposeAmplitude {
    fn generate_sample(&self, ctx: OscillatorCtx, time: f32) -> f32 {
        let (osc_ctx, osc_time) = match self.osc_params {
            Some((_t, c)) => (c, time),
            None => (ctx, time),
        };
        let (amp_ctx, amp_time) = match self.amp_params {
            Some((_t, c)) => (c, time),
            None => (ctx, time),
        };

        let amp = self.amplitude_mod.generate_sample(amp_ctx, amp_time);
        amp * self.osc.generate_sample(osc_ctx, osc_time)
    }
}

impl OscillatorControls for ComposeAmplitude {
    fn draw_controls(&mut self, ui: &mut egui::Ui) -> Option<OscillatorModification> {
        let mut base_osc_modification = None;
        let mut amp_modification = None;
        let mut osc_param_modification = None;
        let mut amp_param_modification = None;

        ui.vertical(|ui| {
            // Base oscillator section
            ui.group(|ui| {
                ui.label("Base Oscillator");
                ui.push_id(0, |ui| {
                    base_osc_modification = self
                        .osc
                        .draw_controls(ui)
                        .map(Box::new)
                        .map(ComposeModifier::Osc1)
                        .map(OscillatorModification::Compose);
                });
                // Base oscillator parameters
                let mut modifier_enabled = ui.data_mut(|data| {
                    *data.get_persisted_mut_or_default(Id::new("compose_amp_osc"))
                });

                if modifier_enabled {
                    osc_param_modification =
                        draw_param_controls(ui, (false, false), &mut self.osc_params);
                }
                if ui
                    .checkbox(&mut modifier_enabled, "Custom Oscillator Parameters")
                    .changed()
                {
                    ui.data_mut(|x| {
                        x.insert_persisted(Id::new("compose_amp_osc"), modifier_enabled)
                    })
                }
            });

            // Amplitude modulator section
            ui.group(|ui| {
                ui.label("Amplitude Modulator");
                ui.push_id(1, |ui| {
                    amp_modification = self
                        .amplitude_mod
                        .draw_controls(ui)
                        .map(Box::new)
                        .map(ComposeModifier::Osc2) // Using Osc2 for amplitude mods as per ComposeAll
                        .map(OscillatorModification::Compose);
                });
                // Amplitude modulator parameters
                let mut modifier_enabled = ui.data_mut(|data| {
                    *data.get_persisted_mut_or_default(Id::new("compose_amp_amp"))
                });
                if modifier_enabled {
                    amp_param_modification =
                        draw_param_controls(ui, (false, true), &mut self.amp_params);
                }
                if ui
                    .checkbox(&mut modifier_enabled, "Custom Amplitude Mod Parameters")
                    .changed()
                {
                    ui.data_mut(|x| {
                        x.insert_persisted(Id::new("compose_amp_amp"), modifier_enabled)
                    })
                }
            });
        });

        base_osc_modification
            .or(amp_modification)
            .or(osc_param_modification)
            .or(amp_param_modification)
    }
}

#[derive(Default, Debug, Clone, PartialEq, PartialOrd)]
pub struct ComposeFreq {
    osc: Box<Simple>,
    freq_mod: Box<Simple>,
    osc_params: Option<(f32, OscillatorCtx)>,
    freq_params: Option<(f32, OscillatorCtx)>,
}

impl ComposeFreq {
    pub fn new(
        osc: Box<Simple>,
        freq_mod: Box<Simple>,
        osc_params: Option<(f32, OscillatorCtx)>,
        freq_params: Option<(f32, OscillatorCtx)>,
    ) -> Self {
        Self {
            osc,
            freq_mod,
            osc_params,
            freq_params,
        }
    }
}

impl Oscillator for ComposeFreq {
    fn generate_sample(&self, ctx: OscillatorCtx, time: f32) -> f32 {
        let (osc_ctx, osc_time) = match self.osc_params {
            Some((_t, c)) => (c.clone(), time),
            None => (ctx.clone(), time),
        };
        let (freq_ctx, freq_time) = match self.freq_params {
            Some((_t, c)) => (c, time),
            None => (ctx, time),
        };

        let freq_mod = self.freq_mod.generate_sample(freq_ctx, freq_time);
        let mut new_ctx = osc_ctx;
        new_ctx.freq *= freq_mod; // Assuming you want multiplicative modulation
        self.osc.generate_sample(new_ctx, osc_time)
    }
}
impl OscillatorControls for ComposeFreq {
    fn draw_controls(&mut self, ui: &mut egui::Ui) -> Option<OscillatorModification> {
        let mut base_osc_modification = None;
        let mut freq_modification = None;
        let mut osc_param_modification = None;
        let mut freq_param_modification = None;

        ui.vertical(|ui| {
            // Base oscillator section
            ui.group(|ui| {
                ui.label("Base Oscillator");
                ui.push_id(0, |ui| {
                    base_osc_modification = self
                        .osc
                        .draw_controls(ui)
                        .map(Box::new)
                        .map(ComposeModifier::Osc1)
                        .map(OscillatorModification::Compose);
                });
                // Base oscillator parameters
                let mut modifier_enabled = ui.data_mut(|data| {
                    *data.get_persisted_mut_or_default(Id::new("compose_freq_osc"))
                });
                if modifier_enabled {
                    osc_param_modification =
                        draw_param_controls(ui, (false, false), &mut self.osc_params);
                }
                if ui
                    .checkbox(&mut modifier_enabled, "Custom Oscillator Parameters")
                    .changed()
                {
                    ui.data_mut(|x| {
                        x.insert_persisted(Id::new("compose_freq_osc"), modifier_enabled)
                    })
                }
            });

            // Frequency modulator section
            ui.group(|ui| {
                ui.label("Frequency Modulator");
                freq_modification = self
                    .freq_mod
                    .draw_controls(ui)
                    .map(Box::new)
                    .map(ComposeModifier::Osc2) // Note: Using Osc3 as per ComposeAll implementation
                    .map(OscillatorModification::Compose);

                let mut modifier_enabled = ui.data_mut(|data| {
                    *data.get_persisted_mut_or_default(Id::new("compose_freq_freq"))
                });
                if modifier_enabled {
                    freq_param_modification =
                        draw_param_controls(ui, (false, true), &mut self.freq_params);
                }
                // Frequency modulator parameters
                if ui
                    .checkbox(&mut modifier_enabled, "Custom Frequency Mod Parameters")
                    .changed()
                {
                    ui.data_mut(|x| {
                        x.insert_persisted(Id::new("compose_freq_freq"), modifier_enabled)
                    })
                }
            });
        });

        base_osc_modification
            .or(freq_modification)
            .or(osc_param_modification)
            .or(freq_param_modification)
    }
}

#[derive(Default, Debug, Clone, PartialEq, PartialOrd)]
pub struct ComposePhase {
    osc: Box<Simple>,
    phase_mod: Box<Simple>,
    osc_params: Option<(f32, OscillatorCtx)>,
    phase_params: Option<(f32, OscillatorCtx)>,
}

impl ComposePhase {
    pub fn new(
        osc: Box<Simple>,
        phase_mod: Box<Simple>,
        osc_params: Option<(f32, OscillatorCtx)>,
        phase_params: Option<(f32, OscillatorCtx)>,
    ) -> Self {
        Self {
            osc,
            phase_mod,
            osc_params,
            phase_params,
        }
    }
}

impl Oscillator for ComposePhase {
    fn generate_sample(&self, ctx: OscillatorCtx, time: f32) -> f32 {
        let (osc_ctx, osc_time) = match self.osc_params {
            Some((_t, c)) => (c.clone(), time),
            None => (ctx.clone(), time),
        };
        let (phase_ctx, phase_time) = match self.phase_params {
            Some((_t, c)) => (c, time),
            None => (ctx, time),
        };

        let phase_mod = self.phase_mod.generate_sample(phase_ctx, phase_time);
        let mut new_ctx = osc_ctx;
        new_ctx.phase += phase_mod;
        self.osc.generate_sample(new_ctx, osc_time)
    }
}
impl OscillatorControls for ComposePhase {
    fn draw_controls(&mut self, ui: &mut egui::Ui) -> Option<OscillatorModification> {
        let mut base_osc_modification = None;
        let mut phase_modification = None;
        let mut osc_param_modification = None;
        let mut phase_param_modification = None;

        ui.vertical(|ui| {
            // Base oscillator section
            ui.group(|ui| {
                ui.label("Base Oscillator");
                ui.push_id(0, |ui| {
                    base_osc_modification = self
                        .osc
                        .draw_controls(ui)
                        .map(Box::new)
                        .map(ComposeModifier::Osc1)
                        .map(OscillatorModification::Compose);
                });
                // Base oscillator parameters
                let mut modifier_enabled = ui.data_mut(|data| {
                    *data.get_persisted_mut_or_default(Id::new("compose_phase_osc"))
                });
                if modifier_enabled {
                    osc_param_modification =
                        draw_param_controls(ui, (false, false), &mut self.osc_params);
                }

                if ui
                    .checkbox(&mut modifier_enabled, "Custom Oscillator Parameters")
                    .changed()
                {
                    ui.data_mut(|x| {
                        x.insert_persisted(Id::new("compose_phase_osc"), modifier_enabled)
                    })
                }
            });

            // Phase modulator section
            ui.group(|ui| {
                ui.label("Phase Modulator");
                ui.push_id(1, |ui| {
                    phase_modification = self
                        .phase_mod
                        .draw_controls(ui)
                        .map(Box::new)
                        .map(ComposeModifier::Osc2)
                        .map(OscillatorModification::Compose);

                    let mut modifier_enabled = ui.data_mut(|data| {
                        *data.get_persisted_mut_or_default(Id::new("compose_phase_phase"))
                    });
                    if modifier_enabled {
                        phase_param_modification =
                            draw_param_controls(ui, (false, true), &mut self.phase_params);
                    }
                    // Phase modulator parameters
                    if ui
                        .checkbox(&mut modifier_enabled, "Custom Phase Mod Parameters")
                        .changed()
                    {
                        ui.data_mut(|x| {
                            x.insert_persisted(Id::new("compose_phase_phase"), modifier_enabled)
                        })
                    }
                });
            });
        });

        base_osc_modification
            .or(phase_modification)
            .or(osc_param_modification)
            .or(phase_param_modification)
    }
}

#[derive(Default, Debug, Clone, PartialEq, PartialOrd)]
pub struct ComposeAll {
    osc: Box<Simple>,
    amplitude_mod: Option<Box<Simple>>,
    freq_mod: Option<Box<Simple>>,
    phase_mod: Option<Box<Simple>>,

    osc_params: Option<(f32, OscillatorCtx)>,
    amp_params: Option<(f32, OscillatorCtx)>,
    freq_params: Option<(f32, OscillatorCtx)>,
    phase_params: Option<(f32, OscillatorCtx)>,
}

impl ComposeAll {
    pub fn new(
        osc: Box<Simple>,
        amplitude_mod: Option<Box<Simple>>,
        freq_mod: Option<Box<Simple>>,
        phase_mod: Option<Box<Simple>>,
        osc_params: Option<(f32, OscillatorCtx)>,
        amp_params: Option<(f32, OscillatorCtx)>,
        freq_params: Option<(f32, OscillatorCtx)>,
        phase_params: Option<(f32, OscillatorCtx)>,
    ) -> Self {
        Self {
            osc,
            amplitude_mod,
            freq_mod,
            phase_mod,
            osc_params,
            amp_params,
            freq_params,
            phase_params,
        }
    }
}

impl Oscillator for ComposeAll {
    fn generate_sample(&self, ctx: OscillatorCtx, time: f32) -> f32 {
        let (mut osc_ctx, osc_time) = match self.osc_params {
            Some((_t, c)) => (c, time),
            None => (ctx.clone(), time),
        };

        if let Some(amp_mod) = &self.amplitude_mod {
            let (amp_ctx, amp_time) = match self.amp_params {
                Some((_t, c)) => (c, time),
                None => (ctx.clone(), time),
            };
            osc_ctx.amplitude *= amp_mod.generate_sample(amp_ctx, amp_time);
        }

        if let Some(freq_mod) = &self.freq_mod {
            let (freq_ctx, freq_time) = match self.freq_params {
                Some((_t, c)) => (c, time),
                None => (ctx.clone(), time),
            };
            osc_ctx.freq *= freq_mod.generate_sample(freq_ctx, freq_time);
        }

        if let Some(phase_mod) = &self.phase_mod {
            let (phase_ctx, phase_time) = match self.phase_params {
                Some((_t, c)) => (c, time),
                None => (ctx, time),
            };
            osc_ctx.phase += phase_mod.generate_sample(phase_ctx, phase_time);
        }

        self.osc.generate_sample(osc_ctx, osc_time)
    }
}

impl OscillatorControls for ComposeAll {
    fn draw_controls(&mut self, ui: &mut egui::Ui) -> Option<OscillatorModification> {
        let mut base_osc_modification = None;

        let mut amp_modification = None;
        let mut freq_modification = None;
        let mut phase_modification = None;

        let mut osc_param_modification = None;
        let mut amp_param_modification = None;
        let mut freq_param_modification = None;
        let mut phase_param_modification = None;

        let mut amp_mod_modification = None;
        let mut freq_mod_modification = None;
        let mut phase_mod_modification = None;

        ui.vertical(|ui| {
            // Base oscillator section

            ui.group(|ui| {
                ui.label("Base Oscillator");
                ui.push_id(-1, |ui| {
                    base_osc_modification = self
                        .osc
                        .draw_controls(ui)
                        .map(Box::new)
                        .map(ComposeModifier::Osc1)
                        .map(OscillatorModification::Compose);
                });
                // Base oscillator parameters
                let mut modifier_enabled = ui.data_mut(|data| {
                    *data.get_persisted_mut_or_default(Id::new("compose_all_osc"))
                });
                if modifier_enabled {
                    osc_param_modification =
                        draw_param_controls(ui, (false, false), &mut self.osc_params);
                }
                if ui
                    .checkbox(&mut modifier_enabled, "Custom Oscillator Parameters")
                    .changed()
                {
                    ui.data_mut(|x| {
                        x.insert_persisted(Id::new("compose_all_osc"), modifier_enabled)
                    })
                }
            });

            // Modulators section
            ui.group(|ui| {
                ui.label("Modulators");

                // Amplitude modulator
                let mut amp_enabled = ui.data_mut(|data| {
                    *data.get_persisted_mut_or_default(Id::new("compose_all_amp_mode"))
                });
                if amp_enabled {
                    amp_mod_modification = Some(OscillatorModification::Compose(
                        ComposeModifier::EnableMod(ModType::Amplitude, amp_enabled),
                    ));
                }
                if ui
                    .checkbox(&mut amp_enabled, "Amplitude Modulation")
                    .changed()
                {
                    ui.data_mut(|x| {
                        x.insert_persisted(Id::new("compose_all_amp_mode"), amp_enabled)
                    })
                }

                if let Some(amp_mod) = self.amplitude_mod.as_mut() {
                    ui.group(|ui| {
                        ui.label("Amplitude Modulator");
                        ui.push_id(1, |ui| {
                            amp_modification = amp_mod
                                .draw_controls(ui)
                                .map(Box::new)
                                .map(ComposeModifier::Osc2)
                                .map(OscillatorModification::Compose);
                        });
                        // Amplitude modulator parameters
                        let mut modifier_enabled = ui.data_mut(|data| {
                            *data.get_persisted_mut_or_default(Id::new("compose_all_amp"))
                        });
                        if modifier_enabled {
                            amp_param_modification =
                                draw_param_controls(ui, (false, true), &mut self.amp_params);
                        }
                        if ui
                            .checkbox(&mut modifier_enabled, "Custom Amplitude Mod Parameters")
                            .changed()
                        {
                            ui.data_mut(|x| {
                                x.insert_persisted(Id::new("compose_all_amp"), modifier_enabled)
                            });
                        }
                    });
                }

                // Frequency modulator

                let mut freq_enabled = ui.data_mut(|data| {
                    *data.get_persisted_mut_or_default(Id::new("compose_all_freq_mode"))
                });
                if freq_enabled {
                    freq_mod_modification = Some(OscillatorModification::Compose(
                        ComposeModifier::EnableMod(ModType::Frequency, freq_enabled),
                    ));
                }

                if ui
                    .checkbox(&mut freq_enabled, "Frequency Modulation")
                    .changed()
                {
                    ui.data_mut(|x| {
                        x.insert_persisted(Id::new("compose_all_freq_mode"), freq_enabled)
                    })
                }

                if let Some(freq_mod) = self.freq_mod.as_mut() {
                    ui.group(|ui| {
                        ui.label("Frequency Modulator");
                        ui.push_id(2, |ui| {
                            freq_modification = freq_mod
                                .draw_controls(ui)
                                .map(Box::new)
                                .map(ComposeModifier::Osc3)
                                .map(OscillatorModification::Compose);
                        });
                        let mut modifier_enabled = ui.data_mut(|data| {
                            *data.get_persisted_mut_or_default(Id::new("compose_all_freq"))
                        });
                        if modifier_enabled {
                            freq_param_modification =
                                draw_param_controls(ui, (true, false), &mut self.freq_params);
                        }
                        if ui
                            .checkbox(&mut modifier_enabled, "Custom Frequency Mod Parameters")
                            .changed()
                        {
                            ui.data_mut(|x| {
                                x.insert_persisted(Id::new("compose_all_freq"), modifier_enabled)
                            })
                        }
                    });
                }

                // Phase modulator
                let mut phase_enabled = ui.data_mut(|data| {
                    *data.get_persisted_mut_or_default(Id::new("compose_all_phase_mode"))
                });

                if phase_enabled{
                    phase_mod_modification = Some(OscillatorModification::Compose(
                        ComposeModifier::EnableMod(ModType::Phase, phase_enabled),
                    ));
                }
                if ui
                    .checkbox(&mut phase_enabled, "Phase Modulation")
                    .changed()
                {
                    ui.data_mut(|x| {
                        x.insert_persisted(Id::new("compose_all_phase_mode"), phase_enabled)
                    })
                }

                if let Some(phase_mod) = self.phase_mod.as_mut() {
                    ui.group(|ui| {
                        ui.label("Phase Modulator");
                        ui.push_id(3, |ui| {
                            phase_modification = phase_mod
                                .draw_controls(ui)
                                .map(Box::new)
                                .map(ComposeModifier::Osc4)
                                .map(OscillatorModification::Compose);
                        });
                        let mut modifier_enabled = ui.data_mut(|data| {
                            *data.get_persisted_mut_or_default(Id::new("compose_all_phase"))
                        });
                        if modifier_enabled {
                            phase_param_modification =
                                draw_param_controls(ui, (true, false), &mut self.freq_params);
                        }
                        if ui
                            .checkbox(&mut modifier_enabled, "Custom Phase Mod Parameters")
                            .changed()
                        {
                            ui.data_mut(|x| {
                                x.insert_persisted(Id::new("compose_all_phase"), modifier_enabled)
                            })
                        }
                    });
                }
            });
        });

        base_osc_modification
            .or(amp_modification.or(freq_modification.or(phase_modification)))
            .or(
                amp_param_modification
                    .or(freq_param_modification
                        .or(phase_param_modification.or(osc_param_modification))),
            )
            .or(amp_mod_modification.or(freq_mod_modification.or(phase_mod_modification)))
    }
}

fn draw_param_controls(
    ui: &mut egui::Ui,
    count: (bool, bool),
    params: &mut Option<(f32, OscillatorCtx)>,
) -> Option<OscillatorModification> {
    let mut modification = None;
    let fun = match count {
        (true, true) => |x| ComposeModifier::Params(ParamType::Phase, x),
        (true, false) => |x| ComposeModifier::Params(ParamType::Frequency, x),
        (false, true) => |x| ComposeModifier::Params(ParamType::Amplitude, x),
        (false, false) => |x| ComposeModifier::Params(ParamType::Osc, x),
    };
    ui.group(|ui| {
        let (mut _time, mut ctx) = *params.get_or_insert((
            0.0,
            OscillatorCtx {
                amplitude: 1.0,
                freq: 1.0,
                phase: 0.0,
                sampling_rate: 44100,
            },
        ));

        /*ui.horizontal(|ui| {
            ui.label("Time:");
            if ui.add(Slider::new(&mut time, 0.1..=1.0)).changed() {
                if let Some(p) = params {
                    p.0 = time;
                };
                modification = Some(OscillatorModification::Compose(fun(*params)));
            }
        });*/
        ui.horizontal(|ui| {
            ui.label("Amplitude:");
            if ui.add(Slider::new(&mut ctx.amplitude, 0.1..=1.0)).changed() {
                if let Some(p) = params {
                    p.1.amplitude = ctx.amplitude;
                    //modification = Some(OscillatorModification::Compose(fun(Some(p))));
                }
                modification = Some(OscillatorModification::Compose(fun(*params)));
            }
        });
        ui.horizontal(|ui| {
            ui.label("Frequency:");
            if ui
                .add(
                    Slider::new(&mut ctx.freq, 0.1..=22500.0), //.logarithmic(true)
                )
                .changed()
            {
                if let Some(p) = params {
                    p.1.freq = ctx.freq;
                }
                modification = Some(OscillatorModification::Compose(fun(*params)));
            }
        });
        ui.horizontal(|ui| {
            ui.label("Phase:");
            if ui.add(Slider::new(&mut ctx.phase, 0.1..=1.0)).changed() {
                if let Some(p) = params {
                    p.1.phase = ctx.phase;
                }
                modification = Some(OscillatorModification::Compose(fun(*params)));
            }
        });
    });
    modification
}
