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
        15
    }

    fn solve(&self, input: &str) -> Solution {
        let numbers = parse_input(input);
        // let numbers = vec![0, 13, 16, 17, 1, 10, 6];

        let part1 = part1(&numbers);
        let part2 = part2(&numbers);

        (part1, part2).into()
    }
}

fn parse_input(input: &str) -> Vec<u32> {
    input
        .trim_end()
        .split(',')
        .filter_map(|s| s.parse().ok())
        .collect()
}

fn part1(numbers: &[u32]) -> u32 {
    const TARGET: u32 = 2020;

    let mut seen = numbers
        .iter()
        .enumerate()
        .map(|(i, &n)| (n, i as u32 + 1))
        .collect::<HashMap<_, _>>();

    (numbers.len() as u32..TARGET).fold(*numbers.last().unwrap(), |last, turn| {
        seen.insert(last, turn)
            .map(|last_occurred| turn - last_occurred)
            .unwrap_or(0)
    })
}

fn part2(numbers: &[u32]) -> u32 {
    const TARGET: u32 = 30_000_000;

    let mut seen: Vec<Option<u32>> = vec![None; TARGET as usize];

    for (i, n) in numbers.iter().enumerate() {
        seen[*n as usize] = Some(i as u32);
    }

    let mut num = 0_u32;
    for i in numbers.len() as u32..TARGET - 1 {
        match seen[num as usize] {
            None => {
                seen[num as usize] = Some(i);
                num = 0;
            }
            Some(t) => {
                seen[num as usize] = Some(i);
                num = i - t;
            }
        };
    }

    num
}

#[cfg(test)]
mod test {
    #![allow(unused_imports)]
    use super::*;
    use crate::solution::Solution;
    use crate::Solver;

    const INPUT: [u32; 3] = [0, 3, 6];
    const INPUT_EXTRA: [[u32; 3]; 6] = [
        [1, 3, 2],
        [2, 1, 3],
        [1, 2, 3],
        [2, 3, 1],
        [3, 2, 1],
        [3, 1, 2],
    ];

    #[test]
    fn test_parse_input() {
        const INPUT: &str = "0,3,6";

        let expected = vec![0, 3, 6];
        let actual = parse_input(INPUT);
        assert_eq!(actual, expected)
    }

    #[test]
    fn example_part1() {
        let expected = 436;
        let actual = part1(&INPUT);
        assert_eq!(actual, expected)
    }

    #[test]
    fn example_part1_extra() {
        let expected_vec = [1, 10, 27, 78, 438, 1836];
        for (actual, &expected) in INPUT_EXTRA.iter().zip(expected_vec.iter()) {
            let actual = part1(actual);
            assert_eq!(actual, expected);
        }
    }

    #[test]
    fn example_part2() {
        let expected = 175594;
        let actual = part2(&INPUT);
        assert_eq!(actual, expected)
    }

    #[test]
    #[ignore = "test takes to long to run"]
    fn example_part2_extra() {
        use std::thread;

        let expected_vec = [2578, 3544142, 261214, 6895259, 18, 362];

        let handle = thread::spawn(move || {
            for (actual, expected) in INPUT_EXTRA.iter().zip(expected_vec.iter()) {
                assert_eq!(part2(actual), *expected);
            }
        });

        handle.join().unwrap();
    }

    #[test]
    fn verify() {
        let solver = super::Solver::new();
        let input = include_str!("../../input/day15.txt");

        let expected: Solution = (276, 31_916).into();
        let actual = solver.solve(input);

        assert_eq!(actual, expected)
    }
}
