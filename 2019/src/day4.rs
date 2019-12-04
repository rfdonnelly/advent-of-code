use std::io;

pub(crate) fn main() -> io::Result<()> {
    println!("day4::part1: {}", num_valid(is_valid_part1));
    println!("day3::part2: {}", num_valid(is_valid_part2));

    Ok(())
}

fn num_valid<F>(is_valid: F) -> usize
where
    F: Fn(Password, u64, u64) -> bool
{
    let mut p = [0; 6];
    let l = to_compare([2, 4, 5, 3, 1, 8]);
    let u = to_compare([7, 6, 5, 7, 4, 7]);
    let mut num_valid = 0;

    for _ in 0..10_usize.pow(6) {
        p = next(p);

        if is_valid(p, l, u) {
            num_valid += 1;
        }
    }

    num_valid
}

fn next(c: Password) -> Password {
    let mut i = 0;
    let mut p = c;

    loop {
        if p[i] == 9 {
            p[i] = 0;
            i += 1;

            if i == p.len() {
                return p;
            }
        } else {
            p[i] += 1;
            return p;
        }
    }
}

type Password = [u8; 6];

fn is_valid_part1(p: Password, l: u64, u: u64) -> bool {
    has_two_or_more_consecutive(p)
    && is_increasing(p)
    && in_range(p, l, u)
}

fn is_valid_part2(p: Password, l: u64, u: u64) -> bool {
    has_exactly_two_consecutive(p)
    && is_increasing(p)
    && in_range(p, l, u)
}

fn has_two_or_more_consecutive(p: Password) -> bool {
    p[0] == p[1]
    || p[1] == p[2]
    || p[2] == p[3]
    || p[3] == p[4]
    || p[4] == p[5]
}

fn has_exactly_two_consecutive(p: Password) -> bool {
    (p[0] == p[1] && p[1] != p[2])
    || (p[0] != p[1] && p[1] == p[2] && p[2] != p[3])
    || (p[1] != p[2] && p[2] == p[3] && p[3] != p[4])
    || (p[2] != p[3] && p[3] == p[4] && p[4] != p[5])
    || (p[3] != p[4] && p[4] == p[5])
}

fn is_increasing(p: Password) -> bool {
    p[5] >= p[4]
    && p[4] >= p[3]
    && p[3] >= p[2]
    && p[2] >= p[1]
    && p[1] >= p[0]
}

fn in_range(p: Password, l: u64, u: u64) -> bool {
    let p_compare = to_compare(p);
    p_compare >= l && p_compare <= u
}

fn to_compare(p: Password) -> u64 {
    (p[0] as u64) << 5*4
    | (p[1] as u64) << 4*4
    | (p[2] as u64) << 3*4
    | (p[3] as u64) << 2*4
    | (p[4] as u64) << 1*4
    | (p[5] as u64) << 0*4
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_is_valid_part1() {
        let l = to_compare([0; 6]);
        let u = to_compare([9; 6]);

        let p = [0, 1, 2, 3, 4, 4];
        assert_eq!(is_valid_part1(p, l, u), true);
    }

    #[test]
    fn test_is_valid_part1_non_adjacent() {
        let l = to_compare([0; 6]);
        let u = to_compare([9; 6]);

        let p = [0, 1, 2, 3, 4, 5];
        assert_eq!(is_valid_part1(p, l, u), false);
    }

    #[test]
    fn test_is_valid_part1_non_incrementing() {
        let l = to_compare([0; 6]);
        let u = to_compare([9; 6]);

        let p = [0, 2, 2, 0, 4, 5];
        assert_eq!(is_valid_part1(p, l, u), false);
    }

    #[test]
    fn test_is_valid_part1_lt_range() {
        let l = to_compare([2, 4, 5, 3, 1, 8]);
        let u = to_compare([7, 6, 5, 7, 4, 7]);

        let p = [1, 2, 2, 3, 4, 5];
        assert_eq!(is_valid_part1(p, l, u), false);
    }

    #[test]
    fn test_is_valid_part1_gt_range() {
        let l = to_compare([2, 4, 5, 3, 1, 8]);
        let u = to_compare([7, 6, 5, 7, 4, 7]);

        let p = [8, 8, 8, 8, 8, 9];
        assert_eq!(is_valid_part1(p, l, u), false);
    }

    #[test]
    fn test_is_valid_part1_in_range() {
        let l = to_compare([2, 4, 5, 3, 1, 8]);
        let u = to_compare([7, 6, 5, 7, 4, 7]);

        let p = [2, 4, 5, 6, 7, 7];
        assert_eq!(is_valid_part1(p, l, u), true);
    }
}
