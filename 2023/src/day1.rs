use aoc_runner_derive::{aoc, aoc_generator};

#[aoc_generator(day1)]
fn parse(input: &str) -> Vec<u32> {
    input.lines().map(|line| {
        let mut digits = line
            .chars()
            .filter_map(to_digit_base_10);
        let first = digits.next().unwrap();
        let last = digits.last().unwrap_or(first);
        first * 10 + last
    })
    .collect()
}

fn to_digit_base_10(c: char) -> Option<u32> {
    c.to_digit(10)
}

#[aoc(day1, part1)]
fn part1(input: &[u32]) -> u32 {
    input.iter().sum()
}

#[aoc(day1, part2)]
fn part2(input: &[u32]) -> String {
    todo!()
}


#[cfg(test)]
mod tests {
    use super::*;
    use indoc::indoc;

    const INPUT: &str = indoc! {"
        1abc2
        pqr3stu8vwx
        a1b2c3d4e5f
        treb7uchet
    "};

    #[test]
    fn test_parse() {
        let expected = vec![
            12,
            38,
            15,
            77,
        ];

        assert_eq!(parse(INPUT), expected);
    }

    #[test]
    fn part1_example() {
        assert_eq!(part1(&parse(INPUT)), 142);
    }

    #[test]
    fn part2_example() {
        assert_eq!(part2(&parse(INPUT)), "<RESULT>");
    }
}
