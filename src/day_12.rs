use std::collections::{HashMap, HashSet};

use crate::{read_lines, Solution};

fn is_big_cave(cave: &str) -> bool {
    cave == cave.to_uppercase()
}

fn is_small_cave(cave: &str) -> bool {
    !is_big_cave(cave)
}

fn add_path<'a>(
    mut map: HashMap<&'a str, Vec<&'a str>>,
    from: &'a str,
    to: &'a str,
) -> HashMap<&'a str, Vec<&'a str>> {
    if to != "start" {
        let edges = map.entry(from).or_insert(vec![]);
        edges.push(to);
    }
    map
}

fn create_map(lines: &Vec<String>) -> HashMap<&str, Vec<&str>> {
    let mut map = HashMap::new();

    for line in lines {
        let (from, to) = line.split_once("-").unwrap();
        map = add_path(map, to, from);
        map = add_path(map, from, to);
    }

    map
}

fn path_has_only_unique_small_caves(path: &Vec<&str>) -> bool {
    let mut caves = HashSet::new();
    for cave in path {
        if is_small_cave(cave) {
            if caves.contains(cave) {
                return false;
            } else {
                caves.insert(cave);
            }
        }
    }

    true
}

fn all_paths<'a, F: Fn(&Vec<&str>) -> bool>(
    allow_visit_again: F,
    map: HashMap<&'a str, Vec<&'a str>>,
) -> Vec<Vec<&'a str>> {
    let mut paths = vec![];
    let mut stack = Vec::new();
    stack.push(vec!["start"]);

    while !stack.is_empty() {
        let path = stack.pop().unwrap();
        let cave = *path.last().unwrap();

        if cave == "end" {
            paths.push(path);
        } else {
            for &adjacent in map.get(cave).unwrap_or(&vec![]) {
                if allow_visit_again(&path) || !path.contains(&adjacent) || is_big_cave(adjacent) {
                    let mut next_path = path.clone();
                    next_path.push(adjacent);
                    stack.push(next_path);
                }
            }
        }
    }

    paths
}

/* Solutions */

fn part01(input: &Vec<String>) -> u64 {
    all_paths(|_| false, create_map(input)).len() as u64
}

fn part02(input: &Vec<String>) -> u64 {
    all_paths(path_has_only_unique_small_caves, create_map(input)).len() as u64
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
