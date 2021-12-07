use crate::input;

const DAY: usize = 7;

pub fn run() {
    let input = input(DAY);
    println!("d{:02}p1: {}", DAY, p1(&input));
    println!("d{:02}p2: {}", DAY, p2(&input));
}

fn p1_cost(a: usize, b: usize) -> usize {
    ((a as i32) - (b as i32)).abs() as usize
}

fn p1(input: &str) -> usize {
    let mut positions = input
        .trim()
        .split(",")
        .map(str::parse::<usize>)
        .map(Result::unwrap)
        .collect::<Vec<usize>>();

    positions.sort();

    let median = positions[positions.len() / 2];
    let cost_fn = |&position| { p1_cost(position, median) };

    positions
        .iter()
        .map(cost_fn)
        .sum()
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
