use crate::path::PathSegment;

pub fn parse(d: &str) -> Vec<PathSegment> {
    to_segments(&split(d))
}

fn to_segments(src: &Vec<String>) -> Vec<PathSegment> {
    let mut ret: Vec<PathSegment> = vec![];
    let mut command = 'M';
    let mut param_count = 2;
    let mut cursor = 0;
    let len = src.len();

    while cursor < len {
        let mut current_cursor = cursor;

        match is_command(src[current_cursor].chars().next().unwrap()) {
            Some(c) => {
                command = c;
                param_count = get_param_count(command);
                current_cursor += 1;
            }
            None => {}
        }
        if current_cursor + param_count > len {
            panic!("Lack of parameter: {}", command);
        }

        let values: Vec<f64> = src[current_cursor..(current_cursor + param_count)]
            .iter()
            .map(|s| match s.parse::<f64>() {
                Ok(v) => v,
                Err(_) => panic!("Unexpected parameter: {}", s),
            })
            .collect();
        ret.push(PathSegment::new(command, values));
        current_cursor += param_count;

        if current_cursor == cursor {
            break;
        }
        cursor = current_cursor;
    }

    ret
}

fn get_param_count(command: char) -> usize {
    match command {
        'H' | 'h' | 'V' | 'v' => 1,
        'M' | 'm' | 'L' | 'l' | 'T' | 't' => 2,
        'Q' | 'q' | 'S' | 's' => 4,
        'C' | 'c' => 6,
        'A' | 'a' => 7,
        _ => 0,
    }
}

fn is_command(c: char) -> Option<char> {
    match c {
        'M' | 'm' | 'L' | 'l' | 'H' | 'h' | 'V' | 'v' | 'Q' | 'q' | 'T' | 't' | 'C' | 'c' | 'S'
        | 's' | 'A' | 'a' | 'Z' | 'z' => Some(c),
        _ => None,
    }
}

type ParserFn = fn(text: &Vec<char>, index: usize) -> Option<(String, usize)>;
static PARSER_FN: [ParserFn; 2] = [parse_number, parse_command];

fn split(d: &str) -> Vec<String> {
    let text: Vec<char> = d.chars().collect();
    let mut cursor = 0;
    let mut ret: Vec<String> = vec![];

    cursor += drop_whitespace(&text, cursor);
    while cursor < text.len() {
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
            panic!("Unexpected token: {}", text.get(cursor).unwrap());
        }
        cursor += drop_whitespace(&text, cursor);
    }
    ret
}

fn drop_whitespace(text: &Vec<char>, index: usize) -> usize {
    let mut cursor = index;

    while cursor < text.len() {
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

    while cursor < text.len() {
        match text.get(cursor) {
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
    fn to_segments_cases() {
        assert_eq!(
            to_segments(&split("M 12 9 L1 -2Z")),
            vec![
                PathSegment::new('M', vec![12.0, 9.0]),
                PathSegment::new('L', vec![1.0, -2.0]),
                PathSegment::new('Z', vec![])
            ]
        );

        assert_eq!(
            to_segments(&split("m 1 2 l3 4z")),
            vec![
                PathSegment::new('m', vec![1.0, 2.0]),
                PathSegment::new('l', vec![3.0, 4.0]),
                PathSegment::new('z', vec![]),
            ]
        );

        assert_eq!(
            to_segments(&split("H 1 V 2 h 3 v 4")),
            vec![
                PathSegment::new('H', vec![1.0]),
                PathSegment::new('V', vec![2.0]),
                PathSegment::new('h', vec![3.0]),
                PathSegment::new('v', vec![4.0]),
            ]
        );

        assert_eq!(
            to_segments(&split("Q 1 2 3 4 q 1 2 3 4")),
            vec![
                PathSegment::new('Q', vec![1.0, 2.0, 3.0, 4.0]),
                PathSegment::new('q', vec![1.0, 2.0, 3.0, 4.0]),
            ]
        );

        assert_eq!(
            to_segments(&split("T 1 2 t 1 2")),
            vec![
                PathSegment::new('T', vec![1.0, 2.0]),
                PathSegment::new('t', vec![1.0, 2.0]),
            ]
        );

        assert_eq!(
            to_segments(&split("C 1 2 3 4 5 6 c 1 2 3 4 5 6")),
            vec![
                PathSegment::new('C', vec![1.0, 2.0, 3.0, 4.0, 5.0, 6.0]),
                PathSegment::new('c', vec![1.0, 2.0, 3.0, 4.0, 5.0, 6.0]),
            ]
        );

        assert_eq!(
            to_segments(&split("S 1 2 3 4 s 1 2 3 4")),
            vec![
                PathSegment::new('S', vec![1.0, 2.0, 3.0, 4.0]),
                PathSegment::new('s', vec![1.0, 2.0, 3.0, 4.0]),
            ]
        );

        assert_eq!(
            to_segments(&split("A 1 2 3 4 5 6 7 a 1 2 3 4 5 6 7")),
            vec![
                PathSegment::new('A', vec![1.0, 2.0, 3.0, 4.0, 5.0, 6.0, 7.0]),
                PathSegment::new('a', vec![1.0, 2.0, 3.0, 4.0, 5.0, 6.0, 7.0]),
            ]
        );
    }

    #[test]
    fn to_segments_inherit_previous_command() {
        assert_eq!(
            to_segments(&split("L 12 9 1 2 H 1 2")),
            vec![
                PathSegment::new('L', vec![12.0, 9.0]),
                PathSegment::new('L', vec![1.0, 2.0]),
                PathSegment::new('H', vec![1.0]),
                PathSegment::new('H', vec![2.0])
            ]
        );
    }

    #[test]
    #[should_panic(expected = "Unexpected parameter: L")]
    fn to_segments_panic_for_invalid_parameter() {
        to_segments(&split("L 1 L 2"));
    }

    #[test]
    #[should_panic(expected = "Lack of parameter: L")]
    fn to_segments_panic_for_lack_of_parameter() {
        to_segments(&split("L 1"));
    }

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

    #[test]
    #[should_panic(expected = "Unexpected token: K")]
    fn split_panic_for_unexpected_token() {
        split("L 1 2 K 1 2");
    }
}
