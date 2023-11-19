use std::ops::{Add, Sub, Mul, Neg};

#[derive(Clone, Copy, Debug, Default)]
pub struct Vector2 {
    pub x: f64,
    pub y: f64
}

impl Vector2 {
    pub(crate) fn new(x: f64, y: f64) -> Vector2 {
        return Vector2 {
            x: x,
            y: y
        }
    }

    pub fn print(&self) {
        println!("x: {}, y: {}", self.x, self.y)
    }
}

impl Add for Vector2 {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        Vector2::new(
            self.x + rhs.x,
            self.y + rhs.y
        )
    }
}

impl Sub for Vector2 {
    type Output = Self;

    fn sub(self, rhs: Self) -> Self::Output {
        Vector2::new(
            self.x - rhs.x,
            self.y - rhs.y
        )
    }
}

impl Mul for Vector2 {
    type Output = Self;

    fn mul(self, rhs: Self) -> Self::Output {
        Vector2::new(
            self.x * rhs.x,
            self.y * rhs.y
        )
    }
}

impl Neg for Vector2 {
    type Output = Self;

    fn neg(self) -> Self::Output {
        Vector2::new(
            -self.x,
            -self.y
        )
    }
}