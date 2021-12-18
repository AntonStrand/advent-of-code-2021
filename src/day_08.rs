use crate::{read_lines, Solution};

fn is_simple_digit(digit: &&str) -> bool {
    match digit.len() {
        2 | 3 | 4 | 7 => true,
        _ => false,
    }
}

fn count_simple_digits(segment: &String) -> usize {
    let (_, digits) = segment.split_once("|").unwrap();
    digits
        .split(" ")
        .into_iter()
        .filter(is_simple_digit)
        .count()
}

fn find_number_by_len(len: usize, signals: &str) -> Vec<&str> {
    signals
        .split(" ")
        .into_iter()
        .find(|signal| signal.len() == len)
        .unwrap()
        .split("")
        .filter(|&x| x.len() > 0)
        .collect()
}

fn parse_digit<'a>(signals: &str, digit: &str) -> Option<&'a str> {
    let one = find_number_by_len(2, signals);
    let four = find_number_by_len(4, signals);

    let contains_one = digit.contains(one.get(0).unwrap()) && digit.contains(one.get(1).unwrap());

    let matches_with_four = digit
        .split("")
        .filter_map(|x| {
            if x.len() > 0 && four.contains(&x) {
                Some(x)
            } else {
                None
            }
        })
        .count();

    match digit.len() {
        7 => Some("8"),
        3 => Some("7"),
        4 => Some("4"),
        2 => Some("1"),
        5 => Some(if contains_one {
            "3"
        } else if matches_with_four == 3 {
            "5"
        } else {
            "2"
        }),
        6 => Some(if contains_one {
            if matches_with_four == 4 {
                "9"
            } else {
                "0"
            }
        } else {
            "6"
        }),
        _ => None,
    }
}

fn parse_segment(segment: &String) -> u64 {
    let (signals, digits) = segment.split_once("|").unwrap();

    digits
        .split(" ")
        .filter_map(|d| parse_digit(signals, d))
        .collect::<Vec<&str>>()
        .join("")
        .parse()
        .unwrap()
}

/* Solutions */

fn part01(input: &Vec<String>) -> usize {
    input.into_iter().map(count_simple_digits).sum()
}

fn part02(input: &Vec<String>) -> u64 {
    input.into_iter().map(parse_segment).sum()
}

pub fn day_08() -> Solution {
    let input = read_lines("./input/day_08.txt");
    let timer = std::time::Instant::now();
    Solution::new(8, part01(&input), part02(&input), timer.elapsed())
}

/* Tests */

#[cfg(test)]
mod tests {
    // Note this useful idiom: importing names from outer (for mod tests) scope.
    use super::*;

    #[test]
    fn test_parse_digit_input_cdfeb() {
        assert_eq!(
            parse_digit(
                "acedgfb cdfbe gcdfa fbcad dab cefabd cdfgeb eafb cagedb ab",
                "cdfeb"
            ),
            Some("5")
        )
    }

    #[test]
    fn test_parse_digit_input_fcadb() {
        assert_eq!(
            parse_digit(
                "acedgfb cdfbe gcdfa fbcad dab cefabd cdfgeb eafb cagedb ab",
                "fcadb"
            ),
            Some("3")
        )
    }

    #[test]
    fn test_parse_digit_input_cefabd() {
        assert_eq!(
            parse_digit(
                "acedgfb cdfbe gcdfa fbcad dab cefabd cdfgeb eafb cagedb ab",
                "cefabd"
            ),
            Some("9")
        )
    }

    #[test]
    fn test_parse_digit_input_cdfgeb() {
        assert_eq!(
            parse_digit(
                "acedgfb cdfbe gcdfa fbcad dab cefabd cdfgeb eafb cagedb ab",
                "cdfgeb"
            ),
            Some("6")
        )
    }

    #[test]
    fn test_parse_digit_input_8() {
        assert_eq!(
            parse_digit(
                "be cfbegad cbdgef fgaecd cgeb fdcge agebfd fecdb fabcd edb",
                "fdgacbe"
            ),
            Some("8")
        )
    }

    #[test]
    fn test_parse_segment_1() {
        assert_eq!(parse_segment(&String::from("be cfbegad cbdgef fgaecd cgeb fdcge agebfd fecdb fabcd edb | fdgacbe cefdb cefbgd gcbe")), 8394)
    }

    #[test]
    fn test_parse_segment_2() {
        assert_eq!(parse_segment(&String::from("edbfga begcd cbg gc gcadebf fbgde acbgfd abcde gfcbed gfec | fcgedb cgb dgebacf gc")), 9781)
    }

    #[test]
    fn test_parse_segment_3() {
        assert_eq!(
            parse_segment(&String::from(
                "fgaebd cg bdaec gdafb agbcfd gdcbef bgcad gfac gcb cdgabef | cg cg fdcagb cbg"
            )),
            1197
        )
    }

    #[test]
    fn test_parse_segment_4() {
        assert_eq!(
            parse_segment(&String::from(
                "fbegcd cbd adcefb dageb afcb bc aefdc ecdab fgdeca fcdbega | efabcd cedba gadfec cb"
            )),
            9361
        )
    }

