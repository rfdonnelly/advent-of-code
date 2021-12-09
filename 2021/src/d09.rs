use crate::input;

const DAY: usize = 9;

pub fn run() {
    let input = input(DAY);
    println!("d{:02}p1: {}", DAY, p1(&input));
    println!("d{:02}p2: {}", DAY, p2(&input));
}

#[derive(Debug, PartialEq)]
struct Map {
    cols: usize,
    values: Vec<u8>,
}

impl From<&str> for Map {
    fn from(s: &str) -> Self {
        let cols = s
            .lines()
            .next()
            .unwrap()
            .len();

        let values = s
            .lines()
            .flat_map(|line| {
                line
                    .chars()
                    .map(|c| {
                        c
                            .to_digit(10)
                            .unwrap() as u8
                    })
            })
            .collect::<Vec<u8>>();

        Self { cols, values }
    }
}

impl Map {
    fn local_minima(&self) -> Vec<u8> {
        self.values
            .iter()
            .enumerate()
            .filter_map(|(i, _)| self.is_local_min(i))
            .collect::<Vec<u8>>()
    }

    /// Checks whether there is a local minimum at index
    ///
    /// Returns None if no local minimum.
    /// Returns Some(v) where v is the value of the local minimum.
    fn is_local_min(&self, index: usize) -> Option<u8> {
        let min_neighbor = self
            .neighbors(index)
            .iter()
            .filter_map(|x| x.as_ref())
            .min_by_key(|(_i, v)| v)
            .unwrap()
            .1;

        if self.values[index] < min_neighbor {
            Some(self.values[index])
        } else {
            None
        }
    }

    fn neighbors(&self, index: usize) -> [Option<(usize, u8)>; 4] {
        let col_index = index % self.cols;

        let at_left_edge = col_index == 0;
        let left =
            if at_left_edge {
                None
            } else {
                let index = index - 1;
                Some((index, self.values[index]))
            };

        let at_right_edge = col_index == self.cols - 1;
        let right =
            if at_right_edge {
                None
            } else {
                let index = index + 1;
                Some((index, self.values[index]))
            };

        let above_index = index.wrapping_sub(self.cols);
        let above = self.values
            .get(above_index)
            .map(|&v| (above_index, v));

        let below_index = index + self.cols;
        let below = self.values
            .get(below_index)
            .map(|&v| (below_index, v));

        [left, right, above, below]
    }
}

fn p1(input: &str) -> usize {
    let map = Map::from(input);

    map
        .local_minima()
        .iter()
        .map(|x| (x + 1) as usize)
        .sum()
}

fn p2(input: &str) -> usize {
    todo!()
}

#[cfg(test)]
mod tests {
    use super::*;

    use indoc::indoc;

    const INPUT: &str = indoc! {"
        2199943210
        3987894921
        9856789892
        8767896789
        9899965678
    "};

    #[test]
    fn p1() {
        assert_eq!(super::p1(INPUT), 15);

        let input = input(super::DAY);
        assert_eq!(super::p1(&input), 465);
    }

    #[test]
    #[ignore]
    fn p2() {
        assert_eq!(super::p2(INPUT), 61229);

        let input = input(super::DAY);
        assert_eq!(super::p2(&input), 1070188);
    }
}
