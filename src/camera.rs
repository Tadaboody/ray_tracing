use crate::ray::Ray;
use crate::vec3::Vec3;

pub struct Camera {
    pub lower_left: Vec3,
    pub horizontal: Vec3,
    pub vertical: Vec3,
    pub origin: Vec3,
}

impl Camera {
    pub fn ray(&self, u: f32, v: f32) -> Ray {
        Ray {
            origin: self.origin,
            direction: self.lower_left + u * self.horizontal + v * self.vertical - self.origin,
        }
    }
}
