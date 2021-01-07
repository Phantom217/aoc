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
        7
    }

    fn solve(&self, input: &str) -> Solution {
        let bags: Bags = {
            let rule_vec: Vec<Rule> = input.lines().map(Rule::from).collect();
            Bags::from(rule_vec)
        };

        let part1 = bags.reverse_bag_count("shiny gold");

        let part2 = bags.count_required_bags("shiny gold");

        (part1, part2).into()
    }
}

struct Bags<'r> {
    bags: HashMap<String, Rule<'r>>,
}

impl<'r> Bags<'r> {
    pub fn new() -> Self {
        Self {
            bags: HashMap::new(),
        }
    }

    pub fn add_bag(&mut self, bag: Rule<'r>) -> Option<Rule> {
        self.bags.insert(bag.get_key(), bag)
    }

    /// Find the number of bags that can contain at least one `search_key` bag.
    ///
    /// Search begins with `search_key` and works its way backwards through the rules.
    pub fn reverse_bag_count(&self, search_key: &str) -> usize {
        let mut count = 0;

        for (_, parent_bag) in &self.bags {
            match self.recursive_bag_count(&parent_bag, search_key) {
                0 => {}
                _ => count += 1,
            }
        }

        count
    }

    pub fn recursive_bag_count(&self, bag: &Rule<'r>, search_key: &str) -> usize {
        let mut count = 0;

        for (child_key, child_count) in bag.contents.iter() {
            if search_key == child_key {
                count += child_count;
            } else {
                if let Some(c) = self.bags.get(child_key) {
                    count += self.recursive_bag_count(&c, search_key);
                }
            }
        }

        count
    }

    pub fn count_required_bags(&self, parent_key: &str) -> usize {
        let mut count = 0;

        let parent_bag = self.bags.get(parent_key).expect("valid key");

        for (child_key, child_count) in parent_bag.contents.iter() {
            if let Some(child) = self.bags.get(child_key) {
                count += child_count * (1 + self.count_required_bags(&child.get_key()));
            } else {
                eprintln!("Could not find {}", child_key);
            }
        }

        count
    }
}

impl<'r> std::convert::From<Vec<Rule<'r>>> for Bags<'r> {
    fn from(rules: Vec<Rule<'r>>) -> Self {
        let mut bags = Bags::new();

        for rule in rules {
            let bag = Rule::from(rule);
            if let Some(b) = bags.add_bag(bag) {
                panic!("There's a duplicate parent rule: {:?}", b);
            };
        }

        bags
    }
}

#[derive(Debug, Eq, PartialEq)]
struct Rule<'r> {
    container: &'r str,
    contents: HashMap<String, usize>,
}

impl<'r> Rule<'r> {
    pub fn get_key(&self) -> String {
        self.container.into()
    }
}

impl<'r> std::convert::From<&'r str> for Rule<'r> {
    fn from(rule: &'r str) -> Self {
        let rule: Vec<_> = rule.split("bags contain").map(|s| s.trim()).collect();
        let container = rule[0];
        let contents_str = rule[1];

        let contents;
        if contents_str == "no other bags." {
            contents = HashMap::new()
        } else {
            contents = contents_str
                .split(",")
                .map(|s| {
                    let mut words = s.split_whitespace();
                    let num: usize = words.next().unwrap().parse().unwrap();
                    let adjective = words.next().unwrap();
                    let color = words.next().unwrap();
                    let bag = format!("{} {}", adjective, color);

                    (bag, num)
                })
                .collect();
        }

        Self {
            container,
            contents,
        }
    }
}

#[cfg(test)]
mod test {
    #![allow(unused_imports)]
    use super::*;
    use crate::solution::Solution;
    use crate::Solver;

    const INPUT: &str = "\
light red bags contain 1 bright white bag, 2 muted yellow bags.
dark orange bags contain 3 bright white bags, 4 muted yellow bags.
bright white bags contain 1 shiny gold bag.
muted yellow bags contain 2 shiny gold bags, 9 faded blue bags.
shiny gold bags contain 1 dark olive bag, 2 vibrant plum bags.
dark olive bags contain 3 faded blue bags, 4 dotted black bags.
vibrant plum bags contain 5 faded blue bags, 6 dotted black bags.
faded blue bags contain no other bags.
dotted black bags contain no other bags.";

    #[test]
    fn test_convert_to_rule() {
        let input = INPUT.lines().next().unwrap();
        assert_eq!(
            input.trim(),
            "light red bags contain 1 bright white bag, 2 muted yellow bags."
        );
        let actual = Rule::from(input);

        let contents: HashMap<String, usize> = {
            let mut c = HashMap::with_capacity(2);
            c.insert(String::from("bright white"), 1);
            c.insert(String::from("muted yellow"), 2);
            c
        };

        let expected = Rule {
            container: "light red",
            contents,
        };

        assert_eq!(actual, expected)
    }

    #[test]
    fn example_part1() {
        let bags = {
            let rule_vec: Vec<Rule> = INPUT.lines().map(Rule::from).collect();
            Bags::from(rule_vec)
        };

        let expected = 4;
        let actual = bags.reverse_bag_count("shiny gold");

        assert_eq!(actual, expected)
    }

    #[test]
    fn example_part2() {
        let bags = {
            let rule_vec: Vec<Rule> = INPUT.lines().map(Rule::from).collect();
            Bags::from(rule_vec)
        };

        let expected = 32;
        let actual = bags.count_required_bags("shiny gold");

        assert_eq!(actual, expected)
    }

    #[test]
    fn correct_solutions() {
        let solver = super::Solver::new();
        let input = include_str!("../../input/day07.txt");

        let expected: Solution = (278, 45157).into();
        let actual = solver.solve(input);

        assert_eq!(actual, expected)
    }
}
