use cpal::traits::{DeviceTrait, HostTrait, StreamTrait};
use cpal::{SampleFormat, Stream};
use eframe::egui;
use std::collections::HashMap;
use std::f32::consts::PI;
use std::sync::{Arc, Mutex};
use std::time::Instant;

#[derive(Clone)]
struct ChorusParameters {
    buffers: Vec<Vec<f32>>,
    positions: Vec<usize>,
    rates: Vec<f32>,
    depths: Vec<f32>,
    phases: Vec<f32>,
    mix: f32,
}

#[derive(Clone)]
struct ReverbParameters {
    comb_filters: Vec<Vec<f32>>,
    comb_positions: Vec<usize>,
    allpass_filters: Vec<Vec<f32>>,
    allpass_positions: Vec<usize>,
    feedback: f32,
    mix: f32,
}

#[derive(Clone)]
struct RingModParameters {
    frequency: f32,
    phase: f32,
    mix: f32,
}
#[derive(Clone)]
struct DelayParameters {
    buffer: Vec<f32>,
    position: usize,
    delay_time: f32,
    feedback: f32,
    mix: f32,
}

#[derive(Clone)]
struct FilterParameters {
    cutoff: f32,
    resonance: f32,
    mix: f32,
    prev_input: f32,
    prev_output: f32,
}

#[derive(Clone)]
struct TremoloParameters {
    rate: f32,
    depth: f32,
    mix: f32,
    phase: f32,
}

// Main effect enum
#[derive(Clone)]
enum Effect {
    Delay(DelayParameters),
    Distortion { drive: f32, mix: f32 },
    Filter(FilterParameters),
    Tremolo(TremoloParameters),
    Chorus(ChorusParameters),
    Reverb(ReverbParameters),
    RingMod(RingModParameters),
}

impl Effect {
    fn process(&mut self, sample: f32, sample_rate: f32) -> f32 {
        match self {
            Effect::Delay(params) => {
                let delayed = params.buffer[params.position];
                params.buffer[params.position] = sample + delayed * params.feedback;
                params.position = (params.position + 1) % params.buffer.len();
                sample * (1.0 - params.mix) + delayed * params.mix
            },
            Effect::Distortion { drive, mix } => {
                let processed = (sample * *drive).tanh();
                sample * (1.0 - *mix) + processed * *mix
            },
            Effect::Filter(params) => {
                let normalized_cutoff = 2.0 * std::f32::consts::PI * params.cutoff / sample_rate;
                let alpha = normalized_cutoff / (1.0 + normalized_cutoff);
                
                let processed = params.prev_output + alpha * (sample - params.prev_output);
                params.prev_output = processed;
                params.prev_input = sample;
                
                sample * (1.0 - params.mix) + processed * params.mix
            },
            Effect::Tremolo(params) => {
                let modulation = (1.0 + (params.phase * 2.0 * std::f32::consts::PI).sin() * params.depth) * 0.5;
                params.phase = (params.phase + params.rate / sample_rate) % 1.0;
                
                let processed = sample * modulation;
                sample * (1.0 - params.mix) + processed * params.mix
            },

            Effect::Chorus(params) => {
                let mut output = 0.0;

                for i in 0..params.buffers.len() {
                    // Update LFO phase
                    params.phases[i] = (params.phases[i] + params.rates[i] / sample_rate) % 1.0;

                    // Calculate delay time with LFO modulation
                    let mod_delay = (1.0 + (params.phases[i] * 2.0 * std::f32::consts::PI).sin() * params.depths[i]) * 0.5;
                    let delay_samples = (mod_delay * (params.buffers[i].len() - 1) as f32) as usize;

                    // Read from buffer
                    let read_pos = (params.positions[i] + params.buffers[i].len() - delay_samples) % params.buffers[i].len();
                    output += params.buffers[i][read_pos];

                    // Write to buffer
                    params.buffers[i][params.positions[i]] = sample;
                    params.positions[i] = (params.positions[i] + 1) % params.buffers[i].len();
                }

                output /= params.buffers.len() as f32;
                sample * (1.0 - params.mix) + output * params.mix
            },
            Effect::Reverb(params) => {
                // Process comb filters in parallel
                let mut comb_output = 0.0;
                for i in 0..params.comb_filters.len() {
                    let delayed = params.comb_filters[i][params.comb_positions[i]];
                    comb_output += delayed;
                    params.comb_filters[i][params.comb_positions[i]] = sample + delayed * params.feedback;
                    params.comb_positions[i] = (params.comb_positions[i] + 1) % params.comb_filters[i].len();
                }
                comb_output /= params.comb_filters.len() as f32;

                // Process allpass filters in series
                let mut allpass_output = comb_output;
                for i in 0..params.allpass_filters.len() {
                    let delayed = params.allpass_filters[i][params.allpass_positions[i]];
                    let input = allpass_output;
                    allpass_output = delayed - input;
                    params.allpass_filters[i][params.allpass_positions[i]] = input + delayed * 0.5;
                    params.allpass_positions[i] = (params.allpass_positions[i] + 1) % params.allpass_filters[i].len();
                }

                sample * (1.0 - params.mix) + allpass_output * params.mix
            },
            Effect::RingMod(params) => {
                let modulator = (params.phase * 2.0 * std::f32::consts::PI).sin();
                params.phase = (params.phase + params.frequency / sample_rate) % 1.0;

                let processed = sample * modulator;
                sample * (1.0 - params.mix) + processed * params.mix
            },
        }
    }

