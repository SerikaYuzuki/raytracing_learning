mod vec3;
mod ray;
mod hit;
mod hittable_table;
mod camera;

use crate::vec3::{dot, Vec3};
use crate::ray::{Ray};
use std::fs::File;
use std::io::{Write};
use crate::camera::Camera;
use crate::hit::{Hittable, Sphere};
use crate::hittable_table::HittableTable;
use rand::Rng;


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
	let ns = 16;
	write!(output, "P3\n{} {}\n255\n", nx, ny)?;

	/*
	Set Canvas
	 */
	let camera: Camera = Default::default();

	/*
	Setup Objects
	 */
	let sphere1 = Sphere {
		centre: Vec3(0., 0., -1.),
		radius: 0.5,
	};
	let sphere2 = Sphere {
		centre: Vec3(0., -100.5, -1.),
		radius: 100.,
	};
	let mut hittable_table: HittableTable = HittableTable { object_list: vec![] };
	hittable_table.object_list.push(sphere1);
	hittable_table.object_list.push(sphere2);

	/*
	Generate Raytracing
	 */
	for j in (0..ny).rev() {
		for i in 0..nx {
			let mut col = Vec3(0., 0., 0.);
			for _s in 0..ns {
				let mut rng = rand::thread_rng();
				let du : f64 = rng.gen();
				let dv : f64 = rng.gen();
				let u = ((i as f64) + du) / (nx as f64);
				let v = ((j as f64) + dv) / (ny as f64);
				let ray = camera.get_ray(u, v);
				col += color(ray, &hittable_table);
			}
			col /= ns as f64;
			col = Vec3(col.0.sqrt(),col.1.sqrt(),col.2.sqrt());
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

fn random_in_unit_sphere() -> Vec3{
	let mut point = Vec3::zero_vec();
	while dot(&point,&point) <= 1. {
		let mut rng = rand::thread_rng();
		let du : f64 = rng.gen();
		let dv : f64 = rng.gen();
		let dw : f64 = rng.gen();
		point = 2.*Vec3(du,dv,dw)-Vec3(1.,1.,1.)
	}
	point
}

fn color(ray: Ray, hittable_table: &HittableTable) -> Vec3 {
	let temp_hit_record = hittable_table.hit_result(ray, 0., 100.);
	if temp_hit_record.is_hit {
		let target = temp_hit_record.hit_point+temp_hit_record.normal+random_in_unit_sphere();
		0.5*color(Ray{ start_point: temp_hit_record.hit_point, ray_direction: target - temp_hit_record.hit_point }, hittable_table)
	} else {
		let unit_direction = ray.ray_direction.unit_vec();
		let t = 0.5 * (unit_direction.y() + 1.);
		(1.0 - t) * Vec3(1., 1., 1.) + t * Vec3(0.5, 0.7, 1.)
	}
}

