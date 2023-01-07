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
        let op = text.get(cursor);
        match op {
            Some(c) => match c {
                '0'..='9' => {
                    cursor += 1;
                    value.push(*c);
                }
                '.' => {
                    cursor += 1;
                    value.push('.');
                }
                _ => {
                    break;
                }
            },
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
        Some(c) => match c {
            'M' | 'm' | 'L' | 'l' | 'H' | 'h' | 'V' | 'v' | 'Q' | 'q' | 'T' | 't' | 'C' | 'c'
            | 'S' | 's' | 'A' | 'a' | 'Z' | 'z' => Some((String::from(*c), 1)),
            _ => None,
        },
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

    #[test]
    fn split_command() {
        assert_eq!(split("M m"), vec!["M", "m"]);
        assert_eq!(split("L l"), vec!["L", "l"]);
        assert_eq!(split("H h"), vec!["H", "h"]);
        assert_eq!(split("V v"), vec!["V", "v"]);
        assert_eq!(split("Q q"), vec!["Q", "q"]);
        assert_eq!(split("T t"), vec!["T", "t"]);
        assert_eq!(split("C c"), vec!["C", "c"]);
        assert_eq!(split("S s"), vec!["S", "s"]);
        assert_eq!(split("A a"), vec!["A", "a"]);
        assert_eq!(split("Z z"), vec!["Z", "z"]);
    }

    #[test]
    fn split_cases() {
        assert_eq!(
            split("M1 2L34,56z"),
            vec!["M", "1", "2", "L", "34", "56", "z"]
        );
        assert_eq!(
            split("M-1.1,2.2L3.4-5.6z"),
            vec!["M", "-1.1", "2.2", "L", "3.4", "-5.6", "z"]
        );
    }
}
