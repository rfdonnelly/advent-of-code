use std::collections::HashMap;
use std::fs;
use std::io;

pub(crate) fn main() -> io::Result<()> {
    let input = fs::read_to_string("input/6")?;

    let input: Vec<Tuple> = parse_input(&input, parse_line);

    println!("day6::part1: {}", part1(&input));
    // println!("day6::part2: {}", part2(input));

    Ok(())
}

fn parse_input<'a, T, F>(s: &'a str, parse_line: F) -> Vec<T>
where
    F: Fn(&'a str) -> T
{
    s
        .lines()
        .map(parse_line)
        .collect()
}

fn parse_line(line: &str) -> Tuple {
    let objs: Vec<&str> = line
        .split(")")
        .collect();

    (objs[0], objs[1])
}

type Map = HashMap<String, String>;
type Tuple<'a> = (&'a str, &'a str);

fn part1(entries: &[Tuple]) -> u32 {
    let orbits = to_map(entries);

    orbits
        .keys()
        .map(|name| num_orbits(name, &orbits))
        .sum::<u32>()
}

fn num_orbits(name: &str, map: &Map) -> u32 {
    let mut name = name;
    let mut count = 0;

    loop {
        match map.get(name) {
            Some(parent) => {
                count += 1;
                name = parent;
            }
            None => {
                return count;
            }
        }
    }
}

fn to_map(entries: &[Tuple]) -> Map {
    let mut hash = HashMap::new();

    for &entry in entries {
        let (value, key) = entry;
        hash.insert(key.into(), value.into());
    }

    hash
}

#[cfg(test)]
mod tests {
    use super::*;

    fn map() -> Map {
        let mut map = HashMap::new();
        map.insert("2".into(), "1".into());
        map.insert("3".into(), "2".into());
        map
    }

    #[test]
    fn test_to_map() {
        let map = map();
        assert_eq!(to_map(&[("1", "2"),("2", "3")]), map);
    }

    #[test]
    fn test_part1() {
        let input = "1)2\n2)3";
        let input: Vec<Tuple> = parse_input(&input, parse_line);
        assert_eq!(part1(&input), 3);
    }
}
