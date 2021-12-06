use crate::input;

const DAY: usize = 5;

pub fn run() {
    let input = input(DAY);
    println!("d{:02}p1: {}", DAY, p1(&input));
    println!("d{:02}p2: {}", DAY, p2(&input));
}

#[derive(Copy, Clone, Debug)]
struct Point {
    x: usize,
    y: usize,
}

impl Default for Point {
    fn default() -> Self {
        Self {
            x: 0,
            y: 0,
        }
    }
}

impl From<&str> for Point {
    fn from(s: &str) -> Self {
        let values = s
            .split(",")
            .map(str::parse::<usize>)
            .filter_map(Result::ok)
            .collect::<Vec<usize>>();

        Self {
            x: values[0],
            y: values[1],
        }
    }
}

#[derive(Copy, Clone, Debug)]
struct Line {
    p0: Point,
    p1: Point,
}

impl From<&str> for Line {
    fn from(s: &str) -> Self {
        let points = s
            .split(" -> ")
            .map(Point::from)
            .collect::<Vec<Point>>();

        Self {
            p0: points[0],
            p1: points[1],
        }
    }
}

#[derive(Clone, Debug)]
struct Board {
    rows: usize,
    cols: usize,
    cells: Vec<usize>,
}

impl Board {
    fn mark_line(&mut self, line: &Line) {
        // Assumes lines are horizontal or vertical (i.e. not diagonal).
        let x_min = line.p0.x.min(line.p1.x);
        let x_max = line.p0.x.max(line.p1.x);
        let y_min = line.p0.y.min(line.p1.y);
        let y_max = line.p0.y.max(line.p1.y);
        for x in x_min..=x_max {
            for y in y_min..=y_max {
                self.mark_point(&Point {x, y});
            }
        }
    }

    fn mark_point(&mut self, p: &Point) {
        let index = p.y * self.rows + p.x;
        self.cells[index] += 1;
    }

    fn count_atleast_two_overlaps(&self) -> usize {
        self.cells
            .iter()
            .filter(|&&x| x >= 2)
            .count()
    }
}

impl From<&[Line]> for Board {
    fn from(lines: &[Line]) -> Self {
        let max = lines
            .iter()
            .flat_map(|line| [line.p0, line.p1])
            .fold(Point::default(), |acc, p| {
                Point {
                    x: acc.x.max(p.x),
                    y: acc.y.max(p.y),
                }
            });

        let rows = max.y + 1;
        let cols = max.x + 1;
        let cells = vec![0; rows * cols];

        let mut board = Self { rows, cols, cells };
        let lines = lines
            .iter()
            .filter(|line| {
                line.p0.x == line.p1.x
                || line.p0.y == line.p1.y
            })
            .for_each(|line| board.mark_line(line));

        board
    }
}

fn p1(input: &str) -> usize {
    let lines = input
        .lines()
        .map(Line::from)
        .collect::<Vec<Line>>();

    let board = Board::from(lines.as_slice());

    board.count_atleast_two_overlaps()
}

fn p2(input: &str) -> usize {
    0
}

#[cfg(test)]
mod tests {
    use indoc::indoc;

    const INPUT: &str = indoc! {"
        0,9 -> 5,9
        8,0 -> 0,8
        9,4 -> 3,4
        2,2 -> 2,1
        7,0 -> 7,4
        6,4 -> 2,0
        0,9 -> 2,9
        3,4 -> 1,4
        0,0 -> 8,8
        5,5 -> 8,2
    "};

    #[test]
    fn p1() {
        assert_eq!(super::p1(INPUT), 5);

        let input = super::input(super::DAY);
        assert_eq!(super::p1(&input), 6267);
    }

    #[test]
    #[ignore]
    fn p2() {
        assert_eq!(super::p2(INPUT), 1924);

        let input = super::input(super::DAY);
        assert_eq!(super::p2(&input), 4590);
    }
}
