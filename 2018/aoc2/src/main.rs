use std::collections::hash_map::HashMap;
use std::io::{self, Read};

fn main() -> io::Result<()> {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input)?;

    let ids: Vec<&str> = input
        .lines()
        .collect();

    println!("hash: {}", hashids(&ids));
    println!("common letters: {}", part2(&ids));

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

fn single_char_diff_pos(a: &str, b: &str) -> Option<usize> {
    // Return None if strings differ by more than one char
    let pairs = a.chars().zip(b.chars());

    let mut diff_count = 0;
    let mut diff_index = 0;

    for pair in pairs.enumerate() {
        let (i, (a, b)) = pair;

        if a != b {
            diff_index = i;

            if diff_count == 1 {
                return None;
            } else {
                diff_count += 1;
            }
        }
    }

    assert!(diff_count <= 1);

    if diff_count > 0 {
        Some(diff_index)
    } else {
        None
    }
}

fn find_single_pair_with_single_char_diff(ids: &[&str]) -> Option<(String, String)> {
    for a in ids {
        for b in ids {
            if a != b {
                if let Some(_) = single_char_diff_pos(a, b) {
                    return Some((a.to_string(), b.to_string()));
                }
            }
        }
    }

    None
}

fn same_chars(a: String, b: String) -> String {
    a.chars().zip(b.chars())
        .filter(|(a, b)| a == b)
        .map(|(a, _)| a)
        .collect()
}

fn part2(ids: &[&str]) -> String {
    if let Some((a, b)) = find_single_pair_with_single_char_diff(ids) {
        same_chars(a, b)
    } else {
        "".into()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example1() {
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

    #[test]
    fn example2() {
        assert_eq!(single_char_diff_pos(&"abcde", &"axcye"), None);
        assert_eq!(single_char_diff_pos(&"fghij", &"fguij"), Some(2));

        let ids = vec!(
            "abcde",
            "fghij",
            "klmno",
            "pqrst",
            "fguij",
            "axcye",
            "wvxyz",
        );

        assert_eq!(find_single_pair_with_single_char_diff(&ids), Some(("fghij".into(), "fguij".into())));

        assert_eq!(same_chars("fghij".into(), "fguij".into()), "fgij".to_string());

        assert_eq!(part2(&ids), "fgij".to_string());
    }
}
