mod code_parser;
mod synthesis;
mod utils;

use synthesis::buffer_builder::build_buffer;
use wasm_bindgen::prelude;

const SAMPLE_RATE: f32 = 44_000.;

// When the `wee_alloc` feature is enabled, use `wee_alloc` as the global
// allocator.
#[cfg(feature = "wee_alloc")]
#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;

#[prelude::wasm_bindgen]
extern "C" {
    fn alert(s: &str);
}

#[prelude::wasm_bindgen]
pub struct PointerAndSize {
    pub pointer: *const f32,
    pub size: usize,
}

#[prelude::wasm_bindgen]
pub fn compile(code: &str) -> Result<PointerAndSize, String> {
    utils::set_panic_hook();
    let parsed_code = code_parser::parser::parse(code.to_string())?;
    let audio_buffer = build_buffer(parsed_code)?;

    Ok(PointerAndSize {
        pointer: audio_buffer.as_ptr(),
        size: audio_buffer.len(),
    })
}


/* ********* TESTS ********** */

#[allow(dead_code)]
fn compile_test(code : &str) -> Result<Vec<f32>, String> {
    let parsed_code = code_parser::parser::parse(code.to_string())?;
    let audio_buffer = build_buffer(parsed_code)?;
    
    Ok(audio_buffer)
}


#[test]
fn compile_plain() {
    let code = std::fs::read_to_string("./tests/codebase/plain.xfzd")
        .expect("Impossible de lire le fichier");

    let buffer = compile_test(&code).unwrap();
    dbg!(buffer);
}

#[test]
fn compile_simple() {
    let code = std::fs::read_to_string("./tests/codebase/simple_bloc.xfzd")
        .expect("Impossible de lire le fichier");

    compile_test(&code).unwrap();
}