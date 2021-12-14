use std::{collections::HashMap, str::Chars};

use crate::{map_pair, map_snd, read, Solution};

type InsertionRules = HashMap<(char, char), char>;

fn pair(mut chars: Chars) -> (char, char) {
    (chars.next().unwrap(), chars.next().unwrap())
}

fn parse_insertion_rules(rules: &str) -> InsertionRules {
    rules
        .lines()
        .map(|rule| {
            let (from, mut to) = map_pair(|s| s.chars(), rule.split_once(" -> ").unwrap());
            (pair(from), to.next().unwrap())
        })
        .collect()
}

fn chunk(template: &str) -> HashMap<(char, char), usize> {
    let mut chunks = HashMap::new();
    let mut previous: Option<char> = None;

    for c in template.chars() {
        if let Some(p) = previous {
            *chunks.entry((p, c)).or_insert(0) += 1;
        }
        previous = Some(c);
    }

    chunks
}

fn step(input: &String, steps: usize) -> usize {
    let (template, rules) = map_snd(parse_insertion_rules, input.split_once("\n\n").unwrap());

    // All characters except the first and last one are counted twice. Therefore I need to add one to both.
    let first = template.chars().next().unwrap();
    let last = template.chars().last().unwrap();

    let (min, max) = (0..steps)
        .into_iter()
        .fold(chunk(template), |polymer_count, _| {
            let mut frequency = polymer_count.clone();

            for ((a, b), count) in polymer_count.into_iter() {
                if let Some(&c) = rules.get(&(a, b)) {
                    *frequency.entry((a, c)).or_insert(0) += count;
                    *frequency.entry((c, b)).or_insert(0) += count;
                    *frequency.entry((a, b)).or_insert(0) -= count;
                } else {
                    *frequency.entry((a, b)).or_insert(0) += count;
                }
            }

            frequency
        })
        .into_iter()
        .fold(
            // Add one to the last and first chars from the template
            vec![(first, 1), (last, 1)]
                .into_iter()
                .collect::<HashMap<char, usize>>(),
            |mut letters, ((a, b), count)| {
                *letters.entry(a).or_insert(0) += count;
                *letters.entry(b).or_insert(0) += count;
                letters
            },
        )
        .values()
        .fold((usize::MAX, 0), |(min, max), &count| {
            (min.min(count), max.max(count))
        });

    (max - min) / 2
}

/* Solutions */

fn part01(input: &String) -> usize {
    step(input, 10)
}

fn part02(input: &String) -> usize {
    step(input, 40)
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
    fn test_part01() {
        let input = read("./input/day_14.txt");
        assert_eq!(part01(&input), 2657)
    }

    #[test]
    fn test_part02() {
        let input = read("./input/day_14.txt");
        assert_eq!(part02(&input), 2911561572630)
    }
}
