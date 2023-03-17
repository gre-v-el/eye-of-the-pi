mod state;
mod darts;
mod ui;
mod toothpicks;

use macroquad::prelude::*;
use state::State;

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
