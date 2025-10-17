use crate::synth::{Synth, SynthMessage};
use cpal::traits::{DeviceTrait, HostTrait, StreamTrait};
use crossbeam_channel::Receiver;

pub fn run_audio(receiver: Receiver<SynthMessage>) {
    let host = cpal::default_host();
    let device = host.default_output_device().unwrap();
    let config = device.default_output_config().unwrap();

    let mut synth = Synth::default();

    let stream = device
        .build_output_stream(
            &config.clone().into(),
            move |data: &mut [f32], _: &cpal::OutputCallbackInfo| {
                // Process incoming messages
                while let Ok(msg) = receiver.try_recv() {
                    match msg {
                        SynthMessage::NoteOn(note, _velocity) => synth.note_on(note),
                        SynthMessage::NoteOff(note) => synth.note_off(note),
                        SynthMessage::UpdateOscillator(part) => {synth.oscillator.update(part)},
                        SynthMessage::UpdateModifier(modifiers) => {synth.continous_modifiers.push(modifiers);},
                    }
                }

                // Generate audio
                for frame in data.chunks_mut(config.channels() as usize) {
                    let sample = synth.generate_sample();

                    // Apply master volume and prevent clipping
                    //sample = sample.clamp(-1.0, 1.0);

                    for channel in frame.iter_mut() {
                        *channel = sample;
                    }
                }
            },
            |err| eprintln!("Audio error: {}", err),
            None,
        )
        .unwrap();

    stream.play().unwrap();

    // Keep the audio thread running
    std::thread::park();
}
