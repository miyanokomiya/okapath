use crate::path::PathSegment;

type ParserFn = fn(text: &str, index: usize) -> Option<(&str, usize)>;
static A: [ParserFn; 2] = [parse_space, parse_number];

pub fn parse(d: &str) -> Vec<PathSegment> {
    let len = d.len();
    let mut cursor = 0;

    while cursor < len {
        for parser_fn in A {
            match parser_fn(d, cursor) {
                Some((text, size)) => {
                    cursor += size;
                    break;
                }
                None => {}
            }
            cursor = len;
        }
    }
    vec![]
}

fn parse_space(text: &str, index: usize) -> Option<(&str, usize)> {
    let mut value: Vec<char> = vec![];
    None
}

fn parse_number(text: &str, index: usize) -> Option<(&str, usize)> {
    let mut value: Vec<char> = vec![];
    None
}

// struct Scanner {
//     cursor: usize,
//     characters: Vec<char>,
// }
//
// impl Scanner {
//     fn new(text: &str) -> Self {
//         Self {
//             cursor: 0,
//             characters: text.chars().collect(),
//         }
//     }
//
//     fn finished(self) -> bool {
//         self.cursor >= self.characters.len()
//     }
//
//     fn step(&mut self) -> Option<&char> {
//         let ret = self.characters.get(self.cursor);
//         self.cursor += 1;
//         ret
//     }
// }
//
