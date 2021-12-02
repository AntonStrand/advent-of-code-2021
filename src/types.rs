use std::fmt::{self, Display};

pub struct Solution(u8, u32, u32);

impl Solution {
    pub fn new(day: u8, part1: u32, part2: u32) -> Solution {
        Solution(day, part1, part2)
    }
}

impl Display for Solution {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "Day {}: (Part 1: {}, Part 2: {})",
            self.0.to_string(),
            self.1.to_string(),
            self.2.to_string()
        )
    }
}

// DIRECTIONS

#[derive(Debug)]
pub enum Direction {
    Forward(u32),
    Down(u32),
    Up(u32),
}

impl Direction {
    pub fn from_str(input: &str) -> Direction {
        let (direction, value) = input.split_once(" ").unwrap();
        let n = value.parse::<u32>().unwrap();

        match direction {
            "forward" => Direction::Forward(n),
            "down" => Direction::Down(n),
            _ => Direction::Up(n),
        }
    }
}
