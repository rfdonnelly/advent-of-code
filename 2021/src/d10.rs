use crate::input;

use tap::prelude::*;

const DAY: usize = 10;

pub fn run() {
    let input = input(DAY);
    println!("d{:02}p1: {}", DAY, p1(&input));
    println!("d{:02}p2: {}", DAY, p2(&input));
}

enum OpenClose {
    Open,
    Close,
}

enum Result {
    Error(usize),
    Incomplete(String)
}

use OpenClose::*;

/// Returns Some(score) if syntax error present
/// Returns None if syntax error not present
fn syntax_error_score(s: &str) -> Result {
    let sequence = s
        .chars()
        .map(|c| match c {
            '(' => (c, Open),
            ')' => (c, Close),
            '{' => (c, Open),
            '}' => (c, Close),
            '<' => (c, Open),
            '>' => (c, Close),
            '[' => (c, Open),
            ']' => (c, Close),
            _ => unreachable!(),
        })
        .collect::<Vec<(char, OpenClose)>>();

    let mut stack = vec![];
    for (c, oc) in sequence {
        if is_char_ok(c, stack.last()) {
            match oc {
                Open => {
                    stack.push(c);
                }
                Close => {
                    stack.pop();
                }
            }
        } else {
            return Result::Error(p1_char_score(c));
        }
    }

    Result::Incomplete(stack.iter().collect())
}

fn is_char_ok(curr: char, tail: Option<&char>) -> bool {
    match curr {
        '(' | '[' | '{' | '<' => true,
        ')' => tail == Some(&'('),
        ']' => tail == Some(&'['),
        '}' => tail == Some(&'{'),
        '>' => tail == Some(&'<'),
        _ => unreachable!(),
    }
}

fn p1_char_score(c: char) -> usize {
    match c {
        ')' => 3,
        ']' => 57,
        '}' => 1197,
        '>' => 25137,
        _ => unreachable!(),
    }
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

fn completion_score(s: &str) -> usize {
    s
        .chars()
        .rev()
        .map(p2_char_score)
        .fold(0, |acc, char_score| acc * 5 + char_score)
}

fn p2_char_score(c: char) -> usize {
    match c {
        '(' => 1,
        '[' => 2,
        '{' => 3,
        '<' => 4,
        _ => unreachable!(),
    }
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
