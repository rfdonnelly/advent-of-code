use crate::input;

use itertools::Itertools;

const DAY: usize = 1;

pub fn run() {
    let input = input(DAY);
    println!("d{:02}p1: {}", DAY, p1(&input));
    println!("d{:02}p2: {}", DAY, p2(&input));
}

fn p1(input: &str) -> usize {
    input
        .lines()
        .map(str::parse::<u32>)
        .map(Result::unwrap)
        .tuple_windows()
        .filter(|(a, b)| b > a)
        .count()
}

fn p2(input: &str) -> usize {
    input
        .lines()
        .map(str::parse::<u32>)
        .map(Result::unwrap)
        .tuple_windows()
        .map(|(a, b, c)| a + b + c)
        .tuple_windows()
        .filter(|(a, b)| b > a)
        .count()
}

#[cfg(test)]
mod tests {
    use indoc::indoc;

    #[test]
    fn p1() {
        let input = indoc! {"
            199
            200
            208
            210
            200
            207
            240
            269
            260
            263
        "};
        assert_eq!(super::p1(input), 7);

        let input = super::input(super::DAY);
        assert_eq!(super::p1(&input), 1462);
    }

    #[test]
    fn p2() {
        let input = indoc! {"
            199
            200
            208
            210
            200
            207
            240
            269
            260
            263
        "};
        assert_eq!(super::p2(input), 5);

        let input = super::input(super::DAY);
        assert_eq!(super::p2(&input), 1497);
    }
}
