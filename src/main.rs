mod camera;
mod hit;
mod ray;
mod sphere;
mod vec3;

use crate::camera::Camera;
use crate::hit::{Hittable, HittableVec};
use crate::ray::Ray;
use crate::sphere::Sphere;
use crate::vec3::Vec3;
use indicatif::ProgressBar;
use itertools::Itertools;

fn color(r: Ray) -> Vec3 {
    let spheres: HittableVec = vec![
        Box::new(Sphere {
            center: Vec3::new(0., 0., -1.0),
            radius: 0.5,
        }),
        Box::new(Sphere {
            center: Vec3::new(0., -100.5, -1.0),
            radius: 100.,
        }),
    ];

    if let Some(hit) = spheres.hit(&r) {
        0.5 * (hit.normal + Vec3::new(1., 1., 1.))
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
    let camera = Camera {
        lower_left: Vec3::new(-2.0, -1.0, -1.0),
        horizontal: Vec3::new(4.0, 0.0, 0.0),
        vertical: Vec3::new(0.0, 2.0, 0.0),
        origin: Vec3::new(0.0, 0.0, 0.0),
    };

    println!("P3\n{} {}\n255", width, height);
    let pb = ProgressBar::new(height * width);
    pb.set_draw_delta((height * width) / 100);
    let iter = (0..height).rev().cartesian_product(0..width);
    for (j, i) in pb.wrap_iter(iter) {
        let u = i as f32 / width as f32;
        let v = j as f32 / height as f32;
        let r = camera.ray(u, v);
        print!("{} ", (color(r) * 255.99).int());
    }
    pb.finish() // Fixed in next indicatif...
}
