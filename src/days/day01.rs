use crate::solution::Solution;

const TARGET: usize = 2020;

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
        1
    }

    // Use a `[bool; 2020]` array to mark which values are present.
    // We can trivially sort them by traversing the array in order and inserting present values.
    // This allows us to break early from the inner loop as soon as `a + 2 * b > 2020`.
    fn solve(&self, input: &str) -> Solution {
        let mut solution = (0, 0);

        let mut nums = [false; 2020];

        let mut num = 0;
        let mut count = 0;
        for c in input.bytes() {
            if c == b'\n' {
                nums[num] = true;
                num = 0;
                count += 1;
            } else {
                num = num * 10 + (c - b'0') as usize;
            }
        }

        let mut num_list = [0; 2020];

        let mut i = 0;
        for (n, b) in nums.iter().enumerate() {
            if *b {
                num_list[i] = n as usize;
                i += 1;
            }
        }

        debug_assert!(i == count);

        'outer: for (i, &a) in num_list.iter().enumerate() {
            let target = TARGET - a;
            if nums[target as usize] {
                solution.0 = a * target;
                if solution.1 != 0 {
                    break 'outer;
                }
            }

            for &b in num_list[i + 1..].iter() {
                if 2 * b > target {
                    break;
                }
                let target2 = target - b;
                if nums[target2 as usize] {
                    solution.1 = a * b * target2;
                    if solution.0 != 0 {
                        break 'outer;
                    }
                }
            }
        }

        solution.into()
    }
}

#[cfg(test)]
mod test {
    #[allow(unused_imports)]
    use super::*;
    use crate::solution::Solution;
    use crate::Solver;

    #[test]
    fn example_part1() {
        let input = "1721\n979\n366\n299\n675\n1456\n";
        let solver = super::Solver::new();

        let expected = 514_579.to_string();
        let actual = solver.solve(&input);

        assert_eq!(actual.part1(), expected)
    }

    #[test]
    fn example_part2() {
        let input = "1721\n979\n366\n299\n675\n1456\n";
        let solver = super::Solver::new();

        let expected = 241_861_950.to_string();
        let actual = solver.solve(&input);

        assert_eq!(actual.part2(), expected)
    }

    fn helper() -> Solution {
        let solver = super::Solver::new();

        let input_path = crate::input_path(1, None);
        let input_string = match std::fs::read_to_string(input_path) {
            Ok(inp) => crate::add_newline(inp),
            Err(error) => panic!("Error while reading input file: {}", error),
        };
        let actual = solver.solve(&input_string);

        actual
    }

    #[test]
    fn verify() {
        let actual = helper();
        let expected: Solution = (1_018_336, 288_756_720).into();

        assert_eq!(actual, expected)
    }
}
