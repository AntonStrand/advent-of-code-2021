use std::{collections::HashMap, fmt::Debug};

use crate::{map_both, read, Solution};

type Algorithm = [char; 512];
struct Image {
    pixels: HashMap<(i32, i32), char>,
    width: usize,
    height: usize,
    lit: u32,
}

impl Image {
    fn copy(&self) -> Image {
        Image {
            pixels: self
                .pixels
                .clone()
                .into_iter()
                .map(|((x, y), v)| ((x + 1, y + 1), v))
                .collect(),
            width: self.width + 2,
            height: self.width + 2,
            lit: 0,
        }
    }

    fn from_str(input: &str) -> Image {
        let mut lit = 0;

        Image {
            pixels: input
                .lines()
                .enumerate()
                .flat_map(|(y, line)| {
                    line.chars()
                        .enumerate()
                        .map(|(x, c)| {
                            ((1 + x as i32, 1 + y as i32), {
                                lit += (c == '#') as u32;
                                char_to_binary(c)
                            })
                        })
                        .collect::<Vec<((i32, i32), char)>>()
                })
                .collect(),
            width: input.lines().last().unwrap().len(),
            height: input.lines().count(),
            lit,
        }
    }

    fn get_algorithm_index(&self, x: usize, y: usize) -> usize {
        let x = x as i32;
        let y = y as i32;

        let binary: String = [
            (x - 1, y - 1),
            (x, y - 1),
            (x + 1, y - 1),
            (x - 1, y),
            (x, y),
            (x + 1, y),
            (x - 1, y + 1),
            (x, y + 1),
            (x + 1, y + 1),
        ]
        .map(|pos| *self.pixels.get(&pos).unwrap_or(&'0'))
        .iter()
        .fold("".to_string(), |b, c| b + &c.to_string());

        usize::from_str_radix(&binary, 2).unwrap()
    }

    fn insert(&mut self, x: usize, y: usize, pixel: char) {
        self.pixels.insert((x as i32, y as i32), pixel);
        self.lit += pixel.to_digit(16).unwrap()
    }
}

impl Debug for Image {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}\nWidth: {}\nHeight: {}\nLit: {}",
            self.pixels
                .clone()
                .into_iter()
                .fold([[" "; 105]; 105], |mut img, ((x, y), pixel)| {
                    img[y as usize][x as usize] = if pixel == '1' { "#" } else { " " };
                    img
                })
                .map(|line| line.join(""))
                .join("\n"),
            self.width,
            self.height,
            self.lit
        )
    }
}

fn char_to_binary(c: char) -> char {
    if c == '.' {
        '0'
    } else {
        '1'
    }
}

fn parse_algorithm(input: &str) -> Algorithm {
    let mut algo = ['.'; 512];

    for (i, c) in input.chars().enumerate() {
        algo[i] = char_to_binary(c);
    }

    algo
}

fn enhance(image: Image, algorithm: Algorithm) -> Image {
    let mut output = image.copy();

    for y in 0..output.height {
        for x in 0..output.width {
            let i = image.get_algorithm_index(x, y);
            output.insert(x + 1, y + 1, algorithm[i]);
        }
    }

    output
}

/* Solutions */

fn part01(input: &String) -> u32 {
    let (algorithm, image) = map_both(
        parse_algorithm,
        Image::from_str,
        input.split_once("\n\n").unwrap(),
    );

    let res = enhance(enhance(image, algorithm), algorithm);

    println!("{:?}", res);

    res.lit
}

fn part02(_input: &String) -> u16 {
    0
}

pub fn day_20() -> Solution {
    let input = read("./input/day_20.txt");
    let timer = std::time::Instant::now();
    Solution::new(20, part01(&input), part02(&input), timer.elapsed())
}

/* Tests */

#[cfg(test)]
mod tests {
    // Note this useful idiom: importing names from outer (for mod tests) scope.
    use super::*;

