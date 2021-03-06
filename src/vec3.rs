use rand::rngs::ThreadRng;
use rand::Rng;
#[derive(Debug, PartialEq, Clone, Copy)]
pub struct Vec3 {
    e: [f32; 3],
}
pub type Point = Vec3;
pub type Color = Vec3;
impl Color {
    pub const BLACK: Color = Color::new(0., 0., 0.);
    pub const WHITE: Color = Color::new(1., 1., 1.);
    pub const GRAY: Color = Color::new(0.5, 0.5, 0.5);
    pub const SKY_BLUE: Color = Color::new(0.5, 0.7, 1.0);
    pub const RED: Color = Color::new(1., 0., 0.);
    pub const YELLOW: Color = Color::new(0.8, 0.8, 0.8);
}

impl Vec3 {
    pub const fn new(x: f32, y: f32, z: f32) -> Vec3 {
        Vec3 { e: [x, y, z] }
    }
    pub fn x(&self) -> f32 {
        self.e[0]
    }
    pub fn y(&self) -> f32 {
        self.e[1]
    }
    pub fn z(&self) -> f32 {
        self.e[2]
    }
    pub fn length_squared(&self) -> f32 {
        self.e.iter().map(|x| x * x).sum()
    }
    pub fn length(&self) -> f32 {
        let rooted = self.length_squared();
        rooted * rooted
    }
    pub fn unit(&self) -> Vec3 {
        *self / self.length()
    }
    pub fn dot(&self, other: &Vec3) -> f32 {
        self.e[0] * other.e[0] + self.e[1] * other.e[1] + self.e[2] * other.e[2]
    }
    pub fn int(&self) -> Vec3 {
        Vec3::new(self.x().floor(), self.y().floor(), self.z().floor())
    }
    pub fn rand(rng: &mut ThreadRng, min: f32, max: f32) -> Vec3 {
        Vec3::new(
            rng.gen_range(min, max),
            rng.gen_range(min, max),
            rng.gen_range(min, max),
        )
    }
    pub fn pixel(&self) -> bmp::Pixel {
        bmp::Pixel::new(self.x() as u8, self.y() as u8, self.z() as u8)
    }
    pub fn map(&self, f: fn(f32) -> f32) -> Vec3 {
        Vec3::new(f(self.x()), f(self.y()), f(self.z()))
    }
}

use std::ops::*;

impl Add for Vec3 {
    type Output = Vec3;
    fn add(self, other: Vec3) -> Vec3 {
        Vec3 {
            e: [
                self.e[0] + other.e[0],
                self.e[1] + other.e[1],
                self.e[2] + other.e[2],
            ],
        }
    }
}

impl Sub for Vec3 {
    type Output = Vec3;
    fn sub(self, other: Vec3) -> Vec3 {
        Vec3 {
            e: [
                self.e[0] - other.e[0],
                self.e[1] - other.e[1],
                self.e[2] - other.e[2],
            ],
        }
    }
}

impl Mul for Vec3 {
    type Output = Vec3;
    fn mul(self, other: Vec3) -> Vec3 {
        Vec3 {
            e: [
                self.e[0] * other.e[0],
                self.e[1] * other.e[1],
                self.e[2] * other.e[2],
            ],
        }
    }
}

impl Div for Vec3 {
    type Output = Vec3;
    fn div(self, other: Vec3) -> Vec3 {
        Vec3 {
            e: [
                self.e[0] / other.e[0],
                self.e[1] / other.e[1],
                self.e[2] / other.e[2],
            ],
        }
    }
}

impl Mul<Vec3> for f32 {
    type Output = Vec3;
    fn mul(self, v: Vec3) -> Vec3 {
        Vec3 {
            e: [self * v.e[0], self * v.e[1], self * v.e[2]],
        }
    }
}

impl Div<f32> for Vec3 {
    type Output = Vec3;
    fn div(self, t: f32) -> Vec3 {
        Vec3 {
            e: [self.e[0] / t, self.e[1] / t, self.e[2] / t],
        }
    }
}

impl Mul<f32> for Vec3 {
    type Output = Vec3;
    fn mul(self, t: f32) -> Vec3 {
        Vec3 {
            e: [self.e[0] * t, self.e[1] * t, self.e[2] * t],
        }
    }
}

impl Neg for Vec3 {
    type Output = Vec3;
    fn neg(self) -> Vec3 {
        Vec3 {
            e: [-self.e[0], -self.e[1], -self.e[2]],
        }
    }
}

impl AddAssign for Vec3 {
    fn add_assign(&mut self, other: Vec3) {
        self.e[0] += other.e[0];
        self.e[1] += other.e[1];
        self.e[2] += other.e[2];
    }
}

impl SubAssign for Vec3 {
    fn sub_assign(&mut self, other: Vec3) {
        self.e[0] -= other.e[0];
        self.e[1] -= other.e[1];
        self.e[2] -= other.e[2];
    }
}

impl MulAssign for Vec3 {
    fn mul_assign(&mut self, other: Vec3) {
        self.e[0] *= other.e[0];
        self.e[1] *= other.e[1];
        self.e[2] *= other.e[2];
    }
}

impl DivAssign for Vec3 {
    fn div_assign(&mut self, other: Vec3) {
        self.e[0] /= other.e[0];
        self.e[1] /= other.e[1];
        self.e[2] /= other.e[2];
    }
}

impl MulAssign<f32> for Vec3 {
    fn mul_assign(&mut self, t: f32) {
        self.e[0] *= t;
        self.e[1] *= t;
        self.e[2] *= t;
    }
}

impl DivAssign<f32> for Vec3 {
    fn div_assign(&mut self, t: f32) {
        let k = 1.0 / t;

        self.e[0] *= k;
        self.e[1] *= k;
        self.e[2] *= k;
    }
}

impl Index<usize> for Vec3 {
    type Output = f32;
    fn index(&self, i: usize) -> &f32 {
        &self.e[i]
    }
}

impl IndexMut<usize> for Vec3 {
    fn index_mut(&mut self, i: usize) -> &mut f32 {
        &mut self.e[i]
    }
}

impl std::fmt::Display for Vec3 {
    fn fmt(&self, fmt: &mut std::fmt::Formatter<'_>) -> std::result::Result<(), std::fmt::Error> {
        write!(fmt, "{} {} {}", self.x(), self.y(), self.z(),)?;
        Ok(())
    }
}
