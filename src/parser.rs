use crate::path::PathSegment;

type ParserFn = fn(text: &Vec<char>, index: usize) -> Option<(String, usize)>;
static PARSER_FN: [ParserFn; 2] = [parse_number, parse_command];

pub fn parse(d: &str) -> Vec<PathSegment> {
    vec![]
}

fn split(d: &str) -> Vec<String> {
    let text: Vec<char> = d.chars().collect();
    let len = text.len();
    let mut cursor = 0;
    let mut ret: Vec<String> = vec![];

    cursor += drop_whitespace(&text, cursor);
    while cursor < len {
        let mut hit = false;

        for parser_fn in PARSER_FN {
            hit = match parser_fn(&text, cursor) {
                Some((value, size)) => {
                    cursor += size;
                    ret.push(value);
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
            Some(' ' | ',') => {
                cursor += 1;
            }
            _ => {
                break;
            }
        }
    }
    cursor - index
}

fn parse_number(text: &Vec<char>, index: usize) -> Option<(String, usize)> {
    let len = text.len();
    let mut cursor = index;
    let mut value: String = String::new();

    match text.get(cursor) {
        Some('-') => {
            cursor += 1;
            value.push('-');
        }
        Some('+') => {
            cursor += 1;
        }
        _ => {}
    }

    while cursor < len {
        let c = text.get(cursor);
        match c {
            Some('0'..='9') => {
                cursor += 1;
                value.push(*c.unwrap());
            }
            Some('.') => {
                cursor += 1;
                value.push('.');
            }
            _ => {
                break;
            }
        }
    }
    if cursor != index {
        Some((value, cursor - index))
    } else {
        None
    }
}

fn parse_command(text: &Vec<char>, index: usize) -> Option<(String, usize)> {
    match text.get(index) {
        Some('M') => Some((String::from("M"), 1)),
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

    #[test]
    fn split_parse_number() {
        assert_eq!(split("M 0"), vec!["M", "0"]);
        assert_eq!(split("M 1234567890"), vec!["M", "1234567890"]);
        assert_eq!(split("M -12"), vec!["M", "-12"]);
        assert_eq!(split("M -12-9"), vec!["M", "-12", "-9"]);
        assert_eq!(split("M +12+9"), vec!["M", "12", "9"]);
        assert_eq!(split("M -1.2 1"), vec!["M", "-1.2", "1"]);
    }
}
