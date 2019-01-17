use std::io::{self, Read};
use std::collections::HashMap;

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
struct Edges {
    parents: Vec<char>,
    children: Vec<char>,
}

impl Edges {
    fn with_child(child: char) -> Edges {
        Edges {
            parents: Vec::new(),
            children: vec![child],
        }
    }

    fn with_parent(parent: char) -> Edges {
        Edges {
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
    edge_map: HashMap<char, Edges>,
}

impl Graph {
    fn new(edges: &[Edge]) -> Graph {
        let mut edge_map: HashMap<char, Edges> = HashMap::new();

        for edge in edges {
            edge_map.entry(edge.from)
                .and_modify(|e| e.add_child(edge.to))
                .or_insert(Edges::with_child(edge.to));

            edge_map.entry(edge.to)
                .and_modify(|e| e.add_parent(edge.from))
                .or_insert(Edges::with_parent(edge.from));
        }

        Graph {
            edge_map,
        }
    }
}

fn parse_lines(lines: &[&str]) -> Vec<Edge> {
    lines
        .iter()
        .map(|line| Edge::from_str(line))
        .collect()
}

fn part1(lines: &[&str]) -> String {
    "".into()
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
        let mut expected = Graph { edge_map: HashMap::new() };
        expected.edge_map.insert('C', Edges {
            parents: Vec::new(),
            children: vec!['A', 'F'],
        });
        expected.edge_map.insert('A', Edges {
            parents: vec!['C'],
            children: vec!['B', 'D'],
        });
        expected.edge_map.insert('B', Edges {
            parents: vec!['A'],
            children: vec!['E'],
        });
        expected.edge_map.insert('D', Edges {
            parents: vec!['A'],
            children: vec!['E'],
        });
        expected.edge_map.insert('F', Edges {
            parents: vec!['C'],
            children: vec!['E'],
        });
        expected.edge_map.insert('E', Edges {
            parents: vec!['B', 'D', 'F'],
            children: Vec::new(),
        });

        assert_eq!(Graph::new(&edges()), expected);
    }

    #[test]
    fn part1() {
        assert_eq!(super::part1(&lines()), "CABDFE");
    }
}
