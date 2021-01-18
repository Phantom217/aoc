use crate::solution::Solution;

pub(crate) struct Solver(());

const SLOPE_PART_1: [(usize, usize); 1] = [(3, 1)];
const SLOPE_PART_2: [(usize, usize); 5] = [(1, 1), (3, 1), (5, 1), (7, 1), (1, 2)];

impl Solver {
    pub fn new() -> Self {
        let solver = Self(());
        assert_solver_day!(solver);
        solver
    }
}

impl crate::Solver for Solver {
    fn day(&self) -> u8 {
        3
    }

    fn solve(&self, input: &str) -> Solution {
        let part1 = count_trees(input, &SLOPE_PART_1);
        let part2 = count_trees(input, &SLOPE_PART_2);

        (part1, part2).into()
    }
}

fn count_trees(input: &str, slopes: &[(usize, usize)]) -> usize {
    let line_width = input.find('\n').unwrap();

    slopes.iter().fold(1, |acc, &(x_step, y_step)| {
        acc * input
            .lines()
            .step_by(y_step)
            .zip((0..line_width).cycle().step_by(x_step))
            .skip(1)
            .filter(|&(line, x_pos)| &line[x_pos..=x_pos] == "#")
            .count()
    })
}

#[cfg(test)]
mod test {
    #![allow(unused_imports)]
    use super::*;
    use crate::solution::Solution;
    use crate::Solver;

    const INPUT: &str = "\
..##.......
#...#...#..
.#....#..#.
..#.#...#.#
.#...##..#.
..#.##.....
.#.#.#....#
.#........#
#.##...#...
#...##....#
.#..#...#.#";

    #[test]
    fn example_part1() {
        let expected = 7;
        let actual = count_trees(INPUT, &SLOPE_PART_1);
        assert_eq!(actual, expected)
    }

    #[test]
    fn example_part2() {
        let expected = 336;
        let actual = count_trees(INPUT, &SLOPE_PART_2);
        assert_eq!(actual, expected)
    }

    #[test]
    fn verify() {
        let solver = super::Solver::new();
        let input = include_str!("../../input/day03.txt");

        let expected: Solution = (240, 0).into();
        let actual = solver.solve(input);

        assert_eq!(actual, expected)
    }
}
