use cpal::traits::{DeviceTrait, HostTrait, StreamTrait};
use cpal::{SampleFormat, Stream};
use eframe::egui;
use std::collections::HashMap;
use std::f32::consts::PI;
use std::sync::{Arc, Mutex};
use std::time::Instant;

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
    freq_sustain: f32,
    freq_release: f32,
    freq_start_mult: f32,
    freq_peak_mult: f32,
    freq_sustain_mult: f32,
    num_harmonics: usize,
    harmonic_weights: [f32; 16],
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
    sustain: f32,
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
        sustain: f32,
        release: f32,
        start_freq: f32,
        peak_freq: f32,
        sustain_freq: f32,
    ) -> Self {
        Self {
            attack,
            decay,
            sustain,
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
                let start_mult = self.start_freq / self.start_freq; // Always 1.0
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
                for h in 0..num_harmonics.min(16) {
                    let harmonic_freq = current_frequency * (h + 1) as f32;
                    if harmonic_freq < sample_rate / 2.0 {
                        // Prevent aliasing
                        let harmonic_phase_step = harmonic_freq * 2.0 * PI / sample_rate;
                        self.harmonic_phases[h] =
                            (self.harmonic_phases[h] + harmonic_phase_step) % (2.0 * PI);
                        sum += harmonic_weights[h] * self.harmonic_phases[h].sin();
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
            freq_sustain: 0.5,
            freq_release: 0.3,
            freq_start_mult: 1.0,
            freq_peak_mult: 2.0,
            freq_sustain_mult: 1.5,
            harmonic_weights: [
                1.0, 0.5, 0.33, 0.25, 0.2, 0.17, 0.14, 0.13, 0.11, 0.1, 0.09, 0.08, 0.07, 0.06,
                0.05, 0.04,
            ],
            num_harmonics: 8,
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
                self.freq_sustain,
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

        if self.voices.is_empty() {
            0.0
        } else {
            self.voices
                .values_mut()
                .map(|voice| voice.get_sample(self.sample_rate))
                .sum::<f32>()
                / self.voices.len() as f32
        }
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

        let map: HashMap<egui::Key, u8> = "zxcvbnm,./asdfghjkl;\'\\qwertyuiop[]`1234567890-="
            .chars()
            .map(|x| egui::Key::from_name(&format!("{x}")).unwrap())
            .enumerate()
            .map(|(i, d)| (d, i as u8))
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
                    ui.add(egui::Slider::new(&mut synth.pitch_bend, 0.5..=2.0).text("Pitch Bend"));
                });
            });
            ui.heading("Keyboard-to-Note Mapping");
            // Render keyboard rows with drag value for note adjustment
            let rows = vec![
                "`1234567890-=".chars().collect::<Vec<_>>(),
                "qwertyuiop[]\\".chars().collect::<Vec<_>>(),
                "asdfghjkl;'".chars().collect::<Vec<_>>(),
                "zxcvbnm,./".chars().collect::<Vec<_>>(),
            ];
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
                        let mut note = col + row * cols;

                        let response =
                            ui.allocate_response(tile_size, egui::Sense::click_and_drag());

                        // Draw tile
                        let mut painter = ui.painter();
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

                        /*
                        let mut note_string = note.to_string();
                        if ui
                            .put(rect, egui::TextEdit::singleline(&mut note_string))
                            .changed()
                        {
                            if let Ok(parsed_note) = note_string.parse::<u8>() {
                                if parsed_note <= 127 {
                                    note = parsed_note;
                                }
                            }
                        }*/

                        // Handle note-on and note-off
                        if response.drag_started() {
                            synth.note_on(note);
                        }

                        if response.drag_released() {
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
                        if self.key_map.contains_key(&key) {
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

        /*
                if ctx.input(|i| i.key_pressed(egui::Key::A)) { self.synth.lock().unwrap().note_on(60); }
                if ctx.input(|i| i.key_pressed(egui::Key::S)) { self.synth.lock().unwrap().note_on(62); }
                if ctx.input(|i| i.key_pressed(egui::Key::D)) { self.synth.lock().unwrap().note_on(64); }
                if ctx.input(|i| i.key_pressed(egui::Key::F)) { self.synth.lock().unwrap().note_on(65); }
                if ctx.input(|i| i.key_pressed(egui::Key::G)) { self.synth.lock().unwrap().note_on(67); }
                if ctx.input(|i| i.key_pressed(egui::Key::H)) { self.synth.lock().unwrap().note_on(69); }
                if ctx.input(|i| i.key_pressed(egui::Key::J)) { self.synth.lock().unwrap().note_on(71); }
                if ctx.input(|i| i.key_pressed(egui::Key::K)) { self.synth.lock().unwrap().note_on(72); }
                if ctx.input(|i| i.key_pressed(egui::Key::L)) { self.synth.lock().unwrap().note_on(74); }
                if ctx.input(|i| i.key_pressed(egui::Key::M)) { self.synth.lock().unwrap().note_on(76); }
                if ctx.input(|i| i.key_pressed(egui::Key::Colon)) { self.synth.lock().unwrap().note_on(75); }
                if ctx.input(|i| i.key_pressed(egui::Key::Quote)) { self.synth.lock().unwrap().note_on(77); }




                if ctx.input(|i| i.key_released(egui::Key::A)) { self.synth.lock().unwrap().note_off(60); }
                if ctx.input(|i| i.key_released(egui::Key::S)) { self.synth.lock().unwrap().note_off(62); }
                if ctx.input(|i| i.key_released(egui::Key::D)) { self.synth.lock().unwrap().note_off(64); }
                if ctx.input(|i| i.key_released(egui::Key::F)) { self.synth.lock().unwrap().note_off(65); }
                if ctx.input(|i| i.key_released(egui::Key::G)) { self.synth.lock().unwrap().note_off(67); }
                if ctx.input(|i| i.key_released(egui::Key::H)) { self.synth.lock().unwrap().note_off(69); }
                if ctx.input(|i| i.key_released(egui::Key::J)) { self.synth.lock().unwrap().note_off(71); }
                if ctx.input(|i| i.key_released(egui::Key::K)) { self.synth.lock().unwrap().note_off(72); }
                if ctx.input(|i| i.key_released(egui::Key::L)) { self.synth.lock().unwrap().note_off(74); }
                if ctx.input(|i| i.key_released(egui::Key::M)) { self.synth.lock().unwrap().note_off(76); }
                if ctx.input(|i| i.key_released(egui::Key::Colon)) { self.synth.lock().unwrap().note_off(75); }
                if ctx.input(|i| i.key_released(egui::Key::Quote)) { self.synth.lock().unwrap().note_off(77); }
        */

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
