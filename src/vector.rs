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

    pub fn multi(self, v: f64) -> Self {
        Self(self.0 * v, self.1 * v)
    }
}

pub fn get_polyline_length(points: &Vec<Vector2>) -> f64 {
    if points.len() <= 1 {
        return 0.0;
    }

    let mut length = 0.0;
    for i in 0..(points.len() - 1) {
        length += (*points.get(i).unwrap() - *points.get(i + 1).unwrap()).norm();
    }
    length
}

pub fn get_bezier_q_points(p0: &Vector2, p1: &Vector2, p2: &Vector2, split: usize) -> Vec<Vector2> {
    Bezier2::new(*p0, *p1, *p2).get_appro_points(split)
}

pub fn get_bezier_c_points(
    p0: &Vector2,
    p1: &Vector2,
    p2: &Vector2,
    p3: &Vector2,
    split: usize,
) -> Vec<Vector2> {
    Bezier3::new(*p0, *p1, *p2, *p3).get_appro_points(split)
}

pub trait Lerpable {
    fn lerp(&self, t: f64) -> Vector2;

    fn get_appro_length(&self, split: usize) -> f64 {
        get_polyline_length(&self.get_appro_points(split))
    }

    fn get_appro_points(&self, split: usize) -> Vec<Vector2> {
        if split <= 1 {
            return vec![self.lerp(0.0), self.lerp(1.0)];
        }

        let step = 1.0 / split as f64;
        let mut points: Vec<Vector2> = vec![];

        for i in 0..=split {
            let q = self.lerp(step * i as f64);
            points.push(q);
        }

        points
    }
}

pub struct Bezier2 {
    p0: Vector2,
    p1: Vector2,
    p2: Vector2,
}

impl Bezier2 {
    pub fn new(p0: Vector2, p1: Vector2, p2: Vector2) -> Self {
        Bezier2 { p0, p1, p2 }
    }
}

impl Lerpable for Bezier2 {
    fn lerp(&self, t: f64) -> Vector2 {
        let a = 1.0 - t;
        self.p0.multi(a * a) + self.p1.multi(2.0 * t * a) + self.p2.multi(t * t)
    }
}

pub struct Bezier3 {
    p0: Vector2,
    p1: Vector2,
    p2: Vector2,
    p3: Vector2,
}

impl Bezier3 {
    pub fn new(p0: Vector2, p1: Vector2, p2: Vector2, p3: Vector2) -> Self {
        Bezier3 { p0, p1, p2, p3 }
    }
}

impl Lerpable for Bezier3 {
    fn lerp(&self, t: f64) -> Vector2 {
        let a = 1.0 - t;
        let aa = a * a;
        let tt = t * t;
        self.p0.multi(aa * a)
            + self.p1.multi(3.0 * aa * t)
            + self.p2.multi(3.0 * a * tt)
            + self.p3.multi(tt * t)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn vector2_norm_cases() {
        assert_eq!(Vector2(3.0, 4.0).norm(), 5.0);
        assert_eq!(Vector2(-1.0, 1.0).norm(), 2.0_f64.sqrt());
    }

    #[test]
    fn vector2_multi_cases() {
        assert_eq!(Vector2(3.0, 4.0).multi(2.0), Vector2(6.0, 8.0));
        assert_eq!(Vector2(3.0, 4.0).multi(-1.0), Vector2(-3.0, -4.0));
    }

    #[test]
    fn get_polyline_length_cases() {
        let p0 = Vector2(0.0, 0.0);
        let p1 = Vector2(10.0, 0.0);
        let p2 = Vector2(10.0, 10.0);
        assert_eq!(get_polyline_length(&vec![]), 0.0);
        assert_eq!(get_polyline_length(&vec![p0]), 0.0);
        assert_eq!(get_polyline_length(&vec![p0, p1]), 10.0);
        assert_eq!(get_polyline_length(&vec![p0, p1, p2]), 20.0);
    }

    #[test]
    fn bezier_q_cases() {
        let p0 = Vector2(0.0, 0.0);
        let p1 = Vector2(10.0, 0.0);
        let p2 = Vector2(10.0, 10.0);
        let target = Bezier2::new(p0, p1, p2);

        assert_eq!(target.get_appro_points(1), vec![p0, p2]);
        assert_eq!(target.get_appro_points(2), vec![p0, Vector2(7.5, 2.5), p2]);
        assert_eq!(
            target.get_appro_points(4),
            vec![
                p0,
                Vector2(4.375, 0.625),
                Vector2(7.5, 2.5),
                Vector2(9.375, 5.625),
                p2
            ]
        );
    }

    #[test]
    fn bezier_c_cases() {
        let p0 = Vector2(0.0, 0.0);
        let p1 = Vector2(10.0, 0.0);
        let p2 = Vector2(0.0, 10.0);
        let p3 = Vector2(10.0, 10.0);
        let target = Bezier3::new(p0, p1, p2, p3);

        assert_eq!(target.get_appro_points(1), vec![p0, p3]);
        assert_eq!(target.get_appro_points(2), vec![p0, Vector2(5.0, 5.0), p3]);
        assert_eq!(
            target.get_appro_points(4),
            vec![
                p0,
                Vector2(4.375, 1.5625),
                Vector2(5.0, 5.0),
                Vector2(5.625, 8.4375),
                p3
            ]
        );
    }
}
