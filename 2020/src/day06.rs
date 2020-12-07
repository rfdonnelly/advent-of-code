use std::collections::HashSet;

use anyhow::Result;

pub fn day(day: usize, input: &str) -> Result<()> {
    println!("day{:02}::part1: {}", day, part1(input)?);
    println!("day{:02}::part2: {}", day, part2(input)?);

    Ok(())
}

fn part1(input: &str) -> Result<usize> {
    let count = input
        .split("\n\n")
        .map(|group| {
            let mut set = HashSet::new();
            for c in group.chars() {
                if !c.is_whitespace() {
                    set.insert(c);
                }
            }
            set.len()
        })
        .sum();
    Ok(count)
}

fn part2(input: &str) -> Result<u32> {
    let count = input
        .split("\n\n")
        .map(|group|
             group
                .lines()
                .map(|person| {
                    let mut answers = 0u32;
                    for c in person.chars() {
                        let index = (c as u32) - ('a' as u32);
                        answers |= 1 << index;
                    }
                    answers
                 })
                 .fold(0xffffffff, |group_answers, person_answers|
                    group_answers & person_answers
                 )
                 .count_ones()
        )
        .sum();

    Ok(count)
}


#[cfg(test)]
mod test {
    use indoc::indoc;

    fn input() -> &'static str {
        let input = indoc!{"
            abc

            a
            b
            c

            ab
            ac

            a
            a
            a
            a

            b
        "};

        input
    }

    #[test]
    fn part1() {
        assert_eq!(super::part1(input()).unwrap(), 11);
    }

    #[test]
    fn part2() {
        assert_eq!(super::part2(input()).unwrap(), 6);
    }
}
