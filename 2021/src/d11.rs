use crate::input;

use std::fmt;

const DAY: usize = 11;

pub fn run() {
    let input = input(DAY);
    let time = std::time::Instant::now();
    println!("d{:02}p1: {} in {:?}", DAY, p1(&input), time.elapsed());
    let time = std::time::Instant::now();
    println!("d{:02}p2: {} in {:?}", DAY, p2(&input), time.elapsed());
}

#[derive(Copy, Clone, PartialEq, Eq)]
struct Grid {
    xs: usize,
    ys: usize,
    flashes: usize,
    values: [[u8; 10]; 10],
}

impl From<&str> for Grid {
    fn from(s: &str) -> Self {
        let xs = s.lines().next().unwrap().len();

        let ys = s.lines().count();

        let values = s
            .lines()
            .enumerate()
            .flat_map(|(y, line)| {
                line.chars().enumerate().map(move |(x, c)| {
                    let value = c.to_digit(10).unwrap() as u8;
                    (x, y, value)
                })
            })
            .fold([[0u8; 10]; 10], |mut values, (x, y, value)| {
                values[x][y] = value;
                values
            });

        Self {
            xs,
            ys,
            flashes: 0,
            values,
        }
    }
}

impl fmt::Debug for Grid {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        writeln!(f)?;

        for y in 0..self.ys {
            for x in 0..self.xs {
                let value = self.values[x][y];
                if value > 9 {
                    write!(f, "x")?;
                } else {
                    write!(f, "{}", value)?;
                }
            }
            writeln!(f)?;
        }

        Ok(())
    }
}

impl Grid {
    fn step_for(&mut self, steps: usize) -> usize {
        (0..steps).for_each(|_| self.step());

        self.flashes
    }

    fn step_until(&mut self, flashes_in_step: usize) -> usize {
        for step in 1.. {
            let flashes = self.flashes;
            self.step();
            let flashes_diff = self.flashes - flashes;
            if flashes_diff == flashes_in_step {
                return step;
            }
        }

        unreachable!()
    }

    fn step(&mut self) {
        for y in 0..self.ys {
            for x in 0..self.xs {
                self.values[x][y] += 1;
            }
        }

        for y in 0..self.ys {
            for x in 0..self.xs {
                self.maybe_flash(x, y);
            }
        }

        for y in 0..self.ys {
            for x in 0..self.xs {
                if self.values[x][y] > 9 {
                    self.values[x][y] = 0;
                }
            }
        }
    }

    fn maybe_flash(&mut self, x: usize, y: usize) {
        if self.values[x][y] == 10 {
            // Increment past 9 so we don't flash again this step
            self.values[x][y] += 1;
            self.flashes += 1;

            let neighbors = [-1, 0, 1]
                .iter()
                .flat_map(|xd| [-1, 0, 1].iter().map(move |yd| (xd, yd)))
                .filter_map(|(xd, yd)| {
                    let xn = (x as i64) + xd;
                    let yn = (y as i64) + yd;
                    let valid = xn >= 0
                        && xn <= (self.xs as i64) - 1
                        && yn >= 0
                        && yn <= (self.ys as i64) - 1
                        && !(xd == &0 && yd == &0);
                    if valid {
                        Some((xn as usize, yn as usize))
                    } else {
                        None
                    }
                })
                .collect::<Vec<(usize, usize)>>();

            for &(xn, yn) in &neighbors {
                if self.values[xn][yn] < 10 {
                    self.values[xn][yn] += 1;
                }
            }

            for &(xn, yn) in &neighbors {
                self.maybe_flash(xn, yn);
            }
        }
    }
}

fn p1(input: &str) -> usize {
    Grid::from(input).step_for(100)
}

fn p2(input: &str) -> usize {
    Grid::from(input).step_until(100)
}

#[cfg(test)]
mod tests {
    use super::*;

    use indoc::indoc;

    const INPUT: &str = indoc! {"
        5483143223
        2745854711
        5264556173
        6141336146
        6357385478
        4167524645
        2176841721
        6882881134
        4846848554
        5283751526
    "};

    #[test]
    fn p1() {
        assert_eq!(super::p1(INPUT), 1656);

        let input = input(DAY);
        assert_eq!(super::p1(&input), 1747);
    }

    #[test]
    fn p2() {
        assert_eq!(super::p2(INPUT), 195);

        let input = input(DAY);
        assert_eq!(super::p2(&input), 505);
    }
}
