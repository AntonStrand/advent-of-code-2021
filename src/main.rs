use std::env;

use advent_of_code::{day_01, day_02, Solution};

fn main() {
    let args: Vec<String> = env::args().collect();
    let days = [day_01, day_02];

    if let Some(day) = args.get(1) {
        let index: usize = day.parse::<usize>().unwrap() - 1;
        days.get(index)
            .map(solve)
            .unwrap_or_else(|| println!("You need to provide a day between 1 - {}", days.len()))
    } else {
        days.iter().for_each(solve);
    }
}

fn solve<T>(solve: T)
where
    T: Fn() -> Solution,
{
    println!("{}", solve());
}
