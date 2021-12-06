use std::collections::HashMap;

use crate::{read_comma_separated_number_input, Solution};

#[derive(Debug, Clone)]
struct CountdownState {
    new: u64,
    day7: u64,
    day6: u64,
    day5: u64,
    day4: u64,
    day3: u64,
    day2: u64,
    day1: u64,
    day0: u64,
}

impl CountdownState {
    fn empty() -> CountdownState {
        CountdownState {
            new: 0,
            day7: 0,
            day6: 0,
            day5: 0,
            day4: 0,
            day3: 0,
            day2: 0,
            day1: 0,
            day0: 0,
        }
    }

    fn new(start_states: Vec<u32>) -> CountdownState {
        let mut counter = HashMap::new();

        start_states.iter().for_each(|day| {
            let count = counter.entry(day).or_insert(0u64);
            *count += 1;
        });

        counter
            .iter()
            .fold(CountdownState::empty(), |mut state, (&&day, &amount)| {
                match day {
                    0 => state.day0 = amount,
                    1 => state.day1 = amount,
                    2 => state.day2 = amount,
                    3 => state.day3 = amount,
                    4 => state.day4 = amount,
                    5 => state.day5 = amount,
                    _ => state.day6 = amount,
                }
                state
            })
    }

    fn simulate_day(state: CountdownState) -> CountdownState {
        CountdownState {
            new: state.day0,
            day7: state.new,
            day6: state.day7 + state.day0,
            day5: state.day6,
            day4: state.day5,
            day3: state.day4,
            day2: state.day3,
            day1: state.day2,
            day0: state.day1,
        }
    }

    fn simulate(&self, days: u16) -> CountdownState {
        (0..days).fold(self.clone(), |state, _| CountdownState::simulate_day(state))
    }

    fn sum(&self) -> u64 {
        self.day0
            + self.day1
            + self.day2
            + self.day3
            + self.day4
            + self.day5
            + self.day6
            + self.day7
            + self.new
    }
}

fn part01(input: &CountdownState) -> u64 {
    input.simulate(80).sum()
}

fn part02(input: &CountdownState) -> u64 {
    input.simulate(256).sum()
}

pub fn day_06() -> Solution {
    let input = CountdownState::new(read_comma_separated_number_input("./input/day_06.txt"));
    Solution::new(6, part01(&input), part02(&input))
}
