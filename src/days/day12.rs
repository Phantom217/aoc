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
        12
    }

    fn solve(&self, input: &str) -> Solution {
        let actions = input.lines().map(Action::from).collect::<Vec<Action>>();

        let part1 = part1(&actions);
        let part2 = part2(&actions);

        (part1, part2).into()
    }
}

fn part1(actions: &[Action]) -> i32 {
    let mut ship = Ship::new();

    for action in actions {
        ship.move_ship(action);
    }

    ship.x.abs() + ship.y.abs()
}

fn part2(actions: &[Action]) -> i32 {
    let mut ship = Ship::new();
    let mut waypoint = Waypoint::new();

    for action in actions {
        match *action {
            Action::F(n) => {
                ship.x += waypoint.x * n as i32;
                ship.y += waypoint.y * n as i32;
            }
            _ => waypoint.move_waypoint(&action),
        }
    }

    ship.x.abs() + ship.y.abs()
}

#[derive(Debug)]
struct Ship {
    x: i32,
    y: i32,
    dir: Direction,
}

impl Ship {
    fn new() -> Self {
        Self {
            x: 0,
            y: 0,
            dir: Direction::E,
        }
    }

    fn move_ship(&mut self, action: &Action) {
        match *action {
            Action::N(n) => self.y += n as i32,
            Action::S(n) => self.y -= n as i32,
            Action::E(n) => self.x += n as i32,
            Action::W(n) => self.x -= n as i32,
            Action::F(n) => match self.dir {
                Direction::N => self.y += n as i32,
                Direction::S => self.y -= n as i32,
                Direction::E => self.x += n as i32,
                Direction::W => self.x -= n as i32,
            },
            _ => self.turn(action),
        }
    }

    fn turn(&mut self, action: &Action) {
        self.dir = self.dir.get_direction(action);
    }
}

#[derive(Debug)]
struct Waypoint {
    x: i32,
    y: i32,
}

impl Waypoint {
    fn new() -> Self {
        Self { x: 10, y: 1 }
    }

    fn move_waypoint(&mut self, action: &Action) {
        match *action {
            Action::N(n) => self.y += n as i32,
            Action::S(n) => self.y -= n as i32,
            Action::E(n) => self.x += n as i32,
            Action::W(n) => self.x -= n as i32,
            Action::L(_) | Action::R(_) => self.rotate(&action),
            Action::F(_) => unreachable!(),
        }
    }

    fn rotate(&mut self, action: &Action) {
        let rotation = match *action {
            Action::L(deg) => 360 - deg,
            Action::R(deg) => deg,
            _ => 0,
        };

        match rotation {
            90 => {
                let tmp = self.x;
                self.x = self.y;
                self.y = -tmp;
            }
            180 => {
                self.x = -self.x;
                self.y = -self.y;
            }
            270 => {
                let tmp = self.x;
                self.x = -self.y;
                self.y = tmp;
            }
            _ => {}
        }
    }
}

#[derive(Debug, Eq, PartialEq, Copy, Clone)]
enum Action {
    /// North
    N(u32),
    /// South
    S(u32),
    /// East
    E(u32),
    /// West
    W(u32),
    /// Left
    L(u32),
    /// Right
    R(u32),
    /// Forward
    F(u32),
}

impl std::convert::From<&str> for Action {
    fn from(a: &str) -> Self {
        let mut iter = a.chars();
        let action = iter.next().unwrap();
        let val = a[1..].trim().parse::<u32>().unwrap();
        match action {
            'N' => Self::N(val),
            'S' => Self::S(val),
            'E' => Self::E(val),
            'W' => Self::W(val),
            'L' => Self::L(val),
            'R' => Self::R(val),
            'F' => Self::F(val),
            _ => unreachable!(),
        }
    }
}

#[derive(Debug)]
enum Direction {
    /// North
    N,
    /// South
    S,
    /// East
    E,
    /// West
    W,
}

impl Direction {
    fn get_direction(&self, action: &Action) -> Self {
        let rotation = match *action {
            Action::L(deg) => 360 - deg,
            Action::R(deg) => deg,
            _ => 0,
        };

        match self {
            Self::N => match rotation {
                90 => Self::E,
                180 => Self::S,
                270 => Self::W,
                _ => Self::N,
            },
            Self::S => match rotation {
                90 => Self::W,
                180 => Self::N,
                270 => Self::E,
                _ => Self::S,
            },
            Self::E => match rotation {
                90 => Self::S,
                180 => Self::W,
                270 => Self::N,
                _ => Self::E,
            },
            Self::W => match rotation {
                90 => Self::N,
                180 => Self::E,
                270 => Self::S,
                _ => Self::W,
            },
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
F10
N3
F7
R90
F11";

    #[test]
    fn test_parse_action() {
        let actual = INPUT.lines().map(Action::from).collect::<Vec<Action>>();
        let expected = vec![
            Action::F(10),
            Action::N(3),
            Action::F(7),
            Action::R(90),
            Action::F(11),
        ];
        assert_eq!(actual, expected)
    }

    #[test]
    fn example_part1() {
        let actions = INPUT.lines().map(Action::from).collect::<Vec<Action>>();
        let expected = 25;
        let actual = part1(&actions);

        assert_eq!(actual, expected)
    }

    #[test]
    fn example_part2() {
        let actions = INPUT.lines().map(Action::from).collect::<Vec<Action>>();
        let expected = 286;
        let actual = part2(&actions);

        assert_eq!(actual, expected)
    }

    #[test]
    fn verify() {
        let solver = super::Solver::new();
        let input = include_str!("../../input/day12.txt");

        let expected: Solution = (757, 51249).into();
        let actual = solver.solve(input);

        assert_eq!(actual, expected)
    }
}
