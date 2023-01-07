use std::ops::Add;
use std::ops::Mul;
use std::ops::Sub;

#[derive(Debug, Copy, Clone, PartialEq)]
pub struct Vector2(pub f64, pub f64);

impl Add for Vector2 {
    type Output = Self;

    fn add(self, other: Self) -> Self {
        Self(self.0 + other.0, self.1 + other.1)
    }
}

impl Sub for Vector2 {
    type Output = Self;

    fn sub(self, other: Self) -> Self {
        Self(self.0 - other.0, self.1 - other.1)
    }
}

impl Mul for Vector2 {
    type Output = Self;

    fn mul(self, other: Self) -> Self {
        Self(self.0 * other.0, self.1 * other.1)
    }
}

impl Vector2 {
    pub fn norm(self) -> f64 {
        (self.0 * self.0 + self.1 * self.1).sqrt()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn norm_cases() {
        assert_eq!(Vector2(3.0, 4.0).norm(), 5.0);
        assert_eq!(Vector2(-1.0, 1.0).norm(), 2.0_f64.sqrt());
    }
}