    fn reset(&mut self) {
        match self {
            Effect::Delay(params) => {
                params.buffer.fill(0.0);
                params.position = 0;
            },
            Effect::Distortion { .. } => {},
            Effect::Filter(params) => {
                params.prev_input = 0.0;
                params.prev_output = 0.0;
            },
            Effect::Tremolo(params) => {
                params.phase = 0.0;
            },
            Effect::Chorus(params) => {
                for buffer in params.buffers.iter_mut() {
                    buffer.fill(0.0);
                }
                params.positions.fill(0);
                params.phases.fill(0.0);
            },
            Effect::Reverb(params) => {
                for buffer in params.comb_filters.iter_mut() {
                    buffer.fill(0.0);
                }
                for buffer in params.allpass_filters.iter_mut() {
                    buffer.fill(0.0);
                }
                params.comb_positions.fill(0);
                params.allpass_positions.fill(0);
            },
            Effect::RingMod(params) => {
                params.phase = 0.0;
            },
        }
    }
}


impl Effect {
    fn new_delay(sample_rate: f32, delay_time: f32, feedback: f32, mix: f32) -> Self {
        let buffer_size = (sample_rate * delay_time) as usize;
        Effect::Delay(DelayParameters {
            buffer: vec![0.0; buffer_size.max(1)],
            position: 0,
            delay_time,
            feedback,
            mix,
        })
    }

    fn new_distortion(drive: f32, mix: f32) -> Self {
        Effect::Distortion { drive, mix }
    }

    fn new_filter(cutoff: f32, resonance: f32, mix: f32) -> Self {
        Effect::Filter(FilterParameters {
            cutoff,
            resonance,
            mix,
            prev_input: 0.0,
            prev_output: 0.0,
        })
    }

    fn new_tremolo(rate: f32, depth: f32, mix: f32) -> Self {
        Effect::Tremolo(TremoloParameters {
            rate,
            depth,
            mix,
            phase: 0.0,
        })
    }

    fn new_chorus(sample_rate: f32, voices: usize, mix: f32) -> Self {
        let max_delay_samples = (sample_rate * 0.030) as usize; // 30ms max delay
        let mut buffers = Vec::new();
        let mut positions = Vec::new();
        let mut rates = Vec::new();
        let mut depths = Vec::new();
        let mut phases = Vec::new();

        for i in 0..voices {
            buffers.push(vec![0.0; max_delay_samples]);
            positions.push(0);
            // Slightly different rates for each voice
            rates.push(0.5 + (i as f32 * 0.2));
            depths.push(0.7);
            phases.push(0.0);
        }

        Effect::Chorus(ChorusParameters {
            buffers,
            positions,
            rates,
            depths,
            phases,
            mix,
        })
    }

