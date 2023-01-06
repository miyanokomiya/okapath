use crate::path::PathSegment;

type ParserFn<'a> = fn(text: &Vec<char>, index: usize) -> Option<(&'a str, usize)>;
static PARSER_FN: [ParserFn; 2] = [parse_number, parse_command];

pub fn parse(d: &str) -> Vec<PathSegment> {
    vec![]
}

fn split(d: &str) -> Vec<&str> {
    let text: Vec<char> = d.chars().collect();
    let len = text.len();
    let mut cursor = 0;
    let mut ret: Vec<&str> = vec![];

    cursor += drop_whitespace(&text, cursor);
    while cursor < len {
        let mut hit = false;

        for parser_fn in PARSER_FN {
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
        cursor += drop_whitespace(&text, cursor);
    }
    ret
}

fn drop_whitespace(text: &Vec<char>, index: usize) -> usize {
    let len = text.len();
    let mut cursor = index;

    while cursor < len {
        match text.get(cursor) {
            Some(' ') => {
                cursor += 1;
            }
            Some(',') => {
                cursor += 1;
            }
            _ => {
                break;
            }
        }
    }
    cursor - index
}

fn parse_number<'a>(text: &Vec<char>, index: usize) -> Option<(&'a str, usize)> {
    let mut value: Vec<char> = vec![];
    None
}

fn parse_command<'a>(text: &Vec<char>, index: usize) -> Option<(&'a str, usize)> {
    match text.get(index) {
        Some('M') => Some(("M", 1)),
        _ => None,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn split_drop_whitespace() {
        assert_eq!(split("M M"), vec!["M", "M"]);
        assert_eq!(split("M  M"), vec!["M", "M"]);
        assert_eq!(split("M,M"), vec!["M", "M"]);
        assert_eq!(split("M,,M"), vec!["M", "M"]);
        assert_eq!(split(" M, ,M "), vec!["M", "M"]);
    }
}
