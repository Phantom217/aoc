use std::collections::HashMap;
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
        19
    }

    fn solve(&self, input: &str) -> Solution {
        let (mut rule_set, messages) = parse_input(input);

        let part1 = part1(&rule_set, &messages);
        let part2 = part2(&mut rule_set, &messages);

        (part1, part2).into()
    }
}

fn part1(rule_set: &RuleSet, messages: &[&str]) -> usize {
    messages
        .iter()
        .filter(|message| rule_set.validate(0, message))
        .count()
}

fn part2(rule_set: &mut RuleSet, messages: &[&str]) -> usize {
    rule_set.rules.insert(8, Rule::from("42 | 42 8"));
    rule_set.rules.insert(11, Rule::from("42 31 | 42 11 31"));

    messages
        .iter()
        .filter(|message| rule_set.validate(0, message))
        .count()
}

fn parse_input(input: &str) -> (RuleSet, Vec<&str>) {
    let mut iter = input.split("\n\n");
    let rule_set = iter
        .next()
        .unwrap()
        .lines()
        .map(|line| {
            let mut iter = line.split(": ");
            let idx = iter.next().unwrap().parse().unwrap();
            let rule = Rule::from(iter.next().unwrap());
            (idx, rule)
        })
        .collect::<RuleSet>();
    let messages = iter.next().unwrap().lines().collect::<Vec<&str>>();
    (rule_set, messages)
}

#[derive(Debug, PartialEq, Eq)]
struct RuleSet<'r> {
    rules: HashMap<usize, Rule<'r>>,
}

impl<'a> RuleSet<'a> {
    fn validate(&self, idx: usize, message: &str) -> bool {
        let to_validate = self.rules.get(&idx).unwrap();
        let remaining = to_validate.validate(message, self);
        remaining.into_iter().any(str::is_empty)
    }
}

impl<'r> std::iter::FromIterator<(usize, Rule<'r>)> for RuleSet<'r> {
    fn from_iter<T: IntoIterator<Item = (usize, Rule<'r>)>>(iter: T) -> Self {
        let mut rules = HashMap::new();
        for (idx, rule) in iter {
            rules.insert(idx, rule);
        }
        Self { rules }
    }
}

#[derive(Debug, PartialEq, Eq)]
enum Rule<'r> {
    Terminator(&'r str),
    Reference(HashSet<Vec<usize>>),
}

impl<'a> Rule<'a> {
    fn validate<'r, 's: 'r>(
        &'r self,
        message: &'s str,
        rules: &'r RuleSet,
    ) -> Box<dyn Iterator<Item = &str> + 'r> {
        if message.is_empty() {
            return Box::new(None.into_iter());
        }

        match self {
            Self::Terminator(s) => {
                if message.starts_with(s) {
                    let (_, unmatched) = message.split_at(s.len());
                    Box::new(Some(unmatched).into_iter())
                } else {
                    Box::new(None.into_iter())
                }
            }
            Self::Reference(branches) => Box::new(branches.iter().flat_map(move |sequence| {
                let mut remaining_message = vec![message];
                for rule in sequence.iter().map(|idx| rules.rules.get(idx).unwrap()) {
                    remaining_message = remaining_message
                        .iter()
                        .flat_map(|msg| rule.validate(msg, rules))
                        .collect()
                }
                remaining_message.into_iter()
            })),
        }
    }
}

impl<'r, 's: 'r> From<&'s str> for Rule<'r> {
    fn from(rule: &'s str) -> Self {
        if rule.starts_with('"') {
            let terminal = rule.strip_prefix('"').unwrap().strip_suffix('"').unwrap();
            Self::Terminator(terminal)
        } else {
            let rule = rule
                .split('|')
                .map(|s| {
                    s.split_whitespace()
                        .filter_map(|s| s.parse().ok())
                        .collect()
                })
                .collect();
            Self::Reference(rule)
        }
    }
}

#[cfg(test)]
mod test {
    #![allow(unused_imports)]
    use super::*;
    use crate::solution::Solution;
    use crate::Solver;

    #[allow(unused)]
    const INPUT: &str = r#"0: 4 1 5
1: 2 3 | 3 2
2: 4 4 | 5 5
3: 4 5 | 5 4
4: "a"
5: "b"

ababbb
bababa
abbbab
aaabbb
aaaabbb

"#;

    #[test]
    fn test_parse_rules() {
        const INPUT: &str = r#"0: 1 2
1: "a"
2: 1 3 | 3 1
3: "b"

"#;

        let (actual, _) = parse_input(INPUT);

        let expected = {
            let mut expected = HashMap::with_capacity(6);
            expected.insert(0, Rule::Reference(vec![vec![1, 2]].into_iter().collect()));
            expected.insert(1, Rule::Terminator("a"));
            expected.insert(
                2,
                Rule::Reference(vec![vec![1, 3], vec![3, 1]].into_iter().collect()),
            );
            expected.insert(3, Rule::Terminator("b"));
            RuleSet { rules: expected }
        };
        assert_eq!(actual, expected)
    }

    #[test]
    fn example_part1() {
        let (rule_set, messages) = parse_input(INPUT);
        let expected = 2;
        let actual = part1(&rule_set, &messages);
        assert_eq!(actual, expected)
    }

    #[test]
    fn example_part2() {
        let (mut rule_set, messages) = parse_input(EXAMPLE_INPUT_P2);
        let expected = 12;
        let actual = part2(&mut rule_set, &messages);
        assert_eq!(actual, expected)
    }

    #[test]
    fn verify() {
        let solver = super::Solver::new();
        let input = include_str!("../../input/day19.txt");

        let expected: Solution = (102, 318).into();
        let actual = solver.solve(input);

        assert_eq!(actual, expected)
    }

    const EXAMPLE_INPUT_P2: &str = r#"42: 9 14 | 10 1
9: 14 27 | 1 26
10: 23 14 | 28 1
1: "a"
11: 42 31
5: 1 14 | 15 1
19: 14 1 | 14 14
12: 24 14 | 19 1
16: 15 1 | 14 14
31: 14 17 | 1 13
6: 14 14 | 1 14
2: 1 24 | 14 4
0: 8 11
13: 14 3 | 1 12
15: 1 | 14
17: 14 2 | 1 7
23: 25 1 | 22 14
28: 16 1
4: 1 1
20: 14 14 | 1 15
3: 5 14 | 16 1
27: 1 6 | 14 18
14: "b"
21: 14 1 | 1 14
25: 1 1 | 1 14
22: 14 14
8: 42
26: 14 22 | 1 20
18: 15 15
7: 14 5 | 1 21
24: 14 1

abbbbbabbbaaaababbaabbbbabababbbabbbbbbabaaaa
bbabbbbaabaabba
babbbbaabbbbbabbbbbbaabaaabaaa
aaabbbbbbaaaabaababaabababbabaaabbababababaaa
bbbbbbbaaaabbbbaaabbabaaa
bbbababbbbaaaaaaaabbababaaababaabab
ababaaaaaabaaab
ababaaaaabbbaba
baabbaaaabbaaaababbaababb
abbbbabbbbaaaababbbbbbaaaababb
aaaaabbaabaaaaababaa
aaaabbaaaabbaaa
aaaabbaabbaaaaaaabbbabbbaaabbaabaaa
babaaabbbaaabaababbaabababaaab
aabbbbbaabbbaaaaaabbbbbababaaaaabbaaabba"#;
}
