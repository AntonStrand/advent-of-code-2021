use crate::{pipe, read_lines, sort_desc, Solution};

#[derive(Debug, PartialEq, Eq)]
enum Char {
    Paren,
    Bracket,
    Brace,
    Angle,
}

impl Char {
    fn new(c: char) -> Char {
        match c {
            '(' | ')' => Char::Paren,
            '[' | ']' => Char::Bracket,
            '{' | '}' => Char::Brace,
            _ => Char::Angle,
        }
    }

    fn is_matching(&self, c: &Char) -> bool {
        self == c
    }

    fn as_checker_points(&self) -> u64 {
        match self {
            Char::Paren => 3,
            Char::Bracket => 57,
            Char::Brace => 1197,
            Char::Angle => 25137,
        }
    }

    fn as_autocomplete_points(&self) -> u64 {
        match self {
            Char::Paren => 1,
            Char::Bracket => 2,
            Char::Brace => 3,
            Char::Angle => 4,
        }
    }
}

#[derive(Debug, PartialEq, Eq)]
struct ParseError {
    expected: Char,
    found: Char,
}

#[derive(Debug, PartialEq, Eq)]
enum Line {
    Ok,
    Incomplete(Vec<Char>),
    Corrupt(ParseError),
}

impl Line {
    fn parse(line: &String) -> Line {
        let mut visited: Vec<Char> = vec![];
        let mut chars = line.chars();
        loop {
            if let Some(c) = chars.next() {
                match c {
                    '(' | '[' | '{' | '<' => visited.push(Char::new(c)),
                    ')' | ']' | '}' | '>' => {
                        let expected = visited.pop().unwrap();
                        let found = Char::new(c);
                        if !found.is_matching(&expected) {
                            break Line::Corrupt(ParseError { expected, found });
                        }
                    }
                    _ => {
                        break Line::Corrupt(ParseError {
                            expected: visited.pop().unwrap(),
                            found: Char::new(c),
                        });
                    }
                }
            } else {
                if visited.is_empty() {
                    break Line::Ok;
                } else {
                    visited.reverse();
                    break Line::Incomplete(visited);
                }
            }
        }
    }

    fn as_checkers_points(line: Line) -> u64 {
        match line {
            Line::Corrupt(error) => error.found.as_checker_points(),
            _ => 0,
        }
    }

    fn to_autocomplete_points(line: Line) -> Option<u64> {
        match line {
            Line::Incomplete(chars) => Some(
                chars
                    .into_iter()
                    .fold(0, |total, c| total * 5 + c.as_autocomplete_points()),
            ),
            _ => None,
        }
    }
}

/* Solutions */

fn part01(input: &Vec<String>) -> u64 {
    input
        .iter()
        .map(pipe(Line::parse, Line::as_checkers_points))
        .sum()
}

fn part02(input: &Vec<String>) -> u64 {
    let scores = sort_desc(
        input
            .iter()
            .filter_map(pipe(Line::parse, Line::to_autocomplete_points))
            .collect(),
    );

    *scores.get(scores.len() / 2).unwrap()
}

pub fn day_10() -> Solution {
    let input = read_lines("./input/day_10.txt");
    let timer = std::time::Instant::now();
    Solution::new(10, part01(&input), part02(&input), timer.elapsed())
}

/* Tests */

#[cfg(test)]
mod tests {
    // Note this useful idiom: importing names from outer (for mod tests) scope.
    use super::*;

    #[test]
    fn test_parse_line_ok() {
        assert_eq!(Line::parse(&String::from("[<>({}){}[([])<>]]")), Line::Ok)
    }

    #[test]
    fn test_parse_line_corrupt_curly() {
        assert_eq!(
            Line::parse(&String::from("{([(<{}[<>[]}>{[]{[(<()>")),
            Line::Corrupt(ParseError {
                expected: Char::Bracket,
                found: Char::Brace
            })
        )
    }

    #[test]
    fn test_parse_line_corrupt_parens() {
        assert_eq!(
            Line::parse(&String::from("[[<[([]))<([[{}[[()]]]")),
            Line::Corrupt(ParseError {
                expected: Char::Bracket,
                found: Char::Paren
            })
        )
    }

    #[test]
    fn test_parse_line_corrupt_bracket() {
        assert_eq!(
            Line::parse(&String::from("[{[{({}]{}}([{[{{{}}([]")),
            Line::Corrupt(ParseError {
                expected: Char::Paren,
                found: Char::Bracket
            })
        )
    }

    #[test]
    fn test_parse_line_corrupt_chevron() {
        assert_eq!(
            Line::parse(&String::from("[<(<(<(<{}))><([]([]()")),
            Line::Corrupt(ParseError {
                expected: Char::Angle,
                found: Char::Paren
            })
        )
    }

    #[test]
    fn test_parse_line_corrupt_angle() {
        assert_eq!(
            Line::parse(&String::from("<{([([[(<>()){}]>(<<{{")),
            Line::Corrupt(ParseError {
                expected: Char::Bracket,
                found: Char::Angle
            })
        )
    }

    #[test]
    fn test_parse_line_incomplete() {
        assert_eq!(
            Line::parse(&String::from("[({(<(())[]>[[{[]{<()<>>")),
            Line::Incomplete(vec![
                Char::new('}'),
                Char::new('}'),
                Char::new(']'),
                Char::new(']'),
                Char::new(')'),
                Char::new('}'),
                Char::new(')'),
                Char::new(']'),
            ])
        )
    }

    #[test]
    fn test_part01() {
        let input = read_lines("./input/day_10.txt");
        assert_eq!(part01(&input), 319329)
    }

    #[test]
    fn test_part02() {
        let input = read_lines("./input/day_10.txt");
        assert_eq!(part02(&input), 3515583998)
    }
}
