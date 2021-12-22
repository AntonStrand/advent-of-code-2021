use std::{collections::HashSet, ops::RangeInclusive};

use crate::{map_pair, read, unsafe_parse, Solution};

#[derive(Debug, PartialEq, Eq, Hash, Clone, Copy)]
struct Coordinate {
    x: isize,
    y: isize,
    z: isize,
}

#[derive(Debug, PartialEq)]
enum Instruction {
    On(HashSet<Coordinate>),
    Off(HashSet<Coordinate>),
}

impl Instruction {
    fn from_str(input: &str) -> Self {
        let (instruction, coord_ranges) = input.split_once(" ").unwrap();
        let ranges: Vec<RangeInclusive<isize>> = coord_ranges
            .split(",")
            .into_iter()
            .map(|range| map_pair(unsafe_parse, range[2..].split_once("..").unwrap()))
            .map(|(from, to)| from..=to)
            .collect();

        let mut coordinates = HashSet::new();

        for x in ranges[0].to_owned() {
            for y in ranges[1].to_owned() {
                for z in ranges[2].to_owned() {
                    coordinates.insert(Coordinate { x, y, z });
                }
            }
        }

        match instruction {
            "on" => Self::On(coordinates),
            "off" => Self::Off(coordinates),
            _ => unreachable!(),
        }
    }
}

fn reboot(instructions: Vec<Instruction>) -> HashSet<Coordinate> {
    let mut cubes_that_are_on = HashSet::new();

    for instruction in instructions {
        match instruction {
            Instruction::On(on) => cubes_that_are_on.extend(on),
            Instruction::Off(off) => {
                cubes_that_are_on = cubes_that_are_on.difference(&off).copied().collect();
            }
        }
    }

    cubes_that_are_on
}

/* Solutions */

fn part01(input: &String) -> usize {
    let instructions = input
        .lines()
        .take_while(|l| l.len() < 40) // Use the length of the line to only get the initialization procedure area. There are 10 in the test data and 20 in the real data therefor I can't use just take.
        .map(Instruction::from_str)
        .collect();

    reboot(instructions).len()
}

fn part02(_input: &String) -> &str {
    "NA"
}

pub fn day_22() -> Solution {
    let input = read("./input/day_22.txt");
    let timer = std::time::Instant::now();
    Solution::new(22, part01(&input), part02(&input), timer.elapsed())
}

/* Tests */

#[cfg(test)]
mod tests {
    // Note this useful idiom: importing names from outer (for mod tests) scope.
    use super::*;
    use std::iter::FromIterator;

    #[test]
    fn test_parse_on_instruction() {
        let instruction = Instruction::from_str("on x=10..12,y=10..12,z=10..12");
        println!("{:#?}", instruction);
        assert_eq!(
            instruction,
            Instruction::On(HashSet::from_iter(vec![
                Coordinate {
                    x: 10,
                    y: 10,
                    z: 10
                },
                Coordinate {
                    x: 10,
                    y: 10,
                    z: 11
                },
                Coordinate {
                    x: 10,
                    y: 10,
                    z: 12
                },
                Coordinate {
                    x: 10,
                    y: 11,
                    z: 10
                },
                Coordinate {
                    x: 10,
                    y: 11,
                    z: 11
                },
                Coordinate {
                    x: 10,
                    y: 11,
                    z: 12
                },
                Coordinate {
                    x: 10,
                    y: 12,
                    z: 10
                },
                Coordinate {
                    x: 10,
                    y: 12,
                    z: 11
                },
                Coordinate {
                    x: 10,
                    y: 12,
                    z: 12
                },
                Coordinate {
                    x: 11,
                    y: 10,
                    z: 10
                },
                Coordinate {
                    x: 11,
                    y: 10,
                    z: 11
                },
                Coordinate {
                    x: 11,
                    y: 10,
                    z: 12
                },
                Coordinate {
                    x: 11,
                    y: 11,
                    z: 10
                },
                Coordinate {
                    x: 11,
                    y: 11,
                    z: 11
                },
                Coordinate {
                    x: 11,
                    y: 11,
                    z: 12
                },
                Coordinate {
                    x: 11,
                    y: 12,
                    z: 10
                },
                Coordinate {
                    x: 11,
                    y: 12,
                    z: 11
                },
                Coordinate {
                    x: 11,
                    y: 12,
                    z: 12
                },
                Coordinate {
                    x: 12,
                    y: 10,
                    z: 10
                },
                Coordinate {
                    x: 12,
                    y: 10,
                    z: 11
                },
                Coordinate {
                    x: 12,
                    y: 10,
                    z: 12
                },
                Coordinate {
                    x: 12,
                    y: 11,
                    z: 10
                },
                Coordinate {
                    x: 12,
                    y: 11,
                    z: 11
                },
                Coordinate {
                    x: 12,
                    y: 11,
                    z: 12
                },
                Coordinate {
                    x: 12,
                    y: 12,
                    z: 10
                },
                Coordinate {
                    x: 12,
                    y: 12,
                    z: 11
                },
                Coordinate {
                    x: 12,
                    y: 12,
                    z: 12
                },
            ]))
        )
    }

    #[test]
    fn test_part01() {
        let input = read("./input/day_22.txt");
        assert_eq!(part01(&input), 644257)
    }

    #[test]
    fn test_part02() {
        let input = read("./input/day_22.txt");
        assert_eq!(part02(&input), "NA")
    }
}
