use crate::hit::Hit;
use crate::ray::Ray;
use crate::vec3::{Color, Point};

pub trait Material {
    fn scatter(&self, _ray: &Ray, _hit: Hit) -> Option<(Color, Ray)>;
}

pub struct Lambertian(pub Color);

fn random_in_unit_sphere() -> Point {
    loop {
        let p = Point::rand(&mut rand::thread_rng(), -1., 1.);
        if p.dot(&p) < 1. {
            return p;
        }
    }
}

impl Material for Lambertian {
    fn scatter(&self, _ray: &Ray, _hit: Hit) -> Option<(Color, Ray)> {
        let scatter_direction = _hit.normal + random_in_unit_sphere();
        Some((
            self.0,
            Ray {
                origin: _hit.point,
                direction: scatter_direction,
            },
        ))
    }
}