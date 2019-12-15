use std::collections::HashMap;
use std::collections::HashSet;
use std::collections::VecDeque;
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

trait DivCeil {
    fn div_ceil(&self, other: Self) -> Self;
}

impl DivCeil for usize {
    fn div_ceil(&self, other: Self) -> Self {
        1 + ((self - 1) / other)
    }
}

#[derive(Clone, Debug)]
struct Chemical {
    name: String,
    units: usize,
}

impl From<&str> for Chemical {
    fn from(s: &str) -> Self {
        let mut units_name = s.split(" ");
        let units = units_name.next().unwrap().parse::<usize>().unwrap();
        let name = units_name.next().unwrap().into();
        Self {name, units}
    }
}

#[derive(Clone, Debug)]
struct Reaction {
    id: String,
    units: usize,
    children: HashMap<String, usize>,
}

impl From<&str> for Reaction {
    fn from(s: &str) -> Self {
        let mut inputs_output = s.split(" => ");
        let inputs = inputs_output.next().unwrap();
        let output = inputs_output.next().unwrap();

        let inputs: Vec<Chemical> = inputs
            .split(", ")
            .map(Chemical::from)
            .collect();

        let output = Chemical::from(output);

        let mut reaction = Self {
            id: output.name,
            units: output.units,
            children: HashMap::new(),
        };

        for input in inputs {
            reaction.children.insert(input.name, input.units);
        }

        reaction
    }
}

fn part1(reactions: Vec<Reaction>) -> usize {
    let map = output_map(&reactions);

    let mut needs: HashMap<&str, usize> = HashMap::new();
    let mut to_visit: VecDeque<&str> = VecDeque::new();
    let mut visited: HashSet<&str> = HashSet::new();

    needs.insert("FUEL", 1);
    to_visit.push_back("FUEL");

    while !to_visit.is_empty() {
        let current = to_visit.pop_front().unwrap();
        let reaction = map.get(current).unwrap();
        let output_need = needs.get(current).unwrap();

        let multiplier = output_need.div_ceil(reaction.units);
        // println!("current:{} multiplier:{}", current, multiplier);
        for (id, units) in &reaction.children {
            let input_need = needs
                .entry(&id)
                .or_insert(0);

            let input_need_delta = multiplier * units;
            // println!("  input:{} {} need:{} delta:{}", units, id, input_need, input_need_delta);
            *input_need += input_need_delta;

            let visit = map.contains_key::<str>(&id) && !visited.contains::<str>(&id);
            if visit {
                to_visit.push_back(&id);
                visited.insert(&id);

            }
        }
    }

    *needs.get("ORE").unwrap()
}

/// Lookup reactions by their output
fn output_map<'a>(reactions: &'a [Reaction]) -> HashMap<&'a str, &'a Reaction> {
    let mut output_map = HashMap::new();

    for reaction in reactions {
        output_map.insert(reaction.id.as_str(), reaction);
    }

    output_map
}

/// Lookup reactions by their input
fn input_map<'a>(reactions: &'a [Reaction]) -> HashMap<&'a str, Vec<&'a Reaction>> {
    let mut input_map = HashMap::new();

    for reaction in reactions {
        for (child_id, _child_units) in &reaction.children {
            input_map
                .entry(child_id.as_str())
                .or_insert(Vec::new())
                .push(reaction);
        }
    }

    input_map
}

fn part2(reactions: Vec<Reaction>) -> usize {
    0
}

#[cfg(test)]
mod tests {
    use super::*;

    use indoc::indoc;

