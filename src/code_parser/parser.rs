#![allow(dead_code)]

use std::{convert::TryInto, fmt::Debug};

use super::lexer::{tokenizer, Token as T};

#[derive(Debug)]
pub struct Axiom {
    pub bpm : u8,
    pub signature : (u8, u8),
    pub blocks : Vec<Block>
}

#[derive(Debug)]
pub enum Block {
    Recursive(RecBlock),
    Instrument(Instrument),
}

#[derive(Debug)]
pub struct RecBlock {
    pub filters : Vec<Filter>,
    pub blocks : Vec<Block>,
}

#[derive(Debug)]
pub struct Instrument {
    pub instrument : String,
    pub filters : Vec<Filter>,
    pub notes : Vec<Note>
}

// TODO : string value
#[derive(Clone, Debug)]
pub struct Filter {
    pub name : String,
    pub value : isize, 
}

#[derive(Debug)]
pub struct Note {
    pub pitch : usize,
}


pub fn parse(code : String) -> Result<Axiom,String> {
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
    expect(T::BpmKw, &tokens[*pointer], pointer)?;

    // BPM value
    let bpm = to_u8(expect_value(&tokens[*pointer], pointer)?)?;
    expect(T::NewLine, &tokens[*pointer], pointer)?;

    // TIME SIGNATURE
    let den : u8 = to_u8(expect_value(&tokens[*pointer], pointer)?)?;
    expect(T::Solidus, &tokens[*pointer], pointer)?;
    let nom : u8 = to_u8(expect_value(&tokens[*pointer], pointer)?)?;
    expect(T::NewLine, &tokens[*pointer], pointer)?;
    let signature = (den,nom);

    let blocks = parse_blocks(pointer, tokens)?;

    Ok(Axiom {bpm, signature, blocks})
}

fn parse_blocks(pointer: &mut usize, tokens: &Vec<T>) -> Result<Vec<Block>,String> {
    let mut blocks= Vec::new();

    'main_loop : loop {
        match tokens.get(*pointer) {
            Some(T::LeftABracket | T::String(_) | T::LeftParenthesis) => blocks.push(parse_block(pointer, tokens)?),
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
        T::LeftABracket | T::LeftParenthesis => Ok(Block::Recursive(parse_recblock(pointer, tokens)?)),
        T::String(_) => Ok(Block::Instrument(parse_instrument(pointer, tokens)?)),
        _ => Err(format!("Expected left angled-bracket or instrument name (block), found {:?}",tokens[*pointer]))
    }
}

fn parse_recblock(pointer : &mut usize, tokens: &Vec<T>) -> Result<RecBlock,String> {
    let filters;
    if tokens[*pointer] == T::LeftABracket {
        filters = parse_filter_list(pointer, tokens)?;
    }
    else {
        filters = vec![];
    }
    expect(T::LeftParenthesis, &tokens[*pointer], pointer)?;
    let blocks = parse_blocks(pointer, tokens)?;

    Ok(RecBlock {filters, blocks})
}

fn parse_instrument(pointer : &mut usize, tokens: &Vec<T>) -> Result<Instrument,String> {
    let instrument = expect_string(&tokens[*pointer],pointer)?;
    let filters;
    if tokens[*pointer] == T::LeftABracket {
        filters = parse_filter_list(pointer, tokens)?;
    }
    else {
        filters = vec![];
    }
    let notes = parse_notes(pointer, tokens)?;
    expect(T::NewLine, &tokens[*pointer], pointer)?;

    Ok(Instrument {filters, instrument, notes})
}

fn parse_notes(pointer : &mut usize, tokens : &Vec<T>) -> Result<Vec<Note>, String> {
    expect(T::LeftParenthesis,&tokens[*pointer], pointer)?;
    let mut notes = Vec::new();

    notes.push(parse_note(pointer, tokens)?);
    while tokens[*pointer] != T::RightParenthesis {
        expect(T::Comma, &tokens[*pointer], pointer)?;
        notes.push(parse_note(pointer,tokens)?);
    }
    *pointer += 1; 
    Ok(notes)
}

fn parse_filter_list(pointer : &mut usize, tokens: &Vec<T>) -> Result<Vec<Filter>,String> {
    
    expect(T::LeftABracket, &tokens[*pointer], pointer)?;

    let mut filter_list = Vec::new();
    'main_loop : while tokens[*pointer] != T::RightABracket {
        filter_list.push(parse_filter(pointer, tokens)?);
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
    let name = expect_string(&tokens[*pointer], pointer)?;
    expect(T::Colon, &tokens[*pointer], pointer)?;
    let value = expect_value(&tokens[*pointer], pointer)?;
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

fn expect(expected_token : T, token : &T, pointer : &mut usize) -> Result<(),String> {
    if expected_token == *token {
        *pointer += 1;
        Ok(())
    }
    else {
        return Err(format!("Expected {:?}, found {:?}", expected_token, token));
    }
}

fn expect_value(token : &T, pointer : &mut usize) -> Result<isize, String> {
    if let T::Value(val) = token {
        *pointer += 1;
        Ok(*val)
    }
    else {
        Err(format!("Expected number value, found {:?}", token))
    }
}

fn expect_string(token : &T, pointer : &mut usize) -> Result<String, String> {
    if let T::String(val) = token {
        *pointer +=1;
        Ok(val.clone())
    }
    else {
        Err(format!("Expected string, found {:?}", token))
    }
}

fn to_u8<A>(x : A) -> Result<u8, String> where A: TryInto<u8>, A: Debug, A: Copy {
    match x.try_into() {
        Ok(a) => Ok(a),
        Err(_) => Err(format!("Cannot convert {:?} to an 8-bit integer.",x)),
    }
}


/* *************TESTS*************** */


#[test]
fn parse_plain() {
    let plain_code = std::fs::read_to_string("./tests/codebase/plain.xfzd")
        .expect("Impossible de lire le fichier");

    let axiom = parse(plain_code).unwrap();
    dbg!(axiom);
}

#[test]
fn parse_simple() {
    let code = std::fs::read_to_string("./tests/codebase/simple_bloc.xfzd")
        .expect("Impossible de lire le fichier");

    let axiom = parse(code).unwrap();
    dbg!(axiom);
}



