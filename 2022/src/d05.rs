use aoc_runner_derive::{aoc, aoc_generator};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
struct Move {
    count: usize,
    from: usize,
    to: usize,
}

#[derive(Debug, Clone, PartialEq, Eq)]
struct Stacks(Vec<Vec<char>>);

#[derive(Debug, Clone, PartialEq, Eq)]
struct Problem {
    stacks: Stacks,
    moves: Vec<Move>,
}

impl From<&str> for Move {
    fn from(s: &str) -> Self {
        let mut numbers = s.split(" ").filter_map(|word| word.parse().ok());
        let count = numbers.next().unwrap();
        let from = numbers.next().unwrap() - 1;
        let to = numbers.next().unwrap() - 1;
        Self { count, from, to }
    }
}

impl From<&str> for Stacks {
    fn from(s: &str) -> Self {
        let cols = (s.lines().next().unwrap().len() + 1) / 4;
        let mut stacks = vec![Vec::new(); cols];
        for line in s.lines() {
            for (i, c) in line
                .char_indices()
                .filter(|(_i, c)| c.is_ascii_alphabetic())
            {
                let col_index = (i + 2) / 4;
                stacks[col_index].push(c);
            }
        }
        stacks.iter_mut().for_each(|v| v.reverse());
        Self(stacks)
    }
}

impl From<&str> for Problem {
    fn from(s: &str) -> Self {
        let mut iter = s.split("\n\n");
        let stacks = Stacks::from(iter.next().unwrap());
        let moves = iter.next().unwrap().lines().map(Move::from).collect();
        Self { stacks, moves }
    }
}

impl Problem {
    fn execute_p1(mut self) -> Self {
        for move_ in &self.moves {
            for _ in 0..move_.count {
                let c = self.stacks.0[move_.from].pop().unwrap();
                self.stacks.0[move_.to].push(c);
            }
        }

        self
    }

    fn execute_p2(mut self) -> Self {
        for move_ in &self.moves {
            let len = self.stacks.0[move_.from].len();
            let new_len = len - move_.count;
            let slice: Vec<char> = self.stacks.0[move_.from][new_len..len]
                .iter()
                .cloned()
                .collect();
            self.stacks.0[move_.to].extend_from_slice(&slice);
            self.stacks.0[move_.from].truncate(new_len);
        }
        self
    }

    fn tops(&self) -> String {
        let mut chars = Vec::new();
        for stack in &self.stacks.0 {
            if let Some(c) = stack.iter().last() {
                chars.push(c);
            }
        }
        chars.iter().cloned().collect()
    }
}

type Input = Problem;

#[aoc_generator(day5)]
fn parse(input: &str) -> Input {
    Problem::from(input)
}

#[aoc(day5, part1)]
fn p1(input: &Input) -> String {
    input.clone().execute_p1().tops()
}

#[aoc(day5, part2)]
fn p2(input: &Input) -> String {
    input.clone().execute_p2().tops()
}

#[cfg(test)]
mod test {
    use super::*;
    use indoc::indoc;

    const INPUT: &str = indoc! {"
            [D]    
        [N] [C]    
        [Z] [M] [P]
         1   2   3 

        move 1 from 2 to 1
        move 3 from 1 to 3
        move 2 from 2 to 1
        move 1 from 1 to 2
    "};

    #[test]
    fn test_parse() {
        let expected = Problem {
            stacks: Stacks(vec![vec!['Z', 'N'], vec!['M', 'C', 'D'], vec!['P']]),
            moves: vec![
                Move {
                    count: 1,
                    from: 1,
                    to: 0,
                },
                Move {
                    count: 3,
                    from: 0,
                    to: 2,
                },
                Move {
                    count: 2,
                    from: 1,
                    to: 0,
                },
                Move {
                    count: 1,
                    from: 0,
                    to: 1,
                },
            ],
        };
        assert_eq!(parse(INPUT), expected);
    }

    #[test]
    fn test_p1() {
        assert_eq!(p1(&parse(INPUT)), "CMZ");
    }

    #[test]
    fn test_p2() {
        assert_eq!(p2(&parse(INPUT)), "MCD");
    }
}
