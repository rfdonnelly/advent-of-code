use crate::input;

use tap::prelude::*;

use std::collections::HashMap;
use std::fmt;

const DAY: usize = 12;

pub fn run() {
    let input = input(DAY);
    println!("d{:02}p1: {}", DAY, p1(&input));
    println!("d{:02}p2: {}", DAY, p2(&input));
}

#[derive(Clone, Copy, PartialEq, Eq, Hash)]
enum Node {
    Start,
    End,
    Big(char, Option<char>),
    Small(char, Option<char>),
}

impl Node {
    fn is_small(&self) -> bool {
        matches!(self, Node::Small(_, _))
    }
}

impl fmt::Debug for Node {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Node::Start => write!(f, "start"),
            Node::End => write!(f, "end"),
            Node::Big(c1, Some(c2)) => write!(f, "{}{}", c1, c2),
            Node::Small(c1, Some(c2)) => write!(f, "{}{}", c1, c2),
            Node::Big(c, None) => write!(f, "{}", c),
            Node::Small(c, None) => write!(f, "{}", c),
        }
    }
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
                if !matches!(b, Node::Start) && !matches!(a, Node::End) {
                    graph.nodes.entry(a).or_insert(Vec::new()).push(b);
                }
                if !matches!(a, Node::Start) && !matches!(b, Node::End) {
                    graph.nodes.entry(b).or_insert(Vec::new()).push(a);
                }
                graph
            })
    }
}

fn p1(input: &str) -> usize {
    let graph = Graph::from(input);

    let mut valid_paths: Vec<Vec<Node>> = Vec::new();
    visit_node(&graph, &vec![], &mut valid_paths, Node::Start, SmallCaveVisitPolicy::Once);

    valid_paths.len()
}

fn contains_two_of_same_small(path: &[Node]) -> bool {
    path
        .iter()
        .filter(|node| node.is_small())
        .map(|node| {
            path
                .iter()
                .filter(|&n| n == node)
                .count()
        })
        .any(|count| count == 2)
}

#[derive(Copy, Clone)]
enum SmallCaveVisitPolicy {
    Once,
    SingleTwice,
}

fn visit_node(graph: &Graph, path: &[Node], valid_paths: &mut Vec<Vec<Node>>, node: Node, policy: SmallCaveVisitPolicy) {
    if matches!(node, Node::End) {
        let path = path
            .to_vec()
            .tap_mut(|v| v.push(node));
        valid_paths.push(path);
        return;
    }

    let neighbors = graph.nodes.get(&node);

    if let Some(neighbors) = neighbors {
        for neighbor in neighbors {
            if node.is_small() {
                let occurrences = path
                    .iter()
                    .filter(|&&n| n == node)
                    .count();

                let max_occurences =
                    match policy {
                        SmallCaveVisitPolicy::Once => {
                            1
                        }
                        SmallCaveVisitPolicy::SingleTwice => {
                            if contains_two_of_same_small(path) {
                                1
                            } else {
                                2
                            }
                        }
                    };

                if occurrences >= max_occurences {
                    return;
                }
            }

            let path = path
                .to_vec()
                .tap_mut(|v| v.push(node));
            visit_node(graph, &path, valid_paths, *neighbor, policy);
        }
    }
}

fn p2(input: &str) -> usize {
    let graph = Graph::from(input);

    let mut valid_paths: Vec<Vec<Node>> = Vec::new();
    visit_node(&graph, &vec![], &mut valid_paths, Node::Start, SmallCaveVisitPolicy::SingleTwice);

    valid_paths.len()
}

#[cfg(test)]
mod tests {
    use super::*;

    use indoc::indoc;

    const INPUT1: &str = indoc! {"
        start-A
        start-b
        A-c
        A-b
        b-d
        A-end
        b-end
    "};

    const INPUT2: &str = indoc! {"
        dc-end
        HN-start
        start-kj
        dc-start
        dc-HN
        LN-dc
        HN-end
        kj-sa
        kj-HN
        kj-dc
    "};

    #[test]
    fn p1() {
        assert_eq!(super::p1(INPUT1), 10);
        assert_eq!(super::p1(INPUT2), 19);

        let input = input(DAY);
        assert_eq!(super::p1(&input), 3887);
    }

    #[test]
    fn p2() {
        assert_eq!(super::p2(INPUT1), 36);
        assert_eq!(super::p2(INPUT2), 103);

        let input = input(DAY);
        assert_eq!(super::p2(&input), 104834);
    }
}
