use crate::vector::{Arc, Bezier2, Bezier3, Lerpable, Vector2};

// https://svgwg.org/specs/paths/#InterfaceSVGPathSegment
#[derive(Debug, PartialEq)]
pub struct PathSegment {
    pub _type: char,
    pub values: Vec<f64>,
}

impl PathSegment {
    pub fn new(_type: char, values: Vec<f64>) -> Self {
        PathSegment { _type, values }
    }
}

pub fn get_path_length(segments: &Vec<PathSegment>) -> f64 {
    let mut length: f64 = 0.0;
    let mut start: Option<Vector2> = None;
    let mut current = Vector2(0.0, 0.0);
    let mut control: Option<Vector2> = None;

    for seg in segments {
        match seg._type {
            'Z' | 'z' => match start {
                Some(s) => {
                    length += (current - s).norm();
                    current = s;
                    control = None;
                }
                _ => {}
            },
            'M' => {
                let p = get_point_m(&seg.values);
                start = Some(p);
                current = p;
                control = None;
            }
            'm' => {
                let p = get_point_m(&seg.values) + current;
                start = Some(p);
                current = p;
                control = None;
            }
            'L' => {
                if start.is_none() {
                    let p = get_point_m(&seg.values);
                    start = Some(p);
                    current = p;
                } else {
                    let (d, p) = get_length_l(&seg.values, &current);
                    length += d;
                    current = p;
                }
                control = None;
            }
            'l' => {
                if start.is_none() {
                    let p = get_point_m(&seg.values);
                    start = Some(p);
                    current = p;
                } else {
                    let (d, p) = get_length_l_relative(&seg.values, &current);
                    length += d;
                    current = p;
                }
                control = None;
            }
            'H' | 'h' => {
                let (d, p) = get_length_h(&seg.values, &current);
                length += d;
                current = p;
                control = None;
            }
            'V' | 'v' => {
                let (d, p) = get_length_v(&seg.values, &current);
                length += d;
                current = p;
                control = None;
            }
            'Q' => {
                let (d, p1, p2) = get_length_q(&seg.values, &current);
                length += d;
                current = p2;
                control = Some(p1);
            }
            'q' => {
                let (d, p1, p2) = get_length_q_relative(&seg.values, &current);
                length += d;
                current = p2;
                control = Some(p1);
            }
            'T' => {
                let (d, p1, p2) = get_length_t(&seg.values, &current, &control.unwrap_or(current));
                length += d;
                current = p2;
                control = Some(p1);
            }
            't' => {
                let (d, p1, p2) =
                    get_length_t_relative(&seg.values, &current, &control.unwrap_or(current));
                length += d;
                current = p2;
                control = Some(p1);
            }
            'C' => {
                let (d, p1, p2) = get_length_c(&seg.values, &current);
                length += d;
                current = p2;
                control = Some(p1);
            }
            'c' => {
                let (d, p1, p2) = get_length_c_relative(&seg.values, &current);
                length += d;
                current = p2;
                control = Some(p1);
            }
            'S' => {
                let (d, p1, p2) = get_length_s(&seg.values, &current, &control.unwrap_or(current));
                length += d;
                current = p2;
                control = Some(p1);
            }
            's' => {
                let (d, p1, p2) =
                    get_length_s_relative(&seg.values, &current, &control.unwrap_or(current));
                length += d;
                current = p2;
                control = Some(p1);
            }
            'A' => {
                let (d, p1) = get_length_a(&seg.values, &current);
                length += d;
                current = p1;
                control = None;
            }
            'a' => {
                let (d, p1) = get_length_a_relative(&seg.values, &current);
                length += d;
                current = p1;
                control = None;
            }
            _ => {}
        };
    }

    length
}

fn get_number(values: &Vec<f64>, i: usize) -> f64 {
    *values.get(i).unwrap()
}

fn get_vector(values: &Vec<f64>, xi: usize, yi: usize) -> Vector2 {
    Vector2(get_number(values, xi), get_number(values, yi))
}

fn get_bool(values: &Vec<f64>, i: usize) -> bool {
    get_number(values, i) != 0.0
}

fn get_point_m(values: &Vec<f64>) -> Vector2 {
    get_vector(values, 0, 1)
}

fn get_length_l(values: &Vec<f64>, from: &Vector2) -> (f64, Vector2) {
    let v = get_vector(values, 0, 1);
    ((v - *from).norm(), v)
}

fn get_length_l_relative(values: &Vec<f64>, from: &Vector2) -> (f64, Vector2) {
    let v = get_vector(values, 0, 1);
    (v.norm(), v + *from)
}

