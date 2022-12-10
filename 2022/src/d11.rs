use aoc_runner_derive::{aoc, aoc_generator};
use std::collections::VecDeque;
use std::cell::RefCell;
use tap::Tap;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
enum Op {
    Add,
    Mul,
}

impl From<&str> for Op {
    fn from(s: &str) -> Self {
        match s {
            "+" => Op::Add,
            "*" => Op::Mul,
            _ => panic!(),
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
enum Operand {
    Value(u64),
    Old,
}

impl From<&str> for Operand {
    fn from(s: &str) -> Self {
        match s {
            "old" => Operand::Old,
            _ => Operand::Value(s.parse().unwrap()),
        }
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
struct Monkey {
    items: VecDeque<u64>,
    op: Op,
    operand: Operand,
    test_div: u64,
    test_true: usize,
    test_false: usize,

    inspected: u64,
}

fn worry_level(old: u64, op: Op, operand: Operand) -> u64 {
    let operand = match operand {
        Operand::Value(value) => value,
        Operand::Old => old,
    };

    match op {
        Op::Add => old + operand,
        Op::Mul => old * operand,
    }
}

impl Monkey {
    fn inspect(&mut self) -> Option<(u64, usize)> {
        match self.items.pop_front() {
            Some(old) => {
                self.inspected += 1;
                let new = worry_level(old, self.op, self.operand) / 3;
                if new % self.test_div == 0 {
                    Some((new, self.test_true))
                } else {
                    Some((new, self.test_false))
                }
            },
            None => None,
        }
    }

    fn inspected(self) -> u64 {
        self.inspected
    }
}

impl From<&str> for Monkey {
    fn from(s: &str) -> Self {
        let mut lines = s
            .lines()
            .map(|s| s.trim());

        lines.next();
        let (_, items) = lines.next().unwrap().split_once(": ").unwrap();
        let items = items.split(", ").map(|item| item.parse().unwrap()).collect();
        let (_, expr) = lines.next().unwrap().split_once("old ").unwrap();
        let (op, operand) = expr.split_once(' ').unwrap();
        let op = Op::from(op);
        let operand = Operand::from(operand);
        let (_, test_div) = lines.next().unwrap().split_once("by ").unwrap();
        let test_div = test_div.parse().unwrap();
        let (_, test_true) = lines.next().unwrap().split_once("monkey ").unwrap();
        let test_true = test_true.parse().unwrap();
        let (_, test_false) = lines.next().unwrap().split_once("monkey ").unwrap();
        let test_false = test_false.parse().unwrap();

        Self {
            items,
            op,
            operand,
            test_div,
            test_true,
            test_false,
            inspected: 0,
        }
    }
}

struct State {
    monkeys: Vec<RefCell<Monkey>>,
}

impl State {
    fn new(monkeys: &[Monkey]) -> Self {
        let monkeys = monkeys
            .iter()
            .cloned()
            .map(RefCell::from)
            .collect();

        Self { monkeys }
    }

    fn next(self) -> Self {
        self.monkeys
            .iter()
            .for_each(|monkey| {
                while let Some((item, monkey_idx)) = monkey.borrow_mut().inspect() {
                    self.monkeys[monkey_idx].borrow_mut().items.push_back(item);
                }
            });

        self
    }
}

type Input = Vec<Monkey>;

#[aoc_generator(day11)]
fn parse(input: &str) -> Input {
    input
        .split("\n\n")
        .map(Monkey::from)
        .collect()
}

#[aoc(day11, part1)]
fn p1(input: &Input) -> u64 {
    let initial_state = State::new(input);

    (0..20)
        .fold(initial_state, |state, _| {
            state.next()
        })
        .monkeys
        .into_iter()
        .map(RefCell::into_inner)
        .map(Monkey::inspected)
        .collect::<Vec<_>>()
        .tap(|vec| {dbg!(&vec); ()})
        .tap_mut(|vec| vec.sort())
        .tap_mut(|vec| vec.reverse())[0..2]
        .tap(|vec| {dbg!(&vec); ()})
        .iter()
        .copied()
        .product()
}

#[aoc(day11, part2)]
fn p2(input: &Input) -> u64 {
    0
}

#[cfg(test)]
mod test {
    use super::*;
    use indoc::indoc;

    const INPUT: &str = indoc! {"
        Monkey 0:
          Starting items: 79, 98
          Operation: new = old * 19
          Test: divisible by 23
            If true: throw to monkey 2
            If false: throw to monkey 3

        Monkey 1:
          Starting items: 54, 65, 75, 74
          Operation: new = old + 6
          Test: divisible by 19
            If true: throw to monkey 2
            If false: throw to monkey 0

        Monkey 2:
          Starting items: 79, 60, 97
          Operation: new = old * old
          Test: divisible by 13
            If true: throw to monkey 1
            If false: throw to monkey 3

        Monkey 3:
          Starting items: 74
          Operation: new = old + 3
          Test: divisible by 17
            If true: throw to monkey 0
            If false: throw to monkey 1
    "};

    #[test]
    fn test_parse() {
        let expected = vec![
            Monkey {
                items: vec![79, 98].into(),
                op: Op::Mul,
                operand: Operand::Value(19),
                test_div: 23,
                test_true: 2,
                test_false: 3,
                inspected: 0,
            }
        ];
        assert_eq!(parse(INPUT)[..1], expected);
    }

    #[test]
    fn test_p1() {
        assert_eq!(p1(&parse(INPUT)), 10605);
    }

    #[test]
    fn test_p2() {
        assert_eq!(p2(&parse(INPUT)), 12);
    }
}
