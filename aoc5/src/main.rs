use std::io::{self, Read};
use std::fmt;
use bit_vec::BitVec;

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

struct Polymer {
    units: Vec<char>,
    skips: BitVec,
}

impl Polymer {
    fn new(s: &str) -> Polymer {
        Polymer {
            units: s.chars().collect(),
            skips: BitVec::from_elem(s.len(), false),
        }
    }

    /// Returns false when no more reductions can be made
    fn reduce(&mut self, start_index: usize) -> Option<usize> {
        let mut index = start_index;
        let mut unit = self.units[index];

        while self.skips[index] {
            index += 1;
        }

        for next_index in start_index+1..self.units.len() {
            if !self.skips[next_index] {
                let next_unit = self.units[next_index];

                if unit == toggle_case(next_unit) {
                    self.skips.set(index, true);
                    self.skips.set(next_index, true);

                    if index == 0 {
                        if next_index + 1 == self.units.len() {
                            // Edge case
                            // Reduced first and last units.  Nothing left.
                            return None
                        } else {
                            // Edge case
                            // Reduced first unit.  Start after "next" unit.
                            return Some(next_index + 1);
                        }
                    } else {
                        // Typical case
                        // Reduced middle unit.  Backtrack.
                        return Some(0);
                    }
                }

                unit = next_unit;
                index = next_index;
            }
        }

        None
    }

    fn minimize(&mut self) -> &Self {
        let mut done = false;
        let mut index = 0;

        while !done {
            let result = self.reduce(index);
            match result {
                Some(i) => index = i,
                None => done = true,
            }
        }

        self
    }
}

impl fmt::Display for Polymer {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        for i in 0..self.units.len() {
            if !self.skips[i] {
                write!(f, "{}", self.units[i])?;
            }
        }

        Ok(())
    }
}

fn toggle_case(c: char) -> char {
    if c.is_ascii_lowercase() {
        c.to_ascii_uppercase()
    } else {
        c.to_ascii_lowercase()
    }
}

fn part1(line: &str) -> usize {
    Polymer::new(line).minimize().to_string().len()
}

fn part2(line: &str) -> String {
    "".into()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1() {
        let mut p = Polymer::new("aA");
        assert_eq!(p.reduce(0), None);
        assert_eq!(p.to_string(), "");

        let mut p = Polymer::new("abBA");
        assert_eq!(p.reduce(0), Some(0));
        assert_eq!(p.reduce(0), None);
        assert_eq!(p.to_string(), "");

        let mut p = Polymer::new("abAB");
        assert_eq!(p.reduce(0), None);
        assert_eq!(p.to_string(), "abAB");

        let mut p = Polymer::new("aabAAB");
        assert_eq!(p.reduce(0), None);
        assert_eq!(p.to_string(), "aabAAB");

        assert_eq!(part1("aA"), 0);
        assert_eq!(part1("abBA"), 0);
        assert_eq!(part1("abAB"), 4);
        assert_eq!(part1("aabAAB"), 6);
        assert_eq!(part1("dabAcCaCBAcCcaDA"), 10);
    }
}
