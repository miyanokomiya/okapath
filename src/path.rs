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
    let mut current = Vector2(0.0, 0.0);
    let mut control: Option<Vector2> = None;
    let mut length: f64 = 0.0;

    for seg in segments {
        let (d, p, c) = get_path_segment_length(&seg, &current, &control);
        current = p;
        control = c;
        length += d;
    }

    length
}

type LengthParams = (f64, Vector2, Option<Vector2>);

fn get_path_segment_length(
    path_segment: &PathSegment,
    from: &Vector2,
    _controll: &Option<Vector2>,
) -> LengthParams {
    match path_segment._type.as_str() {
        "M" => get_length_m(&path_segment.values),
        "m" => get_length_m_relative(&path_segment.values, from),
        "L" => get_length_l(&path_segment.values, from),
        "l" => get_length_l_relative(&path_segment.values, from),
        "H" | "h" => get_length_h(&path_segment.values, from),
        "V" | "v" => get_length_v(&path_segment.values, from),
        _ => (0.0, *from, None),
    }
}

fn get_length_m(values: &Vec<f64>) -> LengthParams {
    let v = Vector2(*values.get(0).unwrap(), *values.get(1).unwrap());
    (0.0, v, None)
}

fn get_length_m_relative(values: &Vec<f64>, from: &Vector2) -> LengthParams {
    let v = Vector2(*values.get(0).unwrap(), *values.get(1).unwrap());
    (0.0, v + *from, None)
}

fn get_length_l(values: &Vec<f64>, from: &Vector2) -> LengthParams {
    let v = Vector2(*values.get(0).unwrap(), *values.get(1).unwrap());
    ((v - *from).norm(), v, None)
}

fn get_length_l_relative(values: &Vec<f64>, from: &Vector2) -> LengthParams {
    let v = Vector2(*values.get(0).unwrap(), *values.get(1).unwrap());
    (v.norm(), v + *from, None)
}

fn get_length_h(values: &Vec<f64>, from: &Vector2) -> LengthParams {
    let d = *values.get(0).unwrap();
    (d, Vector2(d + from.0, from.1), None)
}

fn get_length_v(values: &Vec<f64>, from: &Vector2) -> LengthParams {
    let d = *values.get(0).unwrap();
    (d, Vector2(from.0, d + from.1), None)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn get_path_segment_length_m() {
        assert_eq!(
            get_path_segment_length(
                &PathSegment::new("M".to_string(), vec![4.0, 6.0]),
                &Vector2(1.0, 2.0),
                &None
            ),
            (0.0, Vector2(4.0, 6.0), None)
        );

        assert_eq!(
            get_path_segment_length(
                &PathSegment::new("m".to_string(), vec![3.0, 4.0]),
                &Vector2(1.0, 2.0),
                &None
            ),
            (0.0, Vector2(4.0, 6.0), None)
        );
    }

    #[test]
    fn get_path_segment_length_l() {
        assert_eq!(
            get_path_segment_length(
                &PathSegment::new("L".to_string(), vec![4.0, 6.0]),
                &Vector2(1.0, 2.0),
                &None
            ),
            (5.0, Vector2(4.0, 6.0), None)
        );

        assert_eq!(
            get_path_segment_length(
                &PathSegment::new("l".to_string(), vec![3.0, 4.0]),
                &Vector2(1.0, 2.0),
                &None
            ),
            (5.0, Vector2(4.0, 6.0), None)
        );
    }

    #[test]
    fn get_path_segment_length_h() {
        assert_eq!(
            get_path_segment_length(
                &PathSegment::new("H".to_string(), vec![4.0]),
                &Vector2(1.0, 2.0),
                &None
            ),
            (4.0, Vector2(5.0, 2.0), None)
        );

        assert_eq!(
            get_path_segment_length(
                &PathSegment::new("h".to_string(), vec![4.0]),
                &Vector2(1.0, 2.0),
                &None
            ),
            (4.0, Vector2(5.0, 2.0), None)
        );
    }

    #[test]
    fn get_path_segment_length_v() {
        assert_eq!(
            get_path_segment_length(
                &PathSegment::new("V".to_string(), vec![4.0]),
                &Vector2(1.0, 2.0),
                &None
            ),
            (4.0, Vector2(1.0, 6.0), None)
        );

        assert_eq!(
            get_path_segment_length(
                &PathSegment::new("v".to_string(), vec![4.0]),
                &Vector2(1.0, 2.0),
                &None
            ),
            (4.0, Vector2(1.0, 6.0), None)
        );
    }
}
