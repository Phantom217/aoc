use std::collections::HashSet;

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
        9
    }

    fn solve(&self, input: &str) -> Solution {
        let input: Vec<usize> = input
            .lines()
            .map(|i| i.parse::<usize>().expect(&format!("Invalid number: {}", i)))
            .collect();

        let part1 = find_anomaly(&input, 25);
        let part2 = find_contiguous_sum(input, part1);

        (part1, part2).into()
    }
}

fn find_anomaly(numbers: &[usize], window: usize) -> usize {
    let mut checking = HashSet::with_capacity(window);
    let mut invalid = 0;

    for (i, val) in numbers.iter().enumerate() {
        if i < window {
            checking.insert(val);
        } else {
            // Check previous `window` for sum.
            let mut found = false;
            for c in &numbers[(i - window)..i] {
                let needed = if let Some(n) = val.checked_sub(*c) {
                    n
                } else {
                    c - val
                };

                if checking.contains(&needed) {
                    found = true;
                    break;
                }
            }

            if !found {
                invalid = *val;
                break;
            }

            // Otherwise rotate checking set.
            checking.remove(&numbers[i - window]);
            checking.insert(val);
        }
    }

    invalid
}

fn find_contiguous_sum(numbers: Vec<usize>, num: usize) -> usize {
    let mut start = 0;
    let mut end = 1;
    let mut sum = numbers[0] + numbers[1];

    loop {
        use std::cmp::Ordering;
        match sum.cmp(&num) {
            Ordering::Equal => break,
            Ordering::Less => {
                end += 1;
                sum += numbers[end];
            }
            Ordering::Greater => {
                sum -= numbers[start];
                start += 1;
            }
        }
    }

    let mut min = usize::MAX;
    let mut max = 0;

    for i in start..=end {
        let n = numbers[i];
        min = min.min(n);
        max = max.max(n);
    }

    min + max
}

#[cfg(test)]
mod test {
    #![allow(unused_imports)]
    use super::*;
    use crate::solution::Solution;
    use crate::Solver;

    const INPUT: &str = "\
35
20
15
25
47
40
62
55
65
95
102
117
150
182
127
219
299
277
309
576";

    #[test]
    fn example_part1() {
        let numbers = INPUT
            .lines()
            .map(|n| n.parse::<usize>().expect("Invalid number"))
            .collect::<Vec<usize>>();

        let actual = find_anomaly(&numbers, 5);
        assert_eq!(actual, 127)
    }

    #[test]
    fn example_part2() {
        let numbers = INPUT
            .lines()
            .map(|n| n.parse::<usize>().expect("Invalid number"))
            .collect::<Vec<usize>>();

        let actual = find_contiguous_sum(numbers, 127);
        assert_eq!(actual, 62)
    }

    #[test]
    fn correct_solution() {
        let input = include_str!("../../input/day09.txt");
        let numbers = input
            .lines()
            .map(|n| n.parse::<usize>().expect("Invalid number"))
            .collect::<Vec<usize>>();

        let expected = 556543474;
        let actual = find_anomaly(&numbers, 25);

        assert_eq!(actual, expected)
    }
}
