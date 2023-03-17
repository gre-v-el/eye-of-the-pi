mod state;
mod darts;
mod ui;
mod toothpicks;

use std::f64::consts::PI;

use macroquad::prelude::*;
use state::State;

const HIT_COL: Color = Color { r: 0.2, g: 0.9, b: 0.2, a: 1.0};
const MISS_COL: Color = Color { r: 0.7, g: 0.7, b: 0.7, a: 1.0};

pub fn error(pi: f64) -> f64 {
	100.0*(pi-PI)/PI
}

#[macroquad::main("eye of the pi")]
async fn main() {

	let mut state = State::Menu;

	loop {
		clear_background(BLACK);

		if let Some(new_state) = state.update() {
			state = new_state;
		}

		next_frame().await;
	}
}
