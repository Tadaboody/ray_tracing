mod ray;
mod vec3;

use indicatif::ProgressBar;
use itertools::Itertools;
use ray::Ray;
use vec3::Vec3;

fn hit_sphere(center: &Vec3, radius: f32, _ray: &Ray) -> Option<Vec3> {
    let oc = _ray.origin - *center;
    let a = _ray.direction.dot(&_ray.direction);
    let b = 2.0 * oc.dot(&_ray.direction);
    let c = oc.dot(&oc) - radius * radius;
    let discriminant = b * b - 4.0 * a * c;
    if discriminant < 0.0 {
        return None;
    }
    let t = -b - discriminant.sqrt() / (2.0 * a);

    if t < 0.0 {
        return None;
    }
    Some(_ray.at(t) - *center)
}

fn color(r: Ray) -> Vec3 {
    let red = Vec3::new(1., 0., 0.);
    let blue = Vec3::new(0.0, 0.0, 1.0);
    if let Some(normal) = hit_sphere(&Vec3::new(0.0, 0.0, -1.0), 0.5, &r) {
        let t = normal.unit().dot(&r.direction.unit());
        t * red + (1. - t) * blue
    } else {
        let unit_direction = r.direction.unit();
        let t = 0.5 * (unit_direction.y() + 1.0);
        let blue = Vec3::new(0.0, 0.0, 1.0);
        let white = Vec3::new(1.0, 1.0, 1.0);
        (1.0 - t) * white + t * blue
    }
}

fn main() {
    let (width, height) = (200, 100);
    let lower_left = Vec3::new(-2.0, -1.0, -1.0);
    let horizontal_length = Vec3::new(4.0, 0.0, 0.0);
    let vertical_length = Vec3::new(0.0, 2.0, 0.0);
    let origin = Vec3::new(0.0, 0.0, 0.0);

    println!("P3\n{} {}\n255", width, height);
    let pb = ProgressBar::new(height * width);
    pb.set_draw_delta((height * width) / 100);
    let iter = (0..height).rev().cartesian_product(0..width);
    for (j, i) in pb.wrap_iter(iter) {
        let u = i as f32 / width as f32;
        let v = j as f32 / height as f32;
        let r = Ray {
            origin: origin,
            direction: lower_left + u * horizontal_length + v * vertical_length,
        };
        println!("{}", (color(r) * 255.99).int());
    }
    pb.finish() // Fixed in indicatif 14.0
}
