use std::fs;

fn load_input(path: &str) -> Vec<usize> {
    fs::read_to_string(path)
        .unwrap()
        .lines()
        .map(|x| x.parse().unwrap())
        .collect()
}

fn find_difference_distribution(adapters: &Vec<usize>) -> (usize, usize, usize) {
    let mut sorted: Vec<usize> = adapters.clone();
    sorted.sort();

    sorted.insert(0, 0); // outlet
    sorted.push(sorted.last().unwrap() + 3); // device

    sorted
        .iter()
        .zip(sorted.iter().skip(1))
        .map(|(x, y)| y - x)
        .fold((0, 0, 0), |acc, x| match x {
            1 => (acc.0 + 1, acc.1, acc.2),
            2 => (acc.0, acc.1 + 1, acc.2),
            3 => (acc.0, acc.1, acc.2 + 1),
            _ => panic!("got unexpected difference"),
        })
}

fn main() {
    let input = load_input("../inputs/day10.txt");

    let diffs = find_difference_distribution(&input);
    println!("{}", diffs.0 * diffs.2);
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
}
