use std::collections::HashMap;

use crate::{map_both, read_lines, Solution};

#[derive(Debug, PartialEq)]
enum Line {
    Straight(Vec<Coordinate>),
    Diagonal(Vec<Coordinate>),
}

impl Line {
    fn new(segment: &String) -> Option<Line> {
        let (c1, c2) = map_both(Coordinate::from_str, segment.split_once(" -> ").unwrap());
        if Line::is_diagonal(c1, c2) {
            Some(Line::to_diagonal(c1, c2))
        } else if Line::is_horizontal(c1, c2) {
            Some(Line::to_horizontal(c1, c2))
        } else if Line::is_vertical(c1, c2) {
            Some(Line::to_vertical(c1, c2))
        } else {
            None
        }
    }

    fn is_horizontal(c1: Coordinate, c2: Coordinate) -> bool {
        c1.y == c2.y && c1.x != c2.x
    }

    fn to_horizontal(c1: Coordinate, c2: Coordinate) -> Line {
        let xs = to_range(c1.x, c2.x);
        let ys = vec![c1.y; xs.len()];
        Line::Straight(Coordinate::range(xs, ys))
    }

    fn is_vertical(c1: Coordinate, c2: Coordinate) -> bool {
        c1.x == c2.x && c1.y != c2.y
    }

    fn to_vertical(c1: Coordinate, c2: Coordinate) -> Line {
        let ys = to_range(c1.y, c2.y);
        let xs = vec![c1.x; ys.len()];
        Line::Straight(Coordinate::range(xs, ys))
    }

    fn is_diagonal(c1: Coordinate, c2: Coordinate) -> bool {
        (c1.x - c2.x).abs() == (c1.y - c2.y).abs()
    }

    fn to_diagonal(c1: Coordinate, c2: Coordinate) -> Line {
        let xs: Vec<i16> = to_range(c1.x, c2.x);
        let ys: Vec<i16> = to_range(c1.y, c2.y);
        Line::Diagonal(Coordinate::range(xs, ys))
    }

    fn all_coordinates(line: &Line) -> Option<Vec<Coordinate>> {
        Some(match line {
            Line::Straight(cs) => cs.to_vec(),
            Line::Diagonal(cs) => cs.to_vec(),
        })
    }

    fn straight_coordinates(line: &Line) -> Option<Vec<Coordinate>> {
        match line {
            Line::Straight(cs) => Some(cs.to_vec()),
            Line::Diagonal(_) => None,
        }
    }
}

fn to_range(x: i16, y: i16) -> Vec<i16> {
    if x > y {
        (y..(x + 1)).collect()
    } else {
        let mut range: Vec<i16> = (x..(y + 1)).collect();
        range.reverse();
        range
    }
}

#[derive(Debug, PartialEq, Clone, Eq, PartialOrd, Ord, Hash, Copy)]
struct Coordinate {
    x: i16,
    y: i16,
}

impl Coordinate {
    fn new(x: i16, y: i16) -> Coordinate {
        Coordinate { x, y }
    }

    fn from_tuple((x, y): (&i16, &i16)) -> Coordinate {
        Coordinate::new(x.to_owned(), y.to_owned())
    }

    fn from_str(str: &str) -> Coordinate {
        let (x, y) = str.split_once(",").unwrap();
        Coordinate::new(x.parse().unwrap(), y.parse().unwrap())
    }

    fn range(xs: Vec<i16>, ys: Vec<i16>) -> Vec<Coordinate> {
        xs.iter()
            .zip(ys.iter())
            .map(Coordinate::from_tuple)
            .collect()
    }
}

fn number_of_overlapping_coordinates(coordinates: Vec<Coordinate>) -> u64 {
    let mut counter = HashMap::new();

    coordinates.iter().for_each(|c| {
        let count = counter.entry(c).or_insert(0);
        *count += 1;
    });

    (counter
        .iter()
        .filter_map(|(&&coordinate, &count)| if count > 1 { Some(coordinate) } else { None })
        .count()) as u64
}

fn get_coordinates<F: Fn(&Line) -> Option<Vec<Coordinate>>>(
    getter: F,
    lines: &Vec<Line>,
) -> Vec<Coordinate> {
    lines.iter().filter_map(getter).flatten().collect()
}

fn part01(lines: &Vec<Line>) -> u64 {
    number_of_overlapping_coordinates(get_coordinates(Line::straight_coordinates, lines))
}

