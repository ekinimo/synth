pub mod effect;
pub mod envelope;
pub mod oscillator;

use core::f32;



pub type Note = u8;

pub struct OscillatorIdx<'a>(&'a Synth,usize);
pub struct EffectIdx(usize);

pub struct ModifierIdx<'a>(& 'a mut Synth,usize);

/*impl <'a> Oscillator for OscillatorIdx<'a>{
    fn generate_sample(& self, ctx: OscillatorCtx, phase: f32) -> f32 {
        let OscillatorIdx(synth,osc_id) = self;
        synth.oscillators[*osc_id].generate_sample(ctx,phase)
    }
}
*/

#[derive(Debug,Copy,Clone,PartialEq,PartialOrd )]
pub struct OscillatorCtx {
    pub amplitude: f32,
    pub freq: f32,
    pub phase: f32,
    pub sampling_rate: usize,
}

impl OscillatorCtx {
    pub fn from_freq_and_amp(freq: f32, amplitude: f32) -> Self {
        Self {
            amplitude,
            freq,
            phase: 0.0,
            sampling_rate: 44100,
        }
    }
}

pub enum SynthMessage {
    NoteOn(Note, f32), // note, velocity
    NoteOff(Note),
    UpdateOscillator(OscillatorModification),
    UpdateModifier(OscillatorModifiers),
}


pub trait Oscillator: Send + Sync {
    fn generate_sample(& self, ctx: OscillatorCtx, phase: f32) -> f32;
    //fn get_idx(&self)->OscillatorIdx;
}

pub trait Effect: Send + Sync {
    fn process(&mut self, sample: f32) -> f32;
    fn get_idx(&self)->EffectIdx;
}


pub trait Modifiers : Send + Sync {
    fn update(&mut self ) -> OscillatorModification;
    
}


/*pub trait OscillatorControls {
    fn draw_controls(&mut self, ui: &mut Ui, sender: Sender<SynthMessage>);
}*/

pub trait OscillatorControls {
    fn draw_controls(&mut self, ui: &mut Ui) -> Option<OscillatorModification>;
}


pub trait AdsrControls {
    fn draw_controls(&mut self, ui: &mut Ui, sender: Sender<SynthMessage>);
}

use crossbeam_channel::Sender;
use effect::EffectStack;
use egui::Ui;
use envelope::ADSR;
use oscillator::{square::SquareDutyCycler, triangle::TriangleDutyCycler, trig::{ Trig}, OscillatorModification, Simple};


pub struct Synth {
    pub volume: f32,
    pub active_notes: [Option<u32>;256], //[note]->ticks_passed
    pub released_notes: [Option<u32>;256],
    pub sampling_rate: usize,

    //For Note Generation
    // Basically freq_of_note = lowest_freq * (note / octave_scale)^num_of_notes
    pub lowest_freq: f32,  //440 Hz
    pub octave_scale: f32, //2
    pub num_of_notes: u8,  //12

    pub oscillator: Simple,

    pub effects: EffectStack,

    pub continous_modifiers : Vec<OscillatorModifiers>,
    pub adsr: ADSR,

    pub loops:[Vec<f32>;10],
    pub recording_idx:Option<u8>,
    pub playing_idx:[Option<usize>;10],
}



#[derive(Debug,Clone,PartialEq, PartialOrd)]
pub enum OscillatorModifiers{
    SquareDutyCycler(SquareDutyCycler),
    TriangleDutyCycler(TriangleDutyCycler),
}

impl Modifiers for OscillatorModifiers{
     fn update(&mut self ) -> OscillatorModification {
        match self {
            OscillatorModifiers::SquareDutyCycler(square_duty_cycler) => {
                //let (a,b) = square_duty_cycler.update();
                //(OscillatorModifiers::SquareDutyCycler(a),b)
                square_duty_cycler.update()
            },
            OscillatorModifiers::TriangleDutyCycler(triangle_duty_cycler) => {
                //let (a,b) = triangle_duty_cycler.update();
                //(OscillatorModifiers::TriangleDutyCycler(a),b)
                //square_duty_cycler.update()
                triangle_duty_cycler.update()
            }
        }
    }
}

