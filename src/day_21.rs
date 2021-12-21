use std::{collections::HashMap, hash::Hash};

use crate::{read_lines, Solution};

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
struct Player {
    position: u32,
    score: u32,
}

impl Player {
    fn new(space: u32) -> Player {
        Player {
            position: space,
            score: 0,
        }
    }

    fn roll_thrice(&mut self, rolls: u32) -> u32 {
        let steps = rolls * 3 + 6;
        self.position = ((self.position + steps - 1) % 10) + 1;
        self.score += self.position;
        rolls + 3
    }

    fn advance(&self, steps: u32) -> Player {
        let position = ((self.position + steps - 1) % 10) + 1;
        Player {
            position,
            score: self.score + position,
        }
    }
}

impl Hash for Player {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        (self.position, self.score).hash(state);
    }
}

const OUTCOMES_OCCURRENCE: [(u32, usize); 7] =
    [(3, 1), (4, 3), (5, 6), (6, 7), (7, 6), (8, 3), (9, 1)];

fn quantum_play(
    cache: &mut HashMap<(Player, Player), (usize, usize)>,
    current_player: Player,
    other_player: Player,
) -> (usize, usize) {
    if other_player.score >= 21 {
        return (0, 1);
    }
    // Re-use score if the players have already been in this stage
    if let Some(&score) = cache.get(&(current_player, other_player)) {
        return score;
    }

    let mut current_win_count = 0;
    let mut other_win_count = 0;

    // Calculate winnings based on die rolls and their occurrences.
    for (steps, occ) in OUTCOMES_OCCURRENCE {
        let (other_wins, current_wins) =
            quantum_play(cache, other_player, current_player.advance(steps));

        current_win_count += current_wins * occ;
        other_win_count += other_wins * occ;
    }

    // Save cache
    cache.insert(
        (current_player, other_player),
        (current_win_count, other_win_count),
    );

    (current_win_count, other_win_count)
}

fn parse_start_positions(input: Vec<String>) -> (u32, u32) {
    let start_positions: Vec<u32> = input
        .into_iter()
        .map(|line| line[line.len() - 1..].parse().unwrap())
        .collect();

    (start_positions[0], start_positions[1])
}

/* Solutions */

fn part01((start_p1, start_p2): &(u32, u32)) -> u32 {
    let mut p1 = Player::new(*start_p1);
    let mut p2 = Player::new(*start_p2);

    let mut rolls = 0;

    let loser = loop {
        rolls = p1.roll_thrice(rolls);
        if p1.score >= 1000 {
            break p2;
        }

        rolls = p2.roll_thrice(rolls);
        if p2.score >= 1000 {
            break p1;
        }
    };

    rolls * loser.score
}

fn part02((start_p1, start_p2): &(u32, u32)) -> usize {
    let (s1, s2) = quantum_play(
        &mut HashMap::new(),
        Player::new(*start_p1),
        Player::new(*start_p2),
    );

    s1.max(s2)
}

pub fn day_21() -> Solution {
    let input = parse_start_positions(read_lines("./input/day_21.txt"));
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
        let input = parse_start_positions(read_lines("./input/day_21.txt"));
        assert_eq!(part01(&input), 925605)
    }

    #[test]
    fn test_part02() {
        let input = parse_start_positions(read_lines("./input/day_21.txt"));
        assert_eq!(part02(&input), 486638407378784)
    }
}
