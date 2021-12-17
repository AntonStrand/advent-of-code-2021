use std::{convert::identity, ops::Range};

use crate::{map_pair, read, unsafe_parse, Solution};

const LEFT: usize = 0;
const RIGHT: usize = 1;
const TOP: usize = 2;
const BOTTOM: usize = 3;

type TargetArea = [i16; 4];

fn range_to_numbers(range: &str) -> (i16, i16) {
    map_pair(unsafe_parse, range.split_once("..").unwrap())
}

fn to_target_area(input: &str) -> TargetArea {
    let mut target_area = [0; 4];

    let ((left, right), (bottom, top)) =
        map_pair(range_to_numbers, input[15..].split_once(", y=").unwrap());

    target_area[LEFT] = left;
    target_area[RIGHT] = right;
    target_area[TOP] = top;
    target_area[BOTTOM] = bottom;

    target_area
}

fn is_in(target: TargetArea, x: i16, y: i16) -> bool {
    x >= target[LEFT] && x <= target[RIGHT] && y <= target[TOP] && y >= target[BOTTOM]
}

fn has_missed(target: TargetArea, x: i16, y: i16) -> bool {
    x > target[RIGHT] || y < target[BOTTOM]
}

fn simulate(target: TargetArea, mut vx: i16, mut vy: i16) -> Option<i16> {
    let mut max_height = 0;
    let mut y = 0;
    let mut x = 0;

    loop {
        // Update movement
        y += vy;
        x += vx;
        vx -= if vx > 0 { 1 } else { 0 };
        vy -= 1;

        max_height = max_height.max(y);

        if is_in(target, x, y) {
            return Some(max_height);
        }

        if has_missed(target, x, y) {
            return None;
        }
    }
}

fn brute(target: TargetArea, y_range: Range<i16>) -> Vec<i16> {
    let mut results = vec![];

    for x in 1..target[RIGHT] + 1 {
        for y in y_range.to_owned() {
            results.push(simulate(target, x, y));
        }
    }

    results.into_iter().filter_map(identity).collect()
}

/* Solutions */

fn part01(input: &String) -> i16 {
    let target = to_target_area(&input);
    brute(target, 1..170).into_iter().max().unwrap_or(0)
}

fn part02(input: &String) -> i16 {
    let target = to_target_area(&input);
    brute(target, target[BOTTOM]..170).len() as i16
}

pub fn day_17() -> Solution {
    let input = read("./input/day_17.txt");
    let timer = std::time::Instant::now();
    Solution::new(17, part01(&input), part02(&input), timer.elapsed())
}

/* Tests */

#[cfg(test)]
mod tests {
    // Note this useful idiom: importing names from outer (for mod tests) scope.
    use super::*;

    #[test]
    fn test_to_target_area_example() {
        assert_eq!(
            to_target_area("target area: x=20..30, y=-10..-5"),
            [20, 30, -5, -10]
        )
    }

    #[test]
    fn test_is_in() {
        assert!(is_in([20, 30, -5, -10], 21, -9))
    }

    #[test]
    fn test_simulate_6_3() {
        let target = to_target_area("target area: x=20..30, y=-10..-5");
        assert_eq!(simulate(target, 6, 3), Some(6))
    }

    #[test]
    fn test_simulate_9_0() {
        let target = to_target_area("target area: x=20..30, y=-10..-5");
        assert_eq!(simulate(target, 9, 0), Some(0))
    }

    #[test]
    fn test_simulate_7_2() {
        let target = to_target_area("target area: x=20..30, y=-10..-5");
        assert_eq!(simulate(target, 7, 2), Some(3))
    }

    #[test]
    fn test_simulate_miss() {
        let target = to_target_area("target area: x=20..30, y=-10..-5");
        assert_eq!(simulate(target, 0, 2), None)
    }

    #[test]
    fn test_to_target_area() {
        assert_eq!(
            to_target_area("target area: x=94..151, y=-156..-103"),
            [94, 151, -103, -156]
        )
    }

    #[test]
    fn test_part01() {
        let input = read("./input/day_17.txt");
        assert_eq!(part01(&input), 12090)
    }

    #[test]
    fn test_part02() {
        let input = read("./input/day_17.txt");
        assert_eq!(part02(&input), 5059)
    }
}