impl Synth {
    fn new() -> Self {
        Self {
            volume: 1.0,
            active_notes: [None;256],
            released_notes: [None;256],
            sampling_rate: 44100,

            lowest_freq: 440.0, //440 Hz
            octave_scale: 2.0,  //2
            num_of_notes: 12,   //12

            oscillator: Simple::Trig(Trig::Sine(Sine)),
            effects: EffectStack{stack:vec![]},
            continous_modifiers: Vec::default(),
            //oscillators: Vec::default(),
            adsr: ADSR::new(44100),
            loops : [const { vec![] };10],
            recording_idx:None,
            playing_idx:[None;10]
        }
    }

    

    pub fn run_modifiers(&mut self, mods:Vec<OscillatorModification>){
        mods.into_iter().for_each(|part| self.oscillator.update(part));
    }

    pub fn calculate_freq(&self, note: Note) -> f32 {
        self.lowest_freq
            * self
                .octave_scale
                .powf(note as f32 / self.num_of_notes as f32)
    }

    pub fn note_on(&mut self, note: u8) {
        if self.active_notes[note as usize].is_some() //|| self.released_notes[note as usize].is_some()
        {
            return;
        }
        //dbg!("on");
        //let adsr = self.adsr.into();
        self.active_notes[note as usize] = Some(0);
    }

    pub fn note_off(&mut self, note: u8) {
        //dbg!("off");
        self.active_notes[note as usize] = None;
        self.released_notes[note as usize] = Some(0);
    }


    
    pub fn generate_sample(&mut self) -> f32 {
        self.released_notes
            .iter_mut()
            .for_each(|tick| *tick = match tick {
                Some(x) if !self.adsr.should_release(*x) => {
                    
                    Some(*x)},
                _ =>{ None},
            })
            ;
        /*self.active_notes.retain(|_, envelope| {
            !envelope.is_released
                || envelope.release_time.unwrap().elapsed().as_secs_f32() < envelope.release
        });*/

        //let cm :Vec<_> = self.continous_modifiers.iter().map(|x| x.clone()).clone().collect();

        

        let mut sample = if self.active_notes.is_empty() && self.released_notes.is_empty()  {
            0.0
        } else {


            let active = self.active_notes
                .iter()
                .enumerate()
                .filter_map(|(note,tick)| match tick {
                    Some(x) => Some((note as Note,x)),
                    None => None,
                })
                .map(|(note, tick)| (tick,self.calculate_freq(note), self.adsr.get_amplitude(*tick)))
                .map(|(tick,freq, amp)| (tick,OscillatorCtx::from_freq_and_amp(freq, amp)))
                .map(|(tick,params)| {
                    let time = *tick as f32 /self.sampling_rate as f32;
                    (time,params)
                })
                .map(|(time,params)| self.oscillator.generate_sample(params, time))
                .sum::<f32>()
                ;

                let passive = self.released_notes
                .iter()
                .enumerate()
                .filter_map(|(note,tick)| match tick {
                    Some(x) => Some((note as Note,x)),
                    None => None,
                })
                .map(|(note, tick)| (tick,self.calculate_freq(note), self.adsr.get_amplitude_after_release(*tick)))
                .map(|(tick,freq, amp)| (tick,OscillatorCtx::from_freq_and_amp(freq, amp)))
                .map(|(tick,params)| {
                    let time = *tick as f32 /self.sampling_rate as f32;
                    (time,params)
                })
                .map(|(time,params)| self.oscillator.generate_sample(params, time))
                .sum::<f32>()
                ;

            (active + passive) / 2.0
        };

        sample = self.effects.calculate(sample);


        if let Some(track) = self.recording_idx{
            self.loops[track as usize].push(sample);
        }

        let mut voice_sample = 0.0;
        let mut voice_count = 1.0;
        for (track,id) in self.playing_idx.iter().enumerate(){
            if let Some(id) = id{
                let current_loop = &self.loops[track];
                voice_sample+=current_loop[*id];
                voice_count +=1.0;
            }
        }

        sample = (sample + voice_sample)/voice_count;

        self.active_notes
            .iter_mut()
            .chain(self.released_notes.iter_mut())
            .filter_map(|tick| match tick {
                Some(x) => Some(x),
                None => None,
            }).for_each(|tick| {
                *tick += 1  ;
                *tick = *tick  % (4*self.sampling_rate as u32);
            });

        

        let vol = self.volume;
        let  mods :Vec< _> = self.continous_modifiers.iter_mut().map(move |x| x.update()).collect();
        self.run_modifiers(mods);
        sample*vol
    }
}

use crate::synth::oscillator::trig::Sine;

impl Default for Synth {
    fn default() -> Self {
        Synth::new()
    }
}
