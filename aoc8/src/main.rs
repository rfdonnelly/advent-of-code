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

fn part1(line: &str) -> usize {
    0
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

fn parse_stream(stream: &[u32]) -> Vec<Node> {
    let mut stack: Vec<usize> = Vec::new();
    let mut nodes: Vec<Node> = Vec::new();

    nodes
}

#[cfg(test)]
mod tests {
    use super::*;

    fn line() -> &'static str {
        "2 3 0 3 10 11 12 1 1 0 1 99 2 1 1 2"
    }

    #[test]
    fn part1() {
    }
}
