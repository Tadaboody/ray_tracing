use crate::ray::Ray;
use crate::vec3::Vec3;

pub struct Hit {
    pub t: f32,
    pub point: Vec3,
    pub normal: Vec3,
}

pub trait Hittable {
    fn hit(&self, _ray: &Ray) -> Option<Hit>;
}