    fn new_reverb(sample_rate: f32, room_size: f32, mix: f32) -> Self {
        // Schroeder reverb implementation
        let comb_delays = [
            (0.0297 * room_size),
            (0.0371 * room_size),
            (0.0411 * room_size),
            (0.0437 * room_size),
        ];
        let allpass_delays = [0.0050, 0.0017];

        let mut comb_filters = Vec::new();
        let mut comb_positions = Vec::new();
        let mut allpass_filters = Vec::new();
        let mut allpass_positions = Vec::new();

        for delay in comb_delays.iter() {
            let size = (sample_rate * delay) as usize;
            comb_filters.push(vec![0.0; size]);
            comb_positions.push(0);
        }

        for delay in allpass_delays.iter() {
            let size = (sample_rate * delay) as usize;
            allpass_filters.push(vec![0.0; size]);
            allpass_positions.push(0);
        }

        Effect::Reverb(ReverbParameters {
            comb_filters,
            comb_positions,
            allpass_filters,
            allpass_positions,
            feedback: 0.84,
            mix,
        })
    }

    fn new_ring_mod(frequency: f32, mix: f32) -> Self {
        Effect::RingMod(RingModParameters {
            frequency,
            phase: 0.0,
            mix,
        })
    }
}

// Simplified effect stack
struct EffectStack {
    effects: Vec<Effect>,
}

impl EffectStack {
    fn new() -> Self {
        Self { effects: Vec::new() }
    }

    fn add_effect(&mut self, effect: Effect) {
        self.effects.push(effect);
    }

    fn process(&mut self, sample: f32, sample_rate: f32) -> f32 {
        let mut processed = sample;
        for effect in self.effects.iter_mut() {
            processed = effect.process(processed, sample_rate);
        }
        processed
    }

    fn reset(&mut self) {
        for effect in self.effects.iter_mut() {
            effect.reset();
        }
    }
}


#[derive(Clone, Copy, PartialEq)]
enum Waveform {
    Sine,
    Square,
    Sawtooth,
    Triangle,
    Noise,
    Additive {
        num_harmonics: usize,
        harmonic_weights: [f32; 16],
    },
}

struct Synth {
    voices: HashMap<u8, Voice>,
    sample_rate: f32,
    pitch_bend: f32,
    waveform: Waveform,
    attack: f32,
    decay: f32,
    sustain: f32,
    release: f32,
    freq_attack: f32,
    freq_decay: f32,
    freq_release: f32,
    freq_start_mult: f32,
    freq_peak_mult: f32,
    freq_sustain_mult: f32,
    num_harmonics: usize,
    harmonic_weights: [f32; 16],
    effects:EffectStack,
}

struct Voice {
    frequency: f32,
    waveform: Waveform,
    envelope: Envelope,
    frequency_envelope: FrequencyEnvelope,
    phase: f32,
    pitch_bend: f32,
    harmonic_phases: [f32; 16],
}

struct Envelope {
    attack: f32,
    decay: f32,
    sustain: f32,
    release: f32,
    start_time: Option<Instant>,
    release_time: Option<Instant>,
    is_released: bool,
}

struct FrequencyEnvelope {
    attack: f32,
    decay: f32,
    release: f32,
    start_time: Option<Instant>,
    release_time: Option<Instant>,
    is_released: bool,
    start_freq: f32,
    peak_freq: f32,
    sustain_freq: f32,
}

impl FrequencyEnvelope {
    fn new(
        attack: f32,
        decay: f32,
        release: f32,
        start_freq: f32,
        peak_freq: f32,
        sustain_freq: f32,
    ) -> Self {
        Self {
            attack,
            decay,
            release,
            start_time: None,
            release_time: None,
            is_released: false,
            start_freq,
            peak_freq,
            sustain_freq,
        }
    }

