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
        23
    }

    fn solve(&self, input: &str) -> Solution {
        let mut cups = Cups::from(input);
        for _ in 0..100 {
            cups.move_cups();
        }
        let part1 = cups.format_part1();

        let mut cups = Cups::from(input).extend(1_000_000);
        for _ in 0..10_000_000 {
            cups.move_cups();
        }
        let part2 = cups.format_part2();

        (part1, part2).into()
    }
}

#[derive(Debug, PartialEq, Eq)]
struct Cups {
    /// Each index corresponds to the 'cup number'. The value stored corresponds to the index of
    /// the next cup in the sequence.
    order: Vec<usize>,
    /// The index into `order` of the current cup.
    current: usize,
}

impl Cups {
    /// Make one move with the current cup orientation.
    fn move_cups(&mut self) {
        // Take 3 cups clockwise of current
        let (c1, c2, c3) = {
            let mut cs = self.next_after(self.current);
            (cs.next().unwrap(), cs.next().unwrap(), cs.next().unwrap())
        };

        // Find index of cup to put the picked up cups after
        let mut next_cup = self.minus_one_cup(self.current);
        while c1 == next_cup || c2 == next_cup || c3 == next_cup {
            next_cup = self.minus_one_cup(next_cup);
        }

        self.order[self.current] = self.order[c3];
        self.order[c3] = self.order[next_cup];
        self.order[next_cup] = c1;
        self.current = self.order[self.current];
    }

    /// Return an iterator over the next cups in line from the given cup.
    fn next_after(&self, cup: usize) -> impl Iterator<Item = usize> + '_ {
        std::iter::successors(Some(cup), move |cup| Some(self.order[*cup])).skip(1)
    }

    /// Minus one from the cup number to get the previous one.
    fn minus_one_cup(&self, cup: usize) -> usize {
        let num_cups = self.order.len() - 1;
        (cup + (num_cups - 1) - 1) % num_cups + 1
    }

    /// Extend the number of cups being used to `capacity` and adjust pointers accordingly.
    fn extend(mut self, capacity: usize) -> Self {
        let (last_cup_idx, _) = self
            .order
            .iter()
            .enumerate()
            .find(|(_, &x)| x == self.current)
            .unwrap();

        self.order[last_cup_idx] = self.order.len();
        self.order.extend(self.order.len() + 1..=capacity);
        self.order.push(self.current);
        self
    }

    /// Get answer in the format required for part 1: cup labels in order starting from the cup
    /// after cup 1.
    fn format_part1(&self) -> usize {
        let mut val = 0;
        for (idx, cup) in self.next_after(1).take(8).enumerate() {
            val += cup * 10_usize.pow((self.order.len() - 3 - idx) as u32);
        }
        val
    }

    /// Get answer in the format required for part 2: product of the two cups immediately clockwise
    /// of cup 1.
    fn format_part2(&self) -> usize {
        self.next_after(1).take(2).product()
    }
}

impl From<&str> for Cups {
    fn from(s: &str) -> Self {
        let raw: Vec<_> = s
            .chars()
            .filter(|c| c.is_digit(10))
            .map(|c| c.to_digit(10).unwrap() as usize)
            .collect();
        let mut iter = raw.iter().peekable();

        let mut order: Vec<usize> = vec![0; raw.len() + 1];

        while let Some(cup) = iter.next() {
            let next = if let Some(next) = iter.peek() {
                **next
            } else {
                raw[0]
            };
            order[*cup] = next;
        }

        Self {
            order,
            current: raw[0],
        }
    }
}

#[cfg(test)]
mod test {
    #![allow(unused_imports)]
    use super::*;
    use crate::solution::Solution;
    use crate::Solver;

    const INPUT: &str = "389125467";

    #[test]
    fn test_parse_input() {
        let expected = Cups {
            order: vec![0, 2, 5, 8, 6, 4, 7, 3, 9, 1],
            current: 3,
        };
        let actual = Cups::from(INPUT);
        assert_eq!(actual, expected)
    }

    #[test]
    fn test_format_part1() {
        let expected = 25_467_389;
        let actual = Cups::from(INPUT).format_part1();
        assert_eq!(actual, expected)
    }

    #[test]
    fn test_format_part2() {
        let expected = 2 * 5;
        let actual = Cups::from(INPUT).format_part2();
        assert_eq!(actual, expected)
    }

    #[test]
    fn test_one_move() {
        let mut cups = Cups::from(INPUT);
        let expected = Cups {
            order: vec![0, 5, 8, 2, 6, 4, 7, 3, 9, 1],
            current: 2,
        };
        let expected = expected.format_part1();
        cups.move_cups();
        let actual = cups.format_part1();

        assert_eq!(actual, expected)
    }

    #[test]
    fn example_part1() {
        // after 10 moves
        let expected = 92_658_374;
        let mut cups = Cups::from(INPUT);
        for _ in 0..10 {
            cups.move_cups();
        }
        let actual = cups.format_part1();
        assert_eq!(actual, expected, "Incorrect solution after 10 moves.");

        // after 100 moves
        let expected = 67_384_529;
        let mut cups = Cups::from(INPUT);
        for _ in 0..100 {
            cups.move_cups();
        }
        let actual = cups.format_part1();
        assert_eq!(actual, expected, "Incorrect solution after 100 moves.");
    }

    #[test]
    fn example_part2() {
        let expected = 149_245_887_792;
        let mut cups = Cups::from(INPUT).extend(1_000_000);
        for _ in 0..10_000_000 {
            cups.move_cups();
        }
        let mut iter = cups.next_after(1).take(2);
        assert_eq!(iter.next().unwrap(), 934001);
        assert_eq!(iter.next().unwrap(), 159792);

        let actual = cups.format_part2();
        assert_eq!(actual, expected)
    }

    #[test]
    fn test_extend() {
        let expected = Cups {
            order: vec![
                0, 2, 5, 8, 6, 4, 7, 10, 9, 1, 11, 12, 13, 14, 15, 16, 17, 18, 19, 20, 3,
            ],
            current: 3,
        };
        let actual = Cups::from(INPUT).extend(20);
        assert_eq!(actual, expected)
    }

    #[test]
    fn verify() {
        let solver = super::Solver::new();
        let input = include_str!("../../input/day23.txt");

        let expected: Solution = (54_896_723_usize, 146_304_752_384_usize).into();
        let actual = solver.solve(input);

        assert_eq!(actual, expected)
    }
}
