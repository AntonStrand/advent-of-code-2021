use crate::{read_directions, Direction, Solution};

fn part01(directions: &Vec<Direction>) -> u64 {
    let (position, depth) = directions
        .iter()
        .fold((0, 0), |(position, depth), direction| match direction {
            Direction::Forward(n) => (position + n, depth),
            Direction::Down(n) => (position, depth + n),
            Direction::Up(n) => (position, depth - n),
        });
    (position * depth) as u64
}

fn part02(directions: &Vec<Direction>) -> u64 {
    let (position, depth, _) = directions.iter().fold(
        (0, 0, 0),
        |(position, depth, aim), direction| match direction {
            Direction::Forward(n) => (position + n, depth + (aim * n), aim),
            Direction::Down(n) => (position, depth, aim + n),
            Direction::Up(n) => (position, depth, aim - n),
        },
    );
    (position * depth) as u64
}

pub fn day_02() -> Solution {
    let input = read_directions("./input/day_02.txt");
    Solution::new(2, part01(&input), part02(&input))
}
