#![feature(hash_drain_filter)]
#[macro_use] pub mod utils;
mod draw;
mod points;
mod game;

use wasm_bindgen::prelude::*;

#[wasm_bindgen(start)]
pub fn start() {
    utils::set_panic_hook();
    game::start();
}

#[wasm_bindgen]
extern {
    fn alert(s: &str);
    fn refresh(v: Vec<i32>);
}