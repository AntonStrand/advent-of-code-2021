use crate::{read_comma_separated_number_input, Solution};

/* Find position */

fn means(input: &Vec<u32>) -> Vec<u32> {
    let sum = input.iter().fold(0f32, |sum, x| sum + (*x as f32));
    let len = input.len() as f32;

    if len == 0.0 {
        vec![0u32]
    } else {
        let mean = sum / len;
        let floor = mean.floor() as u32;
        let ceil = mean.ceil() as u32;

        if floor == ceil {
            vec![floor]
        } else {
            vec![floor, ceil]
        }
    }
}

fn median(input: &Vec<u32>) -> u32 {
    let index = input.len() / 2;
    let mut numbers = input.clone();
    numbers.sort();

    *numbers.get(index).unwrap()
}

/* Calculate fuel consumption */

fn get_steps(from: u32, to: u32) -> u64 {
    (from as i32 - to as i32).abs() as u64
}

fn calc_trip(from: u32, to: u32) -> u64 {
    let steps = get_steps(from, to) + 1;

    (0..steps).into_iter().fold(0u64, |fuel, step| fuel + step)
}

fn calc_fuel<F: Fn(u32, u32) -> u64>(calculator: F, position: u32, positions: &Vec<u32>) -> u64 {
    positions.iter().map(|&x| calculator(position, x)).sum()
}

/* Solutions */

fn part01(input: &Vec<u32>) -> u64 {
    calc_fuel(get_steps, median(input), input)
}

fn part02(input: &Vec<u32>) -> u64 {
    means(input)
        .iter()
        .map(|&position| calc_fuel(calc_trip, position, input))
        .min()
        .unwrap()
}

pub fn day_07() -> Solution {
    let input = read_comma_separated_number_input("./input/day_07.txt");
    Solution::new(7, part01(&input), part02(&input))
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
        assert_eq!(means(&vec![1, 1, 1, 1]), [1])
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
    fn test_part02() {
        assert_eq!(part02(&vec![16, 1, 2, 0, 4, 2, 7, 1, 2, 14]), 168)
    }

    #[test]
    fn test_part02_final() {
        let input = &read_comma_separated_number_input("./input/day_07.txt");
        assert_eq!(part02(&input), 100220525)
    }
}
