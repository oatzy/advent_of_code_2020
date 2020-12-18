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

fn main() {
    let input = fs::read_to_string("../inputs/day18.txt").unwrap();
    let res: usize = input.lines().map(|s| evaluate(s)).sum();
    println!("{}", res);
}

mod test {
    use super::*;

    #[test]
    fn test_part2() {
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
