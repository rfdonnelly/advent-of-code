use crate::input;

use tap::prelude::*;

use std::collections::HashMap;

const DAY: usize = 12;

pub fn run() {
    let input = input(DAY);
    println!("d{:02}p1: {}", DAY, p1(&input));
    println!("d{:02}p2: {}", DAY, p2(&input));
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
enum Node {
    Start,
    End,
    Big(char, Option<char>),
    Small(char, Option<char>),
}

impl From<&str> for Node {
    fn from(s: &str) -> Self {
        match s {
            "start" => Node::Start,
            "end" => Node::End,
            _ => {
                let mut chars = s.chars();
                let c = chars.next().unwrap();
                match c {
                    c if c.is_lowercase() => Node::Small(c.into(), chars.next()),
                    c if c.is_uppercase() => Node::Big(c.into(), chars.next()),
                    _ => unreachable!(),
                }
            }
        }
    }
}

#[derive(Debug)]
struct Graph {
    nodes: HashMap<Node, Vec<Node>>,
}

impl Graph {
    fn new() -> Self {
        Self { nodes: HashMap::new() }
    }
}

impl From<&str> for Graph {
    fn from(s: &str) -> Self {
        s
            .lines()
            .map(|line| {
                line
                    .split_once("-")
                    .unwrap()
                    .pipe(|(a, b)| (Node::from(a), Node::from(b)))
            })
            .fold(Graph::new(), |mut graph, (a, b)| {
                graph.nodes.entry(a).or_insert(Vec::new()).push(b);
                if !matches!(a, Node::Start) && !matches!(b, Node::End) {
                    graph.nodes.entry(b).or_insert(Vec::new()).push(a);
                }
                graph
            })
    }
}

fn p1(input: &str) -> usize {
    todo!()
}

fn p2(input: &str) -> usize {
    todo!()
}

#[cfg(test)]
mod tests {
    use super::*;

    use indoc::indoc;

    const INPUT: &str = indoc! {"
        start-A
        start-b
        A-c
        A-b
        b-d
        A-end
        b-end
    "};

    #[test]
    fn p1() {
        dbg!(Graph::from(INPUT));
        assert_eq!(super::p1(INPUT), 1656);

        let input = input(DAY);
        assert_eq!(super::p1(&input), 1747);
    }

    #[test]
    fn p2() {
        assert_eq!(super::p2(INPUT), 195);

        // let input = input(DAY);
        // assert_eq!(super::p2(&input), 2165057169);
    }
}
