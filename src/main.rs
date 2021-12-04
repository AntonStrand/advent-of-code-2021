use std::env;

use advent_of_code::{day_01, day_02, day_03, day_04, Solution};

fn main() {
    let days = [day_01, day_02, day_03, day_04];

    match parse_arg_day() {
        Some(day) => solve_day(&days, day),
        None => days.iter().for_each(solve),
    }
}

fn parse_arg_day() -> Option<usize> {
    env::args()
        .collect::<Vec<String>>()
        .get(1)
        .map(|day| day.parse::<usize>().unwrap() - 1)
}

fn solve_day<F: Fn() -> Solution>(days: &[F], day: usize) {
    days.get(day)
        .map(solve)
        .unwrap_or_else(|| println!("You need to provide a day between 1 - {}", days.len()))
}

fn solve<F: Fn() -> Solution>(solve: F) {
    println!("{}", solve());
}
