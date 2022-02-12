mod utils;

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
    pub pointer: *const f64,
    pub size : usize
}

#[prelude::wasm_bindgen]
pub fn compile(code : &str) -> Result<PointerAndSize,String> {
    utils::set_panic_hook();
    
    let parsed_code = parse(code)?;
    let audio_buffer = build_audio_buffer(&parsed_code);
    //write_buffer(audio_buffer);

    Ok(PointerAndSize{pointer:audio_buffer.as_ptr(),size:audio_buffer.len()})
}

fn parse(code : &str) -> Result<Instruction,String> {

    match str::parse::<usize>(code) {
        Ok(freq) => Ok(Instruction::Freq(freq)),
        Err(_) => Err(String::from("Impossible d'analyser le code !"))
    }
}

fn build_audio_buffer(parsed_code : &Instruction) -> Vec<f64> {
    let Instruction::Freq(freq) = parsed_code;
    
    let mut buffer = Vec::with_capacity(FREQ_ECH*3);
    let _2pif : f64 = 2f64*std::f64::consts::PI*(*freq as f64);
    for i in 0..FREQ_ECH*3 {
        buffer.push(f64::sin(_2pif*(i as f64/FREQ_ECH as f64)));
    }

    buffer
}