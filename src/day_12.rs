use std::collections::{HashMap, HashSet};

use crate::{is_upper, map_pair, read_lines, Solution};

type CaveSystem = HashMap<Cave, Vec<Cave>>;

fn add_path(mut case_system: CaveSystem, from: &Cave, to: &Cave) -> CaveSystem {
    if to != &Cave::Start {
        let edges = case_system.entry(from.to_owned()).or_insert(vec![]);
        edges.push(to.to_owned());
    }
    case_system
}

fn create_map(lines: Vec<String>) -> CaveSystem {
    let mut case_system = HashMap::new();

    for line in lines {
        let (from, to) = map_pair(Cave::new, line.split_once("-").unwrap());
        case_system = add_path(case_system, &to, &from);
        case_system = add_path(case_system, &from, &to);
    }

    case_system
}

#[derive(Debug, PartialEq, Eq, Hash, Clone)]
enum Cave {
    Start,
    End,
    Big(String),
    Small(String),
}

impl Cave {
    fn new(cave: &str) -> Cave {
        match cave {
            "start" => Cave::Start,
            "end" => Cave::End,
            name if is_upper(name) => Cave::Big(name.to_string()),
            name => Cave::Small(name.to_string()),
        }
    }

    fn is_small(&self) -> bool {
        match self {
            &Cave::Small(_) => true,
            _ => false,
        }
    }
}

#[derive(Debug, Clone)]
struct Path {
    visited: HashSet<Cave>,
    current: Cave,
    has_duplicate: bool,
    allow_visit_again: bool,
}

impl Path {
    fn new(allow_visit_again: bool) -> Path {
        Path {
            visited: HashSet::new(),
            current: Cave::Start,
            has_duplicate: false,
            allow_visit_again,
        }
    }

    fn current_cave(&self) -> &Cave {
        &self.current
    }

    fn append(&self, cave: &Cave) -> Path {
        let mut visited = self.visited.to_owned();

        if cave.is_small() {
            visited.insert(cave.to_owned());
        }

        let has_duplicate = if self.has_duplicate {
            self.has_duplicate
        } else {
            self.visited.contains(cave)
        };

        Path {
            visited,
            current: cave.to_owned(),
            has_duplicate,
            allow_visit_again: self.allow_visit_again,
        }
    }

    fn allows_adjacent(path: &Path, cave: &Cave) -> bool {
        if path.allow_visit_again {
            !path.has_duplicate || !path.visited.contains(cave)
        } else {
            !path.visited.contains(cave)
        }
    }

    fn has_reached_end(cave: &Cave) -> bool {
        cave == &Cave::End
    }
}

fn count_paths(allow_visit_again: bool, cave_system: &CaveSystem) -> usize {
    let mut paths = vec![];
    let mut stack = Vec::new();
    stack.push(Path::new(allow_visit_again));

    while !stack.is_empty() {
        let path = stack.pop().unwrap();
        let cave = path.current_cave();

        if Path::has_reached_end(cave) {
            paths.push(path);
        } else {
            for adjacent in cave_system.get(cave).unwrap_or(&vec![]) {
                if Path::allows_adjacent(&path, &adjacent) {
                    stack.push(path.append(adjacent));
                }
            }
        }
    }

    paths.len()
}

/* Solutions */

fn part01(cave_system: &CaveSystem) -> usize {
    count_paths(false, cave_system)
}

fn part02(cave_system: &CaveSystem) -> usize {
    count_paths(true, cave_system)
}

pub fn day_12() -> Solution {
    let input = create_map(read_lines("./input/day_12.txt"));
    let timer = std::time::Instant::now();
    Solution::new(12, part01(&input), part02(&input), timer.elapsed())
}

/* Tests */

#[cfg(test)]
mod tests {
    // Note this useful idiom: importing names from outer (for mod tests) scope.
    use super::*;

    #[test]
    fn test_part01() {
        let input = create_map(read_lines("./input/day_12.txt"));
        assert_eq!(part01(&input), 4691)
    }

    #[test]
    #[ignore]
    fn test_part02() {
        let input = create_map(read_lines("./input/day_12.txt"));
        assert_eq!(part02(&input), 140718)
    }
}
