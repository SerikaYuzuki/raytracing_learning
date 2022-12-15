use crate::ray::Ray;
use crate::vec3::Vec3;

pub struct Camera {
	pub lower_left_corner: Vec3,
	pub horizontal: Vec3,
	pub vertical: Vec3,
	pub origin: Vec3,
}

impl Default for Camera {
	fn default() -> Self {
		Camera {
			lower_left_corner: Vec3(-2., -1., -1.),
			horizontal: Vec3(4., 0., 0.),
			vertical: Vec3(0., 2., 0.),
			origin: Vec3(0., 0., 0.),
		}
	}
}

impl Camera {
	pub fn get_ray(&self, u: f64, v: f64) -> Ray {
		Ray { start_point: self.origin, ray_direction: self.lower_left_corner+u*self.horizontal+v*self.vertical-self.origin }
	}
}
