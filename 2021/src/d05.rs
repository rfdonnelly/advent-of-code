use crate::input;

const DAY: usize = 5;

pub fn run() {
    let input = input(DAY);
    let time = std::time::Instant::now();
    println!("d{:02}p1: {} in {:?}", DAY, p1(&input), time.elapsed());
    let time = std::time::Instant::now();
    println!("d{:02}p2: {} in {:?}", DAY, p2(&input), time.elapsed());
}

#[derive(Copy, Clone, Debug, PartialEq, Eq)]
struct Point {
    x: usize,
    y: usize,
}

impl Default for Point {
    fn default() -> Self {
        Self { x: 0, y: 0 }
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
        let points = s.split(" -> ").map(Point::from).collect::<Vec<Point>>();

        Self {
            p0: points[0],
            p1: points[1],
        }
    }
}

#[derive(PartialEq)]
enum Orientation {
    Horizontal,
    Vertical,
    Diagonal,
}

impl Line {
    fn orientation(&self) -> Orientation {
        if self.p0.x == self.p1.x {
            Orientation::Vertical
        } else if self.p0.y == self.p1.y {
            Orientation::Horizontal
        } else {
            Orientation::Diagonal
        }
    }

    fn to_points(&self) -> Vec<Point> {
        let x_min = self.p0.x.min(self.p1.x);
        let x_max = self.p0.x.max(self.p1.x);
        let y_min = self.p0.y.min(self.p1.y);
        let y_max = self.p0.y.max(self.p1.y);

        match self.orientation() {
            Orientation::Vertical => {
                let x = self.p0.x;
                (y_min..=y_max).map(|y| Point { x, y }).collect()
            }
            Orientation::Horizontal => {
                let y = self.p0.y;
                (x_min..=x_max).map(|x| Point { x, y }).collect()
            }
            Orientation::Diagonal => {
                let x_range: Vec<usize> = if self.p0.x < self.p1.x {
                    (x_min..=x_max).collect()
                } else {
                    (x_min..=x_max).rev().collect()
                };

                let y_range: Vec<usize> = if self.p0.y < self.p1.y {
                    (y_min..=y_max).collect()
                } else {
                    (y_min..=y_max).rev().collect()
                };

                x_range
                    .iter()
                    .zip(y_range.iter())
                    .map(|(&x, &y)| Point { x, y })
                    .collect()
            }
        }
    }
}

#[derive(Clone, Debug)]
struct Board {
    lines: Vec<Line>,
    rows: usize,
    cols: usize,
}

impl Board {
    fn point_to_index(&self, p: &Point) -> usize {
        p.y * self.rows + p.x
    }

    fn count_non_diagonal_overlaps(&self) -> usize {
        let cells = vec![0; self.rows * self.cols];

        self.lines
            .iter()
            .filter(|line| line.orientation() != Orientation::Diagonal)
            .flat_map(Line::to_points)
            .fold(cells, |mut cells, p| {
                let index = self.point_to_index(&p);
                cells[index] += 1;
                cells
            })
            .iter()
            .filter(|&&x| x >= 2)
            .count()
    }

    fn count_overlaps(&self) -> usize {
        let cells = vec![0; self.rows * self.cols];

        let cells = self
            .lines
            .iter()
            .flat_map(Line::to_points)
            .fold(cells, |mut cells, p| {
                let index = self.point_to_index(&p);
                cells[index] += 1;
                cells
            });

        cells.iter().filter(|&&x| x >= 2).count()
    }
}

impl From<&str> for Board {
    fn from(s: &str) -> Self {
        let lines = s.lines().map(Line::from).collect::<Vec<Line>>();

        let max =
            lines
                .iter()
                .flat_map(|line| [line.p0, line.p1])
                .fold(Point::default(), |acc, p| Point {
                    x: acc.x.max(p.x),
                    y: acc.y.max(p.y),
                });

        let rows = max.y + 1;
        let cols = max.x + 1;

        Self { lines, rows, cols }
    }
}

fn p1(input: &str) -> usize {
    let board = Board::from(input);
    board.count_non_diagonal_overlaps()
}

fn p2(input: &str) -> usize {
    let board = Board::from(input);
    board.count_overlaps()
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
    fn p2() {
        let points = super::Line::from("1,1 -> 3,3").to_points();
        assert_eq!(
            points,
            vec![
                super::Point { x: 1, y: 1 },
                super::Point { x: 2, y: 2 },
                super::Point { x: 3, y: 3 },
            ]
        );
        let points = super::Line::from("9,7 -> 7,9").to_points();
        assert_eq!(
            points,
            vec![
                super::Point { x: 9, y: 7 },
                super::Point { x: 8, y: 8 },
                super::Point { x: 7, y: 9 },
            ]
        );

        assert_eq!(super::p2(INPUT), 12);

        let input = super::input(super::DAY);
        assert_eq!(super::p2(&input), 20196);
    }
}