    #[test]
    fn test_parse_algorithm() {
        assert_eq!(parse_algorithm("..#.#..#####.#.#.#.###.##.....###.##.#..###.####..#####..#....#..#..##..###..######.###...####..#..#####..##..#.#####...##.#.#..#.##..#.#......#.###.######.###.####...#.##.##..#..#..#####.....#.#....###..#.##......#.....#..#..#..##..#...##.######.####.####.#.#...#.......#..#.#.#...####.##.#......#..#...##.#.##..#...##.#.##..###.#......#.#.......#.#.#.####.###.##...#.....####.#..#..#.##.#....##..#.####....##...##..#...#......#.#.......#.......##..####..#...#.#.#...##..#.#..###..#####........#..####......#..#"), ['0', '0', '1', '0', '1', '0', '0', '1', '1', '1', '1', '1', '0', '1', '0', '1', '0', '1', '0', '1', '1', '1', '0', '1', '1', '0', '0', '0', '0', '0', '1', '1', '1', '0', '1', '1', '0', '1', '0', '0', '1', '1', '1', '0', '1', '1', '1', '1', '0', '0', '1', '1', '1', '1', '1', '0', '0', '1', '0', '0', '0', '0', '1', '0', '0', '1', '0', '0', '1', '1', '0', '0', '1', '1', '1', '0', '0', '1', '1', '1', '1', '1', '1', '0', '1', '1', '1', '0', '0', '0', '1', '1', '1', '1', '0', '0', '1', '0', '0', '1', '1', '1', '1', '1', '0', '0', '1', '1', '0', '0', '1', '0', '1', '1', '1', '1', '1', '0', '0', '0', '1', '1', '0', '1', '0', '1', '0', '0', '1', '0', '1', '1', '0', '0', '1', '0', '1', '0', '0', '0', '0', '0', '0', '1', '0', '1', '1', '1', '0', '1', '1', '1', '1', '1', '1', '0', '1', '1', '1', '0', '1', '1', '1', '1', '0', '0', '0', '1', '0', '1', '1', '0', '1', '1', '0', '0', '1', '0', '0', '1', '0', '0', '1', '1', '1', '1', '1', '0', '0', '0', '0', '0', '1', '0', '1', '0', '0', '0', '0', '1', '1', '1', '0', '0', '1', '0', '1', '1', '0', '0', '0', '0', '0', '0', '1', '0', '0', '0', '0', '0', '1', '0', '0', '1', '0', '0', '1', '0', '0', '1', '1', '0', '0', '1', '0', '0', '0', '1', '1', '0', '1', '1', '1', '1', '1', '1', '0', '1', '1', '1', '1', '0', '1', '1', '1', '1', '0', '1', '0', '1', '0', '0', '0', '1', '0', '0', '0', '0', '0', '0', '0', '1', '0', '0', '1', '0', '1', '0', '1', '0', '0', '0', '1', '1', '1', '1', '0', '1', '1', '0', '1', '0', '0', '0', '0', '0', '0', '1', '0', '0', '1', '0', '0', '0', '1', '1', '0', '1', '0', '1', '1', '0', '0', '1', '0', '0', '0', '1', '1', '0', '1', '0', '1', '1', '0', '0', '1', '1', '1', '0', '1', '0', '0', '0', '0', '0', '0', '1', '0', '1', '0', '0', '0', '0', '0', '0', '0', '1', '0', '1', '0', '1', '0', '1', '1', '1', '1', '0', '1', '1', '1', '0', '1', '1', '0', '0', '0', '1', '0', '0', '0', '0', '0', '1', '1', '1', '1', '0', '1', '0', '0', '1', '0', '0', '1', '0', '1', '1', '0', '1', '0', '0', '0', '0', '1', '1', '0', '0', '1', '0', '1', '1', '1', '1', '0', '0', '0', '0', '1', '1', '0', '0', '0', '1', '1', '0', '0', '1', '0', '0', '0', '1', '0', '0', '0', '0', '0', '0', '1', '0', '1', '0', '0', '0', '0', '0', '0', '0', '1', '0', '0', '0', '0', '0', '0', '0', '1', '1', '0', '0', '1', '1', '1', '1', '0', '0', '1', '0', '0', '0', '1', '0', '1', '0', '1', '0', '0', '0', '1', '1', '0', '0', '1', '0', '1', '0', '0', '1', '1', '1', '0', '0', '1', '1', '1', '1', '1', '0', '0', '0', '0', '0', '0', '0', '0', '1', '0', '0', '1', '1', '1', '1', '0', '0', '0', '0', '0', '0', '1', '0', '0', '1'])
    }

    #[test]
    fn test_get_algorithm_index() {
        let image = Image::from_str(
            r#"#..#.
#....
##..#
..#..
..###"#,
        );
        assert_eq!(image.get_algorithm_index(3, 3), 34)
    }

    #[test]
    fn test_part01() {
        let input = read("./input/day_20.txt");
        assert_eq!(part01(&input), 0)
    }
    // < 5814
    // < 5686

    #[test]
    fn test_part02() {
        let input = read("./input/day_20.txt");
        assert_eq!(part02(&input), 0)
    }
}