    fn get_frequency_multiplier(&self) -> f32 {
        if let Some(start_time) = self.start_time {
            let elapsed = start_time.elapsed().as_secs_f32();

            if self.is_released {
                if let Some(release_time) = self.release_time {
                    let release_elapsed = release_time.elapsed().as_secs_f32();
                    return if release_elapsed >= self.release {
                        1.0 // Return to base frequency
                    } else {
                        let sustain_mult = self.sustain_freq / self.start_freq;
                        // Interpolate from sustain frequency to base frequency
                        sustain_mult * (1.0 - release_elapsed / self.release)
                            + 1.0 * (release_elapsed / self.release)
                    };
                }
            }

            if elapsed < self.attack {
                // Interpolate from start frequency to peak frequency
                let progress = elapsed / self.attack;
                let start_mult = 1.0;
                let peak_mult = self.peak_freq / self.start_freq;
                start_mult + (peak_mult - start_mult) * progress
            } else if elapsed < self.attack + self.decay {
                // Interpolate from peak frequency to sustain frequency
                let progress = (elapsed - self.attack) / self.decay;
                let peak_mult = self.peak_freq / self.start_freq;
                let sustain_mult = self.sustain_freq / self.start_freq;
                peak_mult + (sustain_mult - peak_mult) * progress
            } else {
                // Hold at sustain frequency
                self.sustain_freq / self.start_freq
            }
        } else {
            1.0 // No modulation if not started
        }
    }
}

impl Envelope {
    fn new(attack: f32, decay: f32, sustain: f32, release: f32) -> Self {
        Self {
            attack,
            decay,
            sustain,
            release,
            start_time: None,
            release_time: None,
            is_released: false,
        }
    }

    fn get_amplitude(&self) -> f32 {
        if let Some(start_time) = self.start_time {
            let elapsed = start_time.elapsed().as_secs_f32();

            if self.is_released {
                if let Some(release_time) = self.release_time {
                    let release_elapsed = release_time.elapsed().as_secs_f32();
                    return if release_elapsed >= self.release {
                        0.0
                    } else {
                        self.sustain * (1.0 - release_elapsed / self.release)
                    };
                }
            }

            if elapsed < self.attack {
                elapsed / self.attack
            } else if elapsed < self.attack + self.decay {
                1.0 - (1.0 - self.sustain) * (elapsed - self.attack) / self.decay
            } else {
                self.sustain
            }
        } else {
            0.0
        }
    }
}

impl Voice {
    fn get_sample(&mut self, sample_rate: f32) -> f32 {
        let base_frequency = self.frequency * self.pitch_bend;
        let freq_multiplier = self.frequency_envelope.get_frequency_multiplier();
        let current_frequency = base_frequency * freq_multiplier;

        let phase_step = current_frequency * 2.0 * PI / sample_rate;
        let amplitude = self.envelope.get_amplitude();

        let sample = match self.waveform {
            Waveform::Sine => self.phase.sin(),
            Waveform::Square => {
                if self.phase.sin() >= 0.0 {
                    1.0
                } else {
                    -1.0
                }
            }
            Waveform::Sawtooth => (self.phase % (2.0 * PI)) / (2.0 * PI) * 2.0 - 1.0,
            Waveform::Triangle => {
                let normalized_phase = (self.phase % (2.0 * PI)) / (2.0 * PI);
                if normalized_phase < 0.5 {
                    normalized_phase * 4.0 - 1.0
                } else {
                    3.0 - normalized_phase * 4.0
                }
            }
            Waveform::Noise => rand::random::<f32>() * 2.0 - 1.0,
            Waveform::Additive {
                num_harmonics,
                harmonic_weights,
            } => {
                let mut sum = 0.0;
                //for h in 0..num_harmonics.min(16) 
                for (h, harmonic_weight) in harmonic_weights.iter().enumerate().take(num_harmonics.min(16))
                {
                    let harmonic_freq = current_frequency * (h + 1) as f32;
                    if harmonic_freq < sample_rate / 2.0 {
                        // Prevent aliasing
                        let harmonic_phase_step = harmonic_freq * 2.0 * PI / sample_rate;
                        self.harmonic_phases[h] =
                            (self.harmonic_phases[h] + harmonic_phase_step) % (2.0 * PI);
                        sum += harmonic_weight * self.harmonic_phases[h].sin();
                    }
                }
                // Normalize output
                sum / (num_harmonics as f32).sqrt()
            }
        };

        self.phase = (self.phase + phase_step) % (2.0 * PI);
        sample * amplitude
    }
}

