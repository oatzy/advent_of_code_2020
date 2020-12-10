use std::collections::HashMap;
use std::fs;

fn load_input(path: &str) -> Vec<usize> {
    let mut input: Vec<usize> = fs::read_to_string(path)
        .unwrap()
        .lines()
        .map(|x| x.parse().unwrap())
        .collect();

    input.sort();
    input.insert(0, 0); // outlet
    input.push(input.last().unwrap() + 3); // device

    input
}

fn find_difference_distribution(adapters: &Vec<usize>) -> (usize, usize, usize) {
    adapters
        .iter()
        .zip(adapters.iter().skip(1))
        .map(|(x, y)| y - x)
        .fold((0, 0, 0), |acc, x| match x {
            1 => (acc.0 + 1, acc.1, acc.2),
            2 => (acc.0, acc.1 + 1, acc.2),
            3 => (acc.0, acc.1, acc.2 + 1),
            _ => panic!("got unexpected difference"),
        })
}

fn count_possible_chains(adapters: &Vec<usize>) -> usize {
    // sum of paths from 0 to N = sum(paths from 0 to (N-1, N-2, N-3))
    // could do this recursively, but without memoization
    // we'd end up doing a lot of repeated calculations
    let mut counts: HashMap<usize, usize> = HashMap::new();
    counts.insert(0, 1);

    for item in adapters.iter().skip(1) {
        counts.insert(
            *item,
            (item.max(&3) - 3..*item)
                .map(|x| counts.get(&x).or(Some(&0)).unwrap())
                .sum(),
        );
    }

    *counts.get(adapters.last().unwrap()).unwrap()
}

fn main() {
    let input = load_input("../inputs/day10.txt");

    let diffs = find_difference_distribution(&input);
    println!("{}", diffs.0 * diffs.2);

    let count = count_possible_chains(&input);
    println!("{}", count);
}

mod test {
    use super::*;

    #[test]
    fn test_part1_short() {
        let input = load_input("../inputs/day10-test1.txt");

        let diffs = find_difference_distribution(&input);

        assert_eq!(diffs.0, 7);
        assert_eq!(diffs.1, 0);
        assert_eq!(diffs.2, 5);

        assert_eq!(diffs.0 * diffs.2, 35);
    }

    #[test]
    fn test_part1_long() {
        let input = load_input("../inputs/day10-test2.txt");

        let diffs = find_difference_distribution(&input);

        assert_eq!(diffs.0, 22);
        assert_eq!(diffs.1, 0);
        assert_eq!(diffs.2, 10);

        assert_eq!(diffs.0 * diffs.2, 220);
    }

    #[test]
    fn test_part2_short() {
        let input = load_input("../inputs/day10-test1.txt");

        let count = count_possible_chains(&input);

        assert_eq!(count, 8);
    }

    #[test]
    fn test_part2_long() {
        let input = load_input("../inputs/day10-test2.txt");

        let count = count_possible_chains(&input);

        assert_eq!(count, 19208);
    }
}
