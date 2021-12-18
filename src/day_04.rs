use crate::{read, to_columns, Solution};

#[derive(Debug)]
struct Board {
    board: Vec<Vec<u8>>,
    all: Vec<u8>,
}

impl Board {
    fn new(input: &str) -> Board {
        let rows = input
            .lines()
            .map(|line| {
                line.split(" ")
                    .filter_map(|c| {
                        if c.is_empty() {
                            None
                        } else {
                            Some(c.parse().unwrap())
                        }
                    })
                    .collect::<Vec<u8>>()
            })
            .collect::<Vec<Vec<u8>>>();

        let cols = to_columns(&rows);

        let all = rows.iter().flatten().map(|x| *x).collect();

        Board {
            board: rows.into_iter().chain(cols.into_iter()).collect(),
            all: all,
        }
    }

    fn is_winner(&self, numbers: &Vec<u8>) -> bool {
        self.board
            .iter()
            .any(|xs| xs.iter().filter(|&x| numbers.contains(x)).count() == 5)
    }

    fn get_winning_score(&self, numbers: &Vec<u8>) -> Option<u32> {
        if self.is_winner(numbers) {
            Some(
                self.all
                    .iter()
                    .filter(|x| !numbers.contains(x))
                    .map(|&x| x as u32)
                    .sum(),
            )
        } else {
            None
        }
    }
}

fn read_input() -> (Vec<Board>, Vec<u8>) {
    let input = read("./input/day_04.txt");
    let (numbers, boards) = input.split_once("\n\n").unwrap();

    let numbers: Vec<u8> = numbers.split(",").map(|n| n.parse().unwrap()).collect();
    let boards: Vec<Board> = boards.split("\n\n").map(Board::new).collect();

    (boards, numbers)
}

fn get_winning_boards((boards, numbers): &(Vec<Board>, Vec<u8>)) -> Vec<(usize, u32, u8)> {
    let mut winners: Vec<(usize, u32, u8)> = boards
        .iter()
        .filter_map(|board| play(board, numbers.clone()))
        .collect();

    winners.sort();
    winners
}

fn play(board: &Board, numbers: Vec<u8>) -> Option<(usize, u32, u8)> {
    let mut i = 5;
    let mut drawn_numbers = numbers[..i].to_vec();

    loop {
        match board.get_winning_score(&drawn_numbers) {
            Some(score) => break Some((i, score, drawn_numbers.pop().unwrap())),
            None => {
                if i == numbers.len() {
                    break None;
                }
                let next_number = *numbers.get(i).unwrap();
                drawn_numbers.push(next_number);
                i += 1;
            }
        }
    }
}

fn part01(input: &(Vec<Board>, Vec<u8>)) -> u32 {
    let (_, score, winning_number) = *get_winning_boards(input).first().unwrap();

    score * (winning_number as u32)
}

fn part02(input: &(Vec<Board>, Vec<u8>)) -> u32 {
    let (_, score, winning_number) = get_winning_boards(input).pop().unwrap();

    score * (winning_number as u32)
}

pub fn day_04() -> Solution {
    let input = read_input();
    let timer = std::time::Instant::now();
    Solution::new(4, part01(&input), part02(&input), timer.elapsed())
}

#[cfg(test)]
mod tests {
    // Note this useful idiom: importing names from outer (for mod tests) scope.
    use super::*;

    #[test]
    fn test_play_should_find_winner() {
        let board = Board::new(
            r#"14 21 17 24  4
            10 16 15  9 19
            18  8 23 26 20
            22 11 13  6  5
            2  0 12  3  7"#,
        );

        let numbers = vec![7, 4, 9, 5, 11, 17, 23, 2, 0, 14, 21, 24];

        assert_eq!(play(&board, numbers), Some((12, 188, 24)));
    }

    #[test]
    fn test_play_should_return_none_if_no_winner() {
        let board = Board::new(
            r#"14 21 1 24 5
    10 16 15  9 19
    18  8 23 26 20
    22 11 13  6  5
    2  0 12  3  7"#,
        );

        let numbers = vec![7, 4, 9, 5, 11, 17, 23, 2, 0, 14, 21, 24];

        assert_eq!(play(&board, numbers), None);
    }
}
