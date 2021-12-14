use crate::input;

use std::ops::BitAnd;

const DAY: usize = 8;

pub fn run() {
    let input = input(DAY);
    let time = std::time::Instant::now();
    println!("d{:02}p1: {} in {:?}", DAY, p1(&input), time.elapsed());
    let time = std::time::Instant::now();
    println!("d{:02}p2: {} in {:?}", DAY, p2(&input), time.elapsed());
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
struct Pattern {
    segments: u8,
    active: u8,
}

impl Default for Pattern {
    fn default() -> Self {
        Self {
            segments: 0,
            active: 0,
        }
    }
}

impl From<&str> for Pattern {
    fn from(s: &str) -> Self {
        s.chars()
            .map(|c| c as u8 - 'a' as u8)
            .fold(Pattern::default(), |mut acc, index| {
                acc.segments |= 1 << index;
                acc.active += 1;
                acc
            })
    }
}

impl BitAnd for Pattern {
    type Output = Self;

    fn bitand(self, rhs: Self) -> Self::Output {
        let segments = self.segments & rhs.segments;
        let active = segments.count_ones() as u8;
        Self { segments, active }
    }
}

// digit active_segments
// 0     6
// 1     2
// 2     5
// 3     5
// 4     4
// 5     5
// 6     6
// 7     3
// 8     7
// 9     6
//
// active_segments possible_digits
// 2               1
// 3               7
// 4               4
// 5               2, 3, 5
// 6               0, 6, 9
// 7               8
impl Pattern {
    fn decode(&self, one: Pattern, four: Pattern) -> usize {
        match self.active {
            2 => 1,
            3 => 7,
            4 => 4,
            5 => {
                if *self & one == one {
                    3
                } else {
                    match (*self & four).active {
                        2 => 2,
                        3 => 5,
                        _ => unreachable!(),
                    }
                }
            }
            6 => {
                if (*self & one).active == 1 {
                    6
                } else if *self & four == four {
                    9
                } else {
                    0
                }
            }
            7 => 8,
            _ => unreachable!(),
        }
    }
}

#[derive(Debug)]
struct Entry {
    patterns: Vec<Pattern>,
    output: Vec<Pattern>,
}

impl From<&str> for Entry {
    fn from(s: &str) -> Self {
        let mut halves = s
            .split(" | ")
            .map(|half| half.split(" ").map(Pattern::from).collect::<Vec<Pattern>>());

        Self {
            patterns: halves.next().unwrap(),
            output: halves.next().unwrap(),
        }
    }
}

impl Entry {
    fn output_value(&self) -> usize {
        let one = self
            .patterns
            .iter()
            .find(|&&segments| segments.active == 2)
            .unwrap();

        let four = self
            .patterns
            .iter()
            .find(|&&segments| segments.active == 4)
            .unwrap();

        self.output
            .iter()
            .map(|segments| segments.decode(*one, *four))
            .enumerate()
            .map(|(i, digit)| digit * 10_usize.pow(3 - i as u32))
            .sum()
    }
}

fn p1(input: &str) -> usize {
    let entries = input.lines().map(Entry::from).collect::<Vec<Entry>>();

    entries
        .iter()
        .flat_map(|entry| entry.output.as_slice())
        .map(|segments| segments.active)
        .filter(|active| [2, 4, 3, 7].iter().any(|x| x == active))
        .count()
}

fn p2(input: &str) -> usize {
    let entries = input.lines().map(Entry::from).collect::<Vec<Entry>>();

    entries.iter().map(Entry::output_value).sum()
}

#[cfg(test)]
mod tests {
    use indoc::indoc;

    const INPUT: &str = indoc! {"
        be cfbegad cbdgef fgaecd cgeb fdcge agebfd fecdb fabcd edb | fdgacbe cefdb cefbgd gcbe
        edbfga begcd cbg gc gcadebf fbgde acbgfd abcde gfcbed gfec | fcgedb cgb dgebacf gc
        fgaebd cg bdaec gdafb agbcfd gdcbef bgcad gfac gcb cdgabef | cg cg fdcagb cbg
        fbegcd cbd adcefb dageb afcb bc aefdc ecdab fgdeca fcdbega | efabcd cedba gadfec cb
        aecbfdg fbg gf bafeg dbefa fcge gcbea fcaegb dgceab fcbdga | gecf egdcabf bgf bfgea
        fgeab ca afcebg bdacfeg cfaedg gcfdb baec bfadeg bafgc acf | gebdcfa ecba ca fadegcb
        dbcfg fgd bdegcaf fgec aegbdf ecdfab fbedc dacgb gdcebf gf | cefg dcbef fcge gbcadfe
        bdfegc cbegaf gecbf dfcage bdacg ed bedf ced adcbefg gebcd | ed bcgafe cdgba cbgef
        egadfb cdbfeg cegd fecab cgb gbdefca cg fgcdab egfdb bfceg | gbdfcae bgc cg cgb
        gcafb gcf dcaebfg ecagb gf abcdeg gaef cafbge fdbac fegbdc | fgae cfgab fg bagce
    "};

    #[test]
    fn p1() {
        assert_eq!(super::p1(INPUT), 26);

        let input = super::input(super::DAY);
        assert_eq!(super::p1(&input), 534);
    }

    #[test]
    fn p2() {
        assert_eq!(super::p2(INPUT), 61229);

        let input = super::input(super::DAY);
        assert_eq!(super::p2(&input), 1070188);
    }
}
