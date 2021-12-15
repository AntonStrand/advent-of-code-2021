use std::{cmp::Reverse, collections::BinaryHeap};

use crate::{parse_number_string, read_lines, Solution};

const N: usize = 100;
const LAST: usize = N - 1;

type Grid = [[u8; N]; N];

fn parse_grid(rows: &Vec<String>) -> Grid {
    let mut grid = [[0u8; N]; N];

    rows.into_iter()
        .map(parse_number_string)
        .enumerate()
        .for_each(|(y, row)| {
            row.into_iter()
                .enumerate()
                .for_each(|(x, risk_level)| grid[y][x] = risk_level)
        });

    grid
}

fn is_last((x, y): (usize, usize)) -> bool {
    x == LAST && y == LAST
}

fn is_in_grid((x, y): &&(usize, usize)) -> bool {
    x < &&N && y < &&N
}

fn adjacent((x, y): (usize, usize)) -> [(usize, usize); 4] {
    [
        // Make sure that the usize won't get an overflow
        (x.wrapping_sub(1), y),
        (x + 1, y),
        (x, y.wrapping_sub(1)),
        (x, y + 1),
    ]
}

fn find_lowest_risk_level(mut map: Grid) -> u32 {
    // Use Reverse to get the lowest instead of the highest risk.
    let mut heap = BinaryHeap::from(vec![(Reverse(0), (0, 0))]);

    while let Some((Reverse(risk), position)) = heap.pop() {
        if is_last(position) {
            return risk;
        }

        adjacent(position)
            .iter()
            // Remove wrapped positions
            .filter(is_in_grid)
            .for_each(|&(x, y)| {
                if map[y][x] > 0 {
                    heap.push((Reverse(risk + map[y][x] as u32), (x, y)));
                    map[y][x] = 0; // "Mark as visited"
                }
            })
    }

    0
}

/* Solutions */

fn part01(input: &Vec<String>) -> u32 {
    find_lowest_risk_level(parse_grid(input))
}

fn part02(input: &Vec<String>) -> u32 {
    0
}

pub fn day_15() -> Solution {
    let input = read_lines("./input/day_15.txt");
    let timer = std::time::Instant::now();
    Solution::new(15, part01(&input), part02(&input), timer.elapsed())
}

/* Tests */

#[cfg(test)]
mod tests {
    // Note this useful idiom: importing names from outer (for mod tests) scope.
    use super::*;

    // #[test]
    // fn test_parse() {
    //     vec![
    //         String::from("1163751742"),
    //         String::from("1381373672"),
    //         String::from("2136511328"),
    //         String::from("3694931569"),
    //         String::from("7463417111"),
    //         String::from("1319128137"),
    //         String::from("1359912421"),
    //         String::from("3125421639"),
    //         String::from("1293138521"),
    //         String::from("2311944581"),
    //     ]
    // }

    // #[test]
    // fn test_part01() {
    //     let input = read("./input/day_15.txt");
    //     assert_eq!(part01(&input), 0)
    // }

    // #[test]
    // fn test_part02() {
    //     let input = read("./input/day_15.txt");
    //     assert_eq!(part02(&input), 0)
    // }
}
