use std::fs;

use anyhow::Result;
use regex::Regex;

fn count_range_valid(input: &str) -> Result<usize> {
    let re = Regex::new(r"^(\d+)\-(\d+) (\w): (\w+)$")?;

    let mut total = 0;

    for line in input.lines() {
        let cap = re.captures(line).unwrap();

        let count = &cap[4].matches(&cap[3]).count();

        if count >= &cap[1].parse::<usize>()? && count <= &cap[2].parse::<usize>()? {
            total += 1;
        }
    }

    Ok(total)
}

fn count_position_valid(input: &str) -> Result<usize> {
    let re = Regex::new(r"^(\d+)\-(\d+) (\w): (\w+)$")?;

    let mut total = 0;

    for line in input.lines() {
        let cap = re.captures(line).unwrap();

        let low = &cap[1].parse::<usize>()? - 1;
        let high = &cap[2].parse::<usize>()? - 1;
        let char = &cap[3];
        let password = &cap[4];

        // xor - i.e. one or other but not both
        if (&password[low..low + 1] == char) ^ (&password[high..high + 1] == char) {
            total += 1;
        }
    }

    Ok(total)
}

fn main() -> Result<()> {
    let input = fs::read_to_string("../inputs/day02.txt")?;

    let part1 = count_range_valid(&input)?;
    let part2 = count_position_valid(&input)?;

    println!("{}", part1);
    println!("{}", part2);

    Ok(())
}

mod test {
    use super::*;

    #[test]
    fn test_count_range_valid() {
        let input = "1-3 a: abcde\n1-3 b: cdefg\n2-9 c: ccccccccc";
        assert!(count_range_valid(input).unwrap() == 2);
    }

    #[test]
    fn test_count_position_valid() {
        let input = "1-3 a: abcde\n1-3 b: cdefg\n2-9 c: ccccccccc";
        assert!(count_position_valid(input).unwrap() == 1);
    }
}
