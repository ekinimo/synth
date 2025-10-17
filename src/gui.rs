use std::collections::HashMap;

use crate::synth::{
    oscillator::{
        square::Square, terraced::*, triangle::{LeftSawtooth, ModifiableTriangle, RightSawtooth}, trig::Sine, OscillatorModification}, Oscillator, OscillatorControls, OscillatorCtx, Synth, SynthMessage
};
use crossbeam_channel::Sender;
use egui::{Color32, Context, ScrollArea, Stroke};
use egui_plot::{Line, Plot, PlotPoints};


#[derive(PartialEq, Clone, Copy)]
enum OscillatorType {
    Sine,
    Square,
    Triangle,
    LeftSaw,
    RightSaw,
    TerracedTriangle,
    TerracedLeftSaw,
    TerracedRightSaw,
}

pub struct SynthGui {
    audio_sender: Sender<SynthMessage>,
    key_map: HashMap<egui::Key, u8>,

    synth:Synth



}

impl SynthGui {


    pub fn new(_cc: &eframe::CreationContext<'_>, audio_sender: Sender<SynthMessage>) -> Self {
        let keyboard = [
            "zxcvbnm,./",
            "asdfghjkl;'\\",
            "qwertyuiop[]",
            "`1234567890-=",
        ];
        let map: HashMap<egui::Key, u8> = keyboard
            .into_iter()
            .enumerate()
            .flat_map(move |(cnt, s)| {
                s.chars()
                    .map(move |x| egui::Key::from_name(&format!("{x}")).unwrap())
                    .enumerate()
                    .map(move |(i, d)| (d, (i + cnt * 5) as u8))
            })
            .collect();
        Self {
            audio_sender,
            key_map: map,
            synth: Synth::default()
        }
    }
}

use crate::synth::Modifiers;
impl eframe::App for SynthGui {
    fn update(&mut self, ctx: &Context, _frame: &mut eframe::Frame) {
        egui::CentralPanel::default().show(ctx, |ui| {
            ui.heading("Modular Synth");
            let  mods :Vec< _> = self.synth.continous_modifiers.iter_mut().map(move |x| x.update()).collect();
            self.synth.run_modifiers(mods);
            ScrollArea::both().show(ui,|ui| {
                ui.horizontal(|ui| {
                    match self.synth.oscillator.draw_controls(ui) {
                        Some(OscillatorModification::AddModifier(modif))=>{
                            self.audio_sender.send(SynthMessage::UpdateModifier(modif)).expect("failed")
                        }
                        Some(msg) => {
                            self.synth.oscillator.update(msg.clone());
                            self.audio_sender.send(SynthMessage::UpdateOscillator(msg)).expect("failed")
                        },
                        None => {},
                    };
                    // self.synth.adsr.draw_controls(ui,self.audio_sender.clone());
                });});

            
            /*ui.heading("Keyboard-to-Note Mapping");
            let rows = [
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
                            let text_edit = egui::TextEdit::singleline(&mut note_string)
                                .desired_width(45.0);
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
             */
        });

        
        // Handle keyboard input
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
            for note in notes {
                match note {
                    (freq, true) => self
                        .audio_sender
                        .send(SynthMessage::NoteOn(freq, 1.0))
                        .unwrap(),
                    (freq, false) => {
                        self.audio_sender.send(SynthMessage::NoteOff(freq)).unwrap()
                    }
                }
            }
        });
    }
}
