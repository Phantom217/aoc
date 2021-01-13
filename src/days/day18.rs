use std::fmt;

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
        18
    }

    fn solve(&self, input: &str) -> Solution {
        let part1 = part1(input);
        let part2 = part2(input);

        (part1, part2).into()
    }
}

fn part1(input: &str) -> u64 {
    let tokens = input
        .lines()
        .map(|line| lexer(line, |_, _| true))
        .collect::<Vec<Vec<Token>>>();
    let expressions = tokens
        .iter()
        .map(|expr| shunting_yard(&expr))
        .collect::<Vec<Vec<Token>>>();

    expressions
        .iter()
        .map(|tokens| evaluate(tokens))
        .sum::<u64>()
}

fn part2(input: &str) -> u64 {
    let tokens = input
        .lines()
        .map(|line| lexer(line, |a, b| matches!((a.token, b.token), ('+', '*'))))
        .collect::<Vec<Vec<Token>>>();
    let expressions = tokens
        .iter()
        .map(|expr| shunting_yard(&expr))
        .collect::<Vec<Vec<Token>>>();

    expressions
        .iter()
        .map(|tokens| evaluate(tokens))
        .sum::<u64>()
}

fn lexer(input: &str, precedince_fn: fn(Operator, Operator) -> bool) -> Vec<Token> {
    input
        .chars()
        .filter(|c| !c.is_whitespace())
        .map(|t| match t {
            '0'..='9' => Token::Digit(u64::from(t.to_digit(10).unwrap())),
            '+' => Operator::new_token(t, |x, y| x + y, precedince_fn),
            '*' => Operator::new_token(t, |x, y| x * y, precedince_fn),
            '(' => Token::LeftParen,
            ')' => Token::RightParen,
            _ => panic!("invalid token: {}", t),
        })
        .collect()
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
struct Operator {
    token: char,
    operation: fn(u64, u64) -> u64,
    precedince_fn: fn(Operator, Operator) -> bool,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Token {
    Digit(u64),
    Operator(Operator),
    LeftParen,
    RightParen,
}

impl Operator {
    fn new_token(
        token: char,
        operation: fn(u64, u64) -> u64,
        precedince_fn: fn(Operator, Operator) -> bool,
    ) -> Token {
        Token::Operator(Self {
            token,
            operation,
            precedince_fn,
        })
    }

    fn eval(&self, x: u64, y: u64) -> u64 {
        (self.operation)(x, y)
    }
}

trait Stack {
    fn top(&self) -> Option<Token>;
}

impl Stack for Vec<Token> {
    fn top(&self) -> Option<Token> {
        if self.is_empty() {
            return None;
        }
        self.get(self.len() - 1).copied()
    }
}

fn take_until(operators: &mut Vec<Token>, output: &mut Vec<Token>, stop: Token) -> bool {
    while let Some(token) = operators.pop() {
        if token == stop {
            return true;
        }
        output.push(token);
    }
    false
}

fn shunting_yard(tokens: &[Token]) -> Vec<Token> {
    let mut output: Vec<Token> = Default::default();
    let mut operators: Vec<Token> = Default::default();

    for &token in tokens {
        match token {
            Token::Digit(_) => output.push(token),
            Token::LeftParen => operators.push(token),
            Token::Operator(op) => {
                while let Some(top) = operators.top() {
                    match top {
                        Token::LeftParen => break,
                        Token::Operator(top_op) => {
                            if (top_op.precedince_fn)(top_op, op) {
                                output.push(operators.pop().unwrap());
                            } else {
                                break;
                            }
                        }
                        _ => todo!("Mayhave to implement this operator: {:?}", top),
                    }
                }
                operators.push(token);
            }
            Token::RightParen => {
                if !take_until(&mut operators, &mut output, Token::LeftParen) {
                    panic!("Mismatched ')'");
                }
            }
        }
    }

    if take_until(&mut operators, &mut output, Token::LeftParen) {
        panic!("Mismatched '('");
    }

    assert!(operators.is_empty());
    output
}

fn evaluate(postfix_tokens: &[Token]) -> u64 {
    let mut stack = Vec::new();

    for &token in postfix_tokens {
        match token {
            Token::Digit(n) => stack.push(n),
            Token::Operator(op) => {
                if let Some(y) = stack.pop() {
                    if let Some(x) = stack.pop() {
                        stack.push(op.eval(x, y));
                        continue;
                    }
                }
                panic!("Missing operand for operator '{}'", op.token);
            }
            _ => unreachable!("Unexpected token {:?} during evaluation", token),
        }
    }

    assert!(stack.len() == 1);
    stack.pop().unwrap()
}

#[cfg(test)]
mod test {
    #![allow(unused_imports)]
    use super::*;
    use crate::solution::Solution;
    use crate::Solver;

    const EXPRESSION: [&str; 6] = [
        "1 + 2 * 3 + 4 * 5 + 6",
        "1 + (2 * 3) + (4 * (5 + 6))",
        "2 * 3 + (4 * 5)",
        "5 + (8 * 3 + 9 + 3 * 4 * 3)",
        "5 * 9 * (7 * 3 * 3 + 9 * 3 + (8 + 6 * 4))",
        "((2 + 4 * 9) * (6 + 9 * 8 + 6) + 6) + 2 + 4 * 2",
    ];

    #[test]
    fn example_1_part1() {
        let expected = 71;
        let actual = part1(EXPRESSION[0]);
        assert_eq!(actual, expected)
    }

    #[test]
    fn example_2_part1() {
        let expected = 51;
        let actual = part1(EXPRESSION[1]);
        assert_eq!(actual, expected)
    }

    #[test]
    fn example_3_part1() {
        let expected = 26;
        let actual = part1(EXPRESSION[2]);
        assert_eq!(actual, expected)
    }

    #[test]
    fn example_4_part1() {
        let expected = 437;
        let actual = part1(EXPRESSION[3]);
        assert_eq!(actual, expected)
    }

    #[test]
    fn example_5_part1() {
        let expected = 12_240;
        let actual = part1(EXPRESSION[4]);
        assert_eq!(actual, expected)
    }

    #[test]
    fn example_6_part1() {
        let expected = 13_632;
        let actual = part1(EXPRESSION[5]);
        assert_eq!(actual, expected)
    }

    #[test]
    fn example_1_part2() {
        let expected = 231;
        let actual = part2(EXPRESSION[0]);
        assert_eq!(actual, expected)
    }

    #[test]
    fn example_2_part2() {
        let expected = 51;
        let actual = part2(EXPRESSION[1]);
        assert_eq!(actual, expected)
    }

    #[test]
    fn example_3_part2() {
        let expected = 46;
        let actual = part2(EXPRESSION[2]);
        assert_eq!(actual, expected)
    }

    #[test]
    fn example_4_part2() {
        let expected = 1445;
        let actual = part2(EXPRESSION[3]);
        assert_eq!(actual, expected)
    }

    #[test]
    fn example_5_part2() {
        let expected = 669_060;
        let actual = part2(EXPRESSION[4]);
        assert_eq!(actual, expected)
    }

    #[test]
    fn example_6_part2() {
        let expected = 23_340;
        let actual = part2(EXPRESSION[5]);
        assert_eq!(actual, expected)
    }

    #[test]
    fn verify() {
        let solver = super::Solver::new();
        let input = include_str!("../../input/day18.txt");

        let expected: Solution = (510_009_915_468_u64, 321_176_691_637_769_u64).into();
        let actual = solver.solve(input);

        assert_eq!(actual, expected)
    }
}
