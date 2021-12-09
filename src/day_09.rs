use std::collections::HashSet;

use crate::{parse_number_string, read_lines, sort_desc, Solution};

#[derive(Debug, PartialEq, Clone, Copy, Eq, Hash)]
struct Point(i16, i16);

#[derive(Debug, PartialEq, Clone, Eq, Hash)]
struct Adjacent {
    value: u8,
    values: Vec<(u8, Point)>,
    position: Point,
}

struct Matrix(Vec<Vec<u8>>);

impl Matrix {
    fn new(input: Vec<String>) -> Matrix {
        Matrix(input.iter().map(parse_number_string).collect())
    }

    fn get(&self, Point(x, y): Point) -> Option<(u8, Point)> {
        if x < 0 || y < 0 {
            None
        } else {
            self.0
                .get(y as usize)
                .and_then(|r| r.get(x as usize))
                .map(|v| (*v, Point(x, y)))
        }
    }

    fn find_adjacent(&self, Point(x, y): Point) -> Adjacent {
        Adjacent {
            value: self.get(Point(x, y)).unwrap().0,
            values: vec![
                self.get(Point(x - 1, y)),
                self.get(Point(x + 1, y)),
                self.get(Point(x, y - 1)),
                self.get(Point(x, y + 1)),
            ]
            .iter()
            .filter_map(|x| *x)
            .collect(),
            position: Point(x, y),
        }
    }

    fn to_adjacents(&self) -> Vec<Adjacent> {
        let width = self.0.get(0).unwrap().len();
        let len = self.0.len();

        (0..len)
            .into_iter()
            .flat_map(|y| {
                (0..width)
                    .into_iter()
                    .map(|x| self.find_adjacent(Point(x as i16, y as i16)))
                    .collect::<Vec<Adjacent>>()
            })
            .collect()
    }

    fn get_low_points(&self) -> Vec<Adjacent> {
        self.to_adjacents()
            .iter()
            .filter(|adjacent| {
                adjacent.value < adjacent.values.iter().map(|(v, _)| *v).min().unwrap()
            })
            .map(|a| a.to_owned())
            .collect()
    }

    fn basin_sizes(&self) -> Vec<u64> {
        self.get_low_points()
            .iter()
            .map(|low_point| self.basin_size(low_point.to_owned()))
            .collect()
    }

    /* DFS */
    fn basin_size(&self, adjacent: Adjacent) -> u64 {
        let mut stack = vec![adjacent];
        let mut discovered = HashSet::new();

        while !stack.is_empty() {
            let a = stack.pop().unwrap();
            if !discovered.contains(&a) {
                discovered.insert(a.to_owned());
                a.values
                    .iter()
                    .filter(|(v, _)| *v < 9 && v > &a.value)
                    .for_each(|(_, point)| stack.push(self.find_adjacent(*point)));
            }
        }

        discovered.len() as u64
    }
}

/* Solutions */

fn part01(input: &Matrix) -> u64 {
    input
        .get_low_points()
        .iter()
        .map(|a| (a.value + 1) as u64)
        .sum()
}

fn part02(input: &Matrix) -> u64 {
    sort_desc(input.basin_sizes())[..3].into_iter().product()
}

pub fn day_09() -> Solution {
    let input = Matrix::new(read_lines("./input/day_09.txt"));
    let timer = std::time::Instant::now();
    Solution::new(9, part01(&input), part02(&input), timer.elapsed())
}

/* Tests */

#[cfg(test)]
mod tests {
    // Note this useful idiom: importing names from outer (for mod tests) scope.
    use super::*;

    #[test]
    fn test_find_adjacent() {
        let matrix = Matrix::new(vec![
            String::from("2199943210"),
            String::from("3987894921"),
            String::from("9856789892"),
            String::from("8767896789"),
            String::from("9899965678"),
        ]);

        assert_eq!(
            matrix.find_adjacent(Point(1, 0)),
            Adjacent {
                value: 1,
                values: vec![(2, Point(0, 0)), (9, Point(2, 0)), (9, Point(1, 1))],
                position: Point(1, 0)
            }
        )
    }

    #[test]
    fn test_part01() {
        let input = Matrix::new(read_lines("./input/day_09.txt"));
        assert_eq!(part01(&input), 572)
    }

    #[test]
    fn test_part02() {
        let input = Matrix::new(read_lines("./input/day_09.txt"));
        assert_eq!(part02(&input), 847044)
    }
}
