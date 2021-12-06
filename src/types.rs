use std::fmt::{self, Display};

pub struct Solution(u8, u64, u64);

impl Solution {
    pub fn new(day: u8, part1: u64, part2: u64) -> Solution {
        Solution(day, part1, part2)
    }

    pub fn view_all(solutions: Vec<Solution>) {
        println!(" ------------------------------------ ",);
        println!(
            "| {0: <3} | {1: >10} | {2: >15} |",
            "Day", "Part 1", "Part 2"
        );
        println!("|-----|------------|-----------------|",);
        solutions
            .iter()
            .for_each(|solution| println!("{}", solution));
        println!(" ------------------------------------ ",);
    }

    pub fn view(solution: Solution) {
        Solution::view_all(vec![solution]);
    }
}

impl Display for Solution {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "|  {0: <2} | {1: >10} | {2: >15} |",
            self.0.to_string(),
            self.1.to_string(),
            self.2.to_string(),
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
