use crate::hit::{HitRecord, Hittable, Sphere};
use crate::ray::Ray;
use crate::vec3::Vec3;

#[derive(Clone)]
pub struct HittableTable {
	pub object_list : Vec<Sphere>
}

impl Hittable for HittableTable {
	fn hit(&self, ray: Ray, t_min: f64, t_max: f64) -> HitRecord {
		let mut temp_hit_record : HitRecord = HitRecord {
			parameter: 0.0,
			hit_point: Vec3(0.0, 0.0, 0.0),
			normal: Vec3(0.0, 0.0, 0.0),
			is_hit: false
		};
		let mut closest_so_far = t_max;
		for item_number in 0..(self.list_size()) {
			let mut temp_temp_hit_record = self.object_list[item_number].hit(ray, t_min, closest_so_far);
			if temp_temp_hit_record.is_hit {
				temp_hit_record = temp_temp_hit_record;
				closest_so_far = temp_temp_hit_record.parameter;
			}
		}
		temp_hit_record
	}
}

impl HittableTable {
	pub fn list_size(&self) -> usize { self.object_list.len() }
}