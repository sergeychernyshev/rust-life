use rusty_life_lib::GameOfLife;
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
pub fn rusty_life() -> String {
    let mut game = GameOfLife::new(20, 20);
    game.randomize();
    return game.to_string();
}
