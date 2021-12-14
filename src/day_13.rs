use std::{
    collections::HashSet,
    fmt::{self, Display},
    hash::{Hash, Hasher},
};

use crate::{map_pair, map_snd, read, unsafe_parse, Solution};

fn hash_to_char_str(hash: u64) -> &'static str {
    match hash {
        3947155009679962619 => "A",
        12467216408455293969 => "B",
        9554965789071909136 => "C",
        14882858084180546402 => "E",
        12493310367934192866 => "F",
        783815720019195611 => "G",
        6867693118687141663 => "H",
        10981057960959392421 => "J",
        1429428095340992053 => "K",
        6440348562121363364 => "L",
        14519949062532219633 => "P",
        5425530575798285626 => "R",
        6876405955879760556 => "Z",
        _ => panic!("Not yet implemented: {}", hash),
    }
}
#[derive(Debug, PartialEq, Eq, Hash, Clone)]
struct Coordinate {
    x: u16,
    y: u16,
}

impl Coordinate {
    fn new(x: u16, y: u16) -> Coordinate {
        Coordinate { x, y }
    }

    fn fold(&self, instruction: &Instruction) -> Coordinate {
        match *instruction {
            Instruction::FoldX(line) => Coordinate::new(line - (self.x - line), self.y),
            Instruction::FoldY(line) => Coordinate::new(self.x, line - (self.y - line)),
        }
    }
}

#[derive(Debug, Clone, PartialEq)]
struct Paper {
    coordinates: HashSet<Coordinate>,
    width: u16,
    length: u16,
}

impl Paper {
    fn new(input: &str) -> Paper {
        let mut width = 0;
        let mut length = 0;
        let mut coordinates = HashSet::new();

        for coordinate in input.lines() {
            let (x, y) = map_pair(unsafe_parse, coordinate.split_once(",").unwrap());

            width = width.max(x);
            length = length.max(y);

            coordinates.insert(Coordinate::new(x, y));
        }

        Paper {
            coordinates,
            width,
            length,
        }
    }

    fn fold(&self, instruction: &Instruction) -> Paper {
        match *instruction {
            Instruction::FoldX(line) => Paper {
                coordinates: self.fold_where(|coordinate| coordinate.x > line, instruction),
                width: line - 1,
                length: self.length,
            },
            Instruction::FoldY(line) => Paper {
                coordinates: self.fold_where(|coordinate| coordinate.y > line, instruction),
                width: self.width,
                length: line - 1,
            },
        }
    }

    fn fold_where<F: Fn(&Coordinate) -> bool>(
        &self,
        predicate: F,
        instruction: &Instruction,
    ) -> HashSet<Coordinate> {
        let (to_be_folded, untouched): (Vec<Coordinate>, HashSet<Coordinate>) = map_snd(
            |untouched| untouched.into_iter().collect(),
            self.to_owned().coordinates.into_iter().partition(predicate),
        );

        let folded: HashSet<Coordinate> = to_be_folded
            .into_iter()
            .map(|coordinate| coordinate.fold(instruction))
            .collect();

        untouched.union(&folded).map(|c| c.to_owned()).collect()
    }

    fn to_letters(&self) -> String {
        let mut letters = vec![];

        for offset in (0..self.width + 1).step_by(5) {
            let mut letter = [[" "; 4]; 6];
            let mut hasher = std::collections::hash_map::DefaultHasher::new();

            for y in 0..6 {
                for x in 0..4 {
                    letter[y][x] = if self
                        .coordinates
                        .contains(&Coordinate::new(x as u16 + offset, y as u16))
                    {
                        "#"
                    } else {
                        "."
                    }
                }
            }

            letter.map(|row| row.join("")).join("").hash(&mut hasher);
            letters.push(hash_to_char_str(hasher.finish()));
        }

        letters.join("")
    }
}

impl Display for Paper {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut grid = vec![];
        for y in 0..self.length + 1 {
            let mut row = vec![];
            for x in 0..self.width + 1 {
                if self.coordinates.contains(&Coordinate::new(x, y)) {
                    row.push("#")
                } else {
                    row.push(" ")
                }
            }
            grid.push(row.join(""));
        }

        write!(f, "{}", grid.join("\n"))
    }
}

#[derive(Debug)]
enum Instruction {
    FoldX(u16),
    FoldY(u16),
}

impl Instruction {
    fn new(input: &str) -> Instruction {
        let (_, line) = map_snd(unsafe_parse, input.split_once("=").unwrap());

        if input.contains("x") {
            Instruction::FoldX(line)
        } else {
            Instruction::FoldY(line)
        }
    }
}

