use std::collections::{hash_set, HashSet};

use crate::solution::Solution;

pub(crate) struct Solver(());

/// The coordinated used to determine the neighbors of a given 'Hex'
const NEIGHBORS: [(isize, isize); 6] = [(1, 0), (0, 1), (-1, 1), (-1, 0), (0, -1), (1, -1)];

impl Solver {
    pub fn new() -> Self {
        let solver = Self(());
        assert_solver_day!(solver);
        solver
    }
}

impl crate::Solver for Solver {
    fn day(&self) -> u8 {
        24
    }

    fn solve(&self, input: &str) -> Solution {
        let hex_grid = HexGrid::from(input);

        let part1 = hex_grid.len();
        let part2 = hex_grid.days(100);

        (part1, part2).into()
    }
}

#[derive(Debug, PartialEq, Eq)]
enum Direction {
    East,
    Southeast,
    Southwest,
    West,
    Northwest,
    Northeast,
}

#[derive(Debug, PartialEq, Eq, Default)]
struct HexGrid(HashSet<Hex>);

impl HexGrid {
    fn insert(&mut self, hex: Hex) {
        if !self.0.remove(&hex) {
            self.0.insert(hex);
        }
    }

    fn len(&self) -> usize {
        self.0.len()
    }

    fn iter(&self) -> hash_set::Iter<Hex> {
        self.0.iter()
    }

    /// Simulates flipping tiles each day according to the following rules:
    ///
    /// - Any *black* tile with *zero* or *more than 2* black tiles immediately adjacent to it is
    /// flipped to *white*.
    ///
    /// - Any *white* tile with *exactly 2* black tiles immediately adjacent to it is flipped to
    /// *black*.
    ///
    /// The rules are applied *simultaneously* to every tile; put another way, it is first
    /// determined which tiles need to be flipped, then they are all flipped at the same time.
    ///
    /// Returns the number of black tiles.
    fn days(mut self, days: usize) -> usize {
        use std::collections::HashMap;

        let mut neighbors = HashMap::new();
        for _ in 0..days {
            neighbors.clear();
            for Hex { index: (q, r) } in self.iter() {
                for (dq, dr) in &NEIGHBORS {
                    *neighbors
                        .entry(Hex {
                            index: (q + dq, r + dr),
                        })
                        .or_insert(0) += 1;
                }
            }

            self = neighbors
                .iter()
                .filter(|(t, &n)| n == 2 || (n == 1 && self.0.contains(t)))
                .map(|(&t, _)| t)
                .collect();
        }

        self.len()
    }
}

impl From<&str> for HexGrid {
    fn from(s: &str) -> Self {
        s.lines()
            .map(|line| {
                let mut iter = line.chars();
                let mut directions: Vec<_> = Vec::new();
                while let Some(c) = iter.next() {
                    match c {
                        'n' => match iter.next().unwrap() {
                            'w' => directions.push(Direction::Northwest),
                            'e' => directions.push(Direction::Northeast),
                            c => unreachable!("Got invalid direction: n{}", c),
                        },
                        's' => match iter.next().unwrap() {
                            'w' => directions.push(Direction::Southwest),
                            'e' => directions.push(Direction::Southeast),
                            c => unreachable!("Got invalid direction: s{}", c),
                        },
                        'e' => directions.push(Direction::East),
                        'w' => directions.push(Direction::West),
                        c => unreachable!("Got invalid direction: {}", c),
                    };
                }
                Hex::from(directions)
            })
            .collect()
    }
}

impl std::iter::FromIterator<Hex> for HexGrid {
    fn from_iter<T: IntoIterator<Item = Hex>>(iter: T) -> Self {
        let mut hex_grid = Self::default();

        for i in iter {
            hex_grid.insert(i)
        }

        hex_grid
    }
}

impl IntoIterator for HexGrid {
    type Item = Hex;
    type IntoIter = <HashSet<<Self as IntoIterator>::Item> as IntoIterator>::IntoIter;

    fn into_iter(self) -> Self::IntoIter {
        self.0.into_iter()
    }
}

#[derive(Debug, PartialEq, Eq, Default, Hash, Clone, Copy)]
struct Hex {
    index: (isize, isize),
}

impl From<Vec<Direction>> for Hex {
    fn from(directions: Vec<Direction>) -> Self {
        let (mut q, mut r) = (0, 0);

        for direction in directions {
            let (dq, dr) = match direction {
                Direction::East => NEIGHBORS[0],
                Direction::Southeast => NEIGHBORS[1],
                Direction::Southwest => NEIGHBORS[2],
                Direction::West => NEIGHBORS[3],
                Direction::Northwest => NEIGHBORS[4],
                Direction::Northeast => NEIGHBORS[5],
            };
            q += dq;
            r += dr;
        }

        Self { index: (q, r) }
    }
}

#[cfg(test)]
mod test {
    #![allow(unused_imports)]
    use super::*;
    use crate::solution::Solution;
    use crate::Solver;

    const INPUT: &str = "\
sesenwnenenewseeswwswswwnenewsewsw
neeenesenwnwwswnenewnwwsewnenwseswesw
seswneswswsenwwnwse
nwnwneseeswswnenewneswwnewseswneseene
swweswneswnenwsewnwneneseenw
eesenwseswswnenwswnwnwsewwnwsene
sewnenenenesenwsewnenwwwse
wenwwweseeeweswwwnwwe
wsweesenenewnwwnwsenewsenwwsesesenwne
neeswseenwwswnwswswnw
nenwswwsewswnenenewsenwsenwnesesenew
enewnwewneswsewnwswenweswnenwsenwsw
sweneswneswneneenwnewenewwneswswnese
swwesenesewenwneswnwwneseswwne
enesenwswwswneneswsenwnewswseenwsese
wnwnesenesenenwwnenwsewesewsesesew
nenewswnwewswnenesenwnesewesw
eneswnwswnwsenenwnwnwwseeswneewsenese
neswnwewnwnwseenwseesewsenwsweewe
wseweeenwnesenwwwswnew";

    #[test]
    fn test_parse_input() {
        const INPUT: &str = "nwwswee\nesew";
        let expected = {
            let mut ex = HashSet::default();
            ex.insert(Hex { index: (0, 1) });
            ex.insert(Hex { index: (0, 0) });
            HexGrid(ex)
        };
        let actual = HexGrid::from(INPUT);
        assert_eq!(actual, expected)
    }

    #[test]
    fn test_double_insert() {
        let expected = HexGrid::default();
        let actual = {
            let mut act = HexGrid::default();
            act.insert(Hex { index: (1, 1) });
            act.insert(Hex { index: (1, 1) });
            act
        };
        assert_eq!(actual, expected)
    }

    #[test]
    fn example_part1() {
        let expected = 10;
        let actual = HexGrid::from(INPUT).len();
        assert_eq!(actual, expected)
    }

    #[test]
    fn example_part2() {
        let expected = 15;
        let actual = HexGrid::from(INPUT).days(1);
        assert_eq!(actual, expected);

        let expected = 12;
        let actual = HexGrid::from(INPUT).days(2);
        assert_eq!(actual, expected);

        let expected = 2208;
        let actual = HexGrid::from(INPUT).days(100);
        assert_eq!(actual, expected)
    }

    #[test]
    fn verify() {
        let solver = super::Solver::new();
        let input = include_str!("../../input/day24.txt");

        let expected: Solution = (538, 4259).into();
        let actual = solver.solve(input);

        assert_eq!(actual, expected)
    }
}
