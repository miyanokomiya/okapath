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
    let mut length: f64 = 0.0;

    for seg in segments {
        let (d, p) = get_path_segment_length(&seg, &current);
        current = p;
        length += d;
    }

    length
}

fn get_path_segment_length(path_segment: &PathSegment, from: &Vector2) -> (f64, Vector2) {
    match path_segment._type.as_str() {
        "L" => get_length_l(&path_segment.values, from),
        "l" => get_length_l_relative(&path_segment.values, from),
        _ => (0.0, *from),
    }
}

fn get_length_l(values: &Vec<f64>, from: &Vector2) -> (f64, Vector2) {
    let v = Vector2(*values.get(0).unwrap(), *values.get(1).unwrap());
    ((v - *from).norm(), v)
}

fn get_length_l_relative(values: &Vec<f64>, from: &Vector2) -> (f64, Vector2) {
    let v = Vector2(*values.get(0).unwrap(), *values.get(1).unwrap());
    (v.norm(), v + *from)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn get_path_segment_length_l() {
        assert_eq!(
            get_path_segment_length(
                &PathSegment::new("L".to_string(), vec![4.0, 6.0]),
                &Vector2(1.0, 2.0)
            ),
            (5.0, Vector2(4.0, 6.0))
        );

        assert_eq!(
            get_path_segment_length(
                &PathSegment::new("l".to_string(), vec![3.0, 4.0]),
                &Vector2(1.0, 2.0)
            ),
            (5.0, Vector2(4.0, 6.0))
        );
    }
}
