use crate::input;

const DAY: usize = 4;

pub fn run() {
    let input = input(DAY);
    println!("d{:02}p1: {}", DAY, p1(&input));
    println!("d{:02}p2: {}", DAY, p2(&input));
}

#[derive(Debug)]
struct Board {
    values: Vec<u8>,
    marks: Vec<bool>,
}

impl Board {
    fn is_win(&self) -> bool {
        // Check rows
        let any_row = self
            .marks
            .chunks(5)
            .map(|row| row.iter().all(|&x| x))
            .any(|x| x);

        if any_row {
            return true;
        }

        // Check columns
        let any_col = (0..5)
            .map(|col_idx| {
                self.marks
                    .chunks(5)
                    .map(|row| row.iter().nth(col_idx).unwrap())
                    .all(|&x| x)
            })
            .any(|x| x);

        any_col
    }

    /// Marks the given value if it exists.
    ///
    /// Returns true if the board has bingo.
    fn mark(&mut self, value: u8) -> bool {
        let find_result = self.values.iter().enumerate().find(|(_i, &x)| x == value);

        if let Some((i, _x)) = find_result {
            self.marks[i] = true;
        }

        self.is_win()
    }

    fn sum_unmarked(&self) -> usize {
        self.marks
            .iter()
            .zip(self.values.iter())
            .filter_map(
                |(&marked, &value)| {
                    if !marked {
                        Some(value as usize)
                    } else {
                        None
                    }
                },
            )
            .sum()
    }
}

impl From<&[&str]> for Board {
    fn from(lines: &[&str]) -> Self {
        let mut lines = lines.iter();

        // Skip first (blank) line
        lines.next();

        let values = lines
            .flat_map(|line| {
                line.split(char::is_whitespace)
                    .map(str::parse::<u8>)
                    .filter_map(Result::ok)
            })
            .collect::<Vec<u8>>();

        let num_values = values.len();
        let marks = vec![false; num_values];

        Self { values, marks }
    }
}

#[derive(Debug)]
struct Bingo {
    draws: Vec<u8>,
    boards: Vec<Board>,
    winners: Vec<bool>,
}

impl From<&str> for Bingo {
    fn from(s: &str) -> Self {
        let mut lines = s.lines();
        let draws = lines
            .next()
            .unwrap()
            .split(",")
            .map(str::parse::<u8>)
            .filter_map(Result::ok)
            .collect::<Vec<u8>>();

        let lines = lines.collect::<Vec<&str>>();
        let boards = lines.chunks(6).map(Board::from).collect::<Vec<Board>>();

        let num_boards = boards.len();
        let winners = vec![false; num_boards];

        Self {
            draws,
            boards,
            winners,
        }
    }
}

#[derive(Clone, Copy)]
enum Until {
    FirstWin,
    LastWin,
}

impl Bingo {
    fn play(&mut self, until: Until) -> usize {
        let (board_idx, draw) = self.determine_winner(until);
        let sum = self.boards[board_idx].sum_unmarked();
        (draw as usize) * sum
    }

    fn determine_winner(&mut self, until: Until) -> (usize, u8) {
        for &draw in self.draws.iter() {
            for (board_idx, board) in self.boards.iter_mut().enumerate() {
                if board.mark(draw) {
                    self.winners[board_idx] = true;
                }

                let winner_determined = match until {
                    Until::FirstWin => self.winners.iter().any(|&x| x),
                    Until::LastWin => self.winners.iter().all(|&x| x),
                };

                if winner_determined {
                    return (board_idx, draw);
                }
            }
        }

        panic!()
    }
}

fn p1(input: &str) -> usize {
    Bingo::from(input).play(Until::FirstWin)
}

fn p2(input: &str) -> usize {
    Bingo::from(input).play(Until::LastWin)
}

#[cfg(test)]
mod tests {
    use indoc::indoc;

    #[test]
    fn p1() {
        let input = indoc! {"
            7,4,9,5,11,17,23,2,0,14,21,24,10,16,13,6,15,25,12,22,18,20,8,19,3,26,1

            22 13 17 11  0
             8  2 23  4 24
            21  9 14 16  7
             6 10  3 18  5
             1 12 20 15 19

             3 15  0  2 22
             9 18 13 17  5
            19  8  7 25 23
            20 11 10 24  4
            14 21 16 12  6

            14 21 17 24  4
            10 16 15  9 19
            18  8 23 26 20
            22 11 13  6  5
             2  0 12  3  7
        "};
        assert_eq!(super::p1(input), 4512);

        let input = super::input(super::DAY);
        assert_eq!(super::p1(&input), 8442);
    }

    #[test]
    fn p2() {
        let input = indoc! {"
            7,4,9,5,11,17,23,2,0,14,21,24,10,16,13,6,15,25,12,22,18,20,8,19,3,26,1

            22 13 17 11  0
             8  2 23  4 24
            21  9 14 16  7
             6 10  3 18  5
             1 12 20 15 19

             3 15  0  2 22
             9 18 13 17  5
            19  8  7 25 23
            20 11 10 24  4
            14 21 16 12  6

            14 21 17 24  4
            10 16 15  9 19
            18  8 23 26 20
            22 11 13  6  5
             2  0 12  3  7
        "};
        assert_eq!(super::p2(input), 1924);

        let input = super::input(super::DAY);
        assert_eq!(super::p2(&input), 4590);
    }
}
