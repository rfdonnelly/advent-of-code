use crate::input;

use tap::prelude::*;

const DAY: usize = 10;

pub fn run() -> String {
    let input = input(DAY);
    let mut output = String::new();
    let time = std::time::Instant::now();
    output += &format!("d{:02}p1: {} in {:?}\n", DAY, p1(&input), time.elapsed());
    let time = std::time::Instant::now();
    output += &format!("d{:02}p2: {} in {:?}\n", DAY, p2(&input), time.elapsed());
    output
}

#[derive(Debug, Copy, Clone)]
struct Delim {
    kind: DelimKind,
    side: Side,
}

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
enum DelimKind {
    Paren,
    Bracket,
    Brace,
    Angle,
}

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
enum Side {
    Left,
    Right,
}

use DelimKind::*;
use Side::*;

impl From<char> for Delim {
    fn from(c: char) -> Self {
        match c {
            '(' => Delim {
                kind: Paren,
                side: Left,
            },
            ')' => Delim {
                kind: Paren,
                side: Right,
            },
            '[' => Delim {
                kind: Bracket,
                side: Left,
            },
            ']' => Delim {
                kind: Bracket,
                side: Right,
            },
            '{' => Delim {
                kind: Brace,
                side: Left,
            },
            '}' => Delim {
                kind: Brace,
                side: Right,
            },
            '<' => Delim {
                kind: Angle,
                side: Left,
            },
            '>' => Delim {
                kind: Angle,
                side: Right,
            },
            _ => unreachable!(),
        }
    }
}

impl Delim {
    fn is_match(&self, other: Option<&Delim>) -> bool {
        if self.side == Left {
            true
        } else if let Some(other) = other {
            self.kind == other.kind && other.side == Left && self.side == Right
        } else {
            unreachable!()
        }
    }

    fn p1_score(&self) -> usize {
        match self.kind {
            Paren => 3,
            Bracket => 57,
            Brace => 1197,
            Angle => 25137,
        }
    }

    fn p2_score(&self) -> usize {
        match self.kind {
            Paren => 1,
            Bracket => 2,
            Brace => 3,
            Angle => 4,
        }
    }
}

enum Result {
    Error(usize),
    Incomplete(Vec<Delim>),
}

/// Returns Some(score) if syntax error present
/// Returns None if syntax error not present
fn syntax_error_score(s: &str) -> Result {
    let delims = s.chars().map(Delim::from).collect::<Vec<Delim>>();

    let mut stack = vec![];
    for delim in delims {
        if delim.is_match(stack.last()) {
            match delim.side {
                Left => {
                    stack.push(delim);
                }
                Right => {
                    stack.pop();
                }
            }
        } else {
            return Result::Error(delim.p1_score());
        }
    }

    Result::Incomplete(stack)
}

fn p1(input: &str) -> usize {
    input
        .lines()
        .filter_map(|line| match syntax_error_score(line) {
            Result::Error(x) => Some(x),
            Result::Incomplete(_) => None,
        })
        .sum()
}

fn completion_score(stack: &[Delim]) -> usize {
    stack
        .iter()
        .rev()
        .map(Delim::p2_score)
        .fold(0, |total_score, delim_score| total_score * 5 + delim_score)
}

fn p2(input: &str) -> usize {
    input
        .lines()
        .filter_map(|line| match syntax_error_score(line) {
            Result::Incomplete(x) => Some(x),
            Result::Error(_) => None,
        })
        .map(|x| completion_score(&x))
        .collect::<Vec<usize>>()
        .tap_mut(|v| v.sort())
        .pipe(|v| v[v.len() / 2])
}

#[cfg(test)]
mod tests {
    use super::*;

    use indoc::indoc;

    const INPUT: &str = indoc! {"
        [({(<(())[]>[[{[]{<()<>>
        [(()[<>])]({[<{<<[]>>(
        {([(<{}[<>[]}>{[]{[(<()>
        (((({<>}<{<{<>}{[]{[]{}
        [[<[([]))<([[{}[[()]]]
        [{[{({}]{}}([{[{{{}}([]
        {<[[]]>}<{[{[{[]{()[[[]
        [<(<(<(<{}))><([]([]()
        <{([([[(<>()){}]>(<<{{
        <{([{{}}[<[[[<>{}]]]>[]]
    "};

    #[test]
    fn p1() {
        assert_eq!(super::p1(INPUT), 26397);

        let input = input(DAY);
        assert_eq!(super::p1(&input), 216297);
    }

    #[test]
    fn p2() {
        assert_eq!(super::p2(INPUT), 288957);

        let input = input(DAY);
        assert_eq!(super::p2(&input), 2165057169);
    }
}
