mod ray;
mod vec3;

use indicatif::ProgressBar;
use itertools::Itertools;
use ray::Ray;
use vec3::Vec3;

fn hit_sphere(center: &Vec3, radius: f32, _ray: &Ray) -> bool {
    let oc = _ray.origin - *center;
    let a = _ray.direction.dot(&_ray.direction);
    let b = 2.0 * oc.dot(&_ray.direction);
    let c = oc.dot(&oc) - radius * radius;
    let desc = b * b - 4.0 * a * c;
    desc > 0.0
}

fn color(r: Ray) -> Vec3 {
    if hit_sphere(&Vec3::new(0.0, 0.0, -1.0), 0.5, &r) {
        Vec3::new(1.0, 0.0, 0.0)
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
    let origin = Vec3::new(0.0, 0.0, 0.0);
    let lower_left = Vec3::new(-2.0, -1.0, -1.0);
    let horizontal = Vec3::new(4.0, 0.0, 0.0);
    let vertical = Vec3::new(0.0, 2.0, 0.0);

    println!("P3\n{} {}\n255", width, height);
    let pb = ProgressBar::new(height * width);
    pb.set_draw_delta((height * width) / 100);
    let iter = (0..height).rev().cartesian_product(0..width);
    for (j, i) in pb.wrap_iter(iter) {
        let u = i as f32 / width as f32;
        let v = j as f32 / height as f32;
        let r = Ray {
            origin: origin,
            direction: lower_left + u * horizontal + v * vertical,
        };
        println!("{}", color(r) * 255.99);
    }
    pb.finish() // Fixed in indicatif 14.0
}
