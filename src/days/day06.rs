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
        6
    }

    fn solve(&self, input: &str) -> Solution {
        let part1: u32 = input
            .split("\n\n")
            .map(|group| {
                group
                    .bytes()
                    .filter(|c| *c != b'\n')
                    // the possible answers are a..=z, which can fit in a u32, where each bit
                    // represents if the answer is present or not.
                    .fold(0_u32, |acc, choice| acc | 1 << (choice - b'a'))
                    .count_ones()
            })
            .sum();

        let part2 = input
            .split("\n\n")
            .map(|group| {
                group
                    .lines()
                    .map(|person| {
                        person
                            .bytes()
                            .fold(0_u32, |acc, choice| acc | 1 << (choice - b'a'))
                    })
                    .fold(std::u32::MAX, |everyone, one| everyone & one)
                    .count_ones()
            })
            .sum();

        (part1, part2).into()
    }
}

#[cfg(test)]
mod test {
    #![allow(unused_imports)]
    use super::*;
    use crate::solution::Solution;
    use crate::Solver;

    const INPUT: &str = "\
abc

a
b
c

ab
ac

a
a
a
a

b";

    #[test]
    fn example_part1() {
        let solver = super::Solver::new();

        let expected = 11.to_string();
        let actual = solver.solve(INPUT);

        assert_eq!(actual.part1(), expected)
    }

    #[test]
    fn example_part2() {
        let solver = super::Solver::new();

        let expected = 6.to_string();
        let actual = solver.solve(INPUT);

        assert_eq!(actual.part2(), expected)
    }

    #[test]
    fn correct_solution() {
        let solver = super::Solver::new();
        let input = include_str!("../../input/day06.txt");

        let expected = (6885, 3550).into();
        let actual = solver.solve(input);

        assert_eq!(actual, expected)
    }
}
