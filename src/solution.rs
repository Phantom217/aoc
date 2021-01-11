use std::fmt;

#[derive(Default, PartialEq)]
pub struct Solution {
    part1: String,
    part2: String,
}

impl Solution {
    pub fn part1(&self) -> &str {
        &self.part1
    }
    pub fn part2(&self) -> &str {
        &self.part2
    }
}

impl fmt::Display for Solution {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "\tPart 1: {}\n\tPart 2: {}", self.part1, self.part2)
    }
}

impl fmt::Debug for Solution {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "\tPart 1: {}\n\tPart 2: {}", self.part1, self.part2)
    }
}

macro_rules! impl_from {
    ($($t:ty),+) => {
        $(impl std::convert::From<($t, $t)> for Solution {
            fn from(parts: ($t, $t)) -> Self {
                Self {
                    part1: parts.0.to_string(),
                    part2: parts.1.to_string(),
                }
            }
        })*
    };
}

impl_from!(usize, isize, u64, u32, i32);

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_fmt() {
        let actual = Solution {
            part1: String::from("1"),
            part2: String::from("2"),
        };
        let expected = "\tPart 1: 1\n\tPart 2: 2";

        assert_eq!(&format!("{}", actual), expected)
    }

    #[test]
    fn test_from() {
        let expected = Solution {
            part1: String::from("1"),
            part2: String::from("2"),
        };
        let actual: Solution = (1, 2).into();

        assert_eq!(actual, expected)
    }
}