fn get_length_h(values: &Vec<f64>, from: &Vector2) -> (f64, Vector2) {
    let d = get_number(values, 0);
    (d, Vector2(d + from.0, from.1))
}

fn get_length_v(values: &Vec<f64>, from: &Vector2) -> (f64, Vector2) {
    let d = get_number(values, 0);
    (d, Vector2(from.0, d + from.1))
}

static SPLIT_COUNT: usize = 20;

fn get_length_q(values: &Vec<f64>, from: &Vector2) -> (f64, Vector2, Vector2) {
    let p1 = get_vector(values, 0, 1);
    let p2 = get_vector(values, 2, 3);
    (
        Bezier2::new(*from, p1, p2).get_appro_length(SPLIT_COUNT),
        p1,
        p2,
    )
}

fn get_length_q_relative(values: &Vec<f64>, from: &Vector2) -> (f64, Vector2, Vector2) {
    let p1 = get_vector(values, 0, 1) + *from;
    let p2 = get_vector(values, 2, 3) + *from;
    (
        Bezier2::new(*from, p1, p2).get_appro_length(SPLIT_COUNT),
        p1,
        p2,
    )
}

fn get_length_t(values: &Vec<f64>, from: &Vector2, control: &Vector2) -> (f64, Vector2, Vector2) {
    let p1 = from.multi(2.0) - *control;
    let p2 = get_vector(values, 0, 1);
    (
        Bezier2::new(*from, p1, p2).get_appro_length(SPLIT_COUNT),
        p1,
        p2,
    )
}

fn get_length_t_relative(
    values: &Vec<f64>,
    from: &Vector2,
    control: &Vector2,
) -> (f64, Vector2, Vector2) {
    let p1 = from.multi(2.0) - *control;
    let p2 = get_vector(values, 0, 1) + *from;
    (
        Bezier2::new(*from, p1, p2).get_appro_length(SPLIT_COUNT),
        p1,
        p2,
    )
}

fn get_length_c(values: &Vec<f64>, from: &Vector2) -> (f64, Vector2, Vector2) {
    let p1 = get_vector(values, 0, 1);
    let p2 = get_vector(values, 2, 3);
    let p3 = get_vector(values, 4, 5);
    (
        Bezier3::new(*from, p1, p2, p3).get_appro_length(SPLIT_COUNT),
        p1,
        p2,
    )
}

fn get_length_c_relative(values: &Vec<f64>, from: &Vector2) -> (f64, Vector2, Vector2) {
    let p1 = get_vector(values, 0, 1) + *from;
    let p2 = get_vector(values, 2, 3) + *from;
    let p3 = get_vector(values, 4, 5) + *from;
    (
        Bezier3::new(*from, p1, p2, p3).get_appro_length(SPLIT_COUNT),
        p1,
        p2,
    )
}

fn get_length_s(values: &Vec<f64>, from: &Vector2, control: &Vector2) -> (f64, Vector2, Vector2) {
    let p1 = from.multi(2.0) - *control;
    let p2 = get_vector(values, 0, 1);
    let p3 = get_vector(values, 2, 3);
    (
        Bezier3::new(*from, p1, p2, p3).get_appro_length(SPLIT_COUNT),
        p1,
        p2,
    )
}

fn get_length_s_relative(
    values: &Vec<f64>,
    from: &Vector2,
    control: &Vector2,
) -> (f64, Vector2, Vector2) {
    let p1 = from.multi(2.0) - *control;
    let p2 = get_vector(values, 0, 1) + *from;
    let p3 = get_vector(values, 2, 3) + *from;
    (
        Bezier3::new(*from, p1, p2, p3).get_appro_length(SPLIT_COUNT),
        p1,
        p2,
    )
}

fn get_length_a(values: &Vec<f64>, from: &Vector2) -> (f64, Vector2) {
    let p1 = get_vector(values, 5, 6);
    (
        Arc::new(
            *from,
            get_number(values, 0),
            get_number(values, 1),
            get_number(values, 2),
            get_bool(values, 3),
            get_bool(values, 4),
            p1,
        )
        .get_appro_length(SPLIT_COUNT),
        p1,
    )
}

