use crate::path::PathSegment;

type ParserFn<'a> = fn(text: &Vec<char>, index: usize) -> Option<(&'a str, usize)>;
static PARSER_FN: [ParserFn; 3] = [parse_space, parse_number, parse_command];

pub fn parse(d: &str) -> Vec<PathSegment> {
    vec![]
}

fn split(d: &str) -> Vec<&str> {
    let text: Vec<char> = d.chars().collect();
    let len = text.len();
    let mut cursor = 0;
    let mut ret: Vec<&str> = vec![];

    while cursor < len {
        let mut hit = false;
        for parser_fn in PARSER_FN {
            println!("{:?} {:?}", text, cursor);
            hit = match parser_fn(&text, cursor) {
                Some((value, size)) => {
                    cursor += size;
                    ret.push(value.clone());
                    true
                }
                None => false,
            };
            if hit {
                break;
            }
        }
        if !hit {
            panic!("Invalid path.");
        }
    }
    ret
}

fn parse_space<'a>(text: &Vec<char>, index: usize) -> Option<(&'a str, usize)> {
    let mut value: Vec<char> = vec![];
    None
}

fn parse_number<'a>(text: &Vec<char>, index: usize) -> Option<(&'a str, usize)> {
    let mut value: Vec<char> = vec![];
    None
}

fn parse_command<'a>(text: &Vec<char>, index: usize) -> Option<(&'a str, usize)> {
    println!("{:?} {:?}", text, index);
    match text.get(index) {
        Some('M') => Some(("M", 1)),
        _ => None,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn split_1() {
        assert_eq!(split("M"), vec!["M"]);
        assert_eq!(split("MM"), vec!["M", "M"]);
    }
}
