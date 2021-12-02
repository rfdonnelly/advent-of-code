use crate::input;

use std::ops::Add;

pub fn run() {
    let input = input(2);
    println!("d02p1: {}", d02p1(&input));
    println!("d02p2: {}", d02p2(&input));
}

struct Vector {
    x: i32,
    y: i32,
}

impl From<&str> for Vector {
    fn from(s: &str) -> Self {
        let mut splits = s.split(" ");
        let direction = splits.next().unwrap();
        let magnitude = splits.next().unwrap().parse::<i32>().unwrap();

        match direction {
            "forward" => Self { x: magnitude, y: 0 },
            "down" => Self { x: 0, y: magnitude },
            "up" => Self { x: 0, y: 0 - magnitude },
            _ => panic!(),
        }
    }
}

impl Add for Vector {
    type Output = Self;

    fn add(self, other: Self) -> Self {
        Self {
            x: self.x + other.x,
            y: self.y + other.y,
        }
    }
}

fn d02p1(input: &str) -> i32 {
    let p = input
        .lines()
        .map(Vector::from)
        .reduce(|a, b| a + b)
        .unwrap();

    p.x * p.y
}

fn d02p2(input: &str) -> usize {
    0
}

#[cfg(test)]
mod tests {
    use indoc::indoc;

    #[test]
    fn d02p1() {
        let input = indoc!{"
            forward 5
            down 5
            forward 8
            up 3
            down 8
            forward 2
        "};
        assert_eq!(super::d02p1(input), 150);

        let input = super::input(2);
        assert_eq!(super::d02p1(&input), 1690020);
    }
}

