mod vec3;
use itertools::Itertools;

fn main() {
    let (width, height) = (200, 100);
    println!("P3\n{} {}\n255", width, height);
    let a = crate::vec3::create([1, 1, 1]);
    for (j, i) in (0..height).rev().cartesian_product(0..width) {
        let r = ((i as f32 / width as f32) * 255.0) as i32;
        let g = ((j as f32 / height as f32) * 255.0) as i32;
        let b = (0.2 * 255.0) as i32;
        println!("{} {} {}", r, g, b);
    }
}
