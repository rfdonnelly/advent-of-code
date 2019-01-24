use std::io::{self, Read};

fn main() -> io::Result<()> {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input)?;

    let lines: Vec<&str> = input
        .lines()
        .collect();

    let line = lines.first().unwrap();

    println!("part1: {}", part1(line));
    println!("part2: {}", part2(line));

    Ok(())
}

fn parse_line(s: &str) -> Vec<u32> {
    s.split(" ")
        .map(|token| token.parse::<u32>().unwrap())
        .collect()
}

fn part1(line: &str) -> u32 {
    let stream = parse_line(line);
    let nodes = parse_nodes(&mut stream.iter());

    nodes
        .into_iter()
        .map(|node| node.metadata)
        .flatten()
        .sum::<u32>()
}

fn part2(line: &str) -> usize {
    0
}

struct Node {
    children: Vec<usize>,
    metadata: Vec<u32>,
}

impl Node {
    fn new(children: u32, metadata: u32) -> Node {
        Node {
            children: Vec::with_capacity(children as usize),
            metadata: Vec::with_capacity(metadata as usize),
        }
    }
}

fn parse_node(stream: &mut dyn Iterator<Item=&u32>, nodes: &mut Vec<Node>) -> usize {
    let num_children = *stream.next().unwrap();
    let num_metadata = *stream.next().unwrap();
    let node = Node::new(num_children, num_metadata);

    let node_index = nodes.len();
    nodes.push(node);

    for _ in 0..num_children {
        let child = parse_node(stream, nodes);
        nodes[node_index].children.push(child);
    }

    for _ in 0..num_metadata {
        nodes[node_index].metadata.push(*stream.next().unwrap());
    }

    node_index
}

fn parse_nodes(stream: &mut dyn Iterator<Item=&u32>) -> Vec<Node> {
    let mut nodes: Vec<Node> = Vec::new();
    parse_node(stream, &mut nodes);
    nodes
}

#[cfg(test)]
mod tests {
    fn line() -> &'static str {
        "2 3 0 3 10 11 12 1 1 0 1 99 2 1 1 2"
    }

    #[test]
    fn part1() {
        assert_eq!(super::part1(&line()), 138);
    }
}
