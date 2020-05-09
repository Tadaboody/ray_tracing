mod vec3;
use itertools::Itertools;
use vec3::Vec3;

fn main() {
    let (width, height) = (200, 100);
    println!("P3\n{} {}\n255", width, height);
    for (j, i) in (0..height).rev().cartesian_product(0..width) {
        let mut v = Vec3::new(i as f32 / width as f32, j as f32 / height as f32, 0.2);
        v *= 255.99;
        println!("{}", v);
    }
}
