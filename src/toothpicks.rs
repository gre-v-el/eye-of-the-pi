use macroquad::prelude::*;

use crate::{HIT_COL, MISS_COL};

pub struct Toothpicks {
	camera: Camera2D,
	target: RenderTarget,
	subdivisions: usize,
	toothpick_length: f32,
	total: usize,
	hits: usize,
}

impl Toothpicks {
	pub fn new(subdivisions: usize, toothpick_length: f32) -> Toothpicks {
		let target = render_target(1000, 1000);
		let mut camera = Camera2D::from_display_rect(Rect { x: 0.0, y: 0.0, w: 1.0, h: 1.0 });
		camera.render_target = Some(target);


		Toothpicks { 
			camera,
			target, 
			subdivisions,
			toothpick_length,
			total: 0, 
			hits: 0 
		}
	}

	pub fn scatter(&mut self, amount: usize) {
		self.total += amount;

		push_camera_state();
		set_camera(&self.camera);

		draw_rectangle(-1.0, -1.0, 2.0, 2.0, Color { r: 0.0, g: 0.0, b: 0.0, a: 1.0 - 0.9999f32.powi(amount as i32) });

		for _ in 0..amount {
			// origin
			let (x0, y0): (f32, f32) = (rand::gen_range(0.0, 1.0), rand::gen_range(0.0, 1.0));
	
			// monte carlo rejection 
			let (mut x1, mut y1) = (rand::gen_range(-1.0, 1.0), rand::gen_range(-1.0, 1.0));
			while x1 == x0 || y1 == y0 || x1*x1 + y1*y1 > 1.0 {
				(x1, y1) = (rand::gen_range(-1.0, 1.0), rand::gen_range(-1.0, 1.0))
			}

			let r = (x1*x1 + y1*y1).sqrt();
			x1 /= r;
			y1 /= r;

			x1 *= self.toothpick_length;
			y1 *= self.toothpick_length;

			x1 += x0;
			y1 += y0;

			
			let cell0 = (x0*self.subdivisions as f32).floor();
			let cell1 = (x1*self.subdivisions as f32).floor();

			let hit = cell0 != cell1;

			if hit {
				self.hits += 1;
			}

			let col = if hit { HIT_COL } else { MISS_COL };

			
			draw_line(x0, y0, x1, y1, self.toothpick_length*0.05, col);
		}
		pop_camera_state();
	}

	pub fn approximate(&self) -> f64 {
		// 2.0 / self.hits as f64 * self.total as f64 * self.toothpick_length as f64 * self.subdivisions as f64
		// 2.0 * self.toothpick_length as f64 * self.total as f64 / self.hits as f64 * self.subdivisions as f64
		2.0 * self.total as f64 / self.hits as f64
	}

	pub fn texture(&self) -> Texture2D {
		push_camera_state();
		set_camera(&self.camera);
		
		for i in 1..self.subdivisions {
			let x = i as f32/self.subdivisions as f32;
			draw_line(x, 0.0, x, 1.0, 0.01, DARKGRAY);
		}
		pop_camera_state();

		self.target.texture
	}

	pub fn total(&self) -> usize {
		self.total
	}

	pub fn hits(&self) -> usize {
		self.hits
	}
}