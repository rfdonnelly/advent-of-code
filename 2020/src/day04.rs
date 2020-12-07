use std::collections::HashMap;
use std::str::FromStr;

use anyhow::Result;

use crate::lib::{self, Error};

pub fn day(day: usize, input: &str) -> Result<()> {
    let data = parse_input(input)?;
    println!("day{:02}::part1: {}", day, part1(&data)?);
    println!("day{:02}::part2: {}", day, part2(&data)?);

    Ok(())
}

fn part1(passports: &[Passport]) -> Result<usize> {
    let num_valid = passports.iter()
            .filter(|passport| passport.is_valid())
            .count();
    Ok(num_valid)
}

fn part2(passports: &[Passport]) -> Result<usize> {
    let num_valid = passports.iter()
            .filter(|passport| passport.is_extra_valid())
            .count();
    Ok(num_valid)
}

#[derive(Debug, Eq, PartialEq)]
struct Passport {
    pairs: HashMap<String, String>
}

impl Passport {
    fn is_valid(&self) -> bool {
        let required_keys = ["byr", "iyr", "eyr", "hgt", "hcl", "ecl", "pid"];
        let _optional_keys = ["cid"];
        required_keys.iter()
            .map(|&required_key| self.pairs.contains_key(required_key))
            .all(|contains_key| contains_key == true)
    }

    fn is_extra_valid(&self) -> bool {
        if !self.is_valid() { return false; }

        let byr = self.get_year("byr");
        if byr < 1920 || byr > 2002 { return false; }

        let iyr = self.get_year("iyr");
        if iyr < 2010 || iyr > 2020 { return false; }

        let eyr = self.get_year("eyr");
        if eyr < 2020 || eyr > 2030 { return false; }

        let hgt = self.pairs.get("hgt").unwrap();
        let hgt_value = hgt.chars().take_while(|c| match c { '0'..='9' => true, _ => false }).collect::<String>().parse::<u32>().unwrap();
        if hgt.contains("cm") {
            if hgt_value < 150 || hgt_value > 193 { return false; }
        } else if hgt.contains("in") {
            if hgt_value < 59 || hgt_value > 76 { return false; }
        } else {
            return false;
        }

        let hcl = self.pairs.get("hcl").unwrap();
        if hcl.len() != 7 { return false; }
        let mut hcl_chars = hcl.chars();
        if hcl_chars.next().unwrap() != '#' { return false; }
        if !hcl_chars.all(|c| match c { '0'..='9' | 'a'..='f' => true, _ => false }) { return false; }

        let valid_ecls = ["amb", "blu", "brn", "gry", "grn", "hzl", "oth"];
        let ecl = self.pairs.get("ecl").unwrap();
        if !valid_ecls.iter().any(|valid_ecl| ecl == valid_ecl) { return false; }

        let pid = self.pairs.get("pid").unwrap();
        if pid.len() != 9 { return false; }
        if !pid.chars().all(|c| match c { '0'..='9' => true, _ => false }) { return false; }

        true
    }

    fn get_year(&self, key: &str) -> u32 {
        let year = self.pairs.get(key).unwrap();
        year.parse::<u32>().unwrap()
    }
}

impl FromStr for Passport {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let pairs = s
            .trim_end()
            .split(|c| c == ' ' || c == '\n');
        let pairs = pairs
            .map(|pair| {
                let pair = pair
                    .split(":")
                    .collect::<Vec<&str>>();

                (pair[0].into(), pair[1].into())
            })
            .collect::<HashMap<_, _>>();

        Ok(Passport { pairs })
    }
}

fn parse_input(input: &str) -> Result<Vec<Passport>> {
    let passports = input.split("\n\n").collect::<Vec<&str>>();
    passports
        .into_iter()
        .map(Passport::from_str)
        .collect::<Result<Vec<_>>>()
}


#[cfg(test)]
mod test {
    use indoc::indoc;
    use super::Passport;

    fn input() -> &'static str {
        let input = indoc!{"
            ecl:gry pid:860033327 eyr:2020 hcl:#fffffd
            byr:1937 iyr:2017 cid:147 hgt:183cm

            iyr:2013 ecl:amb cid:350 eyr:2023 pid:028048884
            hcl:#cfa07d byr:1929

            hcl:#ae17e1 iyr:2013
            eyr:2024
            ecl:brn pid:760753108 byr:1931
            hgt:179cm

            hcl:#cfa07d eyr:2025 pid:166559648
            iyr:2011 ecl:brn hgt:59in
        "};

        input
    }

    fn passports() -> Vec<Passport> {
        super::parse_input(input()).unwrap()
    }

    #[test]
    fn part1() {
        assert_eq!(super::part1(&passports()).unwrap(), 2);
    }

    #[test]
    fn part2_invalid() {
        let input = indoc!{"
            eyr:1972 cid:100
            hcl:#18171d ecl:amb hgt:170 pid:186cm iyr:2018 byr:1926

            iyr:2019
            hcl:#602927 eyr:1967 hgt:170cm
            ecl:grn pid:012533040 byr:1946

            hcl:dab227 iyr:2012
            ecl:brn hgt:182cm pid:021572410 eyr:2020 byr:1992 cid:277

            hgt:59cm ecl:zzz
            eyr:2038 hcl:74454a iyr:2023
            pid:3556412378 byr:2007
        "};
        let input = super::parse_input(input).unwrap();
        assert_eq!(super::part2(&input).unwrap(), 0);
    }

    #[test]
    fn part2_valid() {
        let input = indoc!{"
            pid:087499704 hgt:74in ecl:grn iyr:2012 eyr:2030 byr:1980
            hcl:#623a2f

            eyr:2029 ecl:blu cid:129 byr:1989
            iyr:2014 pid:896056539 hcl:#a97842 hgt:165cm

            hcl:#888785
            hgt:164cm byr:2001 iyr:2015 cid:88
            pid:545766238 ecl:hzl
            eyr:2022

            iyr:2010 hgt:158cm hcl:#b6652a ecl:blu byr:1944 eyr:2021 pid:093154719
        "};
        let input = super::parse_input(input).unwrap();
        assert_eq!(super::part2(&input).unwrap(), 4);
    }
}
