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
        14
    }

    fn solve(&self, input: &str) -> Solution {
        let ops = input.lines().map(Op::from).collect::<Vec<Op>>();

        let part1 = part1(&ops);
        let part2 = part2(&ops);

        (part1, part2).into()
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
enum Op {
    Mask(Vec<u8>),
    Mem(usize, u64),
}

impl std::convert::From<&str> for Op {
    fn from(line: &str) -> Self {
        match &line[..3] {
            "mem" => {
                let mem = line[4..].split("] = ").collect::<Vec<_>>();
                let reg = mem[0].parse().expect("Invalid input");
                let val = mem[1].parse().expect("Invalid input");
                Self::Mem(reg, val)
            }
            "mas" => {
                assert!(line[7..].len() == 36);
                let mask = line[7..].as_bytes().to_vec();
                Self::Mask(mask)
            }
            _ => unreachable!("Unable to match line: {}", line),
        }
    }
}

fn part1(ops: &[Op]) -> u64 {
    let mut memory: HashMap<usize, u64> = Default::default();
    let mut mask_0 = 0;
    let mut mask_1 = 0;

    for op in ops.iter() {
        match op {
            Op::Mask(m) => {
                let (m0, m1, _) = parse_mask(m);
                mask_0 = m0;
                mask_1 = m1;
            }
            Op::Mem(reg, val) => {
                let val = (val | mask_1) & mask_0;
                *memory.entry(*reg).or_default() = val;
            }
        }
    }

    memory.values().sum()
}

fn part2(ops: &[Op]) -> u64 {
    let mut memory: HashMap<usize, u64> = Default::default();
    let mut mask_1 = 0;
    let mut floating_bits: Vec<u64> = Default::default();

    for op in ops.iter() {
        match op {
            Op::Mask(m) => {
                let (_, m1, floating_mask) = parse_mask(m);
                mask_1 = m1;

                floating_bits.clear();
                for i in 0..36 {
                    if floating_mask & (1 << i) != 0 {
                        floating_bits.push(i);
                    }
                }
            }
            Op::Mem(reg, val) => {
                let perms = 2_usize.pow(floating_bits.len() as u32);
                for perm in 0..perms {
                    let mut reg = *reg | mask_1 as usize;

                    for idx in 0..floating_bits.len() {
                        if perm & (1 << idx) != 0 {
                            reg |= 1 << floating_bits[idx];
                        } else {
                            reg &= !(1 << floating_bits[idx]);
                        }
                    }
                    *memory.entry(reg).or_default() = *val;
                }
            }
        }
    }

    memory.values().sum()
}

fn parse_mask(mask: &[u8]) -> (u64, u64, u64) {
    assert!(mask.len() == 36);

    let mask_0 = mask
        .iter()
        .rev()
        .enumerate()
        .filter(|(_, &b)| b == b'0')
        .fold(u64::MAX, |acc, (i, _)| acc & !(1 << i));

    let mask_1 = mask
        .iter()
        .rev()
        .enumerate()
        .filter(|(_, &b)| b == b'1')
        .fold(0, |acc, (i, _)| acc | (1 << i));

    let mask_x = mask
        .iter()
        .rev()
        .enumerate()
        .filter(|(_, &b)| b == b'X')
        .fold(0, |acc, (i, _)| acc | (1 << i));

    (mask_0, mask_1, mask_x)
}

#[cfg(test)]
mod test {
    #![allow(unused_imports)]
    use super::*;
    use crate::solution::Solution;
    use crate::Solver;

    const INPUT: &str = "\
mask = XXXXXXXXXXXXXXXXXXXXXXXXXXXXX1XXXX0X
mem[8] = 11
mem[7] = 101
mem[8] = 0";

    #[test]
    fn test_parse_input() {
        let actual = INPUT.lines().map(Op::from).collect::<Vec<_>>();
        let expected = vec![
            Op::Mask(b"XXXXXXXXXXXXXXXXXXXXXXXXXXXXX1XXXX0X".to_vec()),
            Op::Mem(8, 11),
            Op::Mem(7, 101),
            Op::Mem(8, 0),
        ];
        assert_eq!(expected, actual)
    }

    #[test]
    fn example_part1() {
        let ops: Vec<Op> = INPUT.lines().map(Op::from).collect();
        let expected = 165;
        let actual = part1(&ops);
        assert_eq!(actual, expected)
    }

    #[test]
    fn example_part2() {
        const INPUT: &str = "\
mask = 000000000000000000000000000000X1001X
mem[42] = 100
mask = 00000000000000000000000000000000X0XX
mem[26] = 1";
        let ops: Vec<Op> = INPUT.lines().map(Op::from).collect();
        let expected = 208;
        let actual = part2(&ops);
        assert_eq!(actual, expected)
    }

    #[test]
    fn correct_solution() {
        let solver = super::Solver::new();
        let input = include_str!("../../input/day14.txt");

        let expected: Solution = (4_297_467_072_083_u64, 5_030_603_328_768).into();
        let actual = solver.solve(input);

        assert_eq!(actual, expected)
    }
}
