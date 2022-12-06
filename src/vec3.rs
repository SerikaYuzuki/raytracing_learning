use std::ops;

#[derive(Clone, Copy)]
pub struct Vec3(pub f64, pub f64, pub f64);

impl Vec3 {
    pub fn x(&self) -> f64 { self.0 }
    pub fn y(&self) -> f64 { self.1 }
    pub fn z(&self) -> f64 { self.2 }
    pub fn r(&self) -> f64 { self.0 }
    pub fn g(&self) -> f64 { self.1 }
    pub fn b(&self) -> f64 { self.2 }
    pub fn length(self) -> f64 {
        (self.0 * self.0 + self.1 * self.1 + self.2 * self.2).sqrt()
    }
    pub fn unit_vec(self) -> Vec3 { Vec3(self.0 / self.length(), self.1 / self.length(), self.2 / self.length()) }
}

impl ops::Add<Vec3> for Vec3 {
    type Output = Vec3;

    fn add(self, _rhs: Vec3) -> Vec3 {
        let mut new_vec: Vec3 = Vec3(0.0, 0.0, 0.0);
        new_vec.0 = self.0 + _rhs.0;
        new_vec.1 = self.1 + _rhs.1;
        new_vec.2 = self.2 + _rhs.2;
        new_vec
    }
}

impl ops::Sub<Vec3> for Vec3 {
    type Output = Vec3;

    fn sub(self, _rhs: Vec3) -> Vec3 {
        let mut new_vec: Vec3 = Vec3(0.0, 0.0, 0.0);
        new_vec.0 = self.0 - _rhs.0;
        new_vec.1 = self.1 - _rhs.1;
        new_vec.2 = self.2 - _rhs.2;
        new_vec
    }
}

impl ops::Mul<Vec3> for Vec3 {
    type Output = Vec3;

    fn mul(self, _rhs: Vec3) -> Vec3 {
        let mut new_vec: Vec3 = Vec3(0.0, 0.0, 0.0);
        new_vec.0 = self.0 * _rhs.0;
        new_vec.1 = self.1 * _rhs.1;
        new_vec.2 = self.2 * _rhs.2;
        new_vec
    }
}

impl ops::Mul<f64> for Vec3 {
    type Output = Vec3;

    fn mul(self, _rhs: f64) -> Vec3 {
        let mut new_vec: Vec3 = Vec3(0.0, 0.0, 0.0);
        new_vec.0 = self.0 * _rhs;
        new_vec.1 = self.1 * _rhs;
        new_vec.2 = self.2 * _rhs;
        new_vec
    }
}

impl ops::Mul<Vec3> for f64 {
    type Output = Vec3;

    fn mul(self, _rhs: Vec3) -> Vec3 {
        let mut new_vec: Vec3 = Vec3(0.0, 0.0, 0.0);
        new_vec.0 = self * _rhs.0;
        new_vec.1 = self * _rhs.1;
        new_vec.2 = self * _rhs.2;
        new_vec
    }
}

impl ops::Div<f64> for Vec3 {
    type Output = Vec3;

    fn div(self, _rhs: f64) -> Vec3 {
        let mut new_vec: Vec3 = Vec3(0.0, 0.0, 0.0);
        new_vec.0 = self.0 / _rhs;
        new_vec.1 = self.1 / _rhs;
        new_vec.2 = self.2 / _rhs;
        new_vec
    }
}

impl ops::Div<Vec3> for Vec3 {
    type Output = Vec3;

    fn div(self, _rhs: Vec3) -> Vec3 {
        let mut new_vec: Vec3 = Vec3(0.0, 0.0, 0.0);
        new_vec.0 = self.0 / _rhs.0;
        new_vec.1 = self.1 / _rhs.1;
        new_vec.2 = self.2 / _rhs.2;
        new_vec
    }
}

pub fn dot(vec1: Vec3, vec2: Vec3) -> f64 {
    vec1.0 * vec2.0 + vec1.1 * vec2.1 + vec1.2 * vec2.2
}

pub fn cross(vec1: Vec3, vec2: Vec3) -> Vec3 {
    Vec3(vec1.1 * vec2.2 - vec1.2 * vec2.1, vec1.2 * vec2.0 - vec1.0 * vec2.2, vec1.0 * vec2.1 - vec1.1 * vec2.0)
}
