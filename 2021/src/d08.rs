use crate::input;

const DAY: usize = 8;

pub fn run() {
    let input = input(DAY);
    println!("d{:02}p1: {}", DAY, p1(&input));
    println!("d{:02}p2: {}", DAY, p2(&input));
}

#[derive(Debug, Clone, Copy)]
struct Segments(u8);

impl From<&str> for Segments {
    fn from(s: &str) -> Self {
        s
            .chars()
            .map(|c| c as u8 - 'a' as u8)
            .fold(Self(0), |mut acc, index| {
                acc.0 |= 1 << index;
                acc
            })
    }
}

impl Segments {
    fn active_segments(&self) -> usize {
        self.0.count_ones() as usize
    }
}

#[derive(Debug)]
struct Entry {
    patterns: Vec<Segments>,
    output: Vec<Segments>,
}

impl From<&str> for Entry {
    fn from(s: &str) -> Self {
        let mut halves = s
            .split(" | ")
            .map(|half| {
                half
                    .split(" ")
                    .map(Segments::from)
                    .collect::<Vec<Segments>>()
            });

        Self {
            patterns: halves.next().unwrap(),
            output: halves.next().unwrap(),
        }
    }
}

fn p1(input: &str) -> usize {
    let entries = input
        .lines()
        .map(Entry::from)
        .collect::<Vec<Entry>>();

    entries
        .iter()
        .flat_map(|entry| entry.output.as_slice())
        .map(Segments::active_segments)
        .filter(|active_segments| [2, 4, 3, 7].iter().any(|x| x == active_segments))
        .count()
}

fn p2(input: &str) -> usize {
    todo!()
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
    #[ignore]
    fn p2() {
        assert_eq!(super::p2(INPUT), 168);

        let input = super::input(super::DAY);
        assert_eq!(super::p2(&input), 97164301);
    }
}
