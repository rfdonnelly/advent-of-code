use std::fs;
use std::io;

pub(crate) fn main() -> io::Result<()> {
    let input = fs::read_to_string("input/1")?;

    let masses: Vec<i32> = input
        .lines()
        .filter_map(|line| line.parse::<i32>().ok())
        .collect();

    println!("day1::part1: {}", part1(&masses));
    println!("day1::part2: {}", part2(&masses));

    Ok(())
}

fn part1(masses: &[i32]) -> i32 {
    masses
        .iter()
        .map(|&x| fuel_required_to_launch(x))
        .sum()
}

fn part2(masses: &[i32]) -> i32 {
    masses
        .iter()
        .map(|&x| total_fuel_required_to_launch(x))
        .sum()
}

fn fuel_required_to_launch(mass: i32) -> i32 {
    mass / 3 - 2
}

fn total_fuel_required_to_launch(mass: i32) -> i32 {
    let mut total = 0;
    let mut current = mass;

    loop {
        let next = fuel_required_to_launch(current);

        if next <= 0 {
            return total;
        } else {
            total += next;
        }

        current = next;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_fuel_required_to_launch() {
        assert_eq!(fuel_required_to_launch(12), 2);
        assert_eq!(fuel_required_to_launch(14), 2);
        assert_eq!(fuel_required_to_launch(1969), 654);
        assert_eq!(fuel_required_to_launch(100756), 33583);
    }

    #[test]
    fn test_total_fuel_required_to_launch() {
        assert_eq!(total_fuel_required_to_launch(14), 2);
        assert_eq!(total_fuel_required_to_launch(1969), 966);
        assert_eq!(total_fuel_required_to_launch(100756), 50346);
    }
}
