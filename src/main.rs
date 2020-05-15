extern crate bmp;
extern crate rand;
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

fn random_in_unit_sphere() -> Vec3 {
    loop {
        let p = Vec3::rand(&mut rand::thread_rng(), -1., 1.);
        if p.dot(&p) < 1. {
            return p;
        }
    }
}

fn color<T: Hittable>(r: Ray, world: &T, remaining_recursions: u32) -> Vec3 {
    if remaining_recursions <= 0 {
        return Vec3::new(0., 0., 0.);
    }

    if let Some(hit) = world.hit(&r) {
        let target = hit.point + hit.normal + random_in_unit_sphere();
        0.5 * color(
            Ray {
                origin: hit.point,
                direction: target - hit.point,
            },
            world,
            remaining_recursions - 1,
        )
    } else {
        let unit_direction = r.direction.unit();
        let t = 0.5 * (unit_direction.y() + 1.0);
        let blue = Vec3::new(0.5, 0.7, 1.0);
        let white = Vec3::new(1.0, 1.0, 1.0);
        (1.0 - t) * white + t * blue
    }
}

fn main() {
    let (width, height) = (200, 100);
    let mut img = bmp::Image::new(width as u32, height as u32);
    let camera = Camera {
        lower_left: Vec3::new(-2.0, -1.0, -1.0),
        horizontal: Vec3::new(4.0, 0.0, 0.0),
        vertical: Vec3::new(0.0, 2.0, 0.0),
        origin: Vec3::new(0.0, 0.0, 0.0),
    };
    let world: HittableVec = vec![
        Box::new(Sphere {
            center: Vec3::new(0., 0., -1.0),
            radius: 0.5,
        }),
        Box::new(Sphere {
            center: Vec3::new(0., -100.5, -1.0),
            radius: 100.,
        }),
    ];
    let sampling_rate = 100;
    let max_depth = 10;

    let pb = ProgressBar::new(height * width);
    pb.set_draw_delta((height * width) / 100);
    for (x, y) in pb.wrap_iter(img.coordinates()) {
        let mut pixel_color = Vec3::new(0., 0., 0.);
        for _ in 0..sampling_rate {
            let u = (x as f32 + rand::random::<f32>()) / width as f32;
            let v = ((height - y as u64 - 1) as f32 + rand::random::<f32>()) as f32 / height as f32;
            let r = camera.ray(u, v);
            pixel_color += color(r, &world, max_depth);
        }
        pixel_color /= sampling_rate as f32;
        let mut gamma_corrected = pixel_color.map(|f| f.sqrt());
        gamma_corrected *= 255.99;
        img.set_pixel(x, y, gamma_corrected.pixel());
    }
    let _ = img.save("res.bmp");
    pb.finish() // Fixed in next indicatif...
}
