use crate::{read_number_input, Solution};

fn increment(current: u32, x: u32, y: u32) -> u32 {
    if x > y {
        current + 1
    } else {
        current
    }
}

fn part01(input: &Vec<u32>) -> u32 {
    let first = input[0];
    let rest = &input[1..];
    rest.iter()
        .fold((first, 0), |(prev, count), x| {
            (*x, increment(count, *x, prev))
        })
        .1
}

fn part02(input: &Vec<u32>) -> u32 {
    let values: Vec<u32> = (0..(input.len() - 2))
        .map(|i| input[i..(i + 3)].iter().sum())
        .collect();

    part01(&values)
}

pub fn day_01() -> Solution {
    let input = read_number_input("./input/day_01.txt");
    Solution::new(1, part01(&input), part02(&input))
}
