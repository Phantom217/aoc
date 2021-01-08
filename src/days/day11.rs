use std::fmt;

use crate::solution::Solution;

pub(crate) struct Solver(());

const ADJACENTS: [(isize, isize); 8] = [
    (-1, -1),
    (-1, 0),
    (-1, 1),
    (0, -1),
    (0, 1),
    (1, -1),
    (1, 0),
    (1, 1),
];

impl Solver {
    pub fn new() -> Self {
        let solver = Self(());
        assert_solver_day!(solver);
        solver
    }
}

impl crate::Solver for Solver {
    fn day(&self) -> u8 {
        11
    }

    fn solve(&self, input: &str) -> Solution {
        let map: Vec<Vec<State>> = input
            .lines()
            .map(|l| l.chars().map(State::from).collect())
            .collect();

        let part1 = run_simulation(&map, should_swap_p1);
        let part2 = run_simulation(&map, should_swap_p2);

        (part1, part2).into()
    }
}

fn should_swap_p1(map: &[Vec<State>], i: usize, j: usize) -> bool {
    let mut neighbors = ADJACENTS
        .iter()
        .map(|&(dy, dx)| (i as isize + dy, j as isize + dx))
        .filter_map(|(y, x)| map.get(y as usize).and_then(|v| v.get(x as usize)));
    match map[i][j] {
        State::Empty => neighbors.all(|&c| c != State::Occupied),
        State::Occupied => neighbors.filter(|&&c| c == State::Occupied).count() >= 4,
        _ => unreachable!(),
    }
}

fn find_neighbor(
    map: &[Vec<State>],
    (dy, dx): (isize, isize),
    (i, j): (usize, usize),
) -> Option<State> {
    let (mut i, mut j) = (i as isize, j as isize);
    loop {
        i += dy;
        j += dx;
        let tile = map
            .get(i as usize)
            .and_then(|row| row.get(j as usize))
            .copied();
        if tile != Some(State::Floor) {
            return tile;
        }
    }
}

fn should_swap_p2(map: &[Vec<State>], i: usize, j: usize) -> bool {
    let mut neighbors = ADJACENTS
        .iter()
        .filter_map(|&dir| find_neighbor(&map, dir, (i, j)));

    match map[i][j] {
        State::Empty => neighbors.all(|s| s != State::Occupied),
        State::Occupied => neighbors.filter(|&s| s == State::Occupied).count() >= 5,
        State::Floor => unreachable!(),
    }
}

fn run_simulation<F: Fn(&[Vec<State>], usize, usize) -> bool>(
    map: &[Vec<State>],
    should_swap: F,
) -> usize {
    let mut map = map.to_vec();
    let mut to_swap = Vec::new();

    loop {
        to_swap.clear();

        for (i, row) in map.iter().enumerate() {
            for j in 0..row.len() {
                if map[i][j] != State::Floor && should_swap(&map, i, j) {
                    to_swap.push((i, j));
                }
            }
        }

        for &(i, j) in &to_swap {
            map[i][j] = match map[i][j] {
                State::Empty => State::Occupied,
                State::Occupied => State::Empty,
                State::Floor => unreachable!(),
            };
        }
        if to_swap.is_empty() {
            break;
        }
    }

    map.iter()
        .flatten()
        .filter(|&&s| s == State::Occupied)
        .count()
}

#[derive(Debug, Eq, PartialEq, Copy, Clone)]
enum State {
    Floor,
    Empty,
    Occupied,
}

impl std::convert::From<char> for State {
    fn from(c: char) -> Self {
        match c {
            '.' => Self::Floor,
            'L' => Self::Empty,
            '#' => Self::Occupied,
            _ => unreachable!(),
        }
    }
}

impl std::fmt::Display for State {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Floor => write!(f, "."),
            Self::Empty => write!(f, "L"),
            Self::Occupied => write!(f, "#"),
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
L.LL.LL.LL
LLLLLLL.LL
L.L.L..L..
LLLL.LL.LL
L.LL.LL.LL
L.LLLLL.LL
..L.L.....
LLLLLLLLLL
L.LLLLLL.L
L.LLLLL.LL";

    #[test]
    fn example_part1() {
        let map: Vec<Vec<State>> = INPUT
            .lines()
            .map(|l| l.chars().map(State::from).collect())
            .collect();

        let expected = 37;

        let actual = run_simulation(&map, should_swap_p1);

        assert_eq!(actual, expected)
    }

    #[test]
    fn example_part2() {
        let map: Vec<Vec<State>> = INPUT
            .lines()
            .map(|l| l.chars().map(State::from).collect())
            .collect();

        let expected = 26;

        let actual = run_simulation(&map, should_swap_p2);

        assert_eq!(actual, expected)
    }

    #[test]
    fn correct_solution() {
        let solver = super::Solver::new();
        let input = include_str!("../../input/day11.txt");

        let expected: Solution = (2178, 1978).into();
        let actual = solver.solve(input);

        assert_eq!(actual, expected)
    }
}
