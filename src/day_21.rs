use crate::{read_lines, Solution};

#[derive(Debug)]
struct Player {
    space: u32,
    score: u32,
}

impl Player {
    fn new(space: u32) -> Player {
        Player { space, score: 0 }
    }

    fn play_turn(&mut self, rolls: u32) -> u32 {
        let steps = rolls * 3 + 6;
        self.space = ((self.space + steps - 1) % 10) + 1;
        self.score += self.space;
        rolls + 3
    }
}

/* Solutions */

fn part01(input: &Vec<String>) -> u32 {
    let start_positions: Vec<u32> = input
        .into_iter()
        .map(|line| line[line.len() - 1..].parse().unwrap())
        .collect();

    let mut p1 = Player::new(start_positions[0]);
    let mut p2 = Player::new(start_positions[1]);

    let mut rolls = 0;

    let loser = loop {
        rolls = p1.play_turn(rolls);
        if p1.score >= 1000 {
            break p2;
        }

        rolls = p2.play_turn(rolls);
        if p2.score >= 1000 {
            break p1;
        }
    };

    rolls * loser.score
}

fn part02(_input: &Vec<String>) -> &str {
    "NA"
}

pub fn day_21() -> Solution {
    let input = read_lines("./input/day_21.txt");
    let timer = std::time::Instant::now();
    Solution::new(21, part01(&input), part02(&input), timer.elapsed())
}

/* Tests */

#[cfg(test)]
mod tests {
    // Note this useful idiom: importing names from outer (for mod tests) scope.
    use super::*;

    #[test]
    fn test_part01() {
        let input = read_lines("./input/day_21.txt");
        assert_eq!(part01(&input), 925605)
    }

    #[test]
    fn test_part02() {
        let input = read_lines("./input/day_21.txt");
        assert_eq!(part02(&input), "NA")
    }
}
