use std::fs;

extern crate nom;

use nom::{
    branch::alt,
    bytes::complete::tag,
    character::complete::digit1,
    combinator::map_res,
    multi::fold_many0,
    sequence::{delimited, tuple},
    IResult,
};

fn digit(n: &str) -> IResult<&str, usize> {
    map_res(digit1, |digit_str: &str| digit_str.parse::<usize>())(n)
}

fn factor(string: &str) -> IResult<&str, usize> {
    alt((digit, delimited(tag("("), expr, tag(")"))))(string)
}

fn term(t: &str) -> IResult<&str, usize> {
    let (s, init) = factor(t).unwrap();
    fold_many0(tuple((tag(" + "), factor)), init, |acc, v| acc + v.1)(s)
}

fn expr(e: &str) -> IResult<&str, usize> {
    let (s, init) = term(e).unwrap();
    fold_many0(tuple((tag(" * "), term)), init, |acc, v| acc * v.1)(s)
}

fn evaluate(string: &str) -> usize {
    let (s, r) = expr(string).unwrap();
    if s.len() > 0 {
        panic!("unparsed {}", s);
    }
    r
}

fn factor1(string: &str) -> IResult<&str, usize> {
    alt((digit, delimited(tag("("), expr1, tag(")"))))(string)
}

fn expr1(e: &str) -> IResult<&str, usize> {
    let (s, init) = factor1(e).unwrap();
    fold_many0(
        tuple((alt((tag(" * "), tag(" + "))), factor1)),
        init,
        |acc, v| if v.0 == " * " { acc * v.1 } else { acc + v.1 },
    )(s)
}

fn evaluate1(string: &str) -> usize {
    let (s, r) = expr1(string).unwrap();
    if s.len() > 0 {
        panic!("unparsed {}", s);
    }
    r
}

fn main() {
    let input = fs::read_to_string("../inputs/day18.txt").unwrap();
    println!("{}", input.lines().map(|s| evaluate1(s)).sum::<usize>());
    println!("{}", input.lines().map(|s| evaluate(s)).sum::<usize>());
}

mod test {
    use super::*;

    #[test]
    fn test_part1() {
        assert_eq!(evaluate1("1 + (2 * 3) + (4 * (5 + 6))"), 51);
        assert_eq!(evaluate1("2 * 3 + (4 * 5)"), 26);
        assert_eq!(evaluate1("5 + (8 * 3 + 9 + 3 * 4 * 3)"), 437);
        assert_eq!(
            evaluate1("5 * 9 * (7 * 3 * 3 + 9 * 3 + (8 + 6 * 4))"),
            12240
        );
        assert_eq!(
            evaluate1("((2 + 4 * 9) * (6 + 9 * 8 + 6) + 6) + 2 + 4 * 2"),
            13632
        );
    }

    #[test]
    fn test_part2() {
        assert_eq!(evaluate("1 + 2 * 3 + 4 * 5 + 6"), 231);
        assert_eq!(evaluate("1 + (2 * 3) + (4 * (5 + 6))"), 51);
        assert_eq!(evaluate("2 * 3 + (4 * 5)"), 46);
        assert_eq!(evaluate("5 + (8 * 3 + 9 + 3 * 4 * 3)"), 1445);
        assert_eq!(
            evaluate("5 * 9 * (7 * 3 * 3 + 9 * 3 + (8 + 6 * 4))"),
            669060
        );
        assert_eq!(
            evaluate("((2 + 4 * 9) * (6 + 9 * 8 + 6) + 6) + 2 + 4 * 2"),
            23340
        );
    }
}
