use crate::{map_fst, read, Solution};

type Algorithm = [bool; 512];

#[derive(Clone)]
struct Image {
    pixels: Vec<Vec<bool>>,
    size: usize,
    lit: u32,
    default_pixel: bool,
    algorithm: Algorithm,
}

impl Image {
    fn new(size: usize, default_pixel: bool, algorithm: Algorithm) -> Image {
        Image {
            pixels: vec![vec![default_pixel; size]; size],
            size,
            lit: 0,
            default_pixel,
            algorithm,
        }
    }

    fn from_str(input: &str) -> Image {
        let (algorithm, image_data) = map_fst(parse_algorithm, input.split_once("\n\n").unwrap());

        Image {
            pixels: image_data
                .lines()
                .map(|line| line.chars().map(|c| is_lit(c)).collect())
                .collect(),
            size: image_data.lines().count(),
            lit: 0, // No need to set this now since it will be reset at when enhancing
            default_pixel: false,
            algorithm,
        }
    }

    fn get_algorithm_index(&self, x: usize, y: usize) -> usize {
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
        .map(|(x, y)| {
            *self
                .pixels
                .get(y)
                .and_then(|row| row.get(x))
                .unwrap_or(&self.default_pixel)
        })
        .iter()
        .fold(String::new(), |s, b| format!("{}{}", s, *b as u8));

        usize::from_str_radix(&binary, 2).unwrap()
    }

    fn insert(&mut self, x: usize, y: usize, pixel: bool) {
        self.pixels[y][x] = pixel;
        self.lit += pixel as u32
    }

    fn enhance(&self) -> Image {
        let mut output = Image::new(self.size + 2, !self.default_pixel, self.algorithm);

        for y in 0..output.size {
            for x in 0..output.size {
                let i = self.get_algorithm_index(x - 1, y - 1);
                output.insert(x, y, self.algorithm[i]);
            }
        }

        output
    }
}

fn is_lit(c: char) -> bool {
    c == '#'
}

fn parse_algorithm(input: &str) -> Algorithm {
    let mut algo = [false; 512];

    for (i, c) in input.chars().enumerate() {
        algo[i] = is_lit(c);
    }

    algo
}

/* Solutions */

fn part01(image: &Image) -> u32 {
    image.enhance().enhance().lit
}

fn part02(image: Image) -> u32 {
    (0..50)
        .into_iter()
        .fold(image, |image, _| image.enhance())
        .lit
}

pub fn day_20() -> Solution {
    let input = Image::from_str(&read("./input/day_20.txt"));
    let timer = std::time::Instant::now();
    Solution::new(20, part01(&input), part02(input), timer.elapsed())
}

/* Tests */

#[cfg(test)]
mod tests {
    // Note this useful idiom: importing names from outer (for mod tests) scope.
    use super::*;

    #[test]
    fn test_get_algorithm_index() {
        let image = Image::from_str(
            r#"..#.#..#####.#.#.#.###.##.....###.##.#..###.####..#####..#....#..#..##..###..######.###...####..#..#####..##..#.#####...##.#.#..#.##..#.#......#.###.######.###.####...#.##.##..#..#..#####.....#.#....###..#.##......#.....#..#..#..##..#...##.######.####.####.#.#...#.......#..#.#.#...####.##.#......#..#...##.#.##..#...##.#.##..###.#......#.#.......#.#.#.####.###.##...#.....####.#..#..#.##.#....##..#.####....##...##..#...#......#.#.......#.......##..####..#...#.#.#...##..#.#..###..#####........#..####......#..#

#..#.
#....
##..#
..#..
..###"#,
        );
        assert_eq!(image.get_algorithm_index(2, 2), 34)
    }

    #[test]
    fn test_part01() {
        let input = Image::from_str(&read("./input/day_20.txt"));
        assert_eq!(part01(&input), 5432)
    }

    #[test]
    fn test_part02() {
        let input = Image::from_str(&read("./input/day_20.txt"));
        assert_eq!(part02(input), 16016)
    }
}
