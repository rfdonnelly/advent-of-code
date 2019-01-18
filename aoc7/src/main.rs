use std::io::{self, Read};
use std::collections::HashMap;
use std::collections::HashSet;

fn main() -> io::Result<()> {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input)?;

    let lines: Vec<&str> = input
	.lines()
        .collect();

    println!("part1: {}", part1(&lines));
    println!("part2: {}", part2(&lines));

    Ok(())
}

#[derive(Debug, PartialEq)]
struct Edge {
    from: char,
    to: char,
}

impl Edge {
    fn from_str(s: &str) -> Edge {
        let tokens: Vec<&str> = s.split(" ").collect();

        Edge {
            from: tokens[1].chars().next().unwrap(),
            to: tokens[7].chars().next().unwrap(),
        }
    }
}

#[derive(Debug, PartialEq)]
struct Node {
    parents: Vec<char>,
    children: Vec<char>,
}

impl Node {
    fn with_child(child: char) -> Node {
        Node {
            parents: Vec::new(),
            children: vec![child],
        }
    }

    fn with_parent(parent: char) -> Node {
        Node {
            parents: vec![parent],
            children: Vec::new(),
        }
    }

    fn add_child(&mut self, child: char) {
        self.children.push(child);
    }

    fn add_parent(&mut self, parent: char) {
        self.parents.push(parent);
    }
}

#[derive(Debug, PartialEq)]
struct Graph {
    nodes: HashMap<char, Node>,
}

impl Graph {
    fn new(edges: &[Edge]) -> Graph {
        let mut nodes: HashMap<char, Node> = HashMap::new();

        for edge in edges {
            nodes.entry(edge.from)
                .and_modify(|e| e.add_child(edge.to))
                .or_insert(Node::with_child(edge.to));

            nodes.entry(edge.to)
                .and_modify(|e| e.add_parent(edge.from))
                .or_insert(Node::with_parent(edge.from));
        }

        for (_id, node) in nodes.iter_mut() {
            node.parents.sort();
            node.children.sort();
        }

        Graph {
            nodes,
        }
    }

    fn roots(&self) -> Vec<char> {
        self.nodes
            .iter()
            .filter(|(_, node)| node.parents.is_empty())
            .map(|(id, _)| *id)
            .collect()
    }

    fn sequence(&self) -> String {
        let mut available = self.roots();
        let mut made_available: HashSet<char> = HashSet::new();
        let mut complete: HashSet<char> = HashSet::new();

        let mut sequence: Vec<char> = Vec::new();

        while !available.is_empty() {
            available.sort_unstable_by(|a, b| b.cmp(a) );
            let node = available.pop().unwrap();

            sequence.push(node);
            complete.insert(node);

            let children = &self.nodes[&node].children;
            for &child in children {
                if !made_available.contains(&child) && self.prereqs_complete(child, &complete) {
                    available.push(child);
                    made_available.insert(child);
                }
            }
        }

        sequence
            .iter()
            .collect()
    }

    fn prereqs_complete(&self, node: char, complete: &HashSet<char>) -> bool {
        self.nodes[&node].parents
            .iter()
            .all(|parent| complete.contains(parent))
    }
}

fn parse_lines(lines: &[&str]) -> Vec<Edge> {
    lines
        .iter()
        .map(|line| Edge::from_str(line))
        .collect()
}

fn part1(lines: &[&str]) -> String {
    let edges = parse_lines(lines);
    let graph = Graph::new(&edges);
    graph.sequence()
}

fn part2(lines: &[&str]) -> u32 {
    0
}

#[cfg(test)]
mod tests {
    use super::*;

    fn lines() -> Vec<&'static str> {
        vec![
            "Step C must be finished before step A can begin.",
            "Step C must be finished before step F can begin.",
            "Step A must be finished before step B can begin.",
            "Step A must be finished before step D can begin.",
            "Step B must be finished before step E can begin.",
            "Step D must be finished before step E can begin.",
            "Step F must be finished before step E can begin.",
        ]
    }

    fn edges() -> Vec<Edge> {
        parse_lines(&lines())
    }

    #[test]
    fn part1_edge() {
        let lines = lines();
        assert_eq!(Edge::from_str(lines[0]), Edge { from: 'C', to: 'A' });
    }

    #[test]
    fn part1_graph() {
        let mut expected = Graph { nodes: HashMap::new() };
        expected.nodes.insert('C', Node {
            parents: Vec::new(),
            children: vec!['A', 'F'],
        });
        expected.nodes.insert('A', Node {
            parents: vec!['C'],
            children: vec!['B', 'D'],
        });
        expected.nodes.insert('B', Node {
            parents: vec!['A'],
            children: vec!['E'],
        });
        expected.nodes.insert('D', Node {
            parents: vec!['A'],
            children: vec!['E'],
        });
        expected.nodes.insert('F', Node {
            parents: vec!['C'],
            children: vec!['E'],
        });
        expected.nodes.insert('E', Node {
            parents: vec!['B', 'D', 'F'],
            children: Vec::new(),
        });

        assert_eq!(Graph::new(&edges()), expected);
    }

    #[test]
    fn part1_roots() {
        let graph = Graph::new(&edges());

        assert_eq!(graph.roots(), vec!['C']);
    }

    #[test]
    fn part1() {
        assert_eq!(super::part1(&lines()), "CABDFE");
    }
}
