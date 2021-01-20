#![allow(clippy::many_single_char_names)]
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
        13
    }

    fn solve(&self, input: &str) -> Solution {
        let (earliest, busses) = parse_input(input);

        let part1 = part1(earliest, &busses);
        let part2 = part2(&busses);

        (part1, part2).into()
    }
}

fn part1(earliest: u32, busses: &[(isize, isize)]) -> isize {
    (0..)
        .filter_map(|i| {
            busses
                .iter()
                .find(|&&(_, b)| (i + earliest as isize) % b == 0)
                .map(|(_, b)| i * b)
        })
        .next()
        .expect("No solution")
}

fn part2(busses: &[(isize, isize)]) -> isize {
    let modulii = busses.iter().map(|&(_, b)| b).collect::<Vec<_>>();
    let residues = busses.iter().map(|&(i, b)| b - i).collect::<Vec<_>>();
    crt(&residues, &modulii).unwrap()
}

/// Extended Euclidean Algorithm
///
/// Returns: (gcd, first Bezout's coefficient, second Bezout's coefficient)
fn egcd(mut x: isize, mut y: isize) -> (isize, isize, isize) {
    let (mut a0, mut a1, mut b0, mut b1) = (1_isize, 0, 0, 1);

    while y != 0 {
        let (q, r) = (x / y, x % y);
        let (c, d) = (a0 - q * a1, b0 - q * b1);

        x = y;
        y = r;
        a0 = a1;
        a1 = c;
        b0 = b1;
        b1 = d;
    }

    (x, a0, b0)
}

fn modulus_inverse(m_i: isize, modulus: isize) -> Option<isize> {
    let (gcd, a, _) = egcd(m_i, modulus);

    // gcd must be 1 otherwise numbers aren't relatively prime.
    if gcd == 1 {
        // in rust -154 % 5 gives -4 instead of 1
        // so to normalize such case we should add modulus and do some math
        return Some((a % modulus + modulus) % modulus);
    }
    None
}

/// Chinese Remainder Theorem
fn crt(residues: &[isize], modulii: &[isize]) -> Option<isize> {
    let prod: isize = modulii.iter().product();
    let mut sum = 0;

    for (&modulus, &residue) in modulii.iter().zip(residues) {
        let m_i = prod / modulus;
        let mod_inverse = modulus_inverse(m_i, modulus)?;
        sum += residue * m_i * mod_inverse;
    }

    Some(sum % prod)
}

fn parse_input(input: &str) -> (u32, Vec<(isize, isize)>) {
    let mut iter = input.lines();
    let earliest = iter.next().unwrap().parse::<u32>().unwrap();
    let busses: Vec<(isize, isize)> = iter
        .next()
        .unwrap()
        .split(',')
        .enumerate()
        .filter_map(|(i, l)| l.parse().ok().map(|b| (i as isize, b)))
        .collect();

    (earliest, busses)
}

#[cfg(test)]
mod test {
    #![allow(unused_imports)]
    use super::*;
    use crate::solution::Solution;
    use crate::Solver;

    const INPUT: &str = "\
939
7,13,x,x,59,x,31,19";

    #[test]
    fn test_parse_input() {
        let expected = (939, vec![(0, 7), (1, 13), (4, 59), (6, 31), (7, 19)]);
        let actual = parse_input(INPUT);
        assert_eq!(actual, expected);
    }

    #[test]
    fn example_part1() {
        let (earliest, busses) = parse_input(INPUT);
        let expected = 295;
        let actual = part1(earliest, &busses);
        assert_eq!(actual, expected)
    }

    #[test]
    fn example_part2() {
        let (_, busses) = parse_input(INPUT);
        let expected = 1_068_781;
        let actual = part2(&busses);
        assert_eq!(actual, expected)
    }

    #[test]
    fn verify() {
        let solver = super::Solver::new();
        let input = include_str!("../../input/day13.txt");

        let expected: Solution = (261_isize, 807_435_693_182_510).into();
        let actual = solver.solve(input);

        assert_eq!(actual, expected)
    }
}
