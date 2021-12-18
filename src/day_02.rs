use crate::{read, Solution};

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

fn read_directions(path: &str) -> Vec<Direction> {
    read(path).lines().map(Direction::from_str).collect()
}

// Solutions

fn part01(directions: &Vec<Direction>) -> u32 {
    let (position, depth) = directions
        .iter()
        .fold((0, 0), |(position, depth), direction| match direction {
            Direction::Forward(n) => (position + n, depth),
            Direction::Down(n) => (position, depth + n),
            Direction::Up(n) => (position, depth - n),
        });
    position * depth
}

fn part02(directions: &Vec<Direction>) -> u32 {
    let (position, depth, _) = directions.iter().fold(
        (0, 0, 0),
        |(position, depth, aim), direction| match direction {
            Direction::Forward(n) => (position + n, depth + (aim * n), aim),
            Direction::Down(n) => (position, depth, aim + n),
            Direction::Up(n) => (position, depth, aim - n),
        },
    );
    position * depth
}

pub fn day_02() -> Solution {
    let input = read_directions("./input/day_02.txt");
    let timer = std::time::Instant::now();
    Solution::new(2, part01(&input), part02(&input), timer.elapsed())
}
