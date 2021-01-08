use std::collections::HashMap;

use crate::solution::Solution;

pub(crate) struct Solver(());

impl Solver {
    pub fn new() -> Self {
        let solver = Self(());
        assert_solver_day!(solver);
        solver
    }
}

impl crate::Solver for Solver {
    fn day(&self) -> u8 {
        4
    }

    fn solve(&self, input: &str) -> Solution {
        let input = parse(input);

        let part1 = part1(&input);
        let part2 = part2(&input);

        (part1, part2).into()
    }
}

fn parse(input: &str) -> Vec<HashMap<&str, &str>> {
    input
        .split("\n\n")
        .map(|passport| {
            passport
                .split_whitespace()
                .map(|field| {
                    let mut field = field.splitn(2, ':');
                    (field.next().unwrap(), field.next().unwrap())
                })
                .collect::<HashMap<_, _>>()
        })
        .collect::<Vec<_>>()
}

const REQUIRED: [&str; 7] = ["byr", "iyr", "eyr", "hgt", "hcl", "ecl", "pid"];

fn part1(input: &[HashMap<&str, &str>]) -> usize {
    input
        .iter()
        .filter(|&passport| REQUIRED.iter().all(|&field| passport.contains_key(field)))
        .count()
}

fn part2(input: &[HashMap<&str, &str>]) -> usize {
    input
        .iter()
        .filter(|&passport| {
            REQUIRED
                .iter()
                .all(|&field| match passport.get_key_value(field) {
                    Some((&field, &value)) => validate(field, value),
                    None => false,
                })
        })
        .count()
}

fn validate(field: &str, value: &str) -> bool {
    match field {
        "byr" => validate_number(value, 4, 1920..=2002),
        "iyr" => validate_number(value, 4, 2010..=2020),
        "eyr" => validate_number(value, 4, 2020..=2030),
        "hgt" => match value.get(value.len() - 2..) {
            Some("in") => validate_number(&value[..value.len() - 2], 2, 59..=76),
            Some("cm") => validate_number(&value[..value.len() - 2], 3, 150..=193),
            _ => false,
        },
        "hcl" => match value.get(0..=0) {
            Some("#") => value[1..].chars().all(|c| c.is_ascii_hexdigit()),
            _ => false,
        },
        "ecl" => matches!(value, "amb" | "blu" | "brn" | "gry" | "grn" | "hzl" | "oth"),
        "pid" => validate_number(value, 9, None),
        _ => false,
    }
}

// TODO: figure out how to make this work in without resorting to generics
//       i.e: range: Option<impl std::ops::RangeBounds<i32>>
fn validate_number<T>(value: &str, len: usize, range: T) -> bool
where
    T: Into<Option<std::ops::RangeInclusive<i32>>>,
{
    len == value.len()
        && match value.parse::<i32>() {
            Ok(i) => match range.into() {
                Some(range) => range.contains(&i),
                _ => true,
            },
            _ => false,
        }
}

#[cfg(test)]
mod test {
    #![allow(unused_imports)]
    use super::*;
    use crate::Solver;

    #[test]
    fn example_part1() {
        const INPUT: &str = "\
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

";

        let input = parse(INPUT);

        let expected = 2;
        let actual = part1(&input);

        assert_eq!(actual, expected)
    }

    #[test]
    fn valid() {
        assert!(validate("byr", "2002"));
        assert!(validate("hgt", "60in"));
        assert!(validate("hgt", "190cm"));
        assert!(validate("hcl", "#123abc"));
        assert!(validate("ecl", "brn"));
        assert!(validate("pid", "000000001"));
    }

    #[test]
    fn invalid() {
        assert!(!validate("byr", "2003"));
        assert!(!validate("hgt", "190in"));
        assert!(!validate("hgt", "190"));
        assert!(!validate("hcl", "#123abz"));
        assert!(!validate("hcl", "123abc"));
        assert!(!validate("ecl", "wat"));
        assert!(!validate("pid", "0123456789"));
    }

    #[test]
    fn solution_part1() {
        let input = parse(include_str!("../../input/day04.txt"));

        let expected = 222;
        let actual = part1(&input);

        assert_eq!(actual, expected)
    }

    #[test]
    fn solution_part2() {
        let input = parse(include_str!("../../input/day04.txt"));

        let expected = 140;
        let actual = part2(&input);

        assert_eq!(actual, expected)
    }
}