    #[test]
    fn test_part1_simple1() {
        let input = indoc!("
            10 ORE => 5 A
            7 A => 1 FUEL
        ");

        let reactions: Vec<Reaction> = parse_input(&input, Reaction::from);
        assert_eq!(part1(reactions), 20);
    }

    #[test]
    fn test_part1_simple2() {
        let input = indoc!("
            10 ORE => 5 A
            5 ORE => 5 B
            7 A, 5 B => 1 FUEL
        ");

        let reactions: Vec<Reaction> = parse_input(&input, Reaction::from);
        assert_eq!(part1(reactions), 25);
    }

    #[test]
    fn test_part1_simple3() {
        let input = indoc!("
            10 ORE => 1 D
            15 B => 5 A
            5 D => 5 B
            5 B, 7 A => 1 FUEL
        ");

        // A: 7
        // B: 5 + 2*15 = 35
        // D: 7*5 = 35
        // ORE: 35*10 = 350

        // 10 ORE => 1 D
        // 5 D => 5 B ==> (5 (10 ORE)) => 5 B
        // 15 B => 5 A ==> (3 (5 (10 ORE)) => 5 A
        // 5 B, 7 A => 1 FUEL ==> (5 (10 ORE)), (2 (3 (5 (10 ORE)))) => 1 FUEL
        //   ==> 50 ORE + 300 ORE

        // Need to visit nodes in certain order
        // Cannot visit nodes before visiting dependent nodes

        let reactions: Vec<Reaction> = parse_input(&input, Reaction::from);
        assert_eq!(part1(reactions), 350);
    }

    #[test]
    fn test_part1_example0() {
        let input = indoc!("
            9 ORE => 2 A
            8 ORE => 3 B
            7 ORE => 5 C
            3 A, 4 B => 1 AB
            5 B, 7 C => 1 BC
            4 C, 1 A => 1 CA
            2 AB, 3 BC, 4 CA => 1 FUEL
        ");

        // AB: 2
        // BC: 3
        // CA: 4
        // C: 4*4 + 3*7 = 16 + 21 = 37
        // A: 4*1 + 2*3 = 4 + 6 = 10
        // B: 3*5 + 2*4 = 15 + 8 = 23
        // ORE: 5*9 + 8*8 + 8*7 = 45 + 64 + 56 = 45 + 120 = 165

        let reactions: Vec<Reaction> = parse_input(&input, Reaction::from);
        assert_eq!(part1(reactions), 165);
    }

    #[test]
    fn test_part1_example1() {
        let input = indoc!("
            157 ORE => 5 NZVS
            165 ORE => 6 DCFZ
            44 XJWVT, 5 KHKGT, 1 QDVJ, 29 NZVS, 9 GPVTF, 48 HKGWZ => 1 FUEL
            12 HKGWZ, 1 GPVTF, 8 PSHF => 9 QDVJ
            179 ORE => 7 PSHF
            177 ORE => 5 HKGWZ
            7 DCFZ, 7 PSHF => 2 XJWVT
            165 ORE => 2 GPVTF
            3 DCFZ, 7 NZVS, 5 HKGWZ, 10 PSHF => 8 KHKGT
        ");

        let reactions: Vec<Reaction> = parse_input(&input, Reaction::from);
        assert_eq!(part1(reactions), 13312);
    }

    #[test]
    fn test_part1_example2() {
        let input = indoc!("
            2 VPVL, 7 FWMGM, 2 CXFTF, 11 MNCFX => 1 STKFG
            17 NVRVD, 3 JNWZP => 8 VPVL
            53 STKFG, 6 MNCFX, 46 VJHF, 81 HVMC, 68 CXFTF, 25 GNMV => 1 FUEL
            22 VJHF, 37 MNCFX => 5 FWMGM
            139 ORE => 4 NVRVD
            144 ORE => 7 JNWZP
            5 MNCFX, 7 RFSQX, 2 FWMGM, 2 VPVL, 19 CXFTF => 3 HVMC
            5 VJHF, 7 MNCFX, 9 VPVL, 37 CXFTF => 6 GNMV
            145 ORE => 6 MNCFX
            1 NVRVD => 8 CXFTF
            1 VJHF, 6 MNCFX => 4 RFSQX
            176 ORE => 6 VJHF
        ");

        let reactions: Vec<Reaction> = parse_input(&input, Reaction::from);
        assert_eq!(part1(reactions), 180697);
    }

    #[test]
    fn test_part1_example3() {
        let input = indoc!("
            171 ORE => 8 CNZTR
            7 ZLQW, 3 BMBT, 9 XCVML, 26 XMNCP, 1 WPTQ, 2 MZWV, 1 RJRHP => 4 PLWSL
            114 ORE => 4 BHXH
            14 VRPVC => 6 BMBT
            6 BHXH, 18 KTJDG, 12 WPTQ, 7 PLWSL, 31 FHTLT, 37 ZDVW => 1 FUEL
            6 WPTQ, 2 BMBT, 8 ZLQW, 18 KTJDG, 1 XMNCP, 6 MZWV, 1 RJRHP => 6 FHTLT
            15 XDBXC, 2 LTCX, 1 VRPVC => 6 ZLQW
            13 WPTQ, 10 LTCX, 3 RJRHP, 14 XMNCP, 2 MZWV, 1 ZLQW => 1 ZDVW
            5 BMBT => 4 WPTQ
            189 ORE => 9 KTJDG
            1 MZWV, 17 XDBXC, 3 XCVML => 2 XMNCP
            12 VRPVC, 27 CNZTR => 2 XDBXC
            15 KTJDG, 12 BHXH => 5 XCVML
            3 BHXH, 2 VRPVC => 7 MZWV
            121 ORE => 7 VRPVC
            7 XCVML => 6 RJRHP
            5 BHXH, 4 VRPVC => 5 LTCX
        ");

        let reactions: Vec<Reaction> = parse_input(&input, Reaction::from);
        assert_eq!(part1(reactions), 2210736);
    }

    #[test]
    fn test_day14() {
        assert_eq!(day14(), (1, 1))
    }
}
