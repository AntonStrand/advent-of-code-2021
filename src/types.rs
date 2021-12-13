use std::{
    fmt::{self, Display},
    time::Duration,
};

#[derive(Clone)]
pub struct Solution {
    day: String,
    part_1: String,
    part_2: String,
    duration: Duration,
}

impl Solution {
    pub fn new<T: ToString>(day: u8, part_1: T, part_2: T, duration: Duration) -> Solution {
        Solution {
            day: day.to_string(),
            part_1: part_1.to_string(),
            part_2: part_2.to_string(),
            duration,
        }
    }

    pub fn view_all(solutions: Vec<Solution>) {
        println!(".------------------------------------------------------------------------.",);
        println!("|                       🎄 Advent of Code - 2021 🎄                      |",);
        println!("├-----┬------------┬-----------------┬-----------------┬-----------------┤",);
        println!(
            "| {: >3} | {: >10} | {: >15} | {: >15} | {: >15} |",
            "Day", "Part 1", "Part 2", "Time (ms)", "Time (ns)"
        );
        println!("├-----┼------------┼-----------------┼-----------------┼-----------------┤",);
        solutions.iter().for_each(Solution::print);
        println!("'-----┴------------┴-----------------┴-----------------┴-----------------'",);
    }

    pub fn view(solution: Solution) {
        Solution::view_all(vec![solution]);
    }

    fn print(solution: &Solution) {
        println!("{}", solution);
    }
}

impl Display for Solution {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "|  {: >2} | {: >10} | {: >15} | {: >15} | {: >15} |",
            self.day,
            self.part_1,
            self.part_2,
            self.duration.as_micros(),
            self.duration.as_nanos(),
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
