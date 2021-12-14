use std::collections::HashMap;

use crate::{map_both, map_snd, read, Solution};

type InsertionRules = HashMap<String, String>;

fn parse_insertion_rules(rules: &str) -> InsertionRules {
    rules
        .lines()
        .map(|rule| map_both(String::from, rule.split_once(" -> ").unwrap()))
        .collect()
}

fn chunk(polymer: &String) -> Vec<(char, char)> {
    let mut chunks = vec![];
    let mut previous: Option<char> = None;

    for c in polymer.chars() {
        if let Some(p) = previous {
            chunks.push((p, c));
        }
        previous = Some(c);
    }

    chunks
}

fn step(insertion_rules: &InsertionRules, polymer: &String) -> String {
    chunk(polymer)
        .iter()
        .enumerate()
        .map(|(i, (a, b))| {
            format!(
                "{}{}",
                insertion_rules
                    .get(&format!("{}{}", a, b))
                    .map(|insert| (format!("{}{}", a, insert)))
                    .unwrap_or(format!("{}{}", a, b))
                    .to_owned(),
                if i == polymer.len() - 2 {
                    b.to_string()
                } else {
                    "".to_string()
                }
            )
        })
        .collect::<Vec<String>>()
        .join("")
}

fn count_chars(input: String) -> HashMap<char, usize> {
    let mut counter = HashMap::new();

    for c in input.chars() {
        let count = counter.entry(c).or_insert(0);
        *count += 1;
    }

    counter
}

fn find_min_max(input: String) -> (usize, usize) {
    count_chars(input)
        .into_iter()
        .fold((usize::MAX, usize::MIN), |(min, max), (c, count)| {
            (min.min(count), max.max(count))
        })
}

fn step_n(times: usize, template: String, insertion_rules: &InsertionRules) -> String {
    (0..times).fold(template, |polymer, _| step(&insertion_rules, &polymer))
}

/* Solutions */

fn part01(input: &String) -> usize {
    let (template, insertion_rules) =
        map_snd(parse_insertion_rules, input.split_once("\n\n").unwrap());

    let (min, max) = find_min_max(step_n(10, template.to_string(), &insertion_rules));

    max - min
}

fn part02(_input: &String) -> usize {
    0
}

pub fn day_14() -> Solution {
    let input = read("./input/day_14.txt");
    let timer = std::time::Instant::now();
    Solution::new(14, part01(&input), part02(&input), timer.elapsed())
}

/* Tests */

#[cfg(test)]
mod tests {
    // Note this useful idiom: importing names from outer (for mod tests) scope.
    use super::*;

    #[test]
    fn test_parse_insertion_rules() {
        let input = "CH -> B\nHH -> N\nCB -> H";

        let mut expected = InsertionRules::new();
        expected.insert(String::from("CH"), String::from("B"));
        expected.insert(String::from("HH"), String::from("N"));
        expected.insert(String::from("CB"), String::from("H"));

        assert_eq!(parse_insertion_rules(input), expected)
    }

    #[test]
    #[ignore]
    fn test_part01() {
        let input = read("./input/day_14.txt");
        assert_eq!(part01(&input), 4691)
    }

    #[test]
    #[ignore]
    fn test_part02() {
        let input = read("./input/day_14.txt");
        assert_eq!(part02(&input), 140718)
    }
}
