use crate::{read_comma_separated_number_input, Solution};

/* Find position */

fn means(input: &Vec<u32>) -> [u32; 2] {
    // Division by 0 is fine as long as the result is a float that will be converted to a u32.
    let mean = input.iter().sum::<u32>() as f32 / input.len() as f32;
    // floor and ceil will turn NaN to 0 when converted to u32.
    [mean.floor() as u32, mean.ceil() as u32]
}

fn median(input: &Vec<u32>) -> u32 {
    let index = input.len() / 2;
    let mut numbers = input.clone();
    numbers.sort();

    // In this case we will get one index
    *numbers.get(index).unwrap()
}

/* Calculate fuel consumption */

fn get_steps(from: u32, to: u32) -> u32 {
    if from > to {
        from - to
    } else {
        to - from
    }
}

fn calc_trip(from: u32, to: u32) -> u32 {
    let steps = get_steps(from, to);
    (steps * (steps + 1)) / 2
}

fn calc_fuel<F: Fn(u32, u32) -> u32>(calculator: F, positions: &Vec<u32>, position: u32) -> u32 {
    positions.iter().map(|&x| calculator(position, x)).sum()
}

/* Solutions */

fn part01(input: &Vec<u32>) -> u32 {
    calc_fuel(get_steps, input, median(input))
}

fn part02(input: &Vec<u32>) -> u32 {
    let fuel_cost = means(input).map(|x| calc_fuel(calc_trip, input, x));
    fuel_cost[0].min(fuel_cost[1])
}

pub fn day_07() -> Solution {
    let input = read_comma_separated_number_input("./input/day_07.txt");
    let timer = std::time::Instant::now();
    Solution::new(7, part01(&input), part02(&input), timer.elapsed())
}

/* Brute force solution - for fun */

#[allow(dead_code)]
fn brute<F: Fn(u32, u32) -> u32 + Copy>(calc: F, input: &Vec<u32>) -> u32 {
    let min = *input.iter().min().unwrap();
    let max = *input.iter().max().unwrap() + 1;

    (min..max)
        .into_iter()
        .fold(u32::MAX, |cost, x| cost.min(calc_fuel(calc, &input, x)))
}

#[allow(dead_code)]
fn brute_01(input: &Vec<u32>) -> u32 {
    brute(get_steps, input)
}

#[allow(dead_code)]
fn brute_02(input: &Vec<u32>) -> u32 {
    brute(calc_trip, input)
}

#[cfg(test)]
mod tests {
    // Note this useful idiom: importing names from outer (for mod tests) scope.
    use super::*;

    #[test]
    fn test_mean_uneven() {
        assert_eq!(means(&vec![16, 1, 2, 0, 4, 2, 7, 1, 2, 14]), [4, 5])
    }

    #[test]
    fn test_mean_even() {
        assert_eq!(means(&vec![1, 1, 1, 1]), [1, 1])
    }

    #[test]
    fn test_mean_empty() {
        assert_eq!(means(&vec![]), [0, 0])
    }

    #[test]
    fn test_median() {
        assert_eq!(median(&vec![16, 1, 2, 0, 4, 2, 7, 1, 2, 14]), 2)
    }

    #[test]
    fn test_calc_trip_0() {
        assert_eq!(calc_trip(16, 5), 66)
    }

    #[test]
    fn test_calc_trip_1() {
        assert_eq!(calc_trip(1, 5), 10)
    }

    #[test]
    fn test_part01() {
        assert_eq!(part01(&vec![16, 1, 2, 0, 4, 2, 7, 1, 2, 14]), 37)
    }

    #[test]
    fn test_part01_final() {
        let input = &read_comma_separated_number_input("./input/day_07.txt");
        assert_eq!(part01(&input), 348664)
    }

    #[test]
    fn test_brute_01() {
        let input = &read_comma_separated_number_input("./input/day_07.txt");
        assert_eq!(brute_01(&input), 348664)
    }

    #[test]
    fn test_part02() {
        assert_eq!(part02(&vec![16, 1, 2, 0, 4, 2, 7, 1, 2, 14]), 168)
    }

    #[test]
    fn test_part02_final() {
        let input = &read_comma_separated_number_input("./input/day_07.txt");
        assert_eq!(part02(&input), 100220525)
    }

    #[test]
    fn test_brute_02() {
        let input = &read_comma_separated_number_input("./input/day_07.txt");
        assert_eq!(part02(&input), 100220525)
    }
}
