use eframe::egui;
use cpal::traits::{DeviceTrait, HostTrait, StreamTrait};
use cpal::{SampleFormat, Stream};
use std::f32::consts::PI;
use std::sync::{Arc, Mutex};
use std::time::Instant;
use std::collections::HashMap;

#[derive(Clone, Copy, PartialEq)]
enum Waveform {
    Sine,
    Square,
    Sawtooth,
    Triangle,
    Noise,
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
}

struct Voice {
    frequency: f32,
    waveform: Waveform,
    envelope: Envelope,
    phase: f32,
    pitch_bend: f32,
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

impl Envelope {
    fn new(attack: f32, decay: f32, sustain: f32, release: f32) -> Self {
        Self {
            attack, decay, sustain, release,
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
        let bent_frequency = self.frequency * self.pitch_bend;
        let phase_step = bent_frequency * 2.0 * PI / sample_rate;
        let amplitude = self.envelope.get_amplitude();

        let sample = match self.waveform {
            Waveform::Sine => self.phase.sin(),
            Waveform::Square => if self.phase.sin() >= 0.0 { 1.0 } else { -1.0 },
            Waveform::Sawtooth => (self.phase % (2.0 * PI)) / (2.0 * PI) * 2.0 - 1.0,
            Waveform::Triangle => {
                let normalized_phase = (self.phase % (2.0 * PI)) / (2.0 * PI);
                if normalized_phase < 0.5 {
                    normalized_phase * 4.0 - 1.0
                } else {
                    3.0 - normalized_phase * 4.0
                }
            },
            Waveform::Noise => rand::random::<f32>() * 2.0 - 1.0,
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
        }
    }

    fn note_on(&mut self, note: u8) {
        if self.voices.contains_key(&note) && !self.voices[&note].envelope.is_released { return ; }
        let frequency = 440.0 * 2.0f32.powf((note as f32 ) / 12.0);
        let mut voice = Voice {
            frequency,
            waveform: self.waveform,
            envelope: Envelope::new(self.attack, self.decay, self.sustain, self.release),
            phase: 0.0,
            pitch_bend: self.pitch_bend,
        };
        voice.envelope.start_time = Some(Instant::now());
        self.voices.insert(note, voice);
    }

    fn note_off(&mut self, note: u8) {
        if let Some(voice) = self.voices.get_mut(&note) {
            voice.envelope.is_released = true;
            voice.envelope.release_time = Some(Instant::now());
        }
    }

    fn get_next_sample(&mut self) -> f32 {
        self.voices.retain(|_, voice| {
            !voice.envelope.is_released || 
            voice.envelope.release_time.unwrap().elapsed().as_secs_f32() < voice.envelope.release
        });

        if self.voices.is_empty() {
            0.0
        } else {
            self.voices.values_mut()
                .map(|voice| voice.get_sample(self.sample_rate))
                .sum::<f32>() / self.voices.len() as f32
        }
    }
}

struct SynthApp {
    synth: Arc<Mutex<Synth>>,
    _stream: Stream,
    key_map : HashMap<egui::Key,u8>,
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
        }.unwrap();

        stream.play().unwrap();

        let map : HashMap<egui::Key,u8>= "zxcvbnm,./asdfghjkl;\'\\qwertyuiop[]`1234567890-=".chars().map(|x| egui::Key::from_name(&format!("{x}")).unwrap()).enumerate().map(|(i,d)| (d,i as u8)).collect();
        Self {
            synth: synth_clone,
            _stream: stream,
            key_map:map,
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
            });

            ui.heading("ADSR Envelope");
            ui.add(egui::Slider::new(&mut synth.attack, 0.01..=1.0).text("Attack"));
            ui.add(egui::Slider::new(&mut synth.decay, 0.01..=1.0).text("Decay"));
            ui.add(egui::Slider::new(&mut synth.sustain, 0.0..=1.0).text("Sustain"));
            ui.add(egui::Slider::new(&mut synth.release, 0.01..=2.0).text("Release"));

            ui.add(egui::Slider::new(&mut synth.pitch_bend, 0.5..=2.0).text("Pitch Bend"));

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
                            let text_edit = egui::TextEdit::singleline(&mut note_string).desired_width(45.0);
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
            let rows :u8 = 5; // Number of rows
            let cols :u8 = 10; // Number of columns

            for row in 0..rows {
                ui.horizontal(|ui| {
                    for col in 0..cols {
                        // Calculate MIDI note
                        let mut note = col+row*cols ;

                        let response = ui.allocate_response(tile_size, egui::Sense::click_and_drag());

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

                        if response.drag_released()  {
                            synth.note_off(note);
                        }
                    }
                });
            }

        }); 

    ctx.input( |i|
            {
     let mut notes = Vec::new();
    for event in &i.events {
    if let egui::Event::Key{key, pressed, ..} = event {
        //println!("{:?} {:?} {} ", &key, pressed, self.key_map[ &]  );
        {
            if self.key_map.contains_key(&key){
            let freq = self.key_map[key];
            notes.push((freq,pressed));
        }
        }
    }
    

                
    }
        let mut synth = self.synth.lock().unwrap();

        for note in notes{
                    match note {
                        (freq,true) => synth.note_on(freq),
                        (freq,false)=> synth.note_off(freq),
                    }
                }
            
    }   );
        
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
) -> Result<Stream, cpal::BuildStreamError>
{
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
