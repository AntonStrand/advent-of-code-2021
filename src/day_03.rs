use crate::{read_lines, Solution};

fn one_is_most_common(input: &Vec<&str>) -> bool {
    let (ones, zeros): (Vec<&str>, Vec<&str>) = input.iter().partition(|x| **x == "1");
    ones.len() >= zeros.len()
}

fn most_common<'a>(input: &Vec<&str>) -> &'a str {
    if one_is_most_common(input) {
        "1"
    } else {
        "0"
    }
}

fn least_common<'a>(input: &Vec<&str>) -> &'a str {
    if one_is_most_common(input) {
        "0"
    } else {
        "1"
    }
}

fn to_decimal(x: String) -> u32 {
    isize::from_str_radix(x.as_str(), 2).unwrap() as u32
}

fn split_binaries<'a>(input: &'a Vec<String>) -> Vec<Vec<&'a str>> {
    input
        .iter()
        .map(|s| s.split("").filter(|c| !c.is_empty()).collect::<Vec<&str>>())
        .collect::<Vec<Vec<&str>>>()
}

fn get_columns<'a>(input: &Vec<Vec<&'a str>>) -> Vec<Vec<&'a str>> {
    let binary_len = input.get(0).unwrap().len();

    let mut columns = vec![];

    for x in 0..binary_len {
        let mut column = vec![];
        for y in 0..input.len() {
            column.push(input[y][x]);
        }
        columns.push(column);
    }

    columns
}

fn part01(input: &Vec<String>) -> u32 {
    let (gamma_rate, epsilon_rate) = get_columns(&split_binaries(input)).iter().fold(
        (String::from(""), String::from("")),
        |(gamma_rate, epsilon_rate), column| {
            (
                gamma_rate + most_common(&column),
                epsilon_rate + least_common(&column),
            )
        },
    );

    to_decimal(gamma_rate) * to_decimal(epsilon_rate)
}

fn get_rating<'a, F: Fn(&Vec<&str>) -> &'a str>(f: F, input: &Vec<Vec<&str>>) -> u32 {
    let mut binaries = input.clone();
    let mut i = 0;
    let mut j = 0;
    while binaries.len() > 1 {
        let x = f(get_columns(&binaries).get(i).unwrap());

        binaries = binaries
            .to_owned()
            .iter()
            .filter(|binary| binary.get(j).unwrap() == &x)
            .map(|b| b.to_owned())
            .collect();

        i = i + 1;
        j = j + 1;
    }

    to_decimal(binaries.get(0).unwrap().join(""))
}

fn part02(input: &Vec<String>) -> u32 {
    let parsed = split_binaries(input);

    let oxygen = get_rating(most_common, &parsed);
    let co2 = get_rating(least_common, &parsed);

    oxygen * co2
}

pub fn day_03() -> Solution {
    let input = read_lines("./input/day_03.txt");
    Solution::new(3, part01(&input), part02(&input))
}

#[cfg(test)]
mod tests {
    // Note this useful idiom: importing names from outer (for mod tests) scope.
    use super::*;

    #[test]
    fn test_get_columns() {
        let input = vec![
            vec!["0", "0", "1", "0", "0"],
            vec!["1", "1", "1", "1", "0"],
            vec!["1", "0", "1", "1", "0"],
        ];
        assert_eq!(
            get_columns(&input),
            vec![
                vec!["0", "1", "1"],
                vec!["0", "1", "0"],
                vec!["1", "1", "1"],
                vec!["0", "1", "1"],
                vec!["0", "0", "0"],
            ]
        );
    }
    #[test]
    fn test_part01() {
        let input = vec![
            String::from("00100"),
            String::from("11110"),
            String::from("10110"),
            String::from("10111"),
            String::from("10101"),
            String::from("01111"),
            String::from("00111"),
            String::from("11100"),
            String::from("10000"),
            String::from("11001"),
            String::from("00010"),
            String::from("01010"),
        ];
        assert_eq!(part01(&input), 198);
    }

    #[test]
    fn test_part02() {
        let input = vec![
            String::from("00100"),
            String::from("11110"),
            String::from("10110"),
            String::from("10111"),
            String::from("10101"),
            String::from("01111"),
            String::from("00111"),
            String::from("11100"),
            String::from("10000"),
            String::from("11001"),
            String::from("00010"),
            String::from("01010"),
        ];
        assert_eq!(part02(&input), 230);
    }
}
