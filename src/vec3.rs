mod vec3 {
    #[derive(Debug, PartialEq)]
    pub struct Vec3 {
        a: [f64; 3],
    }

    impl Vec3 {
        fn create(a: &[f64; 3]) -> Vec3 {
            Vec3 { a: *a }
        }
        fn x(&self) -> f64 {
            self.a[0]
        }
        fn y(&self) -> f64 {
            self.a[1]
        }
        fn z(&self) -> f64 {
            self.a[2]
        }
        fn length_squared(&self) -> f64 {
            self.a.iter().map(|x| x * x).sum()
        }
        fn length(&self) -> f64 {
            let rooted = self.length_squared();
            rooted * rooted
        }
    }
    use std::ops::{Add, AddAssign, Mul, MulAssign};
    impl Add for Vec3 {
        type Output = Vec3;
        fn add(self, other: Self) -> Self {
            Vec3 {
                a: self
                    .a
                    .iter()
                    .zip(other.a.iter())
                    .map(|(a, b)| a + b)
                    .collect(),
            }
        }
    }
}
#[cfg(test)]
mod tests {
    #[test]
    fn t_add() {
        asserteq!(vec3{[1,2]} + vec3{[1,1]},vec3[2,3]);
    }
}
