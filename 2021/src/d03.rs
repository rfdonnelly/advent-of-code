use crate::input;

use std::ops::Add;

const DAY: usize = 3;

pub fn run() {
    let input = input(DAY);
    println!("d{:02}p1: {}", DAY, p1(&input));
    println!("d{:02}p2: {}", DAY, p2(&input));
}

fn p1(input: &str) -> i32 {
    input
        .lines()
        .count() as i32
}

fn p2(input: &str) -> i32 {
    input
        .lines()
        .count() as i32
}

#[cfg(test)]
mod tests {
    use indoc::indoc;

    #[test]
    fn p1() {
        let input = indoc! {"
        "};
        assert_eq!(super::p1(input), -1);

        let input = super::input(super::DAY);
        assert_eq!(super::p1(&input), -1);
    }

    #[test]
    fn p2() {
        let input = indoc! {"
        "};
        assert_eq!(super::p2(input), -1);

        let input = super::input(super::DAY);
        assert_eq!(super::p2(&input), -1);
    }
}
