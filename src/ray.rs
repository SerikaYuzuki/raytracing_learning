use crate::vec3::{cross, dot, Vec3};

#[derive(Clone, Copy)]
pub struct Ray {
    pub start_point: Vec3,
    pub ray_direction: Vec3,
}

impl Ray {
    pub fn end_point (self, parameter : f64) -> Vec3 { self.start_point + parameter * self.ray_direction}
}


