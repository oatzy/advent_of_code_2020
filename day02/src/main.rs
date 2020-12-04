use std::fs;

#[macro_use]
extern crate lazy_static;

use anyhow::Result;
use regex::Regex;

struct Password {
    low: usize,
    high: usize,
    char: String,
    password: String,
}

impl From<&str> for Password {
    fn from(string: &str) -> Self {
        lazy_static! {
            static ref RE: Regex = Regex::new(r"^(\d+)\-(\d+) (\w): (\w+)$").unwrap();
        }

        let cap = RE.captures(string).unwrap();

        Password {
            low: (&cap[1]).parse().unwrap(),
            high: (&cap[2]).parse().unwrap(),
            char: (&cap[3]).into(),
            password: (&cap[4]).into(),
        }
    }
}

fn valid_range(password: &Password) -> bool {
    let count = password.password.matches(&password.char).count();

    count >= password.low && count <= password.high
}

fn valid_position(password: &Password) -> bool {
    let p = password;
    // xor - i.e. one or other but not both
    (&p.password[(p.low - 1)..p.low] == p.char) ^ (&p.password[(p.high - 1)..p.high] == p.char)
}

fn count_valid<F>(input: &str, validator: F) -> usize
where
    F: Fn(&Password) -> bool,
{
    input.lines().map(Password::from).filter(validator).count()
}

fn main() -> Result<()> {
    let input = fs::read_to_string("../inputs/day02.txt")?;

    let part1 = count_valid(&input, valid_range);
    let part2 = count_valid(&input, valid_position);

    println!("{}", part1);
    println!("{}", part2);

    Ok(())
}

mod test {
    use super::*;

    #[test]
    fn test_count_range_valid() {
        let input = "1-3 a: abcde\n1-3 b: cdefg\n2-9 c: ccccccccc";
        assert_eq!(count_valid(input, valid_range), 2);
    }

    #[test]
    fn test_count_position_valid() {
        let input = "1-3 a: abcde\n1-3 b: cdefg\n2-9 c: ccccccccc";
        assert_eq!(count_valid(input, valid_position), 1);
    }
}
