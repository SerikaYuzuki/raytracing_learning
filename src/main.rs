mod vec3;
mod ray;
mod hit;

use crate::vec3::{cross, dot, Vec3};
use crate::ray::{Ray};
use std::fs::File;
use std::io::prelude::*;
use std::io::{Write, BufReader, BufRead, Error};


fn main() -> std::io::Result<()> {
	let path = "lines.ppm";
	let mut output = File::create(path)?;

	let nx = 200;
	let ny = 100;
	write!(output, "P3\n{} {}\n255\n", nx, ny)?;

	let lower_left_corner = Vec3(-2., -1., -1.);
	let horizontal = Vec3(4., 0., 0.);
	let vertical = Vec3(0., 2., 0.);
	let origin = Vec3(0., 0., 0.);

	for j in (0..ny).rev() {
		for i in 0..nx {
			let u = (i as f64) / (nx as f64);
			let v = (j as f64) / (ny as f64);
			let ray = Ray { start_point: origin, ray_direction: lower_left_corner + u * horizontal + v * vertical };
			let col = color(ray);
			let ir = (255.99 * col.0) as i64;
			let ig = (255.99 * col.1) as i64;
			let ib = (255.99 * col.2) as i64;

			write!(output, "{} {} {}\n", ir, ig, ib)?;
		}
	}

	Ok(())
}

fn color(ray: Ray) -> (Vec3) {
	let t = hit_sphere(Vec3(0., 0., -1.), 0.5, ray);
	if t > 0. {
		let n = (ray.end_point(t) - Vec3(0.,0.,-1.)).unit_vec();
		return 0.5*Vec3(n.0 +1.,n.1+1.,n.2+1.)
	}
	let unit_direction = ray.ray_direction.unit_vec();
	let t = 0.5 * (unit_direction.y() + 1.);
	let vec3 = (1.0 - t) * Vec3(1., 1., 1.) + t * Vec3(0.5, 0.7, 1.);
	vec3
}

fn hit_sphere(centre : Vec3, radius : f64, ray : Ray) -> f64 {
	let oc = ray.start_point -centre;
	let a = dot(ray.ray_direction, ray.ray_direction);
	let b = 2. * dot(oc, ray.ray_direction);
	let c = dot(oc,oc) - radius*radius;
	let discriminant = b*b -4.*a*c;
	return if discriminant < 0. { -1. } else { (-b - discriminant.sqrt()) / (2. * a) }
}
