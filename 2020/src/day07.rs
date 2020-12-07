use std::str::FromStr;
use std::collections::{HashMap, HashSet};

use anyhow::Result;

use crate::lib;

pub fn day(day: usize, input: &str) -> Result<()> {
    let data = lib::parse_input(input)?;
    println!("day{:02}::part1: {}", day, part1(&data)?);
    println!("day{:02}::part2: {}", day, part2(&data)?);

    Ok(())
}

fn part1(rules: &[Rule]) -> Result<usize> {
    let mut map: HashMap<&str, Vec<&str>> = HashMap::new();

    for rule in rules {
        for item in rule.content.iter() {
            let entry = map.entry(&item.1).or_insert_with(|| Vec::new());
            entry.push(&rule.desc);
        }
    }

    let mut containing = HashSet::new();
    gather_containing("shiny gold", &map, &mut containing);

    Ok(containing.len())
}

fn gather_containing<'a>(key: &'a str, map: &HashMap<&str, Vec<&'a str>>, containing: &mut HashSet<&'a str>) {
    if map.contains_key(key) {
        for desc in map.get(key).unwrap() {
            gather_containing(desc, map, containing);
            containing.insert(desc);
        }
    }
}

fn part2(rules: &[Rule]) -> Result<u32> {
    let mut map: HashMap<&str, Vec<&(u32, String)>> = HashMap::new();

    for rule in rules {
        let entry = map.entry(&rule.desc).or_insert_with(|| Vec::new());
        for item in rule.content.iter() {
            entry.push(item);
        }
    }

    let count = count_bags("shiny gold", &map);
    Ok(count)
}

fn count_bags(key: &str, map: &HashMap<&str, Vec<&(u32, String)>>) -> u32 {
    map.get(key)
        .unwrap()
        .iter()
        .map(|(qty, desc)| qty + qty * count_bags(desc, map))
        .sum::<u32>()
}

#[derive(Debug, Eq, PartialEq)]
struct Rule {
    desc: String,
    content: Vec<(u32, String)>,
}

impl FromStr for Rule {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut iter = s.split(" ");

        let desc = iter.by_ref().take(2).collect::<Vec<&str>>().join(" ");
        let mut content = Vec::new();
        iter.next();
        iter.next();
        loop {
            if let Some(qty) = iter.next() {
                let qty = if let Ok(qty) = qty.parse::<u32>() {
                    qty
                } else {
                    break
                };
                let desc = iter.by_ref().take(2).collect::<Vec<&str>>().join(" ");
                content.push((qty, desc));
                iter.next();
            } else {
                break;
            }
        }

        Ok(Rule { desc, content })
    }
}

#[cfg(test)]
mod test {
    use indoc::indoc;
    use super::{lib, Rule};

    #[test]
    fn parse() {
        let input = indoc!{"
            faded cyan bags contain 1 dim brown bag, 5 wavy magenta bags, 3 vibrant chartreuse bags, 4 muted fuchsia bags.
        "};

        let actual: Vec<Rule> = lib::parse_input(input).unwrap();
        let expected = vec![
            Rule { desc: "faded cyan".into(), content: vec![
                (1, "dim brown".into()),
                (5, "wavy magenta".into()),
                (3, "vibrant chartreuse".into()),
                (4, "muted fuchsia".into()),
            ] },
        ];

        assert_eq!(actual, expected);
    }

    fn data() -> Vec<Rule> {
        let input = indoc!{"
            light red bags contain 1 bright white bag, 2 muted yellow bags.
            dark orange bags contain 3 bright white bags, 4 muted yellow bags.
            bright white bags contain 1 shiny gold bag.
            muted yellow bags contain 2 shiny gold bags, 9 faded blue bags.
            shiny gold bags contain 1 dark olive bag, 2 vibrant plum bags.
            dark olive bags contain 3 faded blue bags, 4 dotted black bags.
            vibrant plum bags contain 5 faded blue bags, 6 dotted black bags.
            faded blue bags contain no other bags.
            dotted black bags contain no other bags.
        "};

        lib::parse_input(input).unwrap()
    }

    #[test]
    fn part1() {
        assert_eq!(super::part1(&data()).unwrap(), 4);
    }

    #[test]
    fn part2() {
        assert_eq!(super::part2(&data()).unwrap(), 32);
    }

    #[test]
    fn part2_2() {
        let input = indoc!{"
            shiny gold bags contain 2 dark red bags.
            dark red bags contain 2 dark orange bags.
            dark orange bags contain 2 dark yellow bags.
            dark yellow bags contain 2 dark green bags.
            dark green bags contain 2 dark blue bags.
            dark blue bags contain 2 dark violet bags.
            dark violet bags contain no other bags.
        "};

        let data = lib::parse_input(input).unwrap();

        assert_eq!(super::part2(&data).unwrap(), 126);
    }
}