    #[test]
    fn test_parse_segment_5() {
        assert_eq!(
            parse_segment(&String::from(
                "aecbfdg fbg gf bafeg dbefa fcge gcbea fcaegb dgceab fcbdga | gecf egdcabf bgf bfgea"
            )),
            4873
        )
    }

    #[test]
    fn test_parse_segment_6() {
        assert_eq!(
            parse_segment(&String::from(
                "fgeab ca afcebg bdacfeg cfaedg gcfdb baec bfadeg bafgc acf | gebdcfa ecba ca fadegcb"
            )),
            8418
        )
    }

    #[test]
    fn test_parse_segment_7() {
        assert_eq!(
            parse_segment(&String::from(
                "dbcfg fgd bdegcaf fgec aegbdf ecdfab fbedc dacgb gdcebf gf | cefg dcbef fcge gbcadfe"
            )),
            4548
        )
    }

    #[test]
    fn test_parse_segment_8() {
        assert_eq!(
            parse_segment(&String::from(
                "bdfegc cbegaf gecbf dfcage bdacg ed bedf ced adcbefg gebcd | ed bcgafe cdgba cbgef"
            )),
            1625
        )
    }

    #[test]
    fn test_parse_segment_9() {
        assert_eq!(
            parse_segment(&String::from(
                "egadfb cdbfeg cegd fecab cgb gbdefca cg fgcdab egfdb bfceg | gbdfcae bgc cg cgb"
            )),
            8717
        )
    }

    #[test]
    fn test_parse_segment_10() {
        assert_eq!(
            parse_segment(&String::from(
                "gcafb gcf dcaebfg ecagb gf abcdeg gaef cafbge fdbac fegbdc | fgae cfgab fg bagce"
            )),
            4315
        )
    }

    #[test]
    fn test_part01() {
        assert_eq!(part01(&vec![
            String::from("be cfbegad cbdgef fgaecd cgeb fdcge agebfd fecdb fabcd edb | fdgacbe cefdb cefbgd gcbe"),
            String::from("edbfga begcd cbg gc gcadebf fbgde acbgfd abcde gfcbed gfec | fcgedb cgb dgebacf gc"),
            String::from("fgaebd cg bdaec gdafb agbcfd gdcbef bgcad gfac gcb cdgabef | cg cg fdcagb cbg"),
            String::from("fbegcd cbd adcefb dageb afcb bc aefdc ecdab fgdeca fcdbega | efabcd cedba gadfec cb"),
            String::from("aecbfdg fbg gf bafeg dbefa fcge gcbea fcaegb dgceab fcbdga | gecf egdcabf bgf bfgea"),
            String::from("fgeab ca afcebg bdacfeg cfaedg gcfdb baec bfadeg bafgc acf | gebdcfa ecba ca fadegcb"),
            String::from("dbcfg fgd bdegcaf fgec aegbdf ecdfab fbedc dacgb gdcebf gf | cefg dcbef fcge gbcadfe"),
            String::from("bdfegc cbegaf gecbf dfcage bdacg ed bedf ced adcbefg gebcd | ed bcgafe cdgba cbgef"),
            String::from("egadfb cdbfeg cegd fecab cgb gbdefca cg fgcdab egfdb bfceg | gbdfcae bgc cg cgb"),
            String::from("gcafb gcf dcaebfg ecagb gf abcdeg gaef cafbge fdbac fegbdc | fgae cfgab fg bagce"),
        ]), 26)
    }

    #[test]
    fn test_part01_final() {
        assert_eq!(part01(&read_lines("./input/day_08.txt")), 365)
    }

    #[test]
    fn test_part02() {
        assert_eq!(part02(&vec![
            String::from("be cfbegad cbdgef fgaecd cgeb fdcge agebfd fecdb fabcd edb | fdgacbe cefdb cefbgd gcbe"),
            String::from("edbfga begcd cbg gc gcadebf fbgde acbgfd abcde gfcbed gfec | fcgedb cgb dgebacf gc"),
            String::from("fgaebd cg bdaec gdafb agbcfd gdcbef bgcad gfac gcb cdgabef | cg cg fdcagb cbg"),
            String::from("fbegcd cbd adcefb dageb afcb bc aefdc ecdab fgdeca fcdbega | efabcd cedba gadfec cb"),
            String::from("aecbfdg fbg gf bafeg dbefa fcge gcbea fcaegb dgceab fcbdga | gecf egdcabf bgf bfgea"),
            String::from("fgeab ca afcebg bdacfeg cfaedg gcfdb baec bfadeg bafgc acf | gebdcfa ecba ca fadegcb"),
            String::from("dbcfg fgd bdegcaf fgec aegbdf ecdfab fbedc dacgb gdcebf gf | cefg dcbef fcge gbcadfe"),
            String::from("bdfegc cbegaf gecbf dfcage bdacg ed bedf ced adcbefg gebcd | ed bcgafe cdgba cbgef"),
            String::from("egadfb cdbfeg cegd fecab cgb gbdefca cg fgcdab egfdb bfceg | gbdfcae bgc cg cgb"),
            String::from("gcafb gcf dcaebfg ecagb gf abcdeg gaef cafbge fdbac fegbdc | fgae cfgab fg bagce"),
            ]), 61229)
    }

    #[test]
    fn test_part02_final() {
        assert_eq!(part02(&read_lines("./input/day_08.txt")), 975706)
    }
}
