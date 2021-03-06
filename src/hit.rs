use crate::material::Material;
use crate::ray::Ray;
use crate::vec3::Vec3;
extern crate noisy_float;

use noisy_float::prelude::n32; // non Nan-able floats for ordering

pub struct Hit {
    pub dist: f32,
    pub point: Vec3,
    pub normal: Vec3,
}

pub trait Hittable {
    fn hit(&self, _ray: &Ray) -> Option<(Hit, &dyn Material)>;
}

pub type HittableVec = Vec<Box<dyn Hittable>>;

impl Hittable for HittableVec {
    fn hit(&self, _ray: &Ray) -> Option<(Hit, &dyn Material)> {
        if self.is_empty() {
            return None;
        }
        self.iter()
            .map(|hittable| hittable.hit(_ray))
            .filter(|hit| hit.is_some())
            .min_by_key(|opt_hit| n32(opt_hit.as_ref().unwrap().0.dist))
            .flatten()
    }
}
