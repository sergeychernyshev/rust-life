use rusty_life_lib::GameOfLife;
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
pub fn rusty_life() -> String {
    let game = GameOfLife::new(20, 20);
    return game.to_string();
}
