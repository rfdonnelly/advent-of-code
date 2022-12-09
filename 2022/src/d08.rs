use aoc_runner_derive::{aoc, aoc_generator};
use ndarray::{Array2, ArrayBase};

type Input = Array2<u32>;

#[aoc_generator(day8)]
fn parse(input: &str) -> Input {
    let cols = input.lines().next().unwrap().len();
    let rows = input.lines().count();
    let heights = input.chars().filter_map(|c| c.to_digit(10));
    ArrayBase::from_iter(heights)
        .into_shape((rows, cols))
        .unwrap()
}

type Position = (usize, usize);

fn is_visible(from: Position, heights: &Input) -> bool {
    is_edge(from, heights)
        || is_visible_up(from, heights)
        || is_visible_down(from, heights)
        || is_visible_left(from, heights)
        || is_visible_right(from, heights)
}

fn is_edge(position: Position, heights: &Input) -> bool {
    let (row, col) = position;
    row == 0 || col == 0 || row == heights.nrows() - 1 || col == heights.ncols() - 1
}

fn is_visible_up(from: Position, heights: &Input) -> bool {
    let (row, col) = from;
    let target_height = heights[from];
    (0..row)
        .rev()
        .map(|row_to| heights[(row_to, col)])
        .find(|&height| height >= target_height)
        .is_none()
}

fn is_visible_down(from: Position, heights: &Input) -> bool {
    let (row, col) = from;
    let target_height = heights[from];
    (row + 1..heights.nrows())
        .map(|row_to| heights[(row_to, col)])
        .find(|&height| height >= target_height)
        .is_none()
}

fn is_visible_left(from: Position, heights: &Input) -> bool {
    let (row, col) = from;
    let target_height = heights[from];
    (0..col)
        .rev()
        .map(|col_to| heights[(row, col_to)])
        .find(|&height| height >= target_height)
        .is_none()
}

fn is_visible_right(from: Position, heights: &Input) -> bool {
    let (row, col) = from;
    let target_height = heights[from];
    (col + 1..heights.ncols())
        .map(|col_to| heights[(row, col_to)])
        .find(|&height| height >= target_height)
        .is_none()
}

fn scenic_score(from: Position, heights: &Input) -> usize {
    scenic_score_up(from, heights)
        * scenic_score_down(from, heights)
        * scenic_score_left(from, heights)
        * scenic_score_right(from, heights)
}

fn scenic_score_up(from: Position, heights: &Input) -> usize {
    let (row, col) = from;
    let target_height = heights[from];
    (0..row)
        .rev()
        .map(|row_to| heights[(row_to, col)])
        .enumerate()
        .find_map(|(i, height)| (height >= target_height).then_some(i + 1))
        .unwrap_or(row)
}

fn scenic_score_down(from: Position, heights: &Input) -> usize {
    let (row, col) = from;
    let target_height = heights[from];
    (row + 1..heights.nrows())
        .map(|row_to| heights[(row_to, col)])
        .enumerate()
        .find_map(|(i, height)| (height >= target_height).then_some(i + 1))
        .unwrap_or(heights.nrows() - row - 1)
}

fn scenic_score_left(from: Position, heights: &Input) -> usize {
    let (row, col) = from;
    let target_height = heights[from];
    (0..col)
        .rev()
        .map(|col_to| heights[(row, col_to)])
        .enumerate()
        .find_map(|(i, height)| (height >= target_height).then_some(i + 1))
        .unwrap_or(col)
}

fn scenic_score_right(from: Position, heights: &Input) -> usize {
    let (row, col) = from;
    let target_height = heights[from];
    (col + 1..heights.ncols())
        .map(|col_to| heights[(row, col_to)])
        .enumerate()
        .find_map(|(i, height)| (height >= target_height).then_some(i + 1))
        .unwrap_or(heights.ncols() - col - 1)
}

#[aoc(day8, part1)]
fn p1(heights: &Input) -> usize {
    heights
        .indexed_iter()
        .filter_map(|(position, _)| is_visible(position, heights).then_some(0))
        .count()
}

#[aoc(day8, part2)]
fn p2(heights: &Input) -> usize {
    heights
        .indexed_iter()
        .map(|(position, _)| scenic_score(position, heights))
        .max()
        .unwrap()
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
        assert_eq!(p2(&parse(INPUT)), 8);
    }
}
