use std::collections::{HashMap, HashSet};

use crate::{is_upper, map_both, read_lines, Solution};

fn add_path(mut map: HashMap<Cave, Vec<Cave>>, from: &Cave, to: &Cave) -> HashMap<Cave, Vec<Cave>> {
    if to != &Cave::Start {
        let edges = map.entry(from.to_owned()).or_insert(vec![]);
        edges.push(to.to_owned());
    }
    map
}

fn create_map(lines: &Vec<String>) -> HashMap<Cave, Vec<Cave>> {
    let mut map = HashMap::new();

    for line in lines {
        let (from, to) = map_both(Cave::new, line.split_once("-").unwrap());
        map = add_path(map, &to, &from);
        map = add_path(map, &from, &to);
    }

    map
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
    path: Vec<Cave>,
    has_duplicate: bool,
    allow_visit_again: bool,
}

impl Path {
    fn new(allow_visit_again: bool) -> Path {
        Path {
            visited: HashSet::new(),
            path: vec![Cave::Start],
            has_duplicate: false,
            allow_visit_again,
        }
    }

    fn last(&self) -> Cave {
        self.path.last().unwrap().to_owned()
    }

    fn append(&self, cave: &Cave) -> Path {
        let mut path = self.path[..].to_vec();
        path.push(cave.to_owned());

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
            path,
            has_duplicate,
            allow_visit_again: self.allow_visit_again,
        }
    }

    fn allow_adjacent(path: &Path, cave: &Cave) -> bool {
        if path.allow_visit_again {
            !path.has_duplicate || !path.visited.contains(cave)
        } else {
            !path.visited.contains(cave)
        }
    }
}

fn all_paths(allow_visit_again: bool, map: HashMap<Cave, Vec<Cave>>) -> Vec<Path> {
    let mut paths = vec![];
    let mut stack = Vec::new();
    stack.push(Path::new(allow_visit_again));

    while !stack.is_empty() {
        let path = stack.pop().unwrap();
        let cave = path.last();

        if cave == Cave::End {
            paths.push(path);
        } else {
            for adjacent in map.get(&cave).unwrap_or(&vec![]) {
                if Path::allow_adjacent(&path, &adjacent) {
                    stack.push(path.append(adjacent));
                }
            }
        }
    }

    paths
}

/* Solutions */

fn part01(input: &Vec<String>) -> usize {
    all_paths(false, create_map(input)).len()
}

fn part02(input: &Vec<String>) -> usize {
    all_paths(true, create_map(input)).len()
}

pub fn day_12() -> Solution {
    let input = read_lines("./input/day_12.txt");
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
        let input = read_lines("./input/day_12.txt");
        assert_eq!(part01(&input), 4691)
    }

    #[test]
    #[ignore]
    fn test_part02() {
        let input = read_lines("./input/day_12.txt");
        assert_eq!(part02(&input), 140718)
    }
}
