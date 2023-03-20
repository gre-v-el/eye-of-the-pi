use std::mem::swap;

use macroquad::{shapes::draw_line, prelude::*};

use crate::ui::draw_centered_text;

pub struct Keyframe {
	x1: f64,
	x2: f64,
}

pub struct Collisions {
	hits: usize,
	simulation_time: f64,
	prev_keyframe_time: f64,
	next_keyframe_time: f64,
	prev_keyframe: Keyframe,
	next_keyframe: Keyframe,
	pub mass_ratio: f64,
	finished: bool,
}

impl Collisions {
	pub fn new(mass_ratio: f64) -> Self {
		Self { 
			hits: 0,
			simulation_time: 0.0, 
			prev_keyframe_time: 0.0, 
			next_keyframe_time: 1.0, 
			prev_keyframe: Keyframe { x1: 1.0, x2: 2.0 }, 
			next_keyframe: Keyframe { x1: 1.0, x2: 1.0 }, 
			mass_ratio,
			finished: false,
		}
	}

	pub fn simulate(&mut self, dt: f64) {
		self.simulation_time += dt;

		while self.simulation_time > self.next_keyframe_time && !self.finished {
			let u1 = (self.next_keyframe.x1 - self.prev_keyframe.x1)/(self.next_keyframe_time - self.prev_keyframe_time);
			let u2 = (self.next_keyframe.x2 - self.prev_keyframe.x2)/(self.next_keyframe_time - self.prev_keyframe_time);
			
			if self.hits % 2 == 0 {
				// https://en.wikipedia.org/wiki/Elastic_collision
	
				let m1 = 1.0;
				let m2 = self.mass_ratio;
	
				let v1 = (m1 - m2)/(m1 + m2)*u1 + 2.0 * m2 / (m1 + m2)*u2;
				let v2 = 2.0 * m1 / (m1 + m2) * u1 + (m2 - m1) / (m1 + m2) * u2;

				let mut t = -self.next_keyframe.x1 / v1;

				if t < 0.0 {
					t = 1.0;
					
					self.finished = true;
				}

				swap(&mut self.prev_keyframe, &mut self.next_keyframe);
				self.next_keyframe = Keyframe { x1: self.prev_keyframe.x1 + t * v1, x2: self.prev_keyframe.x2 + v2 * t };

				self.prev_keyframe_time = self.next_keyframe_time;
				self.next_keyframe_time += t;
			}
			else {
				let v1 = -u1;
				let v2 =  u2;

				let mut t = (self.next_keyframe.x2 - self.next_keyframe.x1) / (v1 - v2);

				if t < 0.0 {
					t = 1.0;

					self.finished = true;
				}

				swap(&mut self.prev_keyframe, &mut self.next_keyframe);
				self.next_keyframe = Keyframe { x1: self.prev_keyframe.x1 + v1 * t, x2: self.prev_keyframe.x2 + v2 * t };

				self.prev_keyframe_time = self.next_keyframe_time;
				self.next_keyframe_time += t;
			}

			self.hits += 1;
		}
	}

	pub fn draw(&self, camera: &Camera2D) {
		let font = Font::default();

		let left = camera.screen_to_world(vec2(20.0, 0.0)).x;
		let right = camera.screen_to_world(vec2(screen_width(), 0.0)).x;
		draw_line(left, -0.9, left, -0.4, 0.01, WHITE);
		draw_line(left, -0.4, right, -0.4, 0.01, WHITE);

		let t = (self.simulation_time - self.prev_keyframe_time)/(self.next_keyframe_time - self.prev_keyframe_time);
		let x1 = (t * self.next_keyframe.x1 + (1.0-t) * self.prev_keyframe.x1)/3.0 + left as f64;
		let x2 = (t * self.next_keyframe.x2 + (1.0-t) * self.prev_keyframe.x2)/3.0 + left as f64;

		let x = self.mass_ratio.log10() as f32;
		let size_ratio = 4.5 * (1.0 + 1.0/(-0.2*x-1.0)) + 1.0;
		draw_rectangle(x1 as f32, -0.5, 0.1, 0.1, BLUE);
		draw_rectangle(x2 as f32 + 0.1, -0.4-0.1*size_ratio, 0.1*size_ratio, 0.1*size_ratio, ORANGE);

		draw_centered_text(vec2(x1 as f32 + 0.05, -0.45), "1", font, 0.1);
		draw_centered_text(vec2(x2 as f32+0.1 + 0.05*size_ratio, -0.4-0.05*size_ratio), format!("{}", self.mass_ratio).as_str(), font, 0.1);
	}

	pub fn hits(&self) -> usize {
		self.hits
	}
}