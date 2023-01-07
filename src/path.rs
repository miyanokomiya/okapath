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
    segments.iter().map(|s| get_path_segment_length(s)).sum()
}

fn get_path_segment_length(path_segment: &PathSegment) -> f64 {
    path_segment.values.len() as f64
}
