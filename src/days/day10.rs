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
        10
    }

    fn solve(&self, input: &str) -> Solution {
        let adapters = {
            let mut input = input
                .lines()
                .map(|n| n.parse().expect("Invalid number"))
                .collect::<Vec<u32>>();
            get_all_nodes(&mut input);
            input
        };

        let part1 = part1(&adapters);
        let part2 = part2(&adapters);

        (part1 as usize, part2).into()
    }
}

fn get_all_nodes(adapters: &mut Vec<u32>) {
    adapters.push(0);
    adapters.push(adapters.iter().max().unwrap() + 3);
    adapters.sort_unstable();
}

fn check_ones_threes(diff: u32, ones: &mut u32, threes: &mut u32) {
    match diff {
        1 => *ones += 1,
        3 => *threes += 1,
        _ => (),
    }
}

fn part1(adapters: &[u32]) -> u32 {
    // let mut ones = 0;
    // let mut threes = 0;
    // for [x, y] in adapters.array_windows() {
    //     check_ones_threes(y - x, &mut ones, &mut threes);
    // }
    //
    // ones * threes

    // alternate way of solving part 1 using iterators.
    let (diff_1, diff_3) =
        adapters
            .array_windows()
            .map(|[x, y]| y - x)
            .fold((0, 0), |(diff_1, diff_3), diff| match diff {
                1 => (diff_1 + 1, diff_3),
                3 => (diff_1, diff_3 + 1),
                _ => unreachable!(),
            });

    diff_1 * diff_3
}

fn part2(adapters: &[u32]) -> usize {
    adapters
        .array_windows()
        .map(|[x, y]| y - x)
        .fold((1, 0, 0), |(diff_1, diff_2, diff_3), diff| match diff {
            1 => (diff_1 + diff_2 + diff_3, diff_1, diff_2),
            2 => (diff_1 + diff_2, 0, diff_1),
            3 => (diff_1, 0, 0),
            _ => unreachable!(),
        })
        .0
}

#[cfg(test)]
mod test {
    #![allow(unused_imports)]
    use super::*;
    use crate::solution::Solution;
    use crate::Solver;

    const SMALL_INPUT: &str = "16\n10\n15\n5\n1\n11\n7\n19\n6\n12\n4";
    const INPUT: &str = "28\n33\n18\n42\n31\n14\n46\n20\n48\n47\n24\n23\n49\n45\n19\n38\n39\n11\n1\n32\n25\n35\n8\n17\n7\n9\n4\n2\n34\n10\n3";

    #[test]
    fn example_part1_small() {
        let adapters: Vec<u32> = {
            let mut input: Vec<u32> = SMALL_INPUT
                .lines()
                .map(|n| n.parse().expect("Invalid number"))
                .collect::<Vec<u32>>();
            get_all_nodes(&mut input);
            input
        };

        let actual = part1(&adapters);
        let expected = 35;
        assert_eq!(actual, expected);
    }

    #[test]
    fn example_part1() {
        let adapters: Vec<u32> = {
            let mut input: Vec<u32> = INPUT
                .lines()
                .map(|n| n.parse().expect("Invalid number"))
                .collect::<Vec<u32>>();
            get_all_nodes(&mut input);
            input
        };

        let actual = part1(&adapters);
        let expected = 220;
        assert_eq!(actual, expected);
    }

    #[test]
    fn example_part2_small() {
        let adapters: Vec<u32> = {
            let mut input: Vec<u32> = SMALL_INPUT
                .lines()
                .map(|n| n.parse().expect("Invalid number"))
                .collect::<Vec<u32>>();
            get_all_nodes(&mut input);
            input
        };

        let expected = 8;
        let actual = part2(&adapters);

        assert_eq!(actual, expected)
    }

    #[test]
    fn example_part2() {
        let adapters: Vec<u32> = {
            let mut input: Vec<u32> = INPUT
                .lines()
                .map(|n| n.parse().expect("Invalid number"))
                .collect::<Vec<u32>>();
            get_all_nodes(&mut input);
            input
        };

        let expected = 19208;
        let actual = part2(&adapters);

        assert_eq!(actual, expected)
    }

    #[test]
    fn correct_solution() {
        let solver = super::Solver::new();
        let input = include_str!("../../input/day10.txt");

        let expected: Solution = (2376_usize, 129586085429248_usize).into();
        let actual = solver.solve(input);

        assert_eq!(actual, expected)
    }
}
