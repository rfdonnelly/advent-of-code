use std::collections::HashMap;
use std::fs;
use std::io;

use crate::lib::parse_input;

pub(crate) fn main() -> io::Result<()> {
    let (part1, part2) = day14();

    println!("day14::part1: {}", part1);
    println!("day14::part2: {}", part2);

    Ok(())
}

fn day14() -> (usize, usize) {
    let input = fs::read_to_string("input/14").unwrap();

    let reactions: Vec<Reaction> = parse_input(&input, Reaction::from);

    (part1(reactions.clone()), part2(reactions))
}

#[derive(Clone, Debug)]
struct ChemicalUnits {
    name: String,
    units: usize,
}

impl From<&str> for ChemicalUnits {
    fn from(s: &str) -> Self {
        let mut units_name = s.split(" ");
        let units = units_name.next().unwrap().parse::<usize>().unwrap();
        let name = units_name.next().unwrap().into();
        Self {name, units}
    }
}

#[derive(Clone, Debug)]
struct Reaction {
    inputs: Vec<ChemicalUnits>,
    output: ChemicalUnits,
}

impl From<&str> for Reaction {
    fn from(s: &str) -> Self {
        let mut inputs_output = s.split(" => ");
        let inputs = inputs_output.next().unwrap();
        let output = inputs_output.next().unwrap();

        let inputs: Vec<ChemicalUnits> = inputs
            .split(", ")
            .map(ChemicalUnits::from)
            .collect();

        let output = ChemicalUnits::from(output);

        Self {inputs, output}
    }
}

fn part1(reactions: Vec<Reaction>) -> usize {
    let map = map_from_slice(reactions);
    let mut units_required: HashMap<String, usize> = HashMap::new();

    let reaction = map.get("FUEL").unwrap();
    for input in &reaction.inputs {
        let units_required = units_required.entry(input.name.clone()).or_insert(0);
        *units_required += input.units;
    }

    dbg!(reaction);
    dbg!(units_required);

    0
}

fn map_from_slice(reactions: Vec<Reaction>) -> HashMap<String, Reaction> {
    let mut map = HashMap::new();
    let mut reactions = reactions;

    for reaction in reactions.drain(..) {
        map.insert(reaction.output.name.clone(), reaction);
    }

    map
}

fn part2(reactions: Vec<Reaction>) -> usize {
    0
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_day14() {
        assert_eq!(day14(), (1, 1))
    }
}
