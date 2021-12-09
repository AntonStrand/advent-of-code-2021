use std::env;

use advent_of_code::{
    day_01, day_02, day_03, day_04, day_05, day_06, day_07, day_08, day_09, Solution,
};

fn main() {
    let days = [
        day_01, day_02, day_03, day_04, day_05, day_06, day_07, day_08, day_09,
    ];

    match parse_arg_day() {
        Some(day) => Solution::view(solve_day(&days, day)),
        None => Solution::view_all(days.map(solve).to_vec()),
    }
}

fn parse_arg_day() -> Option<usize> {
    env::args()
        .collect::<Vec<String>>()
        .get(1)
        .map(|day| day.parse::<usize>().unwrap() - 1)
}

fn solve_day<F: Fn() -> Solution>(days: &[F], day: usize) -> Solution {
    days.get(day)
        .map(solve)
        .unwrap_or_else(|| panic!("You need to provide a day between 1 - {}", days.len()))
}

fn solve<F: Fn() -> Solution>(solve: F) -> Solution {
    solve()
}
