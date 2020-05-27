use crate::hit::Hit;
use crate::ray::Ray;
use crate::vec3::{Color, Point, Vec3};

pub trait Material {
    fn scatter(&self, _ray: &Ray, _hit: Hit) -> Option<(Color, Ray)>;
}

pub struct Lambertian(pub Color);

fn random_in_unit_sphere() -> Point {
    loop {
        let p = Point::rand(&mut rand::thread_rng(), -1., 1.);
        if p.length_squared() < 1. {
            return p;
        }
    }
}

fn random_in_hemisphere(normal: &Vec3) -> Point {
    let in_unit = random_in_unit_sphere();
    if in_unit.dot(&normal) > 0. {
        in_unit
    } else {
        -in_unit
    }
}

impl Material for Lambertian {
    fn scatter(&self, _ray: &Ray, _hit: Hit) -> Option<(Color, Ray)> {
        let scatter_direction = _hit.normal + random_in_hemisphere(&_hit.normal);
        Some((
            self.0,
            Ray {
                origin: _hit.point,
                direction: scatter_direction,
            },
        ))
    }
}

pub struct Metal(pub Color);
impl Metal {
    fn reflect(vector: &Vec3, normal: &Vec3) -> Vec3 {
        *vector - ((2. * vector.dot(normal)) * *normal)
    }
}
impl Material for Metal {
    fn scatter(&self, ray: &Ray, hit: Hit) -> std::option::Option<(Color, Ray)> {
        let reflection = Metal::reflect(&ray.direction.unit(), &hit.normal);
        let scatter = Ray {
            origin: hit.point,
            direction: reflection,
        };
        if scatter.direction.dot(&hit.normal) <= 0. {
            return None;
        }
        Some((self.0, scatter))
    }
}
