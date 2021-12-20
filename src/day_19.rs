use crate::{read, Solution};

/* Solutions */

fn part01(_input: &String) -> &str {
    "NA"
}

fn part02(_input: &String) -> &str {
    "NA"
}

pub fn day_19() -> Solution {
    let input = read("./input/day_19.txt");
    let timer = std::time::Instant::now();
    Solution::new(19, part01(&input), part02(&input), timer.elapsed())
}

/* Tests */

#[cfg(test)]
mod tests {
    // Note this useful idiom: importing names from outer (for mod tests) scope.
    use super::*;

    #[test]
    fn test_part01() {
        let input = read("./input/day_19.txt");
        assert_eq!(part01(&input), "NA")
    }

    #[test]
    fn test_part02() {
        let input = read("./input/day_19.txt");
        assert_eq!(part02(&input), "NA")
    }
}
