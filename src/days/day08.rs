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
        8
    }

    fn solve(&self, input: &str) -> Solution {
        let instructions: Vec<Op> = input.lines().map(Op::from).collect();

        let instr_len = instructions.len();
        let mut vm = Vm::new(instructions);

        let part1 = {
            vm.execute(-1);
            vm.acc
        };

        let part2 = {
            (0..instr_len)
                .map(|i| vm.execute(i as i32))
                .find(|&found| found)
                .unwrap();
            vm.acc
        };

        (part1, part2).into()
    }
}

struct Vm {
    boot_code: Vec<Op>,
    ip: i32,
    acc: i32,
}

impl Vm {
    fn new(boot_code: Vec<Op>) -> Self {
        Self {
            boot_code,
            ip: 0,
            acc: 0,
        }
    }

    fn execute(&mut self, changed_ip: i32) -> bool {
        self.ip = 0;
        self.acc = 0;

        let mut visited = vec![false; self.boot_code.len()];

        while !visited[self.ip as usize] {
            visited[self.ip as usize] = true;
            match (&self.boot_code[self.ip as usize], self.ip == changed_ip) {
                (Op::Acc(n), _) => {
                    self.acc += n;
                    self.ip += 1;
                }
                (Op::Jmp(n), false) | (Op::Nop(n), true) => self.ip += n,
                (Op::Jmp(_), true) | (Op::Nop(_), false) => self.ip += 1,
            }

            if self.ip == self.boot_code.len() as i32 {
                return true;
            }
        }

        false
    }
}

#[derive(Debug, Copy, Clone, Eq, PartialEq)]
enum Op {
    /// `acc` increases or decreases a single global balue called the *accumulator* by the value
    /// given in the argument.
    Acc(i32),
    /// `jmp` jumps to a new instruction relative to itself. The next instruction to execute is
    /// found using the argument as an *offset* from the `jmp` instruction.
    Jmp(i32),
    /// `nop` stands for *No OPeration* - it does nothing. The instruction immediately below it is
    /// executed next.
    Nop(i32),
}

impl From<&str> for Op {
    fn from(instruction: &str) -> Self {
        let instruction = instruction.split(' ').collect::<Vec<_>>();
        let op = instruction[0];
        let arg = instruction[1].parse::<i32>().unwrap();

        match op {
            "acc" => Op::Acc(arg),
            "jmp" => Op::Jmp(arg),
            "nop" => Op::Nop(arg),
            _ => unreachable!("Unrecognized operation: {}", op),
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
nop +0
acc +1
jmp +4
acc +3
jmp -3
acc -99
acc +1
jmp -4
acc +6";

    #[test]
    fn test_parse_op() {
        let expected = Op::Jmp(3);
        let actual = Op::from("jmp +3");
        assert_eq!(actual, expected);

        let expected = Op::Jmp(-3);
        let actual = Op::from("jmp -3");
        assert_eq!(actual, expected);
    }

    #[test]
    #[should_panic(expected = "Unrecognized operation: zzz")]
    fn test_parse_invalid_op() {
        Op::from("zzz +0");
    }

    #[test]
    fn example_part1() {
        let input = INPUT.lines().map(Op::from).collect::<Vec<Op>>();

        let expected = 5;
        let actual = {
            let mut vm = Vm::new(input);
            vm.execute(-1);
            vm.acc
        };
        assert_eq!(actual, expected);
    }

    #[test]
    fn verify() {
        let solver = super::Solver::new();
        let input = include_str!("../../input/day08.txt");

        let expected: Solution = (1949, 2092).into();
        let actual = solver.solve(input);

        assert_eq!(actual, expected)
    }
}
