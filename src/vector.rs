use std::f64::consts::PI;
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

    pub fn rotate(self, r: f64) -> Self {
        let sin = r.sin();
        let cos = r.cos();
        Self(cos * self.0 + -sin * self.1, sin * self.0 + cos * self.1)
    }

    pub fn dot(self, b: Vector2) -> f64 {
        self.0 * b.0 + self.1 * b.1
    }

    pub fn cross(self, b: Vector2) -> f64 {
        self.0 * b.1 - self.1 * b.0
    }

    pub fn radian(self, to: Vector2) -> f64 {
        (self.dot(to) / self.norm() / to.norm()).acos() * self.cross(to).signum()
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

fn lerp(p0: &Vector2, p1: &Vector2, t: f64) -> Vector2 {
    *p0 + (*p1 - *p0).multi(t)
}

pub struct Bezier2 {
    p0: Vector2,
    p1: Vector2,
    p2: Vector2,
}

impl Bezier2 {
    pub fn new(p0: Vector2, p1: Vector2, p2: Vector2) -> Self {
        Self { p0, p1, p2 }
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
        Self { p0, p1, p2, p3 }
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

pub struct Arc {
    p0: Vector2,
    rx: f64,
    ry: f64,
    rotation: f64,
    large: bool,
    sweep: bool,
    p1: Vector2,

    c: Vector2,
    theta: f64,
    dtheta: f64,
    sinr: f64,
    cosr: f64,
}

impl Arc {
    pub fn new(
        p0: Vector2,
        rx: f64,
        ry: f64,
        rotation: f64,
        large: bool,
        sweep: bool,
        p1: Vector2,
    ) -> Self {
        let r = rotation.to_radians();
        let a = Vector2((p0.0 - p1.0) / 2.0, (p0.1 - p1.1) / 2.0).rotate(-r);
        let ax2 = a.0 * a.0;
        let ay2 = a.1 * a.1;

        let l = ax2 / rx / rx + ay2 / ry / ry;
        let (rxa, rya) = if l > 1.0 {
            let lsqrt = l.sqrt();
            (rx.abs() * lsqrt, ry.abs() * lsqrt)
        } else {
            (rx.abs(), ry.abs())
        };

        let rx2 = rxa * rxa;
        let ry2 = rya * rya;
        let b = Vector2(rxa * a.1 / rya, -rya * a.0 / rxa)
            .multi(((rx2 * ry2 - rx2 * ay2 - ry2 * ax2) / (rx2 * ay2 + ry2 * ax2)).sqrt())
            .multi(if large == sweep { -1.0 } else { 1.0 });

        let c = b.rotate(r) + (p0 + p1).multi(0.5);

        let u = Vector2((a.0 - b.0) / rxa, (a.1 - b.1) / rya);
        let v = Vector2((-a.0 - b.0) / rxa, (-a.1 - b.1) / rya);
        let theta = Vector2(1.0, 0.0).radian(u);
        let dtheta_tmp = u.radian(v) % (2.0 * PI);
        println!("{:?} {:?} {:?}", u, v, dtheta_tmp);
        let dtheta = if !sweep && 0.0 < dtheta_tmp {
            dtheta_tmp - (2.0 * PI)
        } else if sweep && dtheta_tmp < 0.0 {
            dtheta_tmp + (2.0 * PI)
        } else {
            dtheta_tmp
        };

        Self {
            p0,
            rx: rxa,
            ry: rya,
            rotation,
            large,
            sweep,
            p1,
            c,
            theta,
            dtheta,
            sinr: r.sin(),
            cosr: r.cos(),
        }
    }

    fn rotate(&self, p: Vector2) -> Vector2 {
        Vector2(
            self.cosr * p.0 + -self.sinr * p.1,
            self.sinr * p.0 + self.cosr * p.1,
        )
    }
}

impl Lerpable for Arc {
    // https://www.w3.org/TR/SVG11/implnote.html#ArcImplementationNotes
    fn lerp(&self, t: f64) -> Vector2 {
        if self.rx == 0.0 || self.ry == 0.0 {
            return lerp(&self.p0, &self.p1, t);
        }

        let r = self.theta + self.dtheta * t;
        self.rotate(Vector2(self.rx * r.cos(), self.ry * r.sin())) + self.c
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
    fn vector2_rotate_cases() {
        let p0 = Vector2(10.0, 0.0).rotate(PI);
        assert_eq!(p0.0.round(), -10.0);
        assert_eq!(p0.1.round(), -0.0);
        let p1 = Vector2(10.0, 0.0).rotate(PI * 0.5);
        assert_eq!(p1.0.round(), 0.0);
        assert_eq!(p1.1.round(), 10.0);
        let p2 = Vector2(10.0, 0.0).rotate(PI * 1.5);
        assert_eq!(p2.0.round(), 0.0);
        assert_eq!(p2.1.round(), -10.0);
        let p3 = Vector2(10.0, 0.0).rotate(-PI * 0.5);
        assert_eq!(p3.0.round(), 0.0);
        assert_eq!(p3.1.round(), -10.0);
    }

    #[test]
    fn vector2_radian_cases() {
        let p = Vector2(1.0, 0.0);
        assert_eq!(p.radian(Vector2(1.0, 0.0)), 0.0);
        assert_eq!(p.radian(Vector2(0.0, 1.0)), PI * 0.5);
        assert_eq!(p.radian(Vector2(-1.0, 0.0)), PI);
        assert_eq!(p.radian(Vector2(0.0, -1.0)), -PI * 0.5);
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

    #[test]
    fn arc_new_large_sweep() {
        let p0 = Vector2(0.0, 0.0);
        let p1 = Vector2(5.0, 5.0);

        let a0 = Arc::new(p0, 5.0, 5.0, 0.0, false, false, p1);
        assert_eq!(a0.p0, p0);
        assert_eq!(a0.rx, 5.0);
        assert_eq!(a0.ry, 5.0);
        assert_eq!(a0.large, false);
        assert_eq!(a0.sweep, false);
        assert_eq!(a0.p1, p1);
        assert_eq!(a0.c, Vector2(5.0, 0.0));
        assert_eq!(a0.theta.to_degrees().round(), 180.0);
        assert_eq!(a0.dtheta.to_degrees().round(), -90.0);

        let a1 = Arc::new(p0, 5.0, 5.0, 0.0, true, false, p1);
        assert_eq!(a1.large, true);
        assert_eq!(a1.c, Vector2(0.0, 5.0));
        assert_eq!(a1.theta.to_degrees().round(), -90.0);
        assert_eq!(a1.dtheta.to_degrees().round(), -270.0);

        let a2 = Arc::new(p0, 5.0, 5.0, 0.0, false, true, p1);
        assert_eq!(a2.sweep, true);
        assert_eq!(a2.c, Vector2(0.0, 5.0));
        assert_eq!(a2.theta.to_degrees().round(), -90.0);
        assert_eq!(a2.dtheta.to_degrees().round(), 90.0);

        let a3 = Arc::new(p0, 5.0, 5.0, 0.0, true, true, p1);
        assert_eq!(a3.large, true);
        assert_eq!(a3.sweep, true);
        assert_eq!(a3.c, Vector2(5.0, 0.0));
        assert_eq!(a3.theta.to_degrees().round(), 180.0);
        assert_eq!(a3.dtheta.to_degrees().round(), 270.0);
    }

    #[test]
    fn arc_new_rx_ry() {
        let p0 = Vector2(0.0, 0.0);

        let a0 = Arc::new(p0, 10.0, 5.0, 0.0, false, false, Vector2(10.0, 5.0));
        assert_eq!(a0.rx, 10.0);
        assert_eq!(a0.ry, 5.0);
        assert_eq!(a0.c, Vector2(10.0, 0.0));
        assert_eq!(a0.theta.to_degrees().round(), 180.0);
        assert_eq!(a0.dtheta.to_degrees().round(), -90.0);

        let a1 = Arc::new(p0, 5.0, 10.0, 0.0, false, false, Vector2(5.0, 10.0));
        assert_eq!(a1.rx, 5.0);
        assert_eq!(a1.ry, 10.0);
        assert_eq!(a1.c, Vector2(5.0, 0.0));
        assert_eq!(a1.theta.to_degrees().round(), 180.0);
        assert_eq!(a1.dtheta.to_degrees().round(), -90.0);
    }

    #[test]
    fn arc_new_rotation() {
        let p0 = Vector2(0.0, 0.0);

        let a0 = Arc::new(p0, 10.0, 5.0, 90.0, false, false, Vector2(5.0, 10.0));
        assert_eq!(a0.rx, 10.0);
        assert_eq!(a0.ry, 5.0);
        assert_eq!(a0.c, Vector2(5.0, 0.0));
        assert_eq!(a0.theta.to_degrees().round(), 90.0);
        assert_eq!(a0.dtheta.to_degrees().round(), -90.0);

        let a0 = Arc::new(p0, 10.0, 5.0, -90.0, false, false, Vector2(5.0, 10.0));
        assert_eq!(a0.rx, 10.0);
        assert_eq!(a0.ry, 5.0);
        assert_eq!(a0.c, Vector2(5.0, 0.0));
        assert_eq!(a0.theta.to_degrees().round(), -90.0);
        assert_eq!(a0.dtheta.to_degrees().round(), -90.0);
    }

    #[test]
    fn arc_points_cases() {
        let p0 = Vector2(100.0, 100.0);
        let p1 = Vector2(150.0, 150.0);
        let target = Arc::new(p0, 50.0, 50.0, 0.0, false, false, p1);

        let s0 = target.get_appro_points(1);
        assert_eq!(s0.get(0).unwrap().0.round(), 100.0);
        assert_eq!(s0.get(0).unwrap().1.round(), 100.0);
        assert_eq!(s0.get(1).unwrap().0.round(), 150.0);
        assert_eq!(s0.get(1).unwrap().1.round(), 150.0);

        let s1 = target.get_appro_points(4);
        assert_eq!(s1.get(0).unwrap().0.round(), 100.0);
        assert_eq!(s1.get(0).unwrap().1.round(), 100.0);
        assert_eq!(s1.get(1).unwrap().0.round(), 104.0);
        assert_eq!(s1.get(1).unwrap().1.round(), 119.0);
        assert_eq!(s1.get(2).unwrap().0.round(), 115.0);
        assert_eq!(s1.get(2).unwrap().1.round(), 135.0);
        assert_eq!(s1.get(3).unwrap().0.round(), 131.0);
        assert_eq!(s1.get(3).unwrap().1.round(), 146.0);
        assert_eq!(s1.get(4).unwrap().0.round(), 150.0);
        assert_eq!(s1.get(4).unwrap().1.round(), 150.0);
    }

    #[test]
    fn arc_points_rotated() {
        let p0 = Vector2(100.0, 100.0);
        let p1 = Vector2(150.0, 150.0);

        let s0 = Arc::new(p0, 50.0, 100.0, 0.0, false, false, p1).get_appro_points(3);
        assert_eq!(s0.get(0).unwrap().0.round(), 100.0);
        assert_eq!(s0.get(0).unwrap().1.round(), 100.0);
        assert_eq!(s0.get(1).unwrap().0.round(), 113.0);
        assert_eq!(s0.get(1).unwrap().1.round(), 130.0);
        assert_eq!(s0.get(2).unwrap().0.round(), 130.0);
        assert_eq!(s0.get(2).unwrap().1.round(), 147.0);
        assert_eq!(s0.get(3).unwrap().0.round(), 150.0);
        assert_eq!(s0.get(3).unwrap().1.round(), 150.0);

        let s1 = Arc::new(p0, 100.0, 50.0, 90.0, false, false, p1).get_appro_points(3);
        assert_eq!(s1.get(0).unwrap().0.round(), 100.0);
        assert_eq!(s1.get(0).unwrap().1.round(), 100.0);
        assert_eq!(s1.get(1).unwrap().0.round(), 113.0);
        assert_eq!(s1.get(1).unwrap().1.round(), 130.0);
        assert_eq!(s1.get(2).unwrap().0.round(), 130.0);
        assert_eq!(s1.get(2).unwrap().1.round(), 147.0);
        assert_eq!(s1.get(3).unwrap().0.round(), 150.0);
        assert_eq!(s1.get(3).unwrap().1.round(), 150.0);
    }
}
