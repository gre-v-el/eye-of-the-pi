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

		set_camera(&self.camera);

		for _ in 0..throws {
			let x = rand::gen_range(0.0, 1.0);
			let y = rand::gen_range(0.0, 1.0);

			let hit = x*x + y*y <= 1.0;

			if hit {
				self.hits += 1;
			}
			let col = if hit { Color { r: 0.2, g: 0.9, b: 0.2, a: 0.3} } else { Color { r: 0.7, g: 0.7, b: 0.7, a: 0.3} };

			draw_circle(x, y, 0.05, col);
		}
	}

	pub fn approximate(&self) -> f64 {
		4.0 * self.hits as f64 / self.total as f64
	}
}