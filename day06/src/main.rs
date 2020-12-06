use std::collections::HashSet;
use std::fs;

fn unique(group: &str) -> usize {
    group
        .chars()
        .filter(|c| c != &'\n')
        .collect::<HashSet<char>>()
        .len()
}

fn common(group: &str) -> usize {
    let members: Vec<&str> = group.lines().collect();
    let mut count = 0;
    for c in members.get(0).unwrap().chars() {
        if members.iter().all(|m| m.contains(c)) {
            count += 1
        }
    }
    count
}

fn main() {
    let input = fs::read_to_string("../inputs/day06.txt").unwrap();

    let part1: usize = input.split("\n\n").map(|g| unique(g)).sum();
    println!("{}", part1);

    let part2: usize = input.split("\n\n").map(|g| common(g)).sum();
    println!("{}", part2);
}

mod test {
    use super::*;

    #[test]
    fn test_part1() {
        let input = fs::read_to_string("../inputs/day06-test.txt").unwrap();
        let part1: usize = input.split("\n\n").map(|g| unique(g)).sum();
        assert_eq!(part1, 11);
    }

    #[test]
    fn test_part2() {
        let input = fs::read_to_string("../inputs/day06-test.txt").unwrap();
        let part2: usize = input.split("\n\n").map(|g| common(g)).sum();
        assert_eq!(part2, 6);
    }
}
