use crate::{parse_number_string, read_lines, Solution};
use std::{cmp::Reverse, collections::BinaryHeap};

type Grid = [[u8; GRID_SIZE]; GRID_SIZE];

fn parse_grid(rows: &Vec<String>) -> Grid {
    let mut grid = [[0u8; GRID_SIZE]; GRID_SIZE];

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

fn is_last<const N: usize>((x, y): (usize, usize)) -> bool {
    x == N - 1 && y == N - 1
}

fn is_in_grid<const N: usize>((x, y): &&(usize, usize)) -> bool {
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

fn find_lowest_risk_level<const N: usize>(mut map: [[u8; N]; N]) -> u16 {
    // Use Reverse to get the lowest instead of the highest risk.
    let mut heap = BinaryHeap::from(vec![(Reverse(0), (0, 0))]);

    while let Some((Reverse(risk), position)) = heap.pop() {
        if is_last::<N>(position) {
            return risk;
        }

        adjacent(position)
            .iter()
            // Remove wrapped positions
            .filter(is_in_grid::<N>)
            .for_each(|&(x, y)| {
                if map[y][x] > 0 {
                    heap.push((Reverse(risk + map[y][x] as u16), (x, y)));
                    map[y][x] = 0; // "Mark as visited"
                }
            })
    }

    0
}

fn scale_grid<const N: usize>(grid: &Grid) -> [[u8; N]; N] {
    let mut scaled_grid = [[0u8; N]; N];
    let mut offset = 0;

    for y in 0..N {
        if y != 0 && y % GRID_SIZE == 0 {
            offset += 1;
        }
        for x in 0..N {
            if x != 0 && x % GRID_SIZE == 0 {
                offset += 1;
            }
            // Add offset, - 1 to be able to use mod 9 and then + 1 to restore to correct value.
            scaled_grid[y][x] = ((grid[y % GRID_SIZE][x % GRID_SIZE] + offset - 1) % 9) + 1;
        }
        offset -= 4;
    }

    scaled_grid
}

/* Solutions */

// Default grid size.
const GRID_SIZE: usize = 100;

fn part01(grid: &Grid) -> u16 {
    find_lowest_risk_level(*grid)
}

fn part02(grid: &Grid) -> u16 {
    const N: usize = GRID_SIZE * 5;
    find_lowest_risk_level(scale_grid::<N>(grid))
}

pub fn day_15() -> Solution {
    let input = parse_grid(&read_lines("./input/day_15.txt"));
    let timer = std::time::Instant::now();
    Solution::new(15, part01(&input), part02(&input), timer.elapsed())
}

/* Tests */

#[cfg(test)]
mod tests {
    // Note this useful idiom: importing names from outer (for mod tests) scope.
    use super::*;

    #[test]
    fn test_part01() {
        let input = parse_grid(&read_lines("./input/day_15.txt"));
        assert_eq!(part01(&input), 595)
    }

    #[test]
    fn test_part02() {
        let input = parse_grid(&read_lines("./input/day_15.txt"));
        assert_eq!(part02(&input), 2914)
    }
}
