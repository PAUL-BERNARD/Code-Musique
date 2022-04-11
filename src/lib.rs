#![allow(non_snake_case)]

mod utils;
mod code_parser;

use code_parser::lexer;

use wasm_bindgen::prelude;

// When the `wee_alloc` feature is enabled, use `wee_alloc` as the global
// allocator.
#[cfg(feature = "wee_alloc")]
#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;

#[prelude::wasm_bindgen]
extern {
    fn alert(s: &str);
}

const FREQ_ECH : usize = 44_000;

enum Instruction {
    Freq(usize)
}

#[prelude::wasm_bindgen]
pub struct PointerAndSize {
    pub pointer: *const f32,
    pub size : usize
}

#[prelude::wasm_bindgen]
pub fn compile(code : &str) -> Result<PointerAndSize,String> {
    utils::set_panic_hook();
    
    let parsed_code = parse(code)?;
    let audio_buffer = build_audio_buffer(&parsed_code);

    Ok(PointerAndSize{pointer:audio_buffer.as_ptr(),size:audio_buffer.len()})
}

fn parse(code : &str) -> Result<Instruction,String> {

    match str::parse::<usize>(code) {
        Ok(freq) => Ok(Instruction::Freq(freq)),
        Err(_) => Err(String::from("Impossible d'analyser le code !"))
    }
}

fn build_audio_buffer(parsed_code : &Instruction) -> Vec<f32> {
    
    let Instruction::Freq(freq) = parsed_code;
    
    let mut buffer = Vec::with_capacity(FREQ_ECH*3);
    let _2piTe : f32 = 2f32*std::f32::consts::PI*(*freq as f32)/FREQ_ECH as f32;
    for i in 0..FREQ_ECH*3 {
        buffer.push(f32::sin(_2piTe*(i as f32)));
    }

    buffer
}