impl Synth {
    fn new(sample_rate: f32) -> Self {
        Self {
            voices: HashMap::new(),
            sample_rate,
            pitch_bend: 1.0,
            waveform: Waveform::Sine,
            attack: 0.1,
            decay: 0.1,
            sustain: 0.7,
            release: 0.3,
            freq_attack: 0.1,
            freq_decay: 0.2,
            freq_release: 0.3,
            freq_start_mult: 1.0,
            freq_peak_mult: 2.0,
            freq_sustain_mult: 1.5,
            harmonic_weights: [
                1.0, 0.5, 0.33, 0.25, 0.2, 0.17, 0.14, 0.13, 0.11, 0.1, 0.09, 0.08, 0.07, 0.06,
                0.05, 0.04,
            ],
            num_harmonics: 8,
            effects:EffectStack::new(),
        }
    }

    fn note_on(&mut self, note: u8) {
        if self.voices.contains_key(&note) && !self.voices[&note].envelope.is_released {
            return;
        }
        let frequency = 440.0 * 2.0f32.powf((note as f32) / 12.0);
        let waveform = match self.waveform {
            Waveform::Additive { .. } => Waveform::Additive {
                num_harmonics: self.num_harmonics,
                harmonic_weights: self.harmonic_weights,
            },
            other => other,
        };
        let mut voice = Voice {
            frequency,
            waveform,
            envelope: Envelope::new(self.attack, self.decay, self.sustain, self.release),
            frequency_envelope: FrequencyEnvelope::new(
                self.freq_attack,
                self.freq_decay,
                self.freq_release,
                frequency,
                frequency * self.freq_peak_mult,
                frequency * self.freq_sustain_mult,
            ),
            phase: 0.0,
            pitch_bend: self.pitch_bend,
            harmonic_phases: [0.0; 16],
        };
        voice.envelope.start_time = Some(Instant::now());
        voice.frequency_envelope.start_time = Some(Instant::now());

        self.voices.insert(note, voice);
    }

    fn note_off(&mut self, note: u8) {
        if let Some(voice) = self.voices.get_mut(&note) {
            voice.envelope.is_released = true;
            voice.envelope.release_time = Some(Instant::now());
            voice.frequency_envelope.is_released = true;
            voice.frequency_envelope.release_time = Some(Instant::now());
        }
    }

    fn get_next_sample(&mut self) -> f32 {
        self.voices.retain(|_, voice| {
            !voice.envelope.is_released
                || voice.envelope.release_time.unwrap().elapsed().as_secs_f32()
                    < voice.envelope.release
        });

        let ret = if self.voices.is_empty() {
            0.0
        } else {
            self.voices
                .values_mut()
                .map(|voice| voice.get_sample(self.sample_rate))
                .sum::<f32>()
                / self.voices.len() as f32
        };

            self.effects.process(ret,self.sample_rate)
    }
}

struct SynthApp {
    synth: Arc<Mutex<Synth>>,
    _stream: Stream,
    key_map: HashMap<egui::Key, u8>,
}

impl SynthApp {
    fn new(_cc: &eframe::CreationContext<'_>) -> Self {
        let host = cpal::default_host();
        let device = host.default_output_device().expect("no output device");
        let config = device.default_output_config().unwrap();
        let sample_rate = config.sample_rate().0 as f32;

        let synth = Arc::new(Mutex::new(Synth::new(sample_rate)));
        let synth_clone = synth.clone();

        let stream = match config.sample_format() {
            SampleFormat::F32 => create_stream(&device, &config.into(), synth_clone.clone()),
            //SampleFormat::I16 => create_stream::<i16>(&device, &config.into(), synth_clone.clone()),
            //SampleFormat::U16 => create_stream::<u16>(&device, &config.into(), synth_clone.clone()),
            _ => panic!("Unsupported format"),
        }
        .unwrap();

        stream.play().unwrap();

        let keyboard = [
            "zxcvbnm,./",
            "asdfghjkl;'\\",
            "qwertyuiop[]",
            "`1234567890-=",
        ];
        let map: HashMap<egui::Key, u8> = keyboard.into_iter()
            .enumerate()
            .flat_map(move |(cnt, s)| {
                s.chars()
                    .map( move |x| egui::Key::from_name(&format!("{x}")).unwrap())
                    .enumerate()
                    .map(move |(i, d)| (d, (i + cnt * 5) as u8))
            })
            .collect();
        Self {
            synth: synth_clone,
            _stream: stream,
            key_map: map,
        }
    }
}

