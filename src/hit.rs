use crate::ray::Ray;
use crate::vec3::{dot, Vec3};

#[derive(Clone, Copy)]
pub struct HitRecord {
	pub parameter: f64,
	pub hit_point: Vec3,
	pub normal: Vec3,
	pub is_hit: bool
}

pub trait Hittable {
	fn hit (&self, ray : Ray, t_min : f64, t_max : f64) -> HitRecord;
}

#[derive(Clone, Copy)]
pub struct Sphere {
	pub centre : Vec3,
	pub radius : f64
}

impl Hittable for Sphere {
	fn hit(&self, ray: Ray, t_min: f64, t_max: f64) -> HitRecord {
		let mut temp_hit_record : HitRecord = HitRecord {
			parameter: 0.0,
			hit_point: Vec3(0.0, 0.0, 0.0),
			normal: Vec3(0.0, 0.0, 0.0),
			is_hit: false
		};
		let oc = ray.start_point - self.centre;
		let a = dot(ray.ray_direction, ray.ray_direction);
		let b = dot(oc, ray.ray_direction);
		let c = dot(oc,oc) - self.radius*self.radius;
		let discriminant = b*b - a*c;
		if discriminant > 0. {
			let temp1 = (-b - (b * b - a * c).sqrt()) / a;
			if temp1 < t_max && temp1 > t_min {
				temp_hit_record.parameter = temp1;
				temp_hit_record.hit_point = ray.end_point(temp1);
				temp_hit_record.normal = (temp_hit_record.hit_point - self.centre) / self.radius;
				temp_hit_record.is_hit =true;
				return temp_hit_record;
			}
			let temp2 = (-b + (b * b - a * c).sqrt()) / a;
			if temp2 < t_max && temp2 > t_min {
				temp_hit_record.parameter = temp2;
				temp_hit_record.hit_point = ray.end_point(temp2);
				temp_hit_record.normal = (temp_hit_record.hit_point - self.centre) / self.radius;
				temp_hit_record.is_hit =true;
				return temp_hit_record;
			}
		}
		return temp_hit_record;
	}
}




