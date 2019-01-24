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

fn parse_line(line: &str) -> (u32, u32) {
    let tokens: Vec<_> = line.split(' ').collect();
    let players = tokens[0].parse::<u32>().unwrap();
    let last_marble = tokens[6].parse::<u32>().unwrap();

    (players, last_marble)
}

struct Marbles {
    ring: Vec<u32>,
}

impl Marbles {
    fn new() -> Self {
        Self {
            ring: Vec::new(),
        }
    }

    // Move left (-)
    fn ccw(&self, from: usize, by: usize) -> usize {
        if self.ring.len() == 1 {
            0
        } else if by <= from {
            from - by
        } else {
            // FIXME double check this
            self.ring.len() - (by - from)
        }
    }

    // Move right (+)
    fn cw(&self, from: usize, by: usize) -> usize {
        if self.ring.len() == 1 {
            0
        } else if by > from {
        }
    }
}

fn part1(line: &str) -> u32 {
    let (players, last_marble) = parse_line(line);

    0
}

fn part2(line: &str) -> u32 {
    0
}

#[cfg(test)]
mod tests {
    fn values() -> Vec<(u32, u32, u32)> {
        vec![
            (10, 1618, 8317),
            (13, 7999, 146373),
            (17, 1104, 2764),
            (21, 6111, 54718),
            (30, 5807, 37305),
        ]
    }

    fn line(players: u32, last_marble: u32) -> String {
        format!("{} players; last marble is worth {} points", players, last_marble)
    }

    #[test]
    fn part1() {
        let values = values();
        for value in values {
            assert_eq!(super::part1(&line(value.0, value.1)), value.2);
        }
    }
}