fn part02(lines: &Vec<Line>) -> u64 {
    number_of_overlapping_coordinates(get_coordinates(Line::all_coordinates, lines))
}

pub fn day_05() -> Solution {
    let input = read_lines("./input/day_05.txt")
        .iter()
        .filter_map(Line::new)
        .collect();
    let timer = std::time::Instant::now();
    Solution::new(5, part01(&input), part02(&input), timer.elapsed())
}

#[cfg(test)]
mod tests {
    // Note this useful idiom: importing names from outer (for mod tests) scope.
    use super::*;

    #[test]
    fn test_positive_horizontal_line() {
        assert_eq!(
            Line::new(&String::from("0,9 -> 5,9")),
            Some(Line::Straight(vec![
                Coordinate::new(5, 9),
                Coordinate::new(4, 9),
                Coordinate::new(3, 9),
                Coordinate::new(2, 9),
                Coordinate::new(1, 9),
                Coordinate::new(0, 9),
            ]))
        )
    }

    #[test]
    fn test_negative_horizontal_line() {
        assert_eq!(
            Line::new(&String::from("9,7 -> 7,7")),
            Some(Line::Straight(vec![
                Coordinate::new(7, 7),
                Coordinate::new(8, 7),
                Coordinate::new(9, 7),
            ]))
        )
    }

    #[test]
    fn test_positive_vertical_line() {
        assert_eq!(
            Line::new(&String::from("0,3 -> 0,7")),
            Some(Line::Straight(vec![
                Coordinate::new(0, 7),
                Coordinate::new(0, 6),
                Coordinate::new(0, 5),
                Coordinate::new(0, 4),
                Coordinate::new(0, 3),
            ]))
        )
    }

    #[test]
    fn test_negative_vertical_line() {
        assert_eq!(
            Line::new(&String::from("9,7 -> 9,5")),
            Some(Line::Straight(vec![
                Coordinate::new(9, 5),
                Coordinate::new(9, 6),
                Coordinate::new(9, 7),
            ]))
        )
    }

    #[test]
    fn test_diagonal_line_1() {
        assert_eq!(
            Line::new(&String::from("1,1 -> 3,3")),
            Some(Line::Diagonal(vec![
                Coordinate::new(3, 3),
                Coordinate::new(2, 2),
                Coordinate::new(1, 1),
            ]))
        )
    }

    #[test]
    fn test_diagonal_line_2() {
        assert_eq!(
            Line::new(&String::from("8,0 -> 0,8")),
            Some(Line::Diagonal(vec![
                Coordinate::new(0, 8),
                Coordinate::new(1, 7),
                Coordinate::new(2, 6),
                Coordinate::new(3, 5),
                Coordinate::new(4, 4),
                Coordinate::new(5, 3),
                Coordinate::new(6, 2),
                Coordinate::new(7, 1),
                Coordinate::new(8, 0),
            ]))
        )
    }

    #[test]
    fn test_number_of_overlapping_coordinates() {
        assert_eq!(
            number_of_overlapping_coordinates(vec![
                Coordinate::new(0, 0),
                Coordinate::new(0, 1),
                Coordinate::new(3, 1),
                Coordinate::new(3, 3),
                Coordinate::new(1, 1),
                Coordinate::new(3, 2),
                Coordinate::new(3, 1),
                Coordinate::new(3, 1),
                Coordinate::new(1, 1),
            ]),
            2
        )
    }

    #[test]
    fn test_lines() {
        let actual: Vec<bool> = vec![
            (Coordinate::new(0, 9), Coordinate::new(5, 9)),
            (Coordinate::new(8, 0), Coordinate::new(0, 8)),
            (Coordinate::new(9, 4), Coordinate::new(3, 4)),
            (Coordinate::new(2, 2), Coordinate::new(2, 1)),
            (Coordinate::new(7, 0), Coordinate::new(7, 4)),
            (Coordinate::new(6, 4), Coordinate::new(2, 0)),
            (Coordinate::new(0, 9), Coordinate::new(2, 9)),
            (Coordinate::new(3, 4), Coordinate::new(1, 4)),
            (Coordinate::new(0, 0), Coordinate::new(8, 8)),
            (Coordinate::new(5, 5), Coordinate::new(8, 2)),
        ]
        .iter()
        .map(|(c1, c2)| Line::is_diagonal(c1.to_owned(), c2.to_owned()))
        .collect();

        assert_eq!(
            actual,
            vec![false, true, false, false, false, true, false, false, true, true]
        )
    }
}
