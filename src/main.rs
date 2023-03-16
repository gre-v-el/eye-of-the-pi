mod state;
mod darts;

use macroquad::prelude::*;
use state::State;

#[macroquad::main("eye of the pi")]
async fn main() {

	let state = State::Menu;

	loop {
		clear_background(BLACK);
		next_frame().await;
	}
}
