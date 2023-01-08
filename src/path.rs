use crate::vector;
use crate::vector::Vector2;

// https://svgwg.org/specs/paths/#InterfaceSVGPathSegment
#[derive(Debug, PartialEq)]
pub struct PathSegment {
    pub _type: String,
    pub values: Vec<f64>,
}

impl PathSegment {
    pub fn new(_type: String, values: Vec<f64>) -> Self {
        PathSegment { _type, values }
    }
}

pub fn get_path_length(segments: &Vec<PathSegment>) -> f64 {
    let mut length: f64 = 0.0;
    let mut start: Option<Vector2> = None;
    let mut current = Vector2(0.0, 0.0);
    let mut control: Option<Vector2> = None;

    for seg in segments {
        match seg._type.as_str() {
            "Z" | "z" => match start {
                Some(s) => {
                    length += (current - s).norm();
                    current = s;
                    control = None;
                }
                _ => {}
            },
            "M" => {
                let p = get_point_m(&seg.values);
                start = Some(p);
                current = p;
                control = None;
            }
            "m" => {
                let p = get_point_m(&seg.values) + current;
                start = Some(p);
                current = p;
                control = None;
            }
            "L" => {
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
            "l" => {
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
            "H" | "h" => {
                let (d, p) = get_length_h(&seg.values, &current);
                length += d;
                current = p;
                control = None;
            }
            "V" | "v" => {
                let (d, p) = get_length_v(&seg.values, &current);
                length += d;
                current = p;
                control = None;
            }
            "Q" => {
                let (d, p1, p2) = get_length_q(&seg.values, &current);
                length += d;
                current = p2;
                control = Some(p1);
            }
            "q" => {
                let (d, p1, p2) = get_length_q_relative(&seg.values, &current);
                length += d;
                current = p2;
                control = Some(p1);
            }
            "T" => {
                let (d, p1, p2) = get_length_t(&seg.values, &current, &control.unwrap_or(current));
                length += d;
                current = p2;
                control = Some(p1);
            }
            "t" => {
                let (d, p1, p2) =
                    get_length_t_relative(&seg.values, &current, &control.unwrap_or(current));
                length += d;
                current = p2;
                control = Some(p1);
            }
            _ => {}
        };
    }

    length
}

fn get_point_m(values: &Vec<f64>) -> Vector2 {
    Vector2(*values.get(0).unwrap(), *values.get(1).unwrap())
}

fn get_length_l(values: &Vec<f64>, from: &Vector2) -> (f64, Vector2) {
    let v = Vector2(*values.get(0).unwrap(), *values.get(1).unwrap());
    ((v - *from).norm(), v)
}

fn get_length_l_relative(values: &Vec<f64>, from: &Vector2) -> (f64, Vector2) {
    let v = Vector2(*values.get(0).unwrap(), *values.get(1).unwrap());
    (v.norm(), v + *from)
}

fn get_length_h(values: &Vec<f64>, from: &Vector2) -> (f64, Vector2) {
    let d = *values.get(0).unwrap();
    (d, Vector2(d + from.0, from.1))
}

fn get_length_v(values: &Vec<f64>, from: &Vector2) -> (f64, Vector2) {
    let d = *values.get(0).unwrap();
    (d, Vector2(from.0, d + from.1))
}

static SPLIT_COUNT: usize = 20;

fn get_length_q(values: &Vec<f64>, from: &Vector2) -> (f64, Vector2, Vector2) {
    let p1 = Vector2(*values.get(0).unwrap(), *values.get(1).unwrap());
    let p2 = Vector2(*values.get(2).unwrap(), *values.get(3).unwrap());
    (
        vector::get_polyline_length(&vector::get_bezier_q_points(from, &p1, &p2, SPLIT_COUNT)),
        p1,
        p2,
    )
}

fn get_length_q_relative(values: &Vec<f64>, from: &Vector2) -> (f64, Vector2, Vector2) {
    let p1 = Vector2(*values.get(0).unwrap(), *values.get(1).unwrap()) + *from;
    let p2 = Vector2(*values.get(2).unwrap(), *values.get(3).unwrap()) + *from;
    (
        vector::get_polyline_length(&vector::get_bezier_q_points(from, &p1, &p2, SPLIT_COUNT)),
        p1,
        p2,
    )
}

fn get_length_t(values: &Vec<f64>, from: &Vector2, control: &Vector2) -> (f64, Vector2, Vector2) {
    let p1 = from.multi(2.0) - *control;
    let p2 = Vector2(*values.get(0).unwrap(), *values.get(1).unwrap());
    (
        vector::get_polyline_length(&vector::get_bezier_q_points(from, &p1, &p2, SPLIT_COUNT)),
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
    let p2 = Vector2(*values.get(0).unwrap(), *values.get(1).unwrap()) + *from;
    (
        vector::get_polyline_length(&vector::get_bezier_q_points(from, &p1, &p2, SPLIT_COUNT)),
        p1,
        p2,
    )
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn get_path_segment_length_z() {
        assert_eq!(
            get_path_length(&vec![
                PathSegment::new("M".to_string(), vec![1.0, 1.0]),
                PathSegment::new("L".to_string(), vec![4.0, 1.0]),
                PathSegment::new("L".to_string(), vec![4.0, 4.0]),
                PathSegment::new("L".to_string(), vec![1.0, 4.0]),
                PathSegment::new("Z".to_string(), vec![]),
                PathSegment::new("m".to_string(), vec![10.0, 10.0]),
                PathSegment::new("l".to_string(), vec![3.0, 0.0]),
                PathSegment::new("l".to_string(), vec![0.0, 3.0]),
                PathSegment::new("l".to_string(), vec![-3.0, 0.0]),
                PathSegment::new("z".to_string(), vec![]),
            ]),
            24.0
        );

        assert_eq!(
            get_path_length(&vec![
                PathSegment::new("L".to_string(), vec![1.0, 1.0]),
                PathSegment::new("L".to_string(), vec![4.0, 1.0]),
                PathSegment::new("L".to_string(), vec![4.0, 4.0]),
                PathSegment::new("L".to_string(), vec![1.0, 4.0]),
                PathSegment::new("Z".to_string(), vec![]),
            ]),
            12.0
        );

        assert_eq!(
            get_path_length(&vec![
                PathSegment::new("l".to_string(), vec![1.0, 1.0]),
                PathSegment::new("l".to_string(), vec![3.0, 0.0]),
                PathSegment::new("l".to_string(), vec![0.0, 3.0]),
                PathSegment::new("l".to_string(), vec![-3.0, 0.0]),
                PathSegment::new("z".to_string(), vec![]),
            ]),
            12.0
        );
    }

    #[test]
    fn get_path_segment_length_m() {
        assert_eq!(
            get_path_length(&vec![
                PathSegment::new("M".to_string(), vec![1.0, 2.0]),
                PathSegment::new("L".to_string(), vec![4.0, 6.0]),
                PathSegment::new("M".to_string(), vec![10.0, 20.0]),
                PathSegment::new("L".to_string(), vec![11.0, 20.0]),
            ]),
            6.0
        );

        assert_eq!(
            get_path_length(&vec![
                PathSegment::new("m".to_string(), vec![1.0, 2.0]),
                PathSegment::new("L".to_string(), vec![4.0, 6.0]),
                PathSegment::new("m".to_string(), vec![10.0, 20.0]),
                PathSegment::new("l".to_string(), vec![0.0, 1.0]),
            ]),
            6.0
        );
    }

    #[test]
    fn get_path_segment_length_l() {
        assert_eq!(
            get_path_length(&vec![
                PathSegment::new("M".to_string(), vec![1.0, 2.0]),
                PathSegment::new("L".to_string(), vec![4.0, 6.0]),
            ]),
            5.0
        );

        assert_eq!(
            get_path_length(&vec![
                PathSegment::new("M".to_string(), vec![1.0, 2.0]),
                PathSegment::new("l".to_string(), vec![3.0, 4.0]),
            ]),
            5.0
        );
    }

    #[test]
    fn get_path_segment_length_h() {
        assert_eq!(
            get_path_length(&vec![
                PathSegment::new("M".to_string(), vec![1.0, 2.0]),
                PathSegment::new("H".to_string(), vec![9.0]),
            ]),
            9.0
        );

        assert_eq!(
            get_path_length(&vec![
                PathSegment::new("M".to_string(), vec![1.0, 2.0]),
                PathSegment::new("h".to_string(), vec![9.0]),
            ]),
            9.0
        );
    }

    #[test]
    fn get_path_segment_length_v() {
        assert_eq!(
            get_path_length(&vec![
                PathSegment::new("M".to_string(), vec![1.0, 2.0]),
                PathSegment::new("V".to_string(), vec![9.0]),
            ]),
            9.0
        );

        assert_eq!(
            get_path_length(&vec![
                PathSegment::new("M".to_string(), vec![1.0, 2.0]),
                PathSegment::new("v".to_string(), vec![9.0]),
            ]),
            9.0
        );
    }

    #[test]
    fn get_path_segment_length_q() {
        assert_eq!(
            get_path_length(&vec![
                PathSegment::new("M".to_string(), vec![10.0, 10.0]),
                PathSegment::new("Q".to_string(), vec![20.0, 10.0, 20.0, 20.0]),
            ])
            .round(),
            16.0
        );

        assert_eq!(
            get_path_length(&vec![
                PathSegment::new("M".to_string(), vec![10.0, 10.0]),
                PathSegment::new("q".to_string(), vec![10.0, 0.0, 10.0, 10.0]),
            ])
            .round(),
            16.0
        );
    }

    #[test]
    fn get_path_segment_length_t() {
        assert_eq!(
            get_path_length(&vec![
                PathSegment::new("M".to_string(), vec![10.0, 10.0]),
                PathSegment::new("Q".to_string(), vec![20.0, 10.0, 20.0, 20.0]),
                PathSegment::new("T".to_string(), vec![30.0, 20.0]),
            ])
            .round(),
            32.0
        );
        assert_eq!(
            get_path_length(&vec![
                PathSegment::new("M".to_string(), vec![10.0, 10.0]),
                PathSegment::new("T".to_string(), vec![20.0, 10.0]),
            ])
            .round(),
            10.0,
            "should treat the point as the control if previous control doesn't exist"
        );

        assert_eq!(
            get_path_length(&vec![
                PathSegment::new("M".to_string(), vec![10.0, 10.0]),
                PathSegment::new("q".to_string(), vec![10.0, 0.0, 10.0, 10.0]),
                PathSegment::new("t".to_string(), vec![10.0, 0.0]),
            ])
            .round(),
            32.0
        );
        assert_eq!(
            get_path_length(&vec![
                PathSegment::new("M".to_string(), vec![10.0, 10.0]),
                PathSegment::new("t".to_string(), vec![10.0, 0.0]),
            ])
            .round(),
            10.0,
            "should treat the point as the control if previous control doesn't exist"
        );
    }
}