fn get_length_a_relative(values: &Vec<f64>, from: &Vector2) -> (f64, Vector2) {
    let p1 = get_vector(values, 5, 6) + *from;
    (
        Arc::new(
            *from,
            get_number(values, 0),
            get_number(values, 1),
            get_number(values, 2),
            get_bool(values, 3),
            get_bool(values, 4),
            p1,
        )
        .get_appro_length(SPLIT_COUNT),
        p1,
    )
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn get_path_segment_length_z() {
        assert_eq!(
            get_path_length(&vec![
                PathSegment::new('M', vec![1.0, 1.0]),
                PathSegment::new('L', vec![4.0, 1.0]),
                PathSegment::new('L', vec![4.0, 4.0]),
                PathSegment::new('L', vec![1.0, 4.0]),
                PathSegment::new('Z', vec![]),
                PathSegment::new('m', vec![10.0, 10.0]),
                PathSegment::new('l', vec![3.0, 0.0]),
                PathSegment::new('l', vec![0.0, 3.0]),
                PathSegment::new('l', vec![-3.0, 0.0]),
                PathSegment::new('z', vec![]),
            ]),
            24.0
        );

        assert_eq!(
            get_path_length(&vec![
                PathSegment::new('L', vec![1.0, 1.0]),
                PathSegment::new('L', vec![4.0, 1.0]),
                PathSegment::new('L', vec![4.0, 4.0]),
                PathSegment::new('L', vec![1.0, 4.0]),
                PathSegment::new('Z', vec![]),
            ]),
            12.0
        );

        assert_eq!(
            get_path_length(&vec![
                PathSegment::new('l', vec![1.0, 1.0]),
                PathSegment::new('l', vec![3.0, 0.0]),
                PathSegment::new('l', vec![0.0, 3.0]),
                PathSegment::new('l', vec![-3.0, 0.0]),
                PathSegment::new('z', vec![]),
            ]),
            12.0
        );
    }

    #[test]
    fn get_path_segment_length_m() {
        assert_eq!(
            get_path_length(&vec![
                PathSegment::new('M', vec![1.0, 2.0]),
                PathSegment::new('L', vec![4.0, 6.0]),
                PathSegment::new('M', vec![10.0, 20.0]),
                PathSegment::new('L', vec![11.0, 20.0]),
            ]),
            6.0
        );

        assert_eq!(
            get_path_length(&vec![
                PathSegment::new('m', vec![1.0, 2.0]),
                PathSegment::new('L', vec![4.0, 6.0]),
                PathSegment::new('m', vec![10.0, 20.0]),
                PathSegment::new('l', vec![0.0, 1.0]),
            ]),
            6.0
        );
    }

    #[test]
    fn get_path_segment_length_l() {
        assert_eq!(
            get_path_length(&vec![
                PathSegment::new('M', vec![1.0, 2.0]),
                PathSegment::new('L', vec![4.0, 6.0]),
            ]),
            5.0
        );

        assert_eq!(
            get_path_length(&vec![
                PathSegment::new('M', vec![1.0, 2.0]),
                PathSegment::new('l', vec![3.0, 4.0]),
            ]),
            5.0
        );
    }

    #[test]
    fn get_path_segment_length_h() {
        assert_eq!(
            get_path_length(&vec![
                PathSegment::new('M', vec![1.0, 2.0]),
                PathSegment::new('H', vec![9.0]),
            ]),
            9.0
        );

        assert_eq!(
            get_path_length(&vec![
                PathSegment::new('M', vec![1.0, 2.0]),
                PathSegment::new('h', vec![9.0]),
            ]),
            9.0
        );
    }

    #[test]
    fn get_path_segment_length_v() {
        assert_eq!(
            get_path_length(&vec![
                PathSegment::new('M', vec![1.0, 2.0]),
                PathSegment::new('V', vec![9.0]),
            ]),
            9.0
        );

        assert_eq!(
            get_path_length(&vec![
                PathSegment::new('M', vec![1.0, 2.0]),
                PathSegment::new('v', vec![9.0]),
            ]),
            9.0
        );
    }

    #[test]
    fn get_path_segment_length_q() {
        assert_eq!(
            get_path_length(&vec![
                PathSegment::new('M', vec![10.0, 10.0]),
                PathSegment::new('Q', vec![20.0, 10.0, 20.0, 20.0]),
            ])
            .round(),
            16.0
        );

        assert_eq!(
            get_path_length(&vec![
                PathSegment::new('M', vec![10.0, 10.0]),
                PathSegment::new('q', vec![10.0, 0.0, 10.0, 10.0]),
            ])
            .round(),
            16.0
        );
    }

    #[test]
    fn get_path_segment_length_t() {
        assert_eq!(
            get_path_length(&vec![
                PathSegment::new('M', vec![10.0, 10.0]),
                PathSegment::new('Q', vec![20.0, 10.0, 20.0, 20.0]),
                PathSegment::new('T', vec![30.0, 20.0]),
            ])
            .round(),
            32.0
        );
        assert_eq!(
            get_path_length(&vec![
                PathSegment::new('M', vec![10.0, 10.0]),
                PathSegment::new('T', vec![20.0, 10.0]),
            ])
            .round(),
            10.0,
            "should treat the point as the control if previous control doesn't exist"
        );

        assert_eq!(
            get_path_length(&vec![
                PathSegment::new('M', vec![10.0, 10.0]),
                PathSegment::new('q', vec![10.0, 0.0, 10.0, 10.0]),
                PathSegment::new('t', vec![10.0, 0.0]),
            ])
            .round(),
            32.0
        );
        assert_eq!(
            get_path_length(&vec![
                PathSegment::new('M', vec![10.0, 10.0]),
                PathSegment::new('t', vec![10.0, 0.0]),
            ])
            .round(),
            10.0,
            "should treat the point as the control if previous control doesn't exist"
        );
    }

    #[test]
    fn get_path_segment_length_c() {
        assert_eq!(
            get_path_length(&vec![
                PathSegment::new('M', vec![10.0, 10.0]),
                PathSegment::new('C', vec![20.0, 10.0, 10.0, 20.0, 20.0, 20.0]),
            ])
            .round(),
            17.0
        );

        assert_eq!(
            get_path_length(&vec![
                PathSegment::new('M', vec![10.0, 10.0]),
                PathSegment::new('c', vec![10.0, 0.0, 0.0, 10.0, 10.0, 10.0]),
            ])
            .round(),
            17.0
        );
    }

    #[test]
    fn get_path_segment_length_s() {
        assert_eq!(
            get_path_length(&vec![
                PathSegment::new('M', vec![10.0, 10.0]),
                PathSegment::new('Q', vec![20.0, 10.0, 20.0, 20.0]),
                PathSegment::new('S', vec![30.0, 20.0, 30.0, 30.0]),
            ])
            .round(),
            33.0
        );

        assert_eq!(
            get_path_length(&vec![
                PathSegment::new('M', vec![10.0, 10.0]),
                PathSegment::new('q', vec![10.0, 0.0, 10.0, 10.0]),
                PathSegment::new('s', vec![10.0, 0.0, 10.0, 10.0]),
            ])
            .round(),
            33.0
        );
    }

    #[test]
    fn get_path_segment_length_a() {
        assert_eq!(
            get_path_length(&vec![
                PathSegment::new('M', vec![10.0, 10.0]),
                PathSegment::new('A', vec![10.0, 10.0, 0.0, 0.0, 0.0, 20.0, 20.0]),
            ])
            .round(),
            16.0
        );
        assert_eq!(
            get_path_length(&vec![
                PathSegment::new('M', vec![10.0, 10.0]),
                PathSegment::new('A', vec![10.0, 10.0, 0.0, 1.0, 0.0, 20.0, 20.0]),
            ])
            .round(),
            47.0
        );
        assert_eq!(
            get_path_length(&vec![
                PathSegment::new('M', vec![10.0, 10.0]),
                PathSegment::new('A', vec![10.0, 10.0, 0.0, 0.0, 1.0, 20.0, 20.0]),
            ])
            .round(),
            16.0
        );
        assert_eq!(
            get_path_length(&vec![
                PathSegment::new('M', vec![10.0, 10.0]),
                PathSegment::new('A', vec![10.0, 10.0, 0.0, 1.0, 1.0, 20.0, 20.0]),
            ])
            .round(),
            47.0
        );
    }

    #[test]
    fn get_path_segment_length_a_relative() {
        assert_eq!(
            get_path_length(&vec![
                PathSegment::new('M', vec![10.0, 10.0]),
                PathSegment::new('a', vec![10.0, 10.0, 0.0, 0.0, 0.0, 10.0, 10.0]),
            ])
            .round(),
            16.0
        );
        assert_eq!(
            get_path_length(&vec![
                PathSegment::new('M', vec![10.0, 10.0]),
                PathSegment::new('a', vec![10.0, 10.0, 0.0, 1.0, 0.0, 10.0, 10.0]),
            ])
            .round(),
            47.0
        );
        assert_eq!(
            get_path_length(&vec![
                PathSegment::new('M', vec![10.0, 10.0]),
                PathSegment::new('a', vec![10.0, 10.0, 0.0, 0.0, 1.0, 10.0, 10.0]),
            ])
            .round(),
            16.0
        );
        assert_eq!(
            get_path_length(&vec![
                PathSegment::new('M', vec![10.0, 10.0]),
                PathSegment::new('a', vec![10.0, 10.0, 0.0, 1.0, 1.0, 10.0, 10.0]),
            ])
            .round(),
            47.0
        );
    }
}
