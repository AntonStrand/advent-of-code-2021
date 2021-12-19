use crate::{read, Solution};
use std::fmt::Debug;

#[derive(PartialEq, Eq, Clone)]
enum Number {
    Natural(u8),
    Pair(Box<Number>, Box<Number>),
}

impl Number {
    fn from_str(input: &str) -> Number {
        Number::parse(&mut input.chars())
    }

    fn parse(input: &mut impl Iterator<Item = char>) -> Number {
        match input.next() {
            Some('[') => {
                let left = Number::parse(input);
                input.next(); // remove ","
                let right = Number::parse(input);
                input.next(); // remove "]"
                Number::pair(left, right)
            }
            Some(v) => Number::Natural(v.to_digit(10).unwrap() as u8),
            _ => unreachable!(),
        }
    }

    fn pair(left: Number, right: Number) -> Number {
        Number::Pair(Box::new(left), Box::new(right))
    }

    fn leftmost(&mut self) -> &mut u8 {
        match self {
            Number::Natural(value) => value,
            Number::Pair(left, _) => left.leftmost(),
        }
    }

    fn rightmost(&mut self) -> &mut u8 {
        match self {
            Number::Natural(value) => value,
            Number::Pair(_, right) => right.rightmost(),
        }
    }

    fn explode(&mut self) -> bool {
        match self {
            Number::Natural(_) => false,
            Number::Pair(left, right) => {
                left.rec_explode(1, None, Some(right.leftmost()))
                    || right.rec_explode(1, Some(left.rightmost()), None)
            }
        }
    }

    fn rec_explode(
        &mut self,
        depth: usize,
        left_value: Option<&mut u8>,
        right_value: Option<&mut u8>,
    ) -> bool {
        match self {
            Number::Natural(_) => false,
            Number::Pair(left, right) => {
                if depth == 4 {
                    if let Number::Natural(left) = left.as_ref() {
                        if let Some(left_value) = left_value {
                            *left_value += left;
                        }
                    }

                    if let Number::Natural(right) = right.as_ref() {
                        if let Some(right_value) = right_value {
                            *right_value += *right;
                        }
                    }

                    *self = Number::Natural(0);

                    true
                } else {
                    left.rec_explode(depth + 1, left_value, Some(right.leftmost()))
                        || right.rec_explode(depth + 1, Some(left.rightmost()), right_value)
                }
            }
        }
    }

    fn split(&mut self) -> bool {
        match self {
            Number::Natural(n) if *n > 9 => {
                *self = Number::pair(Number::Natural(*n / 2), Number::Natural((*n + 1) / 2));
                true
            }
            Number::Pair(left, right) => left.split() || right.split(),
            _ => false,
        }
    }

    fn add(x: Number, y: Number) -> Number {
        let mut n = Number::pair(x, y);
        while n.explode() || n.split() {}
        n
    }

    fn magnitude(&self) -> u16 {
        match self {
            Number::Natural(n) => *n as u16,
            Number::Pair(left, right) => 3 * left.magnitude() + 2 * right.magnitude(),
        }
    }
}

impl Debug for Number {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Number::Natural(value) => write!(f, "{}", value),
            Number::Pair(left, right) => write!(f, "[{:?},{:?}]", left, right),
        }
    }
}

/* Solutions */

fn part01(input: &String) -> u16 {
    input
        .lines()
        .map(Number::from_str)
        .reduce(Number::add)
        .unwrap()
        .magnitude()
}

fn part02(input: &String) -> u16 {
    let ns = input.lines().map(Number::from_str).collect::<Vec<_>>();
    let mut max = 0;

    for i in 0..ns.len() {
        for j in i + 1..ns.len() {
            if ns[i] != ns[j] {
                max = max.max(Number::add(ns[i].clone(), ns[j].clone()).magnitude());
            }
        }
    }

    max
}

pub fn day_18() -> Solution {
    let input = read("./input/day_18.txt");
    let timer = std::time::Instant::now();
    Solution::new(18, part01(&input), part02(&input), timer.elapsed())
}

/* Tests */

#[cfg(test)]
mod tests {
    // Note this useful idiom: importing names from outer (for mod tests) scope.
    use super::*;

    #[test]
    fn test_parse_pair() {
        assert_eq!(
            Number::parse(&mut "[1,2]".chars()),
            Number::Pair(Box::new(Number::Natural(1)), Box::new(Number::Natural(2)))
        )
    }

    #[test]
    fn test_parse_nested_pair() {
        assert_eq!(
            Number::parse(&mut "[[1,2],3]".chars()),
            Number::Pair(
                Box::new(Number::Pair(
                    Box::new(Number::Natural(1)),
                    Box::new(Number::Natural(2))
                )),
                Box::new(Number::Natural(3))
            )
        )
    }

    #[test]
    fn test_add() {
        let n1 = Number::from_str("[[[[4,3],4],4],[7,[[8,4],9]]]");
        let n2 = Number::from_str("[1,1]");

        assert_eq!(
            Number::add(n1, n2),
            Number::from_str("[[[[0,7],4],[[7,8],[6,0]]],[8,1]]")
        )
    }

    #[test]
    fn test_explode_left() {
        let mut number = Number::from_str("[[[[[9,8],1],2],3],4]");
        let exploded = number.explode();
        assert_eq!(
            (exploded, number),
            (true, Number::from_str("[[[[0,9],2],3],4]"))
        )
    }

    #[test]
    fn test_explode_right() {
        let mut result = Number::from_str("[7,[6,[5,[4,[3,2]]]]]");
        result.explode();
        assert_eq!(result, Number::from_str("[7,[6,[5,[7,0]]]]"))
    }
    #[test]
    fn test_explode_nested() {
        let mut result = Number::from_str("[[3,[2,[1,[7,3]]]],[6,[5,[4,[3,2]]]]]");
        let exploded = result.explode();
        assert_eq!(
            (exploded, result),
            (true, Number::from_str("[[3,[2,[8,0]]],[9,[5,[4,[3,2]]]]]"))
        )
    }

    #[test]
    fn test_not_explode() {
        let mut result = Number::from_str("[4,[3,2]]");
        let exploded = result.explode();
        assert_eq!((exploded, result), (false, Number::from_str("[4,[3,2]]")))
    }

    #[test]
    fn test_not_explode_nested() {
        let mut result =
            Number::from_str("[[[[6,6],[7,6]],[[7,7],[7,0]]],[[[7,7],[7,7]],[[7,8],[9,9]]]]");
        let exploded = result.explode();
        assert_eq!(
            (exploded, result),
            (
                false,
                Number::from_str("[[[[6,6],[7,6]],[[7,7],[7,0]]],[[[7,7],[7,7]],[[7,8],[9,9]]]]")
            )
        )
    }

    #[test]
    fn test_part01() {
        let input = read("./input/day_18.txt");
        assert_eq!(part01(&input), 4243)
    }

    #[test]
    fn test_part02() {
        let input = read("./input/day_18.txt");
        assert_eq!(part02(&input), 4701)
    }
}
