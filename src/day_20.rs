use crate::{read, Solution};

/* Solutions */

fn part01(_input: &String) -> u32 {
    0
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
    fn test_part01() {
        let input = read("./input/day_20.txt");
        assert_eq!(part01(&input), 0)
    }

    #[test]
    fn test_part02() {
        let input = read("./input/day_20.txt");
        assert_eq!(part02(&input), 0)
    }
}
