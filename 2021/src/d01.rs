use crate::input;

use itertools::Itertools;

pub fn run() {
    let input = input(1);
    println!("d01p1: {}", d01p1(&input));
    println!("d01p2: {}", d01p2(&input));
}

fn d01p1(input: &str) -> usize {
    input
        .lines()
        .map(str::parse::<u32>)
        .map(Result::unwrap)
        .tuple_windows()
        .filter(|(a, b)| b > a)
        .count()
}

fn d01p2(input: &str) -> usize {
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
    fn d01p1() {
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
        assert_eq!(super::d01p1(input), 7);

        let input = super::input(1);
        assert_eq!(super::d01p1(&input), 1462);
    }

    #[test]
    fn d01p2() {
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
        assert_eq!(super::d01p2(input), 5);

        let input = super::input(1);
        assert_eq!(super::d01p2(&input), 1497);
    }
}
