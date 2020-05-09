mod ray;
mod vec3;

use ray::Ray;
use vec3::Vec3;
use itertools::Itertools;

fn color(r: Ray) -> Vec3 {
    let unit_direction = r.direction.unit();
    let t = 0.5 * (unit_direction.y() + 1.0);
    let blue = Vec3::new(0.0, 0.0, 1.0);
    let white = Vec3::new(1.0, 1.0, 1.0);
    (1.0 - t) * white + t * blue
}

fn main() {
    let (width, height) = (200, 100);
    let origin = Vec3::new(0.0, 0.0, 0.0);
    let lower_left = Vec3::new(-2.0, -1.0, -1.0);
    let horizontal = Vec3::new(4.0, 0.0, 0.0);
    let vertical = Vec3::new(0.0, 2.0, 0.0);

    println!("P3\n{} {}\n255", width, height);
    for (j, i) in (0..height).rev().cartesian_product(0..width) {
        let u = i as f32 / width as f32;
        let v = j as f32 / height as f32;
        let r = Ray {
            origin: origin,
            direction: lower_left + u * horizontal + v * vertical,
        };
        println!("{}", color(r) * 255.99);
    }
}
