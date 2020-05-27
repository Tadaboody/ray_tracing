extern crate bmp;
extern crate rand;
mod camera;
mod hit;
mod material;
mod ray;
mod sphere;
mod vec3;

use crate::camera::Camera;
use crate::hit::{Hittable, HittableVec};
use crate::material::{Lambertian, Metal};
use crate::ray::Ray;
use crate::sphere::Sphere;
use crate::vec3::{Color, Vec3};
use indicatif::ProgressBar;

fn color<T: Hittable>(r: Ray, world: &T, remaining_recursions: u32) -> Color {
    if remaining_recursions <= 0 {
        // No light source found, absorb the light!
        return Color::GRAY;
    }

    if let Some((hit, material)) = world.hit(&r) {
        if let Some((attenuation, scatter)) = material.scatter(&r, hit) {
            attenuation * color(scatter, world, remaining_recursions - 1)
        } else {
            Color::GRAY
        }
    } else {
        let unit_direction = r.direction.unit();
        let t = 0.5 * (unit_direction.y() + 1.0);
        (1.0 - t) * Color::WHITE + t * Color::SKY_BLUE
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
            material: Box::new(Lambertian(Color::new(0.7, 0.3, 0.3))),
        }),
        Box::new(Sphere {
            center: Vec3::new(0., -100.5, -1.0),
            radius: 100.,
            material: Box::new(Lambertian(Color::new(0.8, 0.8, 0.0))),
        }),
        Box::new(Sphere {
            center: Vec3::new(1., 0., -1.),
            radius: 0.5,
            material: Box::new(Metal(Color::new(0.8, 0.6, 0.2))),
        }),
        Box::new(Sphere {
            center: Vec3::new(-1., 0., -1.),
            radius: 0.5,
            material: Box::new(Metal(Color::YELLOW)),
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
    let _ = img.save("result.bmp");
    pb.finish() // Fixed in next indicatif...
}
