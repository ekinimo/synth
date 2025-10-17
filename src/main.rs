
mod audio;
mod gui;
mod synth;

use crossbeam_channel::bounded;
use std::thread;

fn main() {
    let (audio_sender, audio_receiver) = bounded(1024);

    let audio_thread = thread::spawn(move || {
        audio::run_audio(audio_receiver);
    });

    let options = eframe::NativeOptions::default();
    eframe::run_native(
        "Modular Synth",
        options,
        Box::new(|cc| Ok(Box::new(gui::SynthGui::new(cc, audio_sender)))),
    )
    .unwrap();

    audio_thread.join().unwrap();
}

