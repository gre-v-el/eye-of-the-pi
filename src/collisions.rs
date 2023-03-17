use std::mem::swap;

use macroquad::{shapes::draw_line, prelude::*};

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
	mass_ratio: f64,
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
		if self.finished { return; }

		self.simulation_time += dt;

		while self.simulation_time > self.next_keyframe_time {
			let u1 = (self.next_keyframe.x1 - self.prev_keyframe.x1)/(self.next_keyframe_time - self.prev_keyframe_time);
			let u2 = (self.next_keyframe.x2 - self.prev_keyframe.x2)/(self.next_keyframe_time - self.prev_keyframe_time);
			
			if self.hits % 2 == 0 {
				// https://en.wikipedia.org/wiki/Elastic_collision
	
				let m1 = 1.0;
				let m2 = self.mass_ratio;
	
				let v1 = (m1 - m2)/(m1 + m2)*u1 + 2.0 * m2 / (m1 + m2)*u2;
				let v2 = 2.0 * m1 / (m1 + m2) * u1 + (m2 - m1) / (m1 + m2) * u2;

				let t = -self.next_keyframe.x1 / v1;

				if t < 0.0 {
					self.finished = true;
					return;
				}

				swap(&mut self.prev_keyframe, &mut self.next_keyframe);
				self.next_keyframe = Keyframe { x1: 0.0, x2: self.prev_keyframe.x2 + v2 * t };

				self.prev_keyframe_time = self.next_keyframe_time;
				self.next_keyframe_time += t;
			}
			else {
				let v1 = -u1;
				let v2 =  u2;

				let t = (self.next_keyframe.x2 - self.next_keyframe.x1) / (v1 - v2);

				if t < 0.0 {
					self.finished = true;
					return;
				}

				swap(&mut self.prev_keyframe, &mut self.next_keyframe);
				self.next_keyframe = Keyframe { x1: self.prev_keyframe.x1 + v1 * t, x2: self.prev_keyframe.x2 + v2 * t };

				self.prev_keyframe_time = self.next_keyframe_time;
				self.next_keyframe_time += t;
			}

			self.hits += 1;
		}
	}

	pub fn draw(&self) {
		draw_line(-0.7, -0.9, -0.7, -0.4, 0.01, WHITE);
		draw_line(0.7, -0.4, -0.7, -0.4, 0.01, WHITE);

		let t = (self.simulation_time - self.prev_keyframe_time)/(self.next_keyframe_time - self.prev_keyframe_time);
		let x1 = (t * self.next_keyframe.x1 + (1.0-t) * self.prev_keyframe.x1)/3.0 - 0.7;
		let x2 = (t * self.next_keyframe.x2 + (1.0-t) * self.prev_keyframe.x2)/3.0 - 0.7;

		draw_rectangle(x1 as f32, -0.5, 0.1, 0.1, BLUE);
		draw_rectangle(x2 as f32 + 0.1, -0.6, 0.2, 0.2, BLUE);

		println!("{}", self.hits);
	}
}