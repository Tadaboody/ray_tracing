use crate::hit::{Hit, Hittable};
use crate::material::Material;
use crate::ray::Ray;
use crate::vec3::Vec3;

pub struct Sphere {
    pub center: Vec3,
    pub radius: f32,
    pub material: Box<dyn Material>,
}

// Returns an option matching if the predicate was matched
fn satisfies<T: Copy>(f: T, pred: fn(T) -> bool) -> Option<T> {
    if pred(f) {
        Some(f)
    } else {
        None
    }
}

impl Hittable for Sphere {
    fn hit(&self, _ray: &Ray) -> Option<(Hit, &dyn Material)> {
        let oc = _ray.origin - self.center;
        let a = _ray.direction.dot(&_ray.direction);
        let b = oc.dot(&_ray.direction);
        let c = oc.dot(&oc) - self.radius * self.radius;
        let discriminant = (b * b) - (a * c);
        if discriminant < 0.0 {
            return None;
        }
        let hit = |num: f32| num > 0.1;
        let possible_dist = satisfies((-b - discriminant.sqrt()) / a, hit)
            .or(satisfies(-b + discriminant.sqrt() / a, hit));
        match possible_dist {
            Some(dist) => {
                let point = _ray.at(dist);

                Some((
                    Hit {
                        dist: dist,
                        point: point,
                        normal: (point - self.center) / self.radius,
                    },
                    self.material.as_ref(),
                ))
            }
            None => None,
        }
    }
}
