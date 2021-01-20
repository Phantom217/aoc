use std::collections::VecDeque;
use std::iter::FromIterator;

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
        22
    }

    fn solve(&self, input: &str) -> Solution {
        let (deck1, deck2) = parse_input(input);

        let part1 = part1(&deck1, &deck2);
        let part2 = part2(&deck1, &deck2);

        (part1, part2).into()
    }
}

fn parse_input(input: &str) -> (Deck, Deck) {
    let mut iter = input.split("\n\n");
    let deck1 = iter
        .next()
        .unwrap()
        .lines()
        .filter_map(|s| s.parse().ok())
        .collect();
    let deck2 = iter
        .next()
        .unwrap()
        .lines()
        .filter_map(|s| s.parse().ok())
        .collect();
    (deck1, deck2)
}

fn part1(deck1: &Deck, deck2: &Deck) -> usize {
    let (mut deck1, mut deck2) = (deck1.clone(), deck2.clone());
    while !deck1.is_empty() && !deck2.is_empty() {
        deck1.combat_round(&mut deck2);
    }
    deck1.score() + deck2.score()
}

fn part2(deck1: &Deck, deck2: &Deck) -> usize {
    let (mut deck1, mut deck2) = (deck1.clone(), deck2.clone());
    deck1.recursive_combat(&mut deck2);
    deck1.score() + deck2.score()
}

#[derive(Debug, Default, Clone, PartialEq, Eq)]
struct Deck(VecDeque<usize>);

impl Deck {
    fn combat_round(&mut self, other: &mut Deck) {
        use std::cmp::Ordering;

        let self_card = self.pop_front().unwrap();
        let other_card = other.pop_front().unwrap();

        let (winner, winning_card, losing_card);

        match self_card.cmp(&other_card) {
            Ordering::Less => {
                winner = other;
                winning_card = other_card;
                losing_card = self_card;
            }
            Ordering::Greater => {
                winner = self;
                winning_card = self_card;
                losing_card = other_card;
            }
            Ordering::Equal => panic!("No rule for equivalent cards."),
        }

        winner.push_back(winning_card);
        winner.push_back(losing_card);
    }

    fn recursive_combat(&mut self, other: &mut Deck) -> Winner {
        use std::cmp::Ordering;
        use std::collections::HashSet;

        let mut previous_rounds: HashSet<(Vec<usize>, Vec<usize>)> = HashSet::new();

        while !self.is_empty() && !other.is_empty() {
            let deck_snapshot = (
                self.0.iter().cloned().collect(),
                other.0.iter().cloned().collect(),
            );
            // Instantly end the game if deck state has occured before, granting Player 1 the win.
            if !previous_rounds.insert(deck_snapshot) {
                return Winner::Player1;
            }

            let self_card = self.pop_front().unwrap();
            let other_card = other.pop_front().unwrap();

            // If both players have at least as many cards remaining in their deck as the value of
            // the card they just drew, the winner of the round is determined by playing a new game
            // of Recursive Combat.
            let winner = if self.len() >= self_card && other.len() >= other_card {
                self.subdeck(self_card)
                    .recursive_combat(&mut other.subdeck(other_card))
            } else {
                match self_card.cmp(&other_card) {
                    Ordering::Less => Winner::Player2,
                    Ordering::Greater => Winner::Player1,
                    Ordering::Equal => panic!("No rule for equivalent cards."),
                }
            };

            let (winning_deck, winning_card, losing_card);

            match winner {
                Winner::Player1 => {
                    winning_deck = &mut *self;
                    winning_card = self_card;
                    losing_card = other_card;
                }
                Winner::Player2 => {
                    winning_deck = &mut *other;
                    winning_card = other_card;
                    losing_card = self_card;
                }
            }

            winning_deck.push_back(winning_card);
            winning_deck.push_back(losing_card);
        }

        if self.is_empty() {
            Winner::Player2
        } else {
            Winner::Player1
        }
    }

    /// Tally the winner's score.
    fn score(&self) -> usize {
        let len = self.0.len();

        let mut score = 0;
        for (idx, card) in self.0.iter().enumerate() {
            score += (len - idx) * card;
        }

        score
    }

    /// Create a subdeck of `num` cards from original deck.
    fn subdeck(&self, num: usize) -> Self {
        Self(self.0.iter().take(num).cloned().collect())
    }

    fn is_empty(&self) -> bool {
        self.0.is_empty()
    }

    fn len(&self) -> usize {
        self.0.len()
    }

    fn pop_front(&mut self) -> Option<usize> {
        self.0.pop_front()
    }

    fn push_back(&mut self, value: usize) {
        self.0.push_back(value)
    }
}

impl From<&str> for Deck {
    fn from(deck: &str) -> Self {
        deck.lines()
            .skip(1)
            .filter_map(|s| s.parse::<usize>().ok())
            .collect()
    }
}

impl FromIterator<usize> for Deck {
    fn from_iter<T: IntoIterator<Item = usize>>(iter: T) -> Self {
        let mut deck = Deck::default();
        for i in iter {
            deck.0.push_back(i);
        }
        deck
    }
}

enum Winner {
    Player1,
    Player2,
}

#[cfg(test)]
mod test {
    #![allow(unused_imports)]
    use super::*;
    use crate::solution::Solution;
    use crate::Solver;

    const INPUT: &str = "\
Player 1:
9
2
6
3
1

Player 2:
5
8
4
7
10";
    const INFINITE: &str = "\
Player 1:
43
19

Player 2:
2
29
14";

    #[test]
    fn test_parse_input() {
        let expected = (
            Deck(VecDeque::from(vec![9, 2, 6, 3, 1])),
            Deck(VecDeque::from(vec![5, 8, 4, 7, 10])),
        );
        let actual = parse_input(INPUT);
        assert_eq!(actual, expected)
    }

    #[test]
    fn example_part1() {
        let (deck1, deck2) = parse_input(INPUT);
        let expected = 306;
        let actual = part1(&deck1, &deck2);
        assert_eq!(actual, expected)
    }

    #[test]
    fn example_part2() {
        let (deck1, deck2) = parse_input(INPUT);
        let expected = 291;
        let actual = part2(&deck1, &deck2);
        assert_eq!(actual, expected)
    }

    #[test]
    fn test_infinite() {
        let (deck1, deck2) = parse_input(INPUT);
        let _ = part2(&deck1, &deck2);
    }

    #[test]
    #[ignore = "test takes a long time"]
    fn verify() {
        let solver = super::Solver::new();
        let input = include_str!("../../input/day22.txt");

        let expected: Solution = (34_005, 32_731).into();
        let actual = solver.solve(input);

        assert_eq!(actual, expected)
    }
}
