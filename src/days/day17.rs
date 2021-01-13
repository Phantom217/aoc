use std::collections::HashSet;

use crate::solution::Solution;

type Grid<const D: usize> = HashSet<[isize; D]>;

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
        17
    }

    fn solve(&self, input: &str) -> Solution {
        let part1 = get_active::<3>(input);
        let part2 = get_active::<4>(input);

        (part1, part2).into()
    }
}

fn get_active<const D: usize>(input: &str) -> usize {
    let grid = parse_input::<D>(input);
    let val = simulate(grid);
    val.len()
}

fn simulate<const D: usize>(mut grid: Grid<D>) -> Grid<D> {
    let offsets = {
        let mut offsets = offsets(0, [0; D]);
        // Get rid of origin coordinate in offsets vec
        let _ = offsets.pop();
        offsets
    };

    for _ in 0..6 {
        let mut active = HashSet::new();
        let mut inactive = HashSet::new();

        for coord in &grid {
            let mut active_neighbors = 0;
            for offset in &offsets {
                let mut neighbor = *coord;
                for i in 0..D {
                    neighbor[i] += offset[i];
                }
                if grid.contains(&neighbor) {
                    active_neighbors += 1;
                } else {
                    inactive.insert(neighbor);
                }
            }

            if active_neighbors == 2 || active_neighbors == 3 {
                active.insert(*coord);
            }
        }

        for coord in &inactive {
            let mut active_neighbors = 0;

            for offset in &offsets {
                let mut neighbor = *coord;
                for i in 0..D {
                    neighbor[i] += offset[i];
                }

                if grid.contains(&neighbor) {
                    active_neighbors += 1;
                }
            }

            if active_neighbors == 3 {
                active.insert(*coord);
            }
        }

        grid = active;
    }

    grid
}

fn offsets<const D: usize>(idx: usize, mut offset_vec: [isize; D]) -> Vec<[isize; D]> {
    if idx == D {
        return vec![offset_vec];
    }

    let mut acc = Vec::with_capacity(usize::pow(3, (D - idx) as u32));
    for &i in &[-1, 1, 0] {
        offset_vec[idx] = i;
        let extend_offsets = offsets(idx + 1, offset_vec);
        acc.extend(extend_offsets);
    }

    acc
}

fn parse_input<const D: usize>(input: &str) -> Grid<D> {
    let mut grid = HashSet::new();
    for (row, line) in input.lines().enumerate() {
        for (col, c) in line.chars().enumerate() {
            let mut coord = [0; D];
            match c {
                '#' => {
                    coord[0] = row as isize;
                    coord[1] = col as isize;
                    grid.insert(coord);
                }
                '.' => (),
                _ => panic!("Unrecognized char: {}", c),
            }
        }
    }

    grid
}

#[cfg(test)]
mod test {
    #![allow(unused_imports)]
    use super::*;
    use crate::solution::Solution;
    use crate::Solver;

    const INPUT: &str = "\
.#.
..#
###
";

    #[test]
    fn test_parse_input() {
        let expected = {
            let coords = [[0, 1, 0], [1, 2, 0], [2, 0, 0], [2, 1, 0], [2, 2, 0]];
            let mut ex = HashSet::with_capacity(5);
            for &coord in &coords {
                ex.insert(coord);
            }
            ex
        };
        let actual = parse_input(INPUT);
        assert_eq!(actual, expected)
    }

    #[test]
    fn test_offsets() {
        let mut expected = vec![
            [-1, -1, -1],
            [-1, -1, 1],
            [-1, -1, 0],
            [-1, 1, -1],
            [-1, 1, 1],
            [-1, 1, 0],
            [-1, 0, -1],
            [-1, 0, 1],
            [-1, 0, 0],
            [0, -1, -1],
            [0, -1, 1],
            [0, -1, 0],
            [0, 1, -1],
            [0, 1, 1],
            [0, 1, 0],
            [0, 0, -1],
            [0, 0, 1],
            [1, -1, -1],
            [1, -1, 1],
            [1, -1, 0],
            [1, 1, -1],
            [1, 1, 1],
            [1, 1, 0],
            [1, 0, -1],
            [1, 0, 1],
            [1, 0, 0],
        ];
        expected.sort();
        let mut actual = offsets::<3>(0, [0; 3]);
        let _ = actual.pop();
        actual.sort();
        assert_eq!(actual, expected)
    }

    #[test]
    fn example_part1() {
        let expected = 112;
        let actual = get_active::<3>(INPUT);
        assert_eq!(actual, expected)
    }

    #[test]
    fn example_part2() {
        let expected = 848;
        let actual = get_active::<4>(INPUT);
        assert_eq!(actual, expected)
    }

    #[test]
    fn verify() {
        let solver = super::Solver::new();
        let input = include_str!("../../input/day17.txt");

        let expected: Solution = (368, 2_696).into();
        let actual = solver.solve(input);

        assert_eq!(actual, expected)
    }
}
