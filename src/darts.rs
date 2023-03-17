use macroquad::prelude::*;

pub struct Darts {
	camera: Camera2D,
	target: RenderTarget,
	total: usize,
	hits: usize,
}

impl Darts {
	pub fn new() -> Darts {
		let target = render_target(1000, 1000);
		let mut camera = Camera2D::from_display_rect(Rect { x: -1.0, y: -1.0, w: 2.0, h: 2.0 });
		camera.render_target = Some(target);
		Darts { 
			camera,
			target, 
			total: 0, 
			hits: 0 
		}
	}

	pub fn shoot(&mut self, throws: usize) {
		self.total += throws;

		push_camera_state();
		set_camera(&self.camera);

		draw_rectangle(-1.0, -1.0, 2.0, 2.0, Color { r: 0.0, g: 0.0, b: 0.0, a: 1.0 - 0.9999f32.powi(throws as i32) });

		for _ in 0..throws {

			let x = rand::gen_range(-1.0, 1.0);
			let y = rand::gen_range(-1.0, 1.0);

			let hit = x*x + y*y < 1.0;

			if hit {
				self.hits += 1;
			}
			let col = if hit { Color { r: 0.2, g: 0.9, b: 0.2, a: 1.0} } else { Color { r: 0.7, g: 0.7, b: 0.7, a: 1.0} };

			draw_circle(x, y, 0.01, col);
		}
		pop_camera_state();
	}

	pub fn approximate(&self) -> f64 {
		4.0 * self.hits as f64 / self.total as f64
	}

	pub fn texture(&self) -> Texture2D {
		self.target.texture
	}

	pub fn total(&self) -> usize {
		self.total
	}

	pub fn hits(&self) -> usize {
		self.hits
	}
}