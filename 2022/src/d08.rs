use aoc_runner_derive::{aoc, aoc_generator};
use ndarray::{Array2, ArrayBase};

type Input = Array2<u32>;

#[aoc_generator(day8)]
fn parse(input: &str) -> Input {
    let cols = input.lines().next().unwrap().len();
    let rows = input.lines().count();
    let heights = input
        .chars()
        .filter_map(|c| c.to_digit(10));
    ArrayBase::from_iter(heights)
        .into_shape((rows, cols))
        .unwrap()
}

type Index = (usize, usize);
fn is_visible(index: Index, input: &Input) -> bool {
    is_visible_up(index, input)
    || is_visible_down(index, input)
    || is_visible_left(index, input)
    || is_visible_right(index, input)
}

fn is_visible_up(index: (usize, usize), input: &Input) -> bool {
    let (row_a, col_a) = index;
    let height_a = input[index];
    dbg!(row_a);
    for row_b in (0..row_a).rev() {
        let index_b = (row_b, col_a);
        let height_b = input[index_b];
        // println!("{index:?} {height_a} {index_b:?} {height_b}");
        if height_b >= height_a {
            return false;
        }
    }

    return true;
}

fn is_visible_down(index: (usize, usize), input: &Input) -> bool {
    let (row_a, col_a) = index;
    let height_a = input[index];
    for row_b in row_a+1..input.nrows() {
        let index_b = (row_b, col_a);
        let height_b = input[index_b];
        if height_b >= height_a {
            return false;
        }
    }

    return true;
}

fn is_visible_left(index: (usize, usize), input: &Input) -> bool {
    let (row_a, col_a) = index;
    let height_a = input[index];
    for col_b in (0..col_a).rev() {
        let index_b = (row_a, col_b);
        let height_b = input[index_b];
        if height_b >= height_a {
            return false;
        }
    }

    return true;
}

fn is_visible_right(index: (usize, usize), input: &Input) -> bool {
    let (row_a, col_a) = index;
    let height_a = input[index];
    for col_b in col_a+1..input.ncols() {
        let index_b = (row_a, col_b);
        let height_b = input[index_b];
        if height_b >= height_a {
            return false;
        }
    }

    return true;
}

#[aoc(day8, part1)]
fn p1(input: &Input) -> usize {
    let (rows, cols) = input.dim();

    let mut nvisible = 0;
    for row in 1..rows-1 {
        for col in 1..cols-1 {
            if is_visible((row, col), input) {
                nvisible += 1;
            }
        }
    }

    let edge_visible = (rows - 1) * 2 + (cols - 1) * 2;
    let interior_visible = nvisible;

    edge_visible + interior_visible
}

#[aoc(day8, part2)]
fn p2(input: &Input) -> u32 {
    0
}

#[cfg(test)]
mod test {
    use super::*;
    use indoc::indoc;
    use ndarray::array;

    const INPUT: &str = indoc! {"
        30373
        25512
        65332
        33549
        35390
    "};

    #[test]
    fn test_parse() {
        let expected = array![
            [3, 0, 3, 7, 3],
            [2, 5, 5, 1, 2],
            [6, 5, 3, 3, 2],
            [3, 3, 5, 4, 9],
            [3, 5, 3, 9, 0],
        ];
        assert_eq!(parse(INPUT), expected);
    }

    #[test]
    fn test_p1() {
        assert_eq!(p1(&parse(INPUT)), 21);
    }

    #[test]
    fn test_p2() {
        assert_eq!(p2(&parse(INPUT)), 12);
    }
}

