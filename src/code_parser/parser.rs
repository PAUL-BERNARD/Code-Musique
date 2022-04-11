#![allow(dead_code)]

use std::convert::TryInto;

use super::lexer::{tokenizer, Token as T};

struct Axiom {
    bpm : u8,
    signature : (u8, u8),
    blocks : Vec<Block>
}

enum Block {
    Recursive(RecBlock),
    Instrument(Instrument),
}

struct RecBlock {
    filters : Vec<Filter>,
    blocks : Vec<Block>,
}

struct Instrument {
    instrument : String,
    filters : Vec<Filter>,
    notes : Vec<Note>
}

struct Filter {
    name : String,
    value : String, 
}

struct Note {
    pitch : u32,
}


fn parse(code : String) {
    let token_list = tokenizer(code);

}

fn syntactical_analysis(tokens : &Vec<T>) -> Result<Axiom,String> {
    let mut pointer : usize = 0;
    parse_axiom(&mut pointer, tokens)
}

fn parse_axiom(pointer : &mut usize, tokens: &Vec<T>) -> Result<Axiom,String> {
    // BPM
    if tokens[*pointer] != T::BpmKw {
        return Err(format!("Expected BPM keyword, found {:?}",tokens[*pointer]));
    }
    *pointer += 1;
    // BPM value
    let mut bpm : u8;
    if let T::Value(val) = tokens[*pointer] {
        bpm = val.try_into().unwrap();
    }
    else {
        return Err(format!("Expected BPM number, found {:?}",tokens[*pointer]));
    }
    *pointer += 1;

    if tokens[*pointer] != T::NewLine {
        return Err(format!("Expected line return, found {:?}",tokens[*pointer]));
    }
    *pointer += 1;

    // TIME SIGNATURE
    let mut signature = (4,4);
    if let T::Value(den) = tokens[*pointer] {
        if tokens[*pointer+1] != T::Solidus {
            match tokens[*pointer+2] {
                T::Value(nom) => {
                    signature = (den.try_into().unwrap(),nom.try_into().unwrap());
                    *pointer += 2;
                    if tokens[*pointer] != T::NewLine {
                        return Err(format!("Expected line return, found {:?}",tokens[*pointer]));
                    }
                    *pointer += 1;
                },
                _ => return Err(format!("Expected signature denominator, found {:?}",tokens[*pointer+2]))
            }
        }
        else {
            return Err(format!("Expected solidus, found {:?}",tokens[*pointer+1]))
        }  
    }


    let mut blocks = Vec::new();

    Ok(Axiom {bpm, signature, blocks})
}

fn parse_blocks(pointer: &mut usize, tokens: &Vec<T>) -> Result<Vec<Block>,String> {
    unimplemented!()
}

fn parse_block(pointer : &mut usize, tokens: &Vec<T>) -> Result<Block,String> {
    unimplemented!()
}

fn parse_recblock(pointer : &mut usize, tokens: &Vec<T>) -> Result<RecBlock,String> {
    unimplemented!()
}

fn parse_instrument(pointer : &mut usize, tokens: &Vec<T>) -> Result<Instrument,String> {
    unimplemented!()
}

fn parse_filter_list(pointer : &mut usize, tokens: &Vec<T>) -> Result<Vec<Filter>,String> {
    unimplemented!()
}

fn parse_filter(pointer : &mut usize, tokens: &Vec<T>) -> Result<Filter,String> {
    unimplemented!()
}

fn parse_note(pointer : &mut usize, tokens: &Vec<T>) -> Result<Note,String> {
    if let T::Value(val) = tokens[*pointer] {
        *pointer += 1;
        let pitch = val.try_into().unwrap();
        return Ok(Note {pitch});
    }
    else {
        return Err(format!("Expected line return, found {:?}",tokens[*pointer]));
    }
}

