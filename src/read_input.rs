use crate::Direction;

fn read(path: &str) -> String {
    std::fs::read_to_string(&path).unwrap()
}

pub fn read_lines(path: &str) -> Vec<String> {
    read(path).lines().map(String::from).collect()
}

pub fn read_number_input(path: &str) -> Vec<u32> {
    read(path)
        .lines()
        .map(|n| n.parse::<u32>().unwrap())
        .collect()
}

pub fn read_directions(path: &str) -> Vec<Direction> {
    read(path).lines().map(Direction::from_str).collect()
}
