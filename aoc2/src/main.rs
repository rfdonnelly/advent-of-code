use std::collections::hash_map::HashMap;
use std::io::{self, Read};

fn main() -> io::Result<()> {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input)?;

    let ids: Vec<&str> = input
        .lines()
        .collect();

    println!("hash: {}", hashids(&ids));

    Ok(())
}

#[derive(Debug, PartialEq)]
struct IDProperties {
    has_two: bool,
    has_three: bool,
}

fn idprops(id: &str) -> IDProperties {
    let mut counts: HashMap<char, u32> = HashMap::new();

    for c in id.chars() {
        counts.entry(c)
            .and_modify(|e| *e += 1)
            .or_insert(1);
    }

    let values: Vec<&u32> = counts.values().collect();

    IDProperties {
        has_two: values.contains(&&2),
        has_three: values.contains(&&3),
    }
}

fn hashprops(props: &[IDProperties]) -> usize {
    let total_twos = props.iter()
        .filter(|props| props.has_two)
        .count();

    let total_threes = props.iter()
        .filter(|props| props.has_three)
        .count();

    total_twos * total_threes
}

fn hashids(ids: &[&str]) -> usize {
    let props: Vec<IDProperties> = ids.iter()
        .map(|id| idprops(id))
        .collect();

    hashprops(&props)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example() {
        let inputs = vec![
            ("abcdef", IDProperties { has_two: false, has_three: false }),
            ("bababc", IDProperties { has_two: true, has_three: true }),
            ("abbcde", IDProperties { has_two: true, has_three: false }),
            ("abcccd", IDProperties { has_two: false, has_three: true }),
            ("aabcdd", IDProperties { has_two: true, has_three: false }),
            ("abcdee", IDProperties { has_two: true, has_three: false }),
            ("ababab", IDProperties { has_two: false, has_three: true }),
        ];

        for input in inputs.iter() {
            assert_eq!(idprops(input.0), input.1);
        }

        let props: Vec<IDProperties> = inputs.iter()
            .map(|input| idprops(input.0))
            .collect();

        assert_eq!(hashprops(&props), 12);

        let ids: Vec<&str> = inputs.iter()
            .map(|input| input.0)
            .collect();

        assert_eq!(hashids(&ids), 12);
    }
}
