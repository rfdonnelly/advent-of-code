use anyhow::Result;

use crate::lib::{self, Error};

pub fn day(day: usize, input: &str) -> Result<()> {
    let data = lib::parse_input(input)?;
    println!("day{:02}::part1: {}", day, part1(&data)?);
    println!("day{:02}::part2: {}", day, part2(&data)?);

    Ok(())
}

fn part1(values: &[u32]) -> Result<u32> {
    let values = {
        let mut values = values.to_vec();
        values.sort();
        // Charging outlet
        values.insert(0, 0);
        // Device
        values.push(values.iter().max().unwrap() + 3);
        values
    };

    let diffs = values.windows(2)
        .map(|window| window[1] - window[0])
        .collect::<Vec<u32>>();

    let accs = diffs.iter().fold([0; 2], |mut accs, diff| {
        match diff {
            1 => accs[0] += 1,
            3 => accs[1] += 1,
            _ => (),
        }
        accs
    });
    Ok(accs.iter().product())
}

fn part2(values: &[u32]) -> Result<u32> {
    Ok(0)
}

#[cfg(test)]
mod test {
    use indoc::indoc;
    use super::lib;

    fn data() -> Vec<u32> {
        let input = indoc!{"
            28
            33
            18
            42
            31
            14
            46
            20
            48
            47
            24
            23
            49
            45
            19
            38
            39
            11
            1
            32
            25
            35
            8
            17
            7
            9
            4
            2
            34
            10
            3
        "};

        lib::parse_input(input).unwrap()
    }

    #[test]
    fn part1() {
        assert_eq!(super::part1(&data()).unwrap(), 220);
    }

    #[test]
    #[ignore]
    fn part2() {
        assert_eq!(super::part2(&data()).unwrap(), 62);
    }
}
