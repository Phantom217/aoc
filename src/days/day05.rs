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
        5
    }

    fn solve(&self, input: &str) -> Solution {
        let (sum, (min, max)) =
            input
                .lines()
                .fold((0, (u32::MAX, 0)), |(mut sum, (min, max)), line| {
                    let seat_id = Seat::from(line).seat_id();
                    sum += seat_id;
                    (sum, (seat_id.min(min), seat_id.max(max)))
                });

        let total_sum = (max * (max + 1) - min * (min - 1)) / 2;
        let part2 = total_sum - sum;

        (max, part2).into()
    }
}

#[derive(Debug, PartialOrd, PartialEq)]
struct Seat {
    row: u32,
    col: u32,
}

impl Seat {
    /// Get seat ID.
    fn seat_id(&self) -> u32 {
        self.row * 8 + self.col
    }
}

impl std::convert::From<&[u8]> for Seat {
    /// Parses the input using the inverse of what the ASCII characters (`F`, `B`, `R`, `L`) represent.
    ///
    /// `F`, `L` are `1`
    /// `B`, `R` are `0`
    fn from(bytes: &[u8]) -> Self {
        let seat = bytes
            .iter()
            .fold(0, |acc, c| (acc << 1) + ((!c as u32) & 0x04) / 0x04);
        let row = seat >> 3;
        let col = seat & 0b111;

        Self { row, col }
    }
}

impl std::convert::From<&str> for Seat {
    /// Parses the input using the inverse of what the ASCII characters (`F`, `B`, `R`, `L`) represent.
    ///
    /// `F`, `L` are `1`
    /// `B`, `R` are `0`
    fn from(bytes: &str) -> Self {
        let seat = bytes
            .bytes()
            .fold(0, |acc, c| (acc << 1) + ((!c as u32) & 0x04) / 0x04);
        let row = seat >> 3;
        let col = seat & 0b111;

        Self { row, col }
    }
}

#[cfg(test)]
mod test {
    #![allow(unused_imports)]
    use super::*;
    use crate::solution::Solution;
    use crate::Solver;

    #[test]
    fn test_seat_location() {
        let expected = Seat::from("FBFBBFFRLR");
        let actual = Seat { row: 44, col: 5 };
        assert_eq!(actual, expected)
    }

    #[test]
    fn test_seat_location_bytes() {
        let expected = Seat::from("FBFBBFFRLR".as_bytes());
        let actual = Seat { row: 44, col: 5 };
        assert_eq!(actual, expected)
    }

    #[test]
    fn verify() {
        let solver = super::Solver::new();
        let input = include_str!("../../input/day05.txt");

        let actual: Solution = (919, 642).into();
        let expected = solver.solve(input);

        assert_eq!(actual, expected)
    }
}
