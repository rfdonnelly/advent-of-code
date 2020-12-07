use std::str::FromStr;

use anyhow::Result;

use crate::lib;

pub fn day(day: usize, input: &str) -> Result<()> {
    let data = lib::parse_input(input)?;
    println!("day{:02}::part1:\n{}", day, part1(&data)?);
    println!("day{:02}::part2:\n{}", day, part2(&data)?);

    Ok(())
}

fn part1(seats: &[Seat]) -> Result<u32> {
    Ok(seats.iter().map(|seat| seat.id).max().unwrap())
}

fn part2(seats: &[Seat]) -> Result<u32> {
    let mut seats = seats.iter()
        .filter_map(|seat|
            if seat.row > 0 && seat.row < 127 {
                Some(seat.id)
            } else {
                None
            }
        )
        .collect::<Vec<u32>>();

    seats.sort();

    let id = seats
        .chunks(2)
        .find_map(|pair|
            if pair[1] - pair[0] == 2 {
                Some(pair[0] + 1)
            } else {
                None
            }
        )
        .unwrap();

    Ok(id)
}

#[derive(Debug, Eq, PartialEq)]
struct Seat {
    row: u32,
    col: u32,
    id: u32,
}

impl FromStr for Seat {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut row = 0;
        let mut row_delta = 64;
        let mut col = 0;
        let mut col_delta = 4;

        for c in s.chars() {
            match c {
                'F' => row_delta /= 2,
                'B' => {
                    row += row_delta;
                    row_delta /= 2;
                }
                'L' => col_delta /= 2,
                'R' => {
                    col += col_delta;
                    col_delta /= 2;
                }
                _ => panic!(),
            }
        }

        let id = row * 8 + col;

        Ok(Seat { row, col, id })
    }
}

#[cfg(test)]
mod test {
    use indoc::indoc;
    use super::{lib, Seat};

    fn seats() -> Vec<Seat> {
        let input = indoc!{"
            FBFBBFFRLR
            BFFFBBFRRR
            FFFBBBFRRR
            BBFFBBFRLL
        "};
        lib::parse_input(input).unwrap()
    }

    #[test]
    fn parse() {
        let input = indoc!{"
            FBFBBFFRLR
            BFFFBBFRRR
            FFFBBBFRRR
            BBFFBBFRLL
        "};

        let actual: Vec<Seat> = lib::parse_input(input).unwrap();
        let expected = vec![
            Seat { row: 44, col: 5, id: 357 },
            Seat { row: 70, col: 7, id: 567 },
            Seat { row: 14, col: 7, id: 119 },
            Seat { row: 102, col: 4, id: 820 },
        ];

        assert_eq!(actual, expected);
    }

    #[test]
    fn part1() {
        assert_eq!(super::part1(&seats()).unwrap(), 820);
    }

    #[test]
    fn part2() {
        assert_eq!(super::part2(&seats()).unwrap(), 336);
    }
}
