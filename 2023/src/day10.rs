use aoc_runner_derive::{aoc, aoc_generator};

use TileType::*;
use Dir::*;

#[derive(Debug, PartialEq, Clone, Copy)]
enum TileType {
    Ground,
    Start,
    Vertical,
    Horizontal,
    BendL,
    BendJ,
    Bend7,
    BendF,
}

#[derive(Debug, Clone, Copy)]
struct Tile {
    typ: TileType,
    connections: Connections,
}

impl Tile {
    fn can_connect(&self, dir: Dir, other: &Tile) -> bool {
        match dir {
            Up => self.connections[Up.to_idx()] && other.connections[Down.to_idx()],
            Down => self.connections[Down.to_idx()] && other.connections[Up.to_idx()],
            Left => self.connections[Left.to_idx()] && other.connections[Right.to_idx()],
            Right => self.connections[Right.to_idx()] && other.connections[Left.to_idx()],
        }
    }
}

#[derive(Debug)]
struct Board {
    cols: usize,
    tiles: Vec<Tile>,
}

impl From<&str> for Board {
    fn from(s: &str) -> Self {
        let cols = s.lines().next().unwrap().len();
        let tiles = s.chars()
            .filter(|c| !c.is_ascii_whitespace())
            .map(Tile::from)
            .collect();

        Self { cols, tiles }
    }
}

impl Board {
    fn neighbors(&self, index: usize) -> [Option<usize>; 4] {
        let cols = self.cols;
        let tiles = self.tiles.len();

        let col = index % cols;

        [
            (index >= cols).then(|| index - cols),
            (index < tiles - cols).then(|| index + cols),
            (col > 0).then(|| index - 1),
            (col < cols - 1).then(|| index + 1),
        ]
    }
}

type Connections = [bool; 4];
#[derive(Debug, Clone, Copy)]
enum Dir {
    Up,
    Down,
    Left,
    Right,
}

impl From<usize> for Dir {
    fn from(u: usize) -> Self {
        match u {
            0 => Up,
            1 => Down,
            2 => Left,
            3 => Right,
            _ => unreachable!(),
        }
    }
}

impl Dir {
    fn to_idx(&self) -> usize {
        match self {
            Up => 0,
            Down => 1,
            Left => 2,
            Right => 3,
        }
    }
}

impl From<char> for Tile {
    fn from(c: char) -> Self {
        let (typ, connections) = match c {
            '.' => (Ground, [false; 4]),
            'S' => (Start, [true; 4]),
            '|' => (Vertical, [true, true, false, false]),
            '-' => (Horizontal, [false, false, true, true]),
            'L' => (BendL, [true, false, false, true]),
            'J' => (BendJ, [true, false, true, false]),
            '7' => (Bend7, [false, true, true, false]),
            'F' => (BendF, [false, true, false, true]),
            _ => unreachable!(),
        };

        Self { typ, connections }
    }
}

#[aoc_generator(day10)]
fn parse(input: &str) -> Board {
    Board::from(input)
}

#[aoc(day10, part1)]
fn part1(input: &Board) -> usize {
    let initial_idx = input.tiles.iter().position(|tile| tile.typ == Start).unwrap();

    let mut prev_idx = initial_idx;
    let mut curr_idx = initial_idx;
    let mut count: usize = 0;

    loop {
        let (_next_dir, next_idx) = input
            .neighbors(curr_idx)
            .iter()
            .enumerate()
            .filter_map(|(dir, maybe_idx)| maybe_idx.map(|idx| (Dir::from(dir), idx)))
            .filter(|&(_, idx)| idx != prev_idx)
            .filter(|&(dir, idx)| {
                input.tiles[curr_idx].can_connect(dir, &input.tiles[idx])
            })
            .next()
            .unwrap();

        // dbg!(&(next_dir, next_idx, input.tiles[next_idx]));

        if next_idx == initial_idx {
            break;
        } else {
            prev_idx = curr_idx;
            curr_idx = next_idx;
            count += 1
        }
    }

    count.div_ceil(2)
}

#[aoc(day10, part2)]
fn part2(input: &Board) -> usize {
    todo!()
}

#[cfg(test)]
mod tests {
    use super::*;

    use indoc::indoc;

    const INPUT: &str = indoc! {"
        7-F7-
        .FJ|7
        SJLL7
        |F--J
        LJ.LJ
    "};

    #[test]
    fn part1_example() {
        assert_eq!(part1(&parse(INPUT)), 8);
    }

    #[test]
    fn part2_example() {
        assert_eq!(part2(&parse(INPUT)), 0);
    }
}
