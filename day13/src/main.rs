use std::fs;

fn part1(input: &str) -> usize {
    let target: usize = input.lines().next().unwrap().parse().unwrap();
    let earliest: usize = input
        .lines()
        .skip(1)
        .next()
        .unwrap()
        .split(",")
        .filter(|&x| x != "x")
        .map(|x| x.parse().unwrap())
        .min_by_key(|&x| x - target % x)
        .unwrap();

    earliest * (earliest - target % earliest)
}

fn main() {
    let input = fs::read_to_string("../inputs/day13.txt").unwrap();

    println!("{}", part1(&input));
}

mod test {
    use super::*;

    #[test]
    fn test_part1() {
        let input = fs::read_to_string("../inputs/day13-test.txt").unwrap();

        assert_eq!(part1(&input), 295);
    }
}
