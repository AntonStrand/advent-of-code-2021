use std::collections::HashSet;

use crate::{parse_number_string, read_lines, Solution};

#[derive(Debug, PartialEq, Clone, Copy, Eq, Hash)]
struct Point(usize, usize);

impl Point {
    fn is_flashing(&self, grid: &Grid) -> bool {
        grid.get(*self) > 9
    }
}

#[derive(Debug, PartialEq, Clone, Copy)]
struct Grid([[u8; 10]; 10]);

impl Grid {
    fn positions() -> [Point; 100] {
        let mut pos = [Point(0, 0); 100];
        let mut index = 0;

        for y in 0..10 {
            for x in 0..10 {
                pos[index] = Point(x, y);
                index += 1;
            }
        }

        pos
    }

    fn new(input: Vec<String>) -> Grid {
        let input: Vec<Vec<u8>> = input.iter().map(parse_number_string).collect();

        Grid(
            Grid::positions()
                .iter()
                .fold([[0; 10]; 10], |mut grid, &Point(x, y)| {
                    grid[y][x] = *input.get(y).and_then(|r| r.get(x)).unwrap();
                    grid
                }),
        )
    }

    fn set(&self, Point(x, y): Point, value: u8) -> Grid {
        let mut grid = self.0;
        grid[x][y] = value;
        Grid(grid)
    }

    fn get(&self, Point(x, y): Point) -> u8 {
        self.0[x][y]
    }

    fn increment_all(&self) -> Grid {
        Grid(
            Grid::positions()
                .iter()
                .fold([[0; 10]; 10], |mut grid, &Point(x, y)| {
                    grid[y][x] = self.0[y][x] + 1;
                    grid
                }),
        )
    }

    fn increment_adjacent(&self, Point(x, y): Point, has_flashed: &HashSet<Point>) -> Grid {
        let x = x as i8;
        let y = y as i8;
        self.clone()
            .increment_if_in_bound(has_flashed, x - 1, y - 1)
            .increment_if_in_bound(has_flashed, x, y - 1)
            .increment_if_in_bound(has_flashed, x + 1, y - 1)
            .increment_if_in_bound(has_flashed, x - 1, y)
            .increment_if_in_bound(has_flashed, x + 1, y)
            .increment_if_in_bound(has_flashed, x - 1, y + 1)
            .increment_if_in_bound(has_flashed, x, y + 1)
            .increment_if_in_bound(has_flashed, x + 1, y + 1)
    }

    fn increment_if_in_bound(mut self, has_flashed: &HashSet<Point>, x: i8, y: i8) -> Grid {
        if x >= 0 && x < 10 && y >= 0 && y < 10 {
            let x = x as usize;
            let y = y as usize;
            if !has_flashed.contains(&Point(x, y)) {
                self.0[x][y] += 1;
            }
        }

        self
    }
}

#[derive(Debug, PartialEq, Clone, Copy)]
struct State {
    grid: Grid,
    flashes: u32,
    step_flashes: u32,
}

impl State {
    fn new(input: Vec<String>) -> State {
        State {
            grid: Grid::new(input),
            flashes: 0,
            step_flashes: 0,
        }
    }

    fn step(&self) -> State {
        let mut grid = self.grid.increment_all();
        let mut flashes = 0u32;
        let mut has_flashed: HashSet<Point> = HashSet::new();
        let mut should_flash: Vec<Point> = vec![];

        loop {
            Grid::positions().iter().for_each(|&point| {
                if point.is_flashing(&grid) && !has_flashed.contains(&point) {
                    flashes += 1;
                    should_flash.push(point);
                    has_flashed.insert(point);
                    grid = grid.set(point, 0);
                }
            });

            if should_flash.is_empty() {
                break;
            } else {
                grid = should_flash.iter().fold(grid, |grid, point| {
                    grid.increment_adjacent(*point, &has_flashed)
                });
                should_flash = vec![];
            }
        }

        State {
            grid,
            flashes: self.flashes + flashes,
            step_flashes: flashes,
        }
    }
}

/* Solutions */

fn part01(input: &State) -> u64 {
    let final_state = (0..100)
        .into_iter()
        .fold(input.clone(), |state, _| state.step());

    final_state.flashes as u64
}

fn part02(input: &State) -> u64 {
    let mut state = *input;
    let mut step = 0;

    loop {
        step += 1;
        state = state.step();
        if state.step_flashes == 100 {
            break;
        }
    }

    step
}

pub fn day_11() -> Solution {
    let input = State::new(read_lines("./input/day_11.txt"));
    let timer = std::time::Instant::now();
    Solution::new(11, part01(&input), part02(&input), timer.elapsed())
}

/* Tests */

#[cfg(test)]
mod tests {
    // Note this useful idiom: importing names from outer (for mod tests) scope.
    use super::*;

