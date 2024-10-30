use std::{path::Path, vec};

use domain::{
    pure_tone::{PureTones, ToneAndDuration},
    text2track::Text2Track,
    text_analyzer::TextAnalyzer,
};

mod domain;
mod infrastructure;
use infrastructure::export_wav::export_wav;

mod utils;

use wasm_bindgen::prelude::*;

#[wasm_bindgen]
extern "C" {
    fn alert(s: &str);
}

#[wasm_bindgen]
pub fn greet() {
    alert("Hello, wasm-game-of-life!");
}
