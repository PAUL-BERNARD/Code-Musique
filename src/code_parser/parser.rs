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

// TODO : string value
struct Filter {
    name : String,
    value : isize, 
}

struct Note {
    pitch : u32,
}


fn parse(code : String) -> Result<Axiom,String> {
    let token_list = tokenizer(code)?;
    let axiom = syntactical_analysis(&token_list)?;

    Ok(axiom)
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
    let mut blocks= Vec::new();

    'main_loop : loop {
        match tokens.get(*pointer) {
            Some(T::LeftABracket | T::String(_)) => blocks.push(parse_block(&mut pointer, tokens)?),
            Some(T::RightParenthesis) => break 'main_loop,
            Some(T::NewLine) => *pointer += 1,
            None => break 'main_loop,
            Some(x) => return Err(format!("Expected '<', instrument name or end of file, found {:?}",*x))
        }
    }

    Ok(blocks)
}

fn parse_block(pointer : &mut usize, tokens: &Vec<T>) -> Result<Block,String> {
    match tokens[*pointer] {
        T::LeftABracket => Ok(Block::Recursive(parse_recblock(pointer, tokens)?)),
        T::String(_) => Ok(Block::Instrument(parse_instrument(pointer, tokens)?)),
        _ => Err(format!("Expected left angled-bracket or instrument name (block), found {:?}",tokens[*pointer]))
    }
}

fn parse_recblock(pointer : &mut usize, tokens: &Vec<T>) -> Result<RecBlock,String> {
    let filters = parse_filter_list(&mut pointer, tokens)?;
    expect(T::LeftParenthesis, tokens[*pointer], &mut pointer)?;
    let blocks = parse_blocks(&mut pointer, tokens)?;

    Ok(RecBlock {filters, blocks})
}

fn parse_instrument(pointer : &mut usize, tokens: &Vec<T>) -> Result<Instrument,String> {
    let instrument = expect_string(tokens[*pointer],&mut pointer)?;
    let filters = parse_filter_list(&mut pointer, tokens)?;
    let notes = parse_notes(&mut pointer, tokens)?;
    expect(T::NewLine, tokens[*pointer], &mut pointer)?;

    Ok(Instrument {filters, instrument, notes})
}

fn parse_notes(pointer : &mut usize, tokens : &Vec<T>) -> Result<Vec<Note>, String> {
    expect(T::LeftParenthesis,tokens[*pointer], &mut pointer)?;
    let mut note : Note;
    let notes = Vec::new();
    'main_loop : loop {
        if let Ok(note) = parse_note(&mut pointer, tokens) {
            notes.push(note);
        }
        else {
            expect(T::RightParenthesis, tokens[*pointer], &mut pointer)?;
            break 'main_loop;
        }
    }
    Ok(notes)
}

fn parse_filter_list(pointer : &mut usize, tokens: &Vec<T>) -> Result<Vec<Filter>,String> {
    if tokens[*pointer] != T::LeftABracket {
        return Err(format!("Expected left angled-bracket, found {:?}",tokens[*pointer]));
    }
    *pointer += 1;

    let mut filter_list = Vec::new();
    // TODO (moyen sûr)
    'main_loop : while tokens[*pointer] != T::RightABracket {
        filter_list.push(parse_filter(&mut pointer, tokens)?);
        if tokens[*pointer] != T::Comma {
            if tokens[*pointer] == T::RightABracket {
                break 'main_loop;
            }
            return Err(format!("Expected comma or right angled-bracket, found {:?}",tokens[*pointer]));
        }
        *pointer += 1;
    }
    *pointer += 1;

    Ok(filter_list)
}

fn parse_filter(pointer : &mut usize, tokens: &Vec<T>) -> Result<Filter,String> {
    let name = expect_string(tokens[*pointer], &mut pointer)?;
    expect(T::Colon, tokens[*pointer], &mut pointer)?;
    let value = expect_value(tokens[*pointer], &mut pointer)?;
    Ok(Filter {name, value})
}

fn parse_note(pointer : &mut usize, tokens: &Vec<T>) -> Result<Note,String> {
    if let T::Value(val) = tokens[*pointer] {
        *pointer += 1;
        let pitch = val.try_into().unwrap();
        return Ok(Note {pitch});
    }
    else {
        return Err(format!("Expected note, found {:?}",tokens[*pointer]));
    }
}

fn expect(expected_token : T, token : T, pointer : &mut usize) -> Result<(),String> {
    if expected_token == token {
        *pointer += 1;
        Ok(())
    }
    else {
        return Err(format!("Expected {:?}, found {:?}", expected_token, token));
    }
}

fn expect_value(token : T, pointer : &mut usize) -> Result<isize, String> {
    if let T::Value(val) = token {
        *pointer += 1;
        Ok(val)
    }
    else {
        Err(format!("Expected number value, found {:?}", token))
    }
}

fn expect_string(token : T, pointer : &mut usize) -> Result<String, String> {
    if let T::String(val) = token {
        *pointer +=1;
        Ok(val)
    }
    else {
        Err(format!("Expected string, found {:?}", token))
    }
}