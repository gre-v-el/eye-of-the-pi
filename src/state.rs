use macroquad::prelude::*;

use crate::{darts::Darts, ui::*, toothpicks::Toothpicks, error, collisions::Collisions};

pub enum State {
	Menu,
	Darts(Darts, bool, usize),
	Toothpicks(Toothpicks, bool, usize),
	Collisions(Collisions, bool),
}

impl State {
	pub fn update(&mut self) -> Option<State> {
		let mut new_state = None;

		let font = Font::default();
		match self {
			Self::Menu => {
				let font = Font::default();

				let camera = camera_from_rect(Rect { x: -1.0, y: -1.0, w: 2.0, h: 2.0 });
				set_camera(&camera);

				draw_centered_text(vec2(0.0, -0.7), "Eye Of The PI", font, 0.25);

				if button(Rect { x: -0.4, y: -0.5, w: 0.8, h: 0.2 }, DARKGRAY, "Darts", &camera, font, 0.15) {
					new_state = Some(State::Darts(Darts::new(), false, 1000));
				}
				if button(Rect { x: -0.4, y: -0.2, w: 0.8, h: 0.2 }, DARKGRAY, "Toothpicks", &camera, font, 0.15) {
					new_state = Some(State::Toothpicks(Toothpicks::new(5, 0.2), false, 10));
				}
				if button(Rect { x: -0.4, y:  0.1, w: 0.8, h: 0.2 }, DARKGRAY, "Collisions", &camera, font, 0.15) {
					new_state = Some(State::Collisions(Collisions::new(100.0), false));
				}
			}
			Self::Darts(darts, running, amount) => {

				if *running {
					darts.shoot(*amount);
				}

				let camera = camera_from_rect(Rect { x: -1.0, y: -1.0, w: 2.0, h: 2.0 });
				set_camera(&camera);

				let left = -0.45;
				let right = 0.6;

				draw_texture_ex(darts.texture(), -0.5, -0.9, WHITE, DrawTextureParams { dest_size: Some(vec2(1.0, 1.0)), ..Default::default()});

				draw_centered_text_stable(vec2(right, 0.3), format!("{}", darts.total()).as_str(), "00000", font, 0.2);
				draw_centered_text(vec2(left, 0.3), "total:", font, 0.2);

				draw_centered_text_stable(vec2(right, 0.5), format!("{}", darts.hits()).as_str(), "00000", font, 0.2);
				draw_centered_text(vec2(left, 0.5), "hits:", font, 0.2);

				draw_centered_text_stable(vec2(right, 0.7), format!("{:.6}", darts.approximate()).as_str(), "0.00000", font, 0.2);
				draw_centered_text(vec2(left, 0.7), "4*hits/total:", font, 0.2);

				draw_centered_text_stable(vec2(right, 0.9), format!("{:.2}%", error(darts.approximate())).as_str(), "0.00000", font, 0.2);
				draw_centered_text(vec2(left, 0.9), "error:", font, 0.2);

				if button(Rect { x: -0.9, y: -0.9, w: 0.3, h: 0.15 }, DARKGRAY, "Back", &camera, font, 0.1) {
					new_state = Some(State::Menu);
				}
				
				if button(Rect { x: 0.6, y: -0.9, w: 0.3, h: 0.15 }, DARKGRAY, if *running {"Pause"} else {"Play"}, &camera, font, 0.1) {
					*running = !(*running);
				}

				let mut exp = (*amount as f32).log10();
				slider(&mut exp, 0.0, 5.0, vec2(0.6, -0.2), 0.3, DARKGRAY, &camera);
				*amount = 10f32.powf(exp) as usize;
			}
			Self::Toothpicks(toothpics, running, amount) => {
				if *running {
					toothpics.scatter(*amount);
				}

				let camera = camera_from_rect(Rect { x: -1.0, y: -1.0, w: 2.0, h: 2.0 });
				set_camera(&camera);

				draw_texture_ex(toothpics.texture(), -0.5, -0.9, WHITE, DrawTextureParams { dest_size: Some(vec2(1.0, 1.0)), ..Default::default()});

				let left = -0.45;
				let right = 0.6;

				draw_centered_text_stable(vec2(right, 0.3), format!("{}", toothpics.total()).as_str(), "00000", font, 0.2);
				draw_centered_text(vec2(left, 0.3), "total:", font, 0.2);

				draw_centered_text_stable(vec2(right, 0.5), format!("{}", toothpics.hits()).as_str(), "00000", font, 0.2);
				draw_centered_text(vec2(left, 0.5), "hits:", font, 0.2);

				draw_centered_text_stable(vec2(right, 0.7), format!("{:.6}", toothpics.approximate()).as_str(), "0.00000", font, 0.2);
				draw_centered_text(vec2(left, 0.7), "2*total/hits:", font, 0.2);

				draw_centered_text_stable(vec2(right, 0.9), format!("{:.2}%", error(toothpics.approximate())).as_str(), "0.00000", font, 0.2);
				draw_centered_text(vec2(left, 0.9), "error:", font, 0.2);

				if button(Rect { x: -0.9, y: -0.9, w: 0.3, h: 0.15 }, DARKGRAY, "Back", &camera, font, 0.1) {
					new_state = Some(State::Menu);
				}
				
				if button(Rect { x: 0.6, y: -0.9, w: 0.3, h: 0.15 }, DARKGRAY, if *running {"Pause"} else {"Play"}, &camera, font, 0.1) {
					*running = !(*running);
				}

				let mut exp = (*amount as f32).log10();
				slider(&mut exp, 0.0, 5.0, vec2(0.6, -0.2), 0.3, DARKGRAY, &camera);
				*amount = 10f32.powf(exp) as usize;
			}
			Self::Collisions(collisions, running) => {
				if *running {
					collisions.simulate(0.005);
				}
				
				let camera = camera_from_rect(Rect { x: -1.0, y: -1.0, w: 2.0, h: 2.0 });
				set_camera(&camera);

				draw_centered_text(vec2(0.0, 0.5), format!("{}", collisions.hits()).as_str(), font, 0.3);

				if button(Rect { x: -0.9, y: -0.3, w: 0.3, h: 0.15 }, DARKGRAY, "Back", &camera, font, 0.1) {
					new_state = Some(State::Menu);
				}
				
				if button(Rect { x: 0.6, y: -0.3, w: 0.3, h: 0.15 }, DARKGRAY, if *running {"Pause"} else {"Play"}, &camera, font, 0.1) {
					*running = !(*running);
				}

				if button(Rect { x: 0.6, y: -0.1, w: 0.3, h: 0.15 }, DARKGRAY, "reset", &camera, font, 0.1) {
					new_state = Some(Self::Collisions(Collisions::new(collisions.mass_ratio), false));
				}

				let mut exp = ((collisions.mass_ratio).log10() / 2.0).round() as f32;
				slider(&mut exp, 0.0, 6.0, vec2(-0.15, -0.2), 0.3, DARKGRAY, &camera);
				collisions.mass_ratio = 100f64.powi(exp as i32);

				collisions.draw(&camera)
			}
			_ => {}
		}

		new_state
	}
}