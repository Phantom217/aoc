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
        2
    }

    fn solve(&self, input: &str) -> Solution {
        let part1 = part1(input);
        let part2 = part2(input);

        (part1, part2).into()
    }
}

fn part1(input: &str) -> usize {
    let pr = parse_password_rules(input).unwrap();

    pr.iter().filter(|entry| entry.is_valid_part1()).count()
}

fn part2(input: &str) -> usize {
    let pr = parse_password_rules(input).unwrap();

    pr.iter().filter(|entry| entry.is_valid_part2()).count()
}

// TODO: use `&str` for `letter` and `password`
// using `&str` over `char` because the letter is always valid UTF-8, we don't need Unicode support.
#[derive(Debug, PartialOrd, PartialEq)]
struct PasswordRules {
    min: usize,
    max: usize,
    letter: char,
    password: String,
}

impl PasswordRules {
    /// Line is valid if character appears between `min` and `max` times in the sequence.
    fn is_valid_part1(&self) -> bool {
        (self.min..=self.max).contains(&self.password.chars().filter(|c| *c == self.letter).count())
    }

    /// Line is valid if character appears exactly *once* at the given indexes `min` and `max`
    /// (indices start at 1).
    fn is_valid_part2(&self) -> bool {
        let x = self.password.as_bytes()[self.min - 1] as char;
        let y = self.password.as_bytes()[self.max - 1] as char;
        (x == self.letter) ^ (y == self.letter)
    }
}

impl std::convert::From<&str> for PasswordRules {
    fn from(rule: &str) -> Self {
        let pr = rule.split(['-', ':', ' '].as_ref()).collect::<Vec<&str>>();
        let min = pr[0].parse::<usize>().unwrap();
        let max = pr[1].parse::<usize>().unwrap();
        let letter = pr[2].parse::<char>().unwrap();

        Self {
            min,
            max,
            letter,
            password: String::from(pr[4]),
        }
    }
}

/// Parse input into a Vec of PasswordRules
fn parse_password_rules(input: &str) -> Result<Vec<PasswordRules>, String> {
    // TODO: make more efficient
    let mut pr_vec = Vec::new();

    for line in input.lines() {
        pr_vec.push(PasswordRules::from(line));
    }

    Ok(pr_vec)
}

#[cfg(test)]
mod test {
    #![allow(unused_imports)]
    use super::*;
    use crate::solution::Solution;
    use crate::Solver;

    const INPUT: &str = "1-3 a: abcde\n1-3 b: cdefg\n2-9 c: ccccccccc";

    #[test]
    fn example_part1() {
        let solver = super::Solver::new();

        let expected = 2.to_string();
        let actual = solver.solve(&INPUT);

        assert_eq!(actual.part1(), expected)
    }

    #[test]
    fn example_part2() {
        let solver = super::Solver::new();

        let expected = 1.to_string();
        let actual = solver.solve(&INPUT);

        assert_eq!(actual.part2(), expected)
    }

    #[test]
    fn parse_password_rules() {
        let actual = {
            let mut a = Vec::with_capacity(3);
            for line in INPUT.lines() {
                a.push(PasswordRules::from(line))
            }
            a
        };

        let expected = vec![
            PasswordRules {
                min: 1,
                max: 3,
                letter: 'a',
                password: "abcde".to_owned(),
            },
            PasswordRules {
                min: 1,
                max: 3,
                letter: 'b',
                password: "cdefg".to_owned(),
            },
            PasswordRules {
                min: 2,
                max: 9,
                letter: 'c',
                password: "ccccccccc".to_owned(),
            },
        ];

        assert_eq!(actual, expected)
    }

    #[test]
    fn test_is_valid_part2() {
        let expected = true;

        let pr = PasswordRules {
            min: 1,
            max: 3,
            letter: 'a',
            password: "abcde".to_owned(),
        };
        let actual = pr.is_valid_part2();

        assert_eq!(actual, expected)
    }

    #[test]
    fn correct_solution() {
        let expected: Solution = (569, 346).into();

        let input = include_str!("../../input/day02.txt");

        let part1 = part1(input);
        let part2 = part2(input);

        let actual: Solution = (part1, part2).into();

        assert_eq!(expected, actual)
    }
}
