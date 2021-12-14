use crate::input;

use tap::prelude::*;

use std::collections::HashMap;
use std::fmt;

const DAY: usize = 12;

pub fn run() {
    let input = input(DAY);
    let time = std::time::Instant::now();
    println!("d{:02}p1: {} in {:?}", DAY, p1(&input), time.elapsed());
    let time = std::time::Instant::now();
    println!("d{:02}p2: {} in {:?}", DAY, p2(&input), time.elapsed());
}

#[derive(Clone, Copy, PartialEq, Eq, Hash)]
enum Node {
    Start,
    End,
    Big(char),
    Small(char),
}

impl Node {
    fn is_small(&self) -> bool {
        matches!(self, Node::Small(_))
    }
}

impl fmt::Debug for Node {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Node::Start => write!(f, "start"),
            Node::End => write!(f, "end"),
            Node::Big(c) => write!(f, "{}", c),
            Node::Small(c) => write!(f, "{}", c),
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
                if c.is_lowercase() {
                    Node::Small(c)
                } else {
                    Node::Big(c)
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
        Self {
            nodes: HashMap::new(),
        }
    }
}

impl From<&str> for Graph {
    fn from(s: &str) -> Self {
        s.lines()
            .map(|line| {
                line.split_once("-")
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

fn allow_one_small(_path: &Path) -> usize {
    1
}

fn p1(input: &str) -> usize {
    let graph = Graph::from(input);

    let mut path = Path::new();
    visit_node(
        &graph,
        &mut path,
        Node::Start,
        allow_one_small,
    )
}

struct Path {
    nodes: Vec<Node>,
    counts: HashMap<Node, usize>,
}

impl Path {
    fn new() -> Self {
        Self {
            nodes: Vec::new(),
            counts: HashMap::new(),
        }
    }

    fn push(&mut self, node: Node) {
        self.nodes.push(node);
        if node.is_small() {
            *self.counts.entry(node).or_insert(0) += 1;
        }
    }

    fn pop(&mut self) {
        let node = self.nodes.pop().unwrap();
        if node.is_small() {
            *self.counts.entry(node).or_insert(1) -= 1;
        }
    }

    fn contains_two_of_same_small(&self) -> bool {
        self.counts
            .iter()
            .any(|(_node, count)| *count == 2)
    }

    fn allow_two_of_same_small(&self) -> usize {
        if self.contains_two_of_same_small() {
            1
        } else {
            2
        }
    }
}

fn visit_node<F>(
    graph: &Graph,
    path: &mut Path,
    node: Node,
    allowed: F,
) -> usize
where
    F: Fn(&Path) -> usize + Copy,
{
    if matches!(node, Node::End) {
        return 1;
    }

    if node.is_small() {
        let occurrences = path.counts.get(&node).unwrap_or(&0);
        if occurrences > &0 && occurrences >= &allowed(path) {
            return 0;
        }
    }

    graph.nodes
        .get(&node)
        .and_then(|neighbors| {
            path.push(node);
            let valid_paths = neighbors
                .iter()
                .map(|neighbor| {
                    visit_node(graph, path, *neighbor, allowed)
                })
                .sum();
            path.pop();

            Some(valid_paths)
        })
        .unwrap_or(0)
}

fn p2(input: &str) -> usize {
    let graph = Graph::from(input);

    let mut path = Path::new();
    visit_node(
        &graph,
        &mut path,
        Node::Start,
        Path::allow_two_of_same_small,
    )
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
