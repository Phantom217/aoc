use std::ops::RangeInclusive;

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
        16
    }

    fn solve(&self, input: &str) -> Solution {
        let notes = Notes::from(input);

        let part1 = part1(&notes);
        let part2 = part2(&notes);

        (part1, part2).into()
    }
}

fn part1(notes: &Notes) -> u16 {
    notes
        .nearby
        .iter()
        .flatten()
        .filter(|n| {
            !notes
                .rules
                .iter()
                .flat_map(|(_, ranges)| ranges)
                .any(|range| range.contains(n))
        })
        .sum()
}

fn part2(notes: &Notes) -> u64 {
    use std::collections::HashSet;

    let mut rules = notes
        .nearby
        .iter()
        .filter(|nearby| {
            nearby.iter().all(|n| {
                notes
                    .rules
                    .iter()
                    .flat_map(|(_, ranges)| ranges)
                    .any(|range| range.contains(n))
            })
        })
        .fold(
            vec![(1 << notes.rules.len()) - 1; notes.rules.len()],
            |mut valid_possibilities, nearby_ticket| {
                for (possibility, ticket_pos) in valid_possibilities.iter_mut().zip(nearby_ticket) {
                    for (i, (_, ranges)) in notes.rules.iter().enumerate() {
                        if !ranges.iter().any(|range| range.contains(ticket_pos)) {
                            *possibility &= !(1_u32 << i);
                        }
                    }
                }
                valid_possibilities
            },
        );

    for _ in 0..rules.len() {
        let powers_of_two = rules
            .iter()
            .filter(|x| x.is_power_of_two())
            .fold(0, |a, b| a | b);

        let mut any_changes = false;
        for num in rules.iter_mut().filter(|x| !x.is_power_of_two()) {
            any_changes = true;
            *num &= !powers_of_two;
        }

        if !any_changes {
            break;
        }
    }

    assert!(rules.iter().all(|x| x.is_power_of_two()));

    let departure_fields: HashSet<u32> = notes
        .rules
        .iter()
        .enumerate()
        .filter(|(_, (field, _))| field.starts_with("departure"))
        .map(|(i, _)| 1 << i)
        .collect();

    rules
        .iter()
        .enumerate()
        .filter(|(_, value)| departure_fields.contains(value))
        .map(|(i, _)| u64::from(notes.your[i]))
        .product()
}

#[derive(Debug, PartialEq, Eq)]
struct Notes<'n> {
    rules: Vec<(&'n str, [RangeInclusive<u16>; 2])>,
    your: Vec<u16>,
    nearby: Vec<Vec<u16>>,
}

impl<'n> std::convert::From<&'n str> for Notes<'n> {
    fn from(input: &'n str) -> Self {
        let mut input = input.split("\n\n");

        let rules = input
            .next()
            .unwrap()
            .lines()
            .map(|line| {
                let mut line = line.split(':');
                let field = line.next().unwrap();
                let ranges = line
                    .next()
                    .unwrap()
                    .split(" or ")
                    .map(|range| {
                        let mut range = range.split('-');
                        let start: u16 = range.next().unwrap().trim().parse().unwrap();
                        let end: u16 = range.next().unwrap().trim().parse().unwrap();

                        RangeInclusive::new(start, end)
                    })
                    .collect::<Vec<RangeInclusive<u16>>>();

                (field, [ranges[0].clone(), ranges[1].clone()])
            })
            .collect();

        let your = input
            .next()
            .unwrap()
            .lines()
            .nth(1)
            .unwrap()
            .split(',')
            .map(|n| {
                n.parse::<u16>()
                    .unwrap_or_else(|_| panic!("Invalid digit: {}", n))
            })
            .collect::<Vec<u16>>();

        let nearby = input
            .next()
            .unwrap()
            .lines()
            .skip(1)
            .map(|line| {
                line.split(',')
                    .map(|n| {
                        n.parse::<u16>()
                            .unwrap_or_else(|_| panic!("Invalid digit: {}", n))
                    })
                    .collect()
            })
            .collect::<Vec<Vec<u16>>>();

        Self {
            rules,
            your,
            nearby,
        }
    }
}

#[cfg(test)]
mod test {
    #![allow(unused_imports)]
    use super::*;
    use crate::solution::Solution;
    use crate::Solver;

    const INPUT_PART_1: &str = "\
class: 1-3 or 5-7
row: 6-11 or 33-44
seat: 13-40 or 45-50

your ticket:
7,1,14

nearby tickets:
7,3,47
40,4,50
55,2,20
38,6,12";

    const INPUT_PART_2: &str = "\
class: 0-1 or 4-19
row: 0-5 or 8-19
seat: 0-13 or 16-19

your ticket:
11,12,13

nearby tickets:
3,9,18
15,1,5
5,14,9";

    #[test]
    fn test_parse_from_str() {
        let expected = Notes {
            rules: vec![
                (
                    "class",
                    [RangeInclusive::new(1, 3), RangeInclusive::new(5, 7)],
                ),
                (
                    "row",
                    [RangeInclusive::new(6, 11), RangeInclusive::new(33, 44)],
                ),
                (
                    "seat",
                    [RangeInclusive::new(13, 40), RangeInclusive::new(45, 50)],
                ),
            ],
            your: vec![7, 1, 14],
            nearby: vec![
                vec![7, 3, 47],
                vec![40, 4, 50],
                vec![55, 2, 20],
                vec![38, 6, 12],
            ],
        };
        let actual = Notes::from(INPUT_PART_1);
        assert_eq!(actual, expected)
    }

    #[test]
    fn example_part1() {
        let expected = 71;
        let actual = part1(&Notes::from(INPUT_PART_1));
        assert_eq!(actual, expected)
    }

    #[test]
    fn verify() {
        let solver = super::Solver::new();
        let input = include_str!("../../input/day16.txt");

        let expected: Solution = (26_941_u16, 634_796_407_951_u64).into();
        let actual = solver.solve(input);

        assert_eq!(actual, expected)
    }
}
