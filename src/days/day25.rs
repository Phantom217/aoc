use crate::solution::Solution;

pub(crate) struct Solver(());

const DIVISOR: usize = 20201227;
const SUBJECT_NUMBER: usize = 7;

impl Solver {
    pub fn new() -> Self {
        let solver = Self(());
        assert_solver_day!(solver);
        solver
    }
}

impl crate::Solver for Solver {
    fn day(&self) -> u8 {
        25
    }

    fn solve(&self, input: &str) -> Solution {
        let encryption_key = part1(input);
        (encryption_key, String::new()).into()
    }
}

fn part1(input: &str) -> usize {
    let (pk1, pk2) = parse_input(input);
    let (mut val, mut loop_size) = (1, 0);
    while val != pk1 && val != pk2 {
        val = val * 7 % DIVISOR;
        loop_size += 1;
    }
    let k1 = if val == pk1 { pk2 } else { pk1 };
    (0..loop_size).fold(1, |x, _| x * k1 % DIVISOR)
}

fn parse_input(input: &str) -> (usize, usize) {
    let keys: Vec<usize> = input
        .trim()
        .lines()
        .filter_map(|l| l.trim().parse().ok())
        .collect();
    assert!(keys.len() == 2);
    (keys[0], keys[1])
}

#[cfg(test)]
mod test {
    #![allow(unused_imports)]
    use super::*;
    use crate::solution::Solution;
    use crate::Solver;

    // subject number: 7
    //
    // card public key: 5764801
    // card loop size: 8
    //
    // door public key: 17807724
    // door loop size: 11
    //
    // encryption key: 14897079

    #[test]
    fn test_parse_input() {
        let expected = (5764801, 17807724);
        let actual = parse_input("5764801\n17807724");
        assert_eq!(actual, expected)
    }

    #[test]
    fn test_get_encryption_key() {
        let input = "5764801\n17807724";
        let actual = part1(input);
        let expected = 14897079;
        assert_eq!(actual, expected)
    }

    #[test]
    fn verify() {
        let solver = super::Solver::new();
        let input = include_str!("../../input/day25.txt");

        let expected: Solution = (2679568, String::new()).into();
        let actual = solver.solve(input);

        assert_eq!(actual, expected)
    }
}
