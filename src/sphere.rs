use crate::hit::{Hit, Hittable};
use crate::ray::Ray;
use crate::vec3::Vec3;

pub struct Sphere {
    pub center: Vec3,
    pub radius: f32,
}

impl Hittable for Sphere {
    fn hit(&self, _ray: &Ray) -> Option<Hit> {
        let oc = _ray.origin - self.center;
        let a = _ray.direction.dot(&_ray.direction);
        let b = 2.0 * oc.dot(&_ray.direction);
        let c = oc.dot(&oc) - self.radius * self.radius;
        let discriminant = (b * b) - (4. * a * c);
        if discriminant < 0.0 {
            return None;
        }
        let dist = (-b - discriminant.sqrt()) / (2. * a);

        if dist < 0.1 {
            return None;
        }
        let point = _ray.at(dist) - self.center;

        Some(Hit {
            dist: dist,
            point: point,
            normal: (point - self.center),
        })
    }
}
