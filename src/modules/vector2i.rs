use std::ops::{Add, Sub, Mul, Neg};

#[derive(Clone, Copy, Debug)]
pub struct Vector2i {
    pub x: i64,
    pub y: i64
}

impl Vector2i {
    pub(crate) fn new(x: i64, y: i64) -> Vector2i {
        return Vector2i {
            x: x,
            y: y
        }
    }

    pub fn print(&self) {
        print!("x: {}, y: {}", self.x, self.y)
    }
}

impl Add for Vector2i {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        Vector2i::new(
            self.x + rhs.x,
            self.y + rhs.y
        )
    }
}

impl Sub for Vector2i {
    type Output = Self;

    fn sub(self, rhs: Self) -> Self::Output {
        Vector2i::new(
            self.x - rhs.x,
            self.y - rhs.y
        )
    }
}

impl Mul for Vector2i {
    type Output = Self;

    fn mul(self, rhs: Self) -> Self::Output {
        Vector2i::new(
            self.x * rhs.x,
            self.y * rhs.y
        )
    }
}

impl Neg for Vector2i {
    type Output = Self;

    fn neg(self) -> Self::Output {
        Vector2i::new(
            -self.x,
            -self.y
        )
    }
}