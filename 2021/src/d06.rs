use crate::input;

const DAY: usize = 6;

pub fn run() -> String {
    let input = input(DAY);
    let mut output = String::new();
    let time = std::time::Instant::now();
    output += &format!("d{:02}p1: {} in {:?}\n", DAY, p1(&input), time.elapsed());
    let time = std::time::Instant::now();
    output += &format!("d{:02}p2: {} in {:?}\n", DAY, p2(&input), time.elapsed());
    output
}

#[derive(Debug)]
struct State {
    fish: [usize; 9],
}

impl From<&str> for State {
    fn from(s: &str) -> Self {
        let fish = s
            .trim()
            .split(",")
            .map(str::parse::<usize>)
            .filter_map(Result::ok)
            .fold([0; 9], |mut acc, age| {
                acc[age] += 1;
                acc
            });

        Self { fish }
    }
}

impl State {
    fn next(mut self) -> Self {
        let zero = self.fish[0];
        self.fish.rotate_left(1);
        self.fish[6] += zero;
        self
    }

    fn count_fish(&self) -> usize {
        self.fish.iter().sum()
    }
}

fn simulate(initial_state: State, days: usize) -> usize {
    (0..days)
        .fold(initial_state, |state, _| state.next())
        .count_fish()
}

fn p1(input: &str) -> usize {
    simulate(State::from(input), 80)
}

fn p2(input: &str) -> usize {
    simulate(State::from(input), 256)
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
    fn p2() {
        assert_eq!(super::p2(INPUT), 26984457539);

        let input = super::input(super::DAY);
        assert_eq!(super::p2(&input), 1631647919273);
    }
}