fn parse(input: String) -> (Paper, Vec<Instruction>) {
    let (coordinates, instruction) = input.split_once("\n\n").unwrap();
    (
        Paper::new(coordinates),
        instruction.lines().map(Instruction::new).collect(),
    )
}

/* Solutions */

fn part01((paper, instructions): &(Paper, Vec<Instruction>)) -> String {
    paper
        .fold(instructions.first().unwrap())
        .coordinates
        .len()
        .to_string()
}

fn part02((paper, instructions): &(Paper, Vec<Instruction>)) -> String {
    instructions
        .iter()
        .fold(paper.to_owned(), |p, instruction| p.fold(instruction))
        .to_letters()
}

pub fn day_13() -> Solution {
    let input = parse(read("./input/day_13.txt"));
    let timer = std::time::Instant::now();
    Solution::new(13, part01(&input), part02(&input), timer.elapsed())
}

/* Tests */

#[cfg(test)]
mod tests {

    // Note this useful idiom: importing names from outer (for mod tests) scope.
    use super::*;

    #[test]
    fn test_fold_y() {
        let paper = Paper::new(
            r#"6,10
0,14
9,10
0,3
10,4
4,11
6,0
6,12
4,1
0,13
10,12
3,4
3,0
8,4
1,10
2,14
8,10
9,0"#,
        );
        let mut expected_coordinates = HashSet::new();

        expected_coordinates.insert(Coordinate::new(0, 0));
        expected_coordinates.insert(Coordinate::new(2, 0));
        expected_coordinates.insert(Coordinate::new(3, 0));
        expected_coordinates.insert(Coordinate::new(6, 0));
        expected_coordinates.insert(Coordinate::new(9, 0));
        expected_coordinates.insert(Coordinate::new(0, 1));
        expected_coordinates.insert(Coordinate::new(4, 1));
        expected_coordinates.insert(Coordinate::new(6, 2));
        expected_coordinates.insert(Coordinate::new(10, 2));
        expected_coordinates.insert(Coordinate::new(0, 3));
        expected_coordinates.insert(Coordinate::new(4, 3));
        expected_coordinates.insert(Coordinate::new(1, 4));
        expected_coordinates.insert(Coordinate::new(3, 4));
        expected_coordinates.insert(Coordinate::new(6, 4));
        expected_coordinates.insert(Coordinate::new(8, 4));
        expected_coordinates.insert(Coordinate::new(9, 4));
        expected_coordinates.insert(Coordinate::new(10, 4));

        let expected = Paper {
            coordinates: expected_coordinates,
            width: 10,
            length: 6,
        };

        assert_eq!(paper.fold(&Instruction::FoldY(7)), expected)
    }

    #[test]
    fn test_fold_twice() {
        let paper = Paper::new(
            r#"6,10
0,14
9,10
0,3
10,4
4,11
6,0
6,12
4,1
0,13
10,12
3,4
3,0
8,4
1,10
2,14
8,10
9,0"#,
        );
        let mut expected_coordinates = HashSet::new();

        expected_coordinates.insert(Coordinate::new(0, 0));
        expected_coordinates.insert(Coordinate::new(1, 0));
        expected_coordinates.insert(Coordinate::new(2, 0));
        expected_coordinates.insert(Coordinate::new(3, 0));
        expected_coordinates.insert(Coordinate::new(4, 0));
        expected_coordinates.insert(Coordinate::new(0, 1));
        expected_coordinates.insert(Coordinate::new(4, 1));
        expected_coordinates.insert(Coordinate::new(0, 2));
        expected_coordinates.insert(Coordinate::new(4, 2));
        expected_coordinates.insert(Coordinate::new(0, 3));
        expected_coordinates.insert(Coordinate::new(4, 3));
        expected_coordinates.insert(Coordinate::new(0, 4));
        expected_coordinates.insert(Coordinate::new(1, 4));
        expected_coordinates.insert(Coordinate::new(2, 4));
        expected_coordinates.insert(Coordinate::new(3, 4));
        expected_coordinates.insert(Coordinate::new(4, 4));

        let expected = Paper {
            coordinates: expected_coordinates,
            width: 4,
            length: 6,
        };

        let actual = paper
            .fold(&Instruction::FoldY(7))
            .fold(&Instruction::FoldX(5));

        println!("{}", actual);

        assert_eq!(actual, expected)
    }

    // #[test]
    // #[ignore]
    // fn test_part02() {
    //     let input = read("./input/day_13.txt");
    //     assert_eq!(part02(&input), 140718)
    // }
}
