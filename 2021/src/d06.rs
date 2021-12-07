use crate::input;

const DAY: usize = 6;

pub fn run() {
    let input = input(DAY);
    println!("d{:02}p1: {}", DAY, p1(&input));
    println!("d{:02}p2: {}", DAY, p2(&input));
}

#[derive(Debug)]
struct State {
    fish: Vec<usize>,
}

impl From<&str> for State {
    fn from(s: &str) -> Self {
        let fish = s
            .trim()
            .split(",")
            .map(str::parse::<usize>)
            .filter_map(Result::ok)
            .collect::<Vec<usize>>();

        Self { fish }
    }
}

impl State {
    fn next(mut self) -> Self {
        let mut new = 0;

        for f in self.fish.iter_mut() {
            if *f == 0 {
                *f = 6;
                new += 1;
            } else {
                *f -= 1;
            }
        }

        for _ in 0..new {
            self.fish.push(8);
        }

        self
    }
}

fn p1(input: &str) -> usize {
    let mut state = State::from(input);
    for _ in 0..80 {
        state = state.next();
    }

    state.fish.len()
}

fn p2(input: &str) -> usize {
    0
}

#[cfg(test)]
mod tests {
    use indoc::indoc;

    const INPUT: &str = indoc! {"
        3,4,3,1,2
    "};

    #[test]
    fn p1() {
        assert_eq!(super::p1(INPUT), 5934);

        let input = super::input(super::DAY);
        assert_eq!(super::p1(&input), 359999);
    }

    #[test]
    #[ignore]
    fn p2() {
        assert_eq!(super::p2(INPUT), 12);

        let input = super::input(super::DAY);
        assert_eq!(super::p2(&input), 20196);
    }
}
