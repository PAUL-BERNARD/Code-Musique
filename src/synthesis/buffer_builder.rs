use std::convert::TryInto;
use std::cmp::min;

use crate::code_parser::parser::*;

use super::{AudioBuffer, oscillator::Oscillator, filters::FilterTrait};

struct BarContext {
    buffer : AudioBuffer,
    beat_count : u8,
    beat_duration : u8,
    spb : f32
}

pub fn build_buffer(tree : Axiom) -> Result<AudioBuffer, String> {
    // seconds per beat
    let spb = 60. / (tree.bpm as f32);
    let (beat_count, beat_duration) = tree.signature;
    let buffer : Vec<f32> = vec![0.;(44_000. * spb) as usize * beat_count as usize];

    let mut context = BarContext {
        buffer,
        beat_count,
        beat_duration,
        spb
    };

    play_blocks(&mut context, tree.blocks, &vec![])?;
    Ok(context.buffer)
}

fn play_blocks(context : &mut BarContext, blocks : Vec<Block>, filters : &Vec<Filter>) -> Result<(), String> {
    
    for block in blocks {
        play_block(context, block, &filters)?;
    }
    
    Ok(())
}

fn play_block(context : &mut BarContext, block : Block, filters : &Vec<Filter>) -> Result<(), String> {

    match block {
        Block::Recursive(recursive_block) => play_recursive_block(context, recursive_block, filters)?,
        Block::Instrument(instrument_block) => play_instrument_block(context, instrument_block, filters)?,
    }

    Ok(())
}

fn play_recursive_block(context : &mut BarContext, block : RecBlock, filters : &Vec<Filter>) -> Result<(), String> {
    
    play_blocks(context, block.blocks, &[&filters[..], &block.filters[..]].concat())?;

    Ok(())
}

fn play_instrument_block(context : &mut BarContext, instrument : Instrument, filters : &Vec<Filter>) -> Result<(), String> {
    
    if instrument.notes.len() != context.beat_count.into() {
        return Err(format!("Invalid number of notes. Found {}, expected {}", instrument.notes.len(), context.beat_count));
    }

    for (i, note) in instrument.notes.into_iter().enumerate() {
        play_note(context, note, &instrument.instrument[..], filters, i)?;
    }
    
    Ok(())
}

fn play_note(context : &mut BarContext, note : Note, instrument : &str, filters : &Vec<Filter>, position : usize) -> Result<(), String> {
    let sound = match instrument {
        "simple" => play_a(note)?,
        _ => return Err(format!("Unknown instrument name : {}",instrument))?,
    };

    let sound_ = apply_filters(sound, filters)?;

    insert_sound(context, sound_, position);
    
    Ok(())
}

fn play_a(note : Note) -> Result<AudioBuffer,String> {
    let frequency = pitch_to_frequency(note.pitch);
    
    let buffer = AudioBuffer::square_wave(35_300, frequency);
    println!("playa1 {:?}",&buffer[buffer.len()-100..]);
    let buffer = buffer.low_pass(3.*frequency as f32);
    println!("playa2 {:?}",&buffer[buffer.len()-100..]);
    let buffer = buffer.adsr(0.5, 0.05, 0.1, 0.7, 0.3)?;
    println!("playa3 {:?}",&buffer[buffer.len()-100..]);
    Ok(buffer)
}

fn pitch_to_frequency(pitch : usize) -> usize {
    const DO : f32 = 261.63;
    const LOG_STEP : f32 = 1.0594630943;
    (DO * LOG_STEP.powi((pitch%12).try_into().unwrap())) as usize >> pitch/12
}


fn apply_filters(sound : AudioBuffer, filters : &Vec<Filter>) -> Result<AudioBuffer, String> {
    let mut sound_ : AudioBuffer = sound;
    for filter in filters {
        sound_ = apply_filter(sound_, filter)?;
    }

    Ok(sound_)
}


fn apply_filter(sound : AudioBuffer, filter : &Filter) -> Result<AudioBuffer, String> {
    match &filter.name[..] {
        // Low-pass filter
        "lp" => {
            let cutoff = filter.value;
            if cutoff < 0 {return Err("Cut-off frequency must be positive !".to_string())}
            let sound_ = sound.low_pass(cutoff as f32);
            

            Ok(sound_)
        }
        "echo" => {
            let delta = filter.value;
            if delta < 0 {return Err("Delta value must be positive !".to_string())}
            let sound_ = sound.echo(delta as f32, 0.8);

            Ok(sound_)
        }
        _ => Err(format!("Unknown filter name :  {}", filter.name))
    }
}

fn insert_sound(context : &mut BarContext, sound : AudioBuffer, position : usize) {
    let bufferlen = context.buffer.len();
    let start_sample : usize =  bufferlen * position / context.beat_count as usize;

    for i in 0..min(sound.len(), bufferlen-start_sample) {
        context.buffer[start_sample+i] += sound[i]; 
    }
}