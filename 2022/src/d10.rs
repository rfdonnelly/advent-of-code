use aoc_runner_derive::{aoc, aoc_generator};

#[derive(Copy, Clone, Debug, PartialEq, Eq)]
enum Instruction {
    Addx(i32),
    Noop,
}

impl From<&str> for Instruction {
    fn from(s: &str) -> Self {
        match s.split_once(' ') {
            Some((_, value)) => Instruction::Addx(value.parse().unwrap()),
            None => Instruction::Noop,
        }
    }
}

impl Instruction {
    fn cycles(&self) -> usize {
        match self {
            Self::Addx(_) => 2,
            Self::Noop => 1,
        }
    }
}

#[derive(Copy, Clone, Debug, PartialEq, Eq)]
struct State {
    x: i32,
    cycle: i32,
    cummulative_signal_strength: i32,
    pixel_x: i32,
}

impl State {
    fn new() -> Self {
        State {
            x: 1,
            cycle: 0,
            cummulative_signal_strength: 0,
            pixel_x: 0,
        }
    }

    fn next(mut self, instr: &Instruction) -> Self {
        for _ in 0..instr.cycles() {
            self.cycle();
        }

        if let Instruction::Addx(value) = instr {
            self.x += value;
        }

        self
    }

    fn signal_strength(&self) -> i32 {
        self.cycle * self.x
    }

    fn is_sample_signal_strength_cycle(&self) -> bool {
        (self.cycle - 20) % 40 == 0
    }

    fn cycle(&mut self) {
        if self.pixel_x >= self.x - 1 && self.pixel_x <= self.x + 1 {
            print!("#");
        } else {
            print!(".");
        }

        self.cycle += 1;
        self.pixel_x = (self.pixel_x + 1) % 40;
        if self.pixel_x == 0 {
            println!("");
        }

        if self.is_sample_signal_strength_cycle() {
            self.cummulative_signal_strength += self.signal_strength();
        }
    }
}

type Input = Vec<Instruction>;

#[aoc_generator(day10)]
fn parse(input: &str) -> Input {
    input
        .lines()
        .map(Instruction::from)
        .collect()
}

#[aoc(day10, part1)]
fn p1(input: &Input) -> i32 {
    input
        .iter()
        .fold(State::new(), |state, instr| {
            state.next(instr)
        })
        .cummulative_signal_strength
}

#[aoc(day10, part2)]
fn p2(input: &Input) -> i32 {
    input
        .iter()
        .fold(State::new(), |state, instr| {
            state.next(instr)
        })
        .cummulative_signal_strength
}

#[cfg(test)]
mod test {
    use super::*;
    use indoc::indoc;

    const INPUT: &str = indoc! {"
        noop
        addx 3
        addx -5
    "};

    const INPUT_P1: &str = indoc! {"
        addx 15
        addx -11
        addx 6
        addx -3
        addx 5
        addx -1
        addx -8
        addx 13
        addx 4
        noop
        addx -1
        addx 5
        addx -1
        addx 5
        addx -1
        addx 5
        addx -1
        addx 5
        addx -1
        addx -35
        addx 1
        addx 24
        addx -19
        addx 1
        addx 16
        addx -11
        noop
        noop
        addx 21
        addx -15
        noop
        noop
        addx -3
        addx 9
        addx 1
        addx -3
        addx 8
        addx 1
        addx 5
        noop
        noop
        noop
        noop
        noop
        addx -36
        noop
        addx 1
        addx 7
        noop
        noop
        noop
        addx 2
        addx 6
        noop
        noop
        noop
        noop
        noop
        addx 1
        noop
        noop
        addx 7
        addx 1
        noop
        addx -13
        addx 13
        addx 7
        noop
        addx 1
        addx -33
        noop
        noop
        noop
        addx 2
        noop
        noop
        noop
        addx 8
        noop
        addx -1
        addx 2
        addx 1
        noop
        addx 17
        addx -9
        addx 1
        addx 1
        addx -3
        addx 11
        noop
        noop
        addx 1
        noop
        addx 1
        noop
        noop
        addx -13
        addx -19
        addx 1
        addx 3
        addx 26
        addx -30
        addx 12
        addx -1
        addx 3
        addx 1
        noop
        noop
        noop
        addx -9
        addx 18
        addx 1
        addx 2
        noop
        noop
        addx 9
        noop
        noop
        noop
        addx -1
        addx 2
        addx -37
        addx 1
        addx 3
        noop
        addx 15
        addx -21
        addx 22
        addx -6
        addx 1
        noop
        addx 2
        addx 1
        noop
        addx -10
        noop
        noop
        addx 20
        addx 1
        addx 2
        addx 2
        addx -6
        addx -11
        noop
        noop
        noop
    "};

    #[test]
    fn test_parse() {}

    #[test]
    fn test_p1() {
        assert_eq!(p1(&parse(INPUT_P1)), 13140);
    }

    #[test]
    fn test_p2() {}
}

