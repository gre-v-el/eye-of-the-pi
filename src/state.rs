use macroquad::prelude::*;

use crate::{darts::Darts, ui::*};

pub enum State {
	Menu,
	Darts(Darts),
}

impl State {
	pub fn update(&mut self) -> Option<State> {
		let mut new_state = None;

		let font = Font::default();
		match self {
			Self::Menu => {
				let camera = camera_from_rect(Rect { x: -1.0, y: -1.0, w: 2.0, h: 2.0 });
				set_camera(&camera);
				if button(Rect { x: -0.3, y: -0.5, w: 0.6, h: 0.2 }, DARKGRAY, "Darts", &camera, font, 0.15) {
					new_state = Some(State::Darts(Darts {  }));
				}
			}
			_ => {}
		}

		new_state
	}
}