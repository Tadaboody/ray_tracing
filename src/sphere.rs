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
        let discriminant = b * b - 4. * a * c;
        if discriminant < 0.0 {
            return None;
        }
        let t = -b - discriminant.sqrt() / (2. * a);

        if t < 0.0 {
            return None;
        }
        let p = _ray.at(t);

        Some(Hit {
            t: t,
            point: p,
            normal: (p - self.center),
        })
    }
}
