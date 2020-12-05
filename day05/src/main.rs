use std::collections::HashSet;
use std::fs;

use anyhow::Result;

fn pass_to_id(pass: &str) -> usize {
    // essentially, we're converting B+R to 1 and F+L to 0
    // then converting the resulting binary to decimal
    pass.chars()
        .rev()
        .enumerate()
        .filter(|(_, c)| c == &'B' || c == &'R')
        .map(|(i, _)| 1 << i)
        .sum()
}

fn main() -> Result<()> {
    let passes: HashSet<usize> = fs::read_to_string("../inputs/day05.txt")?
        .lines()
        .map(pass_to_id)
        .collect();

    let part1 = passes.iter().max().unwrap();
    println!("{}", part1);

    let part2 = (1..(1 << 10))
        // this is nutty -> &(*x + 1)
        .find(|x| passes.contains(&(*x - 1)) && passes.contains(&(*x + 1)) && !passes.contains(x))
        .unwrap();
    println!("{}", part2);

    Ok(())
}

mod test {
    use super::pass_to_id;

    #[test]
    fn test_pass_to_id() {
        assert_eq!(pass_to_id("FBFBBFFRLR"), 357);
        assert_eq!(pass_to_id("BFFFBBFRRR"), 567);
        assert_eq!(pass_to_id("FFFBBBFRRR"), 119);
        assert_eq!(pass_to_id("BBFFBBFRLL"), 820);
    }
}
