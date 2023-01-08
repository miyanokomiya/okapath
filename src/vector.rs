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

pub fn get_bezier_q_point(p0: &Vector2, p1: &Vector2, p2: &Vector2, t: f64) -> Vector2 {
    let a = 1.0 - t;
    p0.multi(a * a) + p1.multi(2.0 * t * a) + p2.multi(t * t)
}

pub fn get_bezier_q_points(p0: &Vector2, p1: &Vector2, p2: &Vector2, split: usize) -> Vec<Vector2> {
    if split <= 1 {
        return vec![*p0, *p2];
    }

    let step = 1.0 / split as f64;
    let mut points: Vec<Vector2> = vec![];

    for i in 0..=split {
        let q = get_bezier_q_point(p0, p1, p2, step * i as f64);
        points.push(q);
    }

    points
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
    fn get_bezier_q_point_cases() {
        let p0 = Vector2(0.0, 0.0);
        let p1 = Vector2(10.0, 0.0);
        let p2 = Vector2(10.0, 10.0);
        assert_eq!(get_bezier_q_point(&p0, &p1, &p2, 0.0), Vector2(0.0, 0.0));
        assert_eq!(
            get_bezier_q_point(&p0, &p1, &p2, 0.25),
            Vector2(4.375, 0.625)
        );
        assert_eq!(get_bezier_q_point(&p0, &p1, &p2, 0.5), Vector2(7.5, 2.5));
        assert_eq!(
            get_bezier_q_point(&p0, &p1, &p2, 0.75),
            Vector2(9.375, 5.625)
        );
        assert_eq!(get_bezier_q_point(&p0, &p1, &p2, 1.0), Vector2(10.0, 10.0));
    }

    #[test]
    fn get_bezier_q_points_cases() {
        let p0 = Vector2(0.0, 0.0);
        let p1 = Vector2(10.0, 0.0);
        let p2 = Vector2(10.0, 10.0);
        assert_eq!(get_bezier_q_points(&p0, &p1, &p2, 1), vec![p0, p2]);
        assert_eq!(
            get_bezier_q_points(&p0, &p1, &p2, 2),
            vec![p0, Vector2(7.5, 2.5), p2]
        );
        assert_eq!(
            get_bezier_q_points(&p0, &p1, &p2, 4),
            vec![
                p0,
                Vector2(4.375, 0.625),
                Vector2(7.5, 2.5),
                Vector2(9.375, 5.625),
                p2
            ]
        );
    }
}