impl eframe::App for SynthApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        egui::CentralPanel::default().show(ctx, |ui| {
            let mut synth = self.synth.lock().unwrap();

            ui.heading("Synthesizer");

            ui.horizontal(|ui| {
                ui.label("Waveform:");
                ui.radio_value(&mut synth.waveform, Waveform::Sine, "Sine");
                ui.radio_value(&mut synth.waveform, Waveform::Square, "Square");
                ui.radio_value(&mut synth.waveform, Waveform::Sawtooth, "Saw");
                ui.radio_value(&mut synth.waveform, Waveform::Triangle, "Triangle");
                ui.radio_value(&mut synth.waveform, Waveform::Noise, "Noise");
                if ui
                    .radio(
                        matches!(synth.waveform, Waveform::Additive { .. }),
                        "Additive",
                    )
                    .clicked()
                {
                    synth.waveform = Waveform::Additive {
                        num_harmonics: synth.num_harmonics,
                        harmonic_weights: synth.harmonic_weights,
                    };
                }
            });

            if matches!(synth.waveform, Waveform::Additive { .. }) {
                ui.add(
                    egui::Slider::new(&mut synth.num_harmonics, 1..=16).text("Number of Harmonics"),
                );

                ui.label("Harmonic Weights:");
                for i in 0..synth.num_harmonics {
                    ui.add(
                        egui::Slider::new(&mut synth.harmonic_weights[i], 0.0..=1.0)
                            .text(format!("Harmonic {}", i + 1)),
                    );
                }
            }

            ui.horizontal(|ui| {
                ui.vertical(|ui| {
                    ui.heading("ADSR Envelope");
                    ui.add(egui::Slider::new(&mut synth.attack, 0.01..=1.0).text("Attack"));
                    ui.add(egui::Slider::new(&mut synth.decay, 0.01..=1.0).text("Decay"));
                    ui.add(egui::Slider::new(&mut synth.sustain, 0.0..=1.0).text("Sustain"));
                    ui.add(egui::Slider::new(&mut synth.release, 0.01..=2.0).text("Release"));
                });
                ui.vertical(|ui| {
                    ui.heading("Frequency Modulation Range");
                    ui.add(
                        egui::Slider::new(&mut synth.freq_start_mult, 0.5..=2.0)
                            .text("Start Multiplier"),
                    );
                    ui.add(
                        egui::Slider::new(&mut synth.freq_peak_mult, 0.5..=4.0)
                            .text("Peak Multiplier"),
                    );
                    ui.add(
                        egui::Slider::new(&mut synth.freq_sustain_mult, 0.5..=3.0)
                            .text("Sustain Multiplier"),
                    );
                });
            });

            ui.add(egui::Slider::new(&mut synth.pitch_bend, 0.5..=2.0).text("Pitch Bend"));
            ui.heading("Effects");
            ui.horizontal(|ui| {
                //let synth = synth.lock().unwrap();
                if ui.button("Add Delay").clicked() {
                    let sr = synth.sample_rate;
                    synth.effects.add_effect(Effect::new_delay(
                        sr,
                        0.3, // delay time
                        0.4, // feedback
                        0.5, // mix
                    ));
                }
                
                if ui.button("Add Distortion").clicked() {
                    synth.effects.add_effect(Effect::new_distortion(2.0, 0.5));
                }
                
                if ui.button("Add Filter").clicked() {
                    synth.effects.add_effect(Effect::new_filter(1000.0, 0.7, 0.5));
                }
                
                if ui.button("Add Tremolo").clicked() {
                    synth.effects.add_effect(Effect::new_tremolo(5.0, 0.5, 0.5));
                }
                if ui.button("Add Chorus").clicked() {
                    let sample_rate = synth.sample_rate;
                    synth.effects.add_effect(Effect::new_chorus(
                    sample_rate,
                    3, // number of voices
                    0.5, // mix
                ));
                }

                if ui.button("Add Reverb").clicked() {
                    let sample_rate = synth.sample_rate;
                    synth.effects.add_effect(Effect::new_reverb(
                    sample_rate,
                    1.0, // room size
                    0.5, // mix
                    ));
                    }

                if ui.button("Add Ring Modulator").clicked() {
                    synth.effects.add_effect(Effect::new_ring_mod(440.0, 0.5));
                        }
                });

            if ui.button("Reset Effects").clicked() {
                synth.effects = EffectStack::new();
            }

            let sample_rate = synth.sample_rate;
            for (index, effect) in synth.effects.effects.iter_mut().enumerate() {
                ui.group(|ui| {
                    match effect {
                        Effect::Delay(params) => {
                            ui.label(format!("Delay {}", index + 1));
                            ui.add(egui::Slider::new(&mut params.delay_time, 0.0..=2.0).text("Delay Time"));
                            ui.add(egui::Slider::new(&mut params.feedback, 0.0..=0.95).text("Feedback"));
                            ui.add(egui::Slider::new(&mut params.mix, 0.0..=1.0).text("Mix"));

                            // Update buffer size if delay time changes
                            let new_size = (sample_rate * params.delay_time) as usize;
                            if params.buffer.len() != new_size {
                                params.buffer = vec![0.0; new_size.max(1)];
                                params.position = 0;
                            }
                        },
                        Effect::Distortion {  ref mut drive,ref mut  mix } => {
                            ui.label(format!("Distortion {}", index + 1));
                            ui.add(egui::Slider::new(drive, 1.0..=10.0).text("Drive"));
                            ui.add(egui::Slider::new( mix, 0.0..=1.0).text("Mix"));
                        },
                        Effect::Filter(params) => {
                            ui.label(format!("Filter {}", index + 1));
                            ui.add(egui::Slider::new(&mut params.cutoff, 20.0..=20000.0).logarithmic(true).text("Cutoff"));
                            ui.add(egui::Slider::new(&mut params.resonance, 0.0..=0.99).text("Resonance"));
                            ui.add(egui::Slider::new(&mut params.mix, 0.0..=1.0).text("Mix"));
                        },
                        Effect::Tremolo(params) => {
                            ui.label(format!("Tremolo {}", index + 1));
                            ui.add(egui::Slider::new(&mut params.rate, 0.1..=20.0).text("Rate"));
                            ui.add(egui::Slider::new(&mut params.depth, 0.0..=1.0).text("Depth"));
                            ui.add(egui::Slider::new(&mut params.mix, 0.0..=1.0).text("Mix"));
                        },
                        Effect::Chorus(params) => {
        ui.label(format!("Chorus {}", index + 1));
        for i in 0..params.rates.len() {
            ui.add(egui::Slider::new(&mut params.rates[i], 0.1..=5.0)
                .text(format!("Voice {} Rate", i + 1)));
            ui.add(egui::Slider::new(&mut params.depths[i], 0.0..=1.0)
                .text(format!("Voice {} Depth", i + 1)));
        }
        ui.add(egui::Slider::new(&mut params.mix, 0.0..=1.0).text("Mix"));
    },
    Effect::Reverb(params) => {
        ui.label(format!("Reverb {}", index + 1));
        ui.add(egui::Slider::new(&mut params.feedback, 0.0..=0.95).text("Feedback"));
        ui.add(egui::Slider::new(&mut params.mix, 0.0..=1.0).text("Mix"));
    },
    Effect::RingMod(params) => {
        ui.label(format!("Ring Modulator {}", index + 1));
        ui.add(egui::Slider::new(&mut params.frequency, 1.0..=2000.0)
            .logarithmic(true)
            .text("Frequency"));
        ui.add(egui::Slider::new(&mut params.mix, 0.0..=1.0).text("Mix"));
    },
                    }
                });
            }

           ui.heading("Keyboard-to-Note Mapping");
            // Render keyboard rows with drag value for note adjustment
            let rows = ["`1234567890-=".chars().collect::<Vec<_>>(),
                "qwertyuiop[]\\".chars().collect::<Vec<_>>(),
                "asdfghjkl;'".chars().collect::<Vec<_>>(),
                "zxcvbnm,./".chars().collect::<Vec<_>>()];
            for row in rows.iter() {
                ui.horizontal(|ui| {
                    for &key_char in row {
                        let key = egui::Key::from_name(&key_char.to_string()).unwrap();
                        let note = self.key_map.entry(key).or_insert(0);

                        // Display key and text input for note
                        ui.vertical(|ui| {
                            ui.label(key_char.to_string());
                            let mut note_string = note.to_string();
                            let text_edit =
                                egui::TextEdit::singleline(&mut note_string).desired_width(45.0);
                            if ui.add(text_edit).changed() {
                                if let Ok(parsed_note) = note_string.parse::<u8>() {
                                    if parsed_note <= 127 {
                                        *note = parsed_note;
                                    }
                                }
                            }
                        });
                    }
                });
            }

            ui.heading("Rectangular Keyboard");
            let tile_size = egui::vec2(50.0, 50.0); // Size of each tile
            let rows: u8 = 5; // Number of rows
            let cols: u8 = 10; // Number of columns

            for row in 0..rows {
                ui.horizontal(|ui| {
                    for col in 0..cols {
                        // Calculate MIDI note
                        let note = col + row * cols;

                        let response =
                            ui.allocate_response(tile_size, egui::Sense::click_and_drag());

                        // Draw tile
                        let painter = ui.painter();
                        let rect = response.rect;
                        painter.rect_filled(
                            rect,
                            5.0, // Corner radius
                            if response.hovered() || response.clicked() {
                                egui::Color32::LIGHT_BLUE
                            } else {
                                egui::Color32::GRAY
                            },
                        );
                        painter.rect_stroke(
                            rect,
                            5.0, // Corner radius
                            egui::Stroke::new(1.0, egui::Color32::BLACK),
                        );
                        if response.drag_started() {
                            synth.note_on(note);
                        }

                        if response.drag_stopped() {
                            synth.note_off(note);
                        }
                    }
                });
            }
        });

        ctx.input(|i| {
            let mut notes = Vec::new();
            for event in &i.events {
                if let egui::Event::Key { key, pressed, .. } = event {
                    //println!("{:?} {:?} {} ", &key, pressed, self.key_map[ &]  );
                    {
                        if self.key_map.contains_key(key) {
                            let freq = self.key_map[key];
                            notes.push((freq, pressed));
                        }
                    }
                }
            }
            let mut synth = self.synth.lock().unwrap();

            for note in notes {
                match note {
                    (freq, true) => synth.note_on(freq),
                    (freq, false) => synth.note_off(freq),
                }
            }
        });

        ctx.request_repaint();
    }
}

fn create_stream(
    device: &cpal::Device,
    config: &cpal::StreamConfig,
    synth: Arc<Mutex<Synth>>,
) -> Result<Stream, cpal::BuildStreamError> {
    device.build_output_stream(
        config,
        move |data: &mut [_], _: &cpal::OutputCallbackInfo| {
            let mut synth = synth.lock().unwrap();
            for sample in data.iter_mut() {
                *sample = synth.get_next_sample();
            }
        },
        |err| eprintln!("Error in audio stream: {}", err),
        None,
    )
}

fn main() -> Result<(), eframe::Error> {
    let options = eframe::NativeOptions::default();
    eframe::run_native(
        "Synthesizer",
        options,
        Box::new(|cc| Ok(Box::new(SynthApp::new(cc)))),
    )
}
