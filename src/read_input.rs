use std::str::FromStr;

use crate::{unsafe_parse, Direction};

pub fn read(path: &str) -> String {
    std::fs::read_to_string(&path).unwrap()
}

pub fn read_lines(path: &str) -> Vec<String> {
    read(path).lines().map(String::from).collect()
}

pub fn read_number_input<T: FromStr>(path: &str) -> Vec<T> {
    read(path).lines().map(unsafe_parse).collect()
}

pub fn read_comma_separated_number_input<T: FromStr>(path: &str) -> Vec<T> {
    read(path).split(",").map(unsafe_parse).collect()
}

pub fn read_directions(path: &str) -> Vec<Direction> {
    read(path).lines().map(Direction::from_str).collect()
}