    #[test]
    fn test_new_grid() {
        let state = State::new(vec![
            String::from("5483143223"),
            String::from("2745854711"),
            String::from("5264556173"),
            String::from("6141336146"),
            String::from("6357385478"),
            String::from("4167524645"),
            String::from("2176841721"),
            String::from("6882881134"),
            String::from("4846848554"),
            String::from("5283751526"),
        ]);

        assert_eq!(
            state,
            State {
                grid: Grid([
                    [5, 4, 8, 3, 1, 4, 3, 2, 2, 3],
                    [2, 7, 4, 5, 8, 5, 4, 7, 1, 1],
                    [5, 2, 6, 4, 5, 5, 6, 1, 7, 3],
                    [6, 1, 4, 1, 3, 3, 6, 1, 4, 6],
                    [6, 3, 5, 7, 3, 8, 5, 4, 7, 8],
                    [4, 1, 6, 7, 5, 2, 4, 6, 4, 5],
                    [2, 1, 7, 6, 8, 4, 1, 7, 2, 1],
                    [6, 8, 8, 2, 8, 8, 1, 1, 3, 4],
                    [4, 8, 4, 6, 8, 4, 8, 5, 5, 4],
                    [5, 2, 8, 3, 7, 5, 1, 5, 2, 6],
                ]),
                flashes: 0,
                step_flashes: 0,
            }
        )
    }

    #[test]
    fn test_step_1() {
        let state = State::new(vec![
            String::from("5483143223"),
            String::from("2745854711"),
            String::from("5264556173"),
            String::from("6141336146"),
            String::from("6357385478"),
            String::from("4167524645"),
            String::from("2176841721"),
            String::from("6882881134"),
            String::from("4846848554"),
            String::from("5283751526"),
        ])
        .step();

        assert_eq!(
            state,
            State {
                grid: Grid([
                    [6, 5, 9, 4, 2, 5, 4, 3, 3, 4],
                    [3, 8, 5, 6, 9, 6, 5, 8, 2, 2],
                    [6, 3, 7, 5, 6, 6, 7, 2, 8, 4],
                    [7, 2, 5, 2, 4, 4, 7, 2, 5, 7],
                    [7, 4, 6, 8, 4, 9, 6, 5, 8, 9],
                    [5, 2, 7, 8, 6, 3, 5, 7, 5, 6],
                    [3, 2, 8, 7, 9, 5, 2, 8, 3, 2],
                    [7, 9, 9, 3, 9, 9, 2, 2, 4, 5],
                    [5, 9, 5, 7, 9, 5, 9, 6, 6, 5],
                    [6, 3, 9, 4, 8, 6, 2, 6, 3, 7],
                ]),
                flashes: 0,
                step_flashes: 0,
            }
        )
    }

    #[test]
    fn test_step_2() {
        let state = State::new(vec![
            String::from("5483143223"),
            String::from("2745854711"),
            String::from("5264556173"),
            String::from("6141336146"),
            String::from("6357385478"),
            String::from("4167524645"),
            String::from("2176841721"),
            String::from("6882881134"),
            String::from("4846848554"),
            String::from("5283751526"),
        ])
        .step()
        .step();

        assert_eq!(
            state,
            State {
                grid: Grid([
                    [8, 8, 0, 7, 4, 7, 6, 5, 5, 5],
                    [5, 0, 8, 9, 0, 8, 7, 0, 5, 4],
                    [8, 5, 9, 7, 8, 8, 9, 6, 0, 8],
                    [8, 4, 8, 5, 7, 6, 9, 6, 0, 0],
                    [8, 7, 0, 0, 9, 0, 8, 8, 0, 0],
                    [6, 6, 0, 0, 0, 8, 8, 9, 8, 9],
                    [6, 8, 0, 0, 0, 0, 5, 9, 4, 3],
                    [0, 0, 0, 0, 0, 0, 7, 4, 5, 6],
                    [9, 0, 0, 0, 0, 0, 0, 8, 7, 6],
                    [8, 7, 0, 0, 0, 0, 6, 8, 4, 8],
                ]),
                flashes: 35,
                step_flashes: 35,
            }
        )
    }

    #[test]
    fn test_step_10() {
        let state = State::new(vec![
            String::from("5483143223"),
            String::from("2745854711"),
            String::from("5264556173"),
            String::from("6141336146"),
            String::from("6357385478"),
            String::from("4167524645"),
            String::from("2176841721"),
            String::from("6882881134"),
            String::from("4846848554"),
            String::from("5283751526"),
        ])
        .step()
        .step()
        .step()
        .step()
        .step()
        .step()
        .step()
        .step()
        .step()
        .step();

        assert_eq!(
            state,
            State {
                grid: Grid([
                    [0, 4, 8, 1, 1, 1, 2, 9, 7, 6],
                    [0, 0, 3, 1, 1, 1, 2, 0, 0, 9],
                    [0, 0, 4, 1, 1, 1, 2, 5, 0, 4],
                    [0, 0, 8, 1, 1, 1, 1, 4, 0, 6],
                    [0, 0, 9, 9, 1, 1, 1, 3, 0, 6],
                    [0, 0, 9, 3, 5, 1, 1, 2, 3, 3],
                    [0, 4, 4, 2, 3, 6, 1, 1, 3, 0],
                    [5, 5, 3, 2, 2, 5, 2, 3, 5, 0],
                    [0, 5, 3, 2, 2, 5, 0, 6, 0, 0],
                    [0, 0, 3, 2, 2, 4, 0, 0, 0, 0],
                ]),
                flashes: 204,
                step_flashes: 29,
            }
        )
    }
    #[test]
    fn test_part01() {
        let input = State::new(read_lines("./input/day_11.txt"));
        assert_eq!(part01(&input), 1613)
    }

    #[test]
    fn test_part02() {
        let input = State::new(read_lines("./input/day_11.txt"));
        assert_eq!(part02(&input), 510)
    }
}
