use std::collections::HashSet;
use std::io::{self, Read};

fn main() -> io::Result<()> {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input)?;

    let deltas: Vec<i32> = input
        .lines()
        .filter_map(|line| line.parse::<i32>().ok())
        .collect();

    println!("sum: {}", deltas.iter().sum::<i32>());
    println!("first repeated sum: {}", first_repeated_sum(deltas).unwrap());

    Ok(())
}

fn first_repeated_sum(deltas: Vec<i32>) -> Option<i32> {
    let mut seen = HashSet::new();
    let mut current = 0;

    seen.insert(current);

    for delta in deltas.iter().cycle() {
        current += delta;

        if seen.contains(&current) {
            return Some(current);
        }

        seen.insert(current);
    }

    None
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example1() {
        let deltas: Vec<i32> = vec![1, -1];
        assert_eq!(first_repeated_sum(deltas), Some(0));
    }

    #[test]
    fn example2() {
        let deltas: Vec<i32> = vec![3, 3, 4, -2, -4];
        assert_eq!(first_repeated_sum(deltas), Some(10));
    }

    #[test]
    fn example3() {
        let deltas: Vec<i32> = vec![-6, 3, 8, 5, -6];
        assert_eq!(first_repeated_sum(deltas), Some(5));
    }

    #[test]
    fn example4() {
        let deltas: Vec<i32> = vec![7, 7, -2, -7, -4];
        assert_eq!(first_repeated_sum(deltas), Some(14));
    }
}
