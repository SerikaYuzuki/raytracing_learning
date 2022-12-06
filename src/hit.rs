use crate::ray::Ray;
use crate::vec3::{dot, Vec3};

#[derive(Clone, Copy)]
pub struct HitRecord {
	pub parameter: f64,
	pub hit_point: Vec3,
	pub normal: Vec3
}

pub trait Hittable {
	fn hit (self, ray : Ray, t_min : f64, t_max : f64, mut hit_record : HitRecord) -> bool;
}

pub struct Sphere {
	centre : Vec3,
	radius : f64
}

impl Hittable for Sphere {
	fn hit(self, ray: Ray, t_min: f64, t_max: f64, mut hit_record: HitRecord) -> bool {
		let oc = ray.start_point -centre;
		let a = dot(ray.ray_direction, ray.ray_direction);
		let b = dot(oc, ray.ray_direction);
		let c = dot(oc,oc) - radius*radius;
		let discriminant = b*b - a*c;
		if discriminant > 0 {
			let mut temp = ((-b - (b * b - a * c).sqrt()) / a);
			if temp < t_max && temp > t_min {
				hit_record.parameter = temp;
				hit_record.hit_point = ray.end_point(temp);
				hit_record.normal = (hit_record.hit_point - self.centre) / self.radius;
				return true
			}
			temp = ((-b + (b * b - a * c).sqrt()) / a);
			if temp < t_max && temp > t_min {
				hit_record.parameter = temp;
				hit_record.hit_point = ray.end_point(temp);
				hit_record.normal = (hit_record.hit_point - self.centre) / self.radius;
				return true
			}
		}
		return false
	}
}

pub struct HittableTable {
	
}


