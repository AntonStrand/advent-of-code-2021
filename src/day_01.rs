use crate::{read_number_input, Solution};

fn part01(input: &Vec<u16>) -> usize {
    input.windows(2).filter(|n| n[0] < n[1]).count()
}

fn part02(input: &Vec<u16>) -> usize {
    input.windows(4).filter(|n| n[0] < n[3]).count()
}

pub fn day_01() -> Solution {
    let input = read_number_input("./input/day_01.txt");
    let timer = std::time::Instant::now();
    Solution::new(1, part01(&input), part02(&input), timer.elapsed())
}

#[cfg(test)]
mod tests {
    // Note this useful idiom: importing names from outer (for mod tests) scope.
    use super::*;

    #[test]
    fn test_part01() {
        let input = read_number_input("./input/day_01.txt");
        assert_eq!(part01(&input), 1759)
    }

    #[test]
    fn test_part02() {
        let input = read_number_input("./input/day_01.txt");
        assert_eq!(part02(&input), 1805)
    }
}
