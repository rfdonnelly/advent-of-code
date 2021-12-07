use crate::input;

const DAY: usize = 7;

pub fn run() {
    let input = input(DAY);
    println!("d{:02}p1: {}", DAY, p1(&input));
    println!("d{:02}p2: {}", DAY, p2(&input));
}

struct Positions(Vec<usize>);

impl From<&str> for Positions {
    fn from(s: &str) -> Self {
        let mut positions = s
            .trim()
            .split(",")
            .map(str::parse::<usize>)
            .map(Result::unwrap)
            .collect::<Vec<usize>>();

        positions.sort();

        Self(positions)
    }
}

impl Positions {
    fn cost<F>(&self, cost_fn: F) -> usize
    where
        F: FnMut(&usize) -> usize
    {
        self.0
            .iter()
            .map(cost_fn)
            .sum()
    }

    fn median(&self) -> usize {
        let index = self.0.len() / 2;
        self.0[index]
    }
}

fn p1_cost(a: usize, b: usize) -> usize {
    ((a as i32) - (b as i32)).abs() as usize
}

fn p1(input: &str) -> usize {
    let positions = Positions::from(input);
    let median = positions.median();
    positions.cost(|&pos| p1_cost(pos, median))
}

fn p2(input: &str) -> usize {
    todo!()
}

#[cfg(test)]
mod tests {
    use indoc::indoc;

    const INPUT: &str = indoc! {"
        16,1,2,0,4,2,7,1,2,14
    "};

    #[test]
    fn p1() {
        assert_eq!(super::p1(INPUT), 37);

        let input = super::input(super::DAY);
        assert_eq!(super::p1(&input), 344297);
    }

    #[test]
    #[ignore]
    fn p2() {
        assert_eq!(super::p2(INPUT), 26984457539);

        let input = super::input(super::DAY);
        assert_eq!(super::p2(&input), 1631647919273);
    }
}
