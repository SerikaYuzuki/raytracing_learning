mod vec3;
mod ray;
mod hit;
mod hittable_table;

use crate::vec3::{Vec3};
use crate::ray::{Ray};
use std::fs::File;
use std::io::{Write};
use crate::hit::{Hittable, Sphere};
use crate::hittable_table::HittableTable;


fn main() -> std::io::Result<()> {

	/*
	Output ppm file
	 */
	let path = "lines.ppm";
	let mut output = File::create(path)?;

	/*
	Initial ppm set
	 */
	let nx = 200;
	let ny = 100;
	write!(output, "P3\n{} {}\n255\n", nx, ny)?;

	/*
	Set Canvas
	 */
	let lower_left_corner = Vec3(-2., -1., -1.);
	let horizontal = Vec3(4., 0., 0.);
	let vertical = Vec3(0., 2., 0.);
	let origin = Vec3(0., 0., 0.);

	/*
	Setup Objects
	 */
	let sphere1 = Sphere{
		centre : Vec3(0.,0.,-1.),
		radius : 0.5};
	let sphere2 = Sphere{
		centre : Vec3(0.,-100.5,-1.),
		radius : 100.};
	let mut hittable_table : HittableTable = HittableTable { object_list: vec![] };
	hittable_table.object_list.push(sphere1);
	hittable_table.object_list.push(sphere2);

	/*
	Generate Raytracing
	 */
	for j in (0..ny).rev() {
		for i in 0..nx {
			let u = (i as f64) / (nx as f64);
			let v = (j as f64) / (ny as f64);
			let ray = Ray { start_point: origin, ray_direction: lower_left_corner + u * horizontal + v * vertical };

			let col = color(ray, &hittable_table);
			let ir = (255.99 * col.0) as i64;
			let ig = (255.99 * col.1) as i64;
			let ib = (255.99 * col.2) as i64;

			write!(output, "{} {} {}\n", ir, ig, ib)?;
		}
	}

	/*
	Path checker
	 */
	Ok(())
}

fn color(ray: Ray, hittable_table : &HittableTable) -> Vec3 {
	let temp_hit_record = hittable_table.hit(ray, 0., 1000.);
	if temp_hit_record.is_hit {
		let normal = temp_hit_record.normal;
		0.5*Vec3(normal.0 +1.,normal.1+1.,normal.2+1.)
	}else {
		let unit_direction = ray.ray_direction.unit_vec();
		let t = 0.5 * (unit_direction.y() + 1.);
		(1.0 - t) * Vec3(1., 1., 1.) + t * Vec3(0.5, 0.7, 1.)
	}
}

