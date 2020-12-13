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

fn find_earliest(times: &Vec<(u64, u64)>) -> u64 {
    let mut multiplier: u64 = 1;
    let mut start: u64 = 0;

    for (offset, factor) in times {
        while (start + offset) % factor != 0 {
            start += multiplier;
        }
        multiplier *= factor;
    }
    start
}

fn part2(input: &str) -> u64 {
    let times: Vec<(u64, u64)> = input
        .lines()
        .skip(1)
        .next()
        .unwrap()
        .split(",")
        .enumerate()
        .filter(|(_, x)| x != &"x")
        .map(|(i, x)| (i as u64, x.parse::<u64>().unwrap()))
        .collect();

    find_earliest(&times)
}

fn main() {
    let input = fs::read_to_string("../inputs/day13.txt").unwrap();

    println!("{}", part1(&input));

    println!("{}", part2(&input));
}

mod test {
    use super::*;

    #[test]
    fn test_part1() {
        let input = fs::read_to_string("../inputs/day13-test.txt").unwrap();

        assert_eq!(part1(&input), 295);
    }

    #[test]
    fn test_part2() {
        let input = fs::read_to_string("../inputs/day13-test.txt").unwrap();

        assert_eq!(part2(&input), 1068781);
    }

    #[test]
    fn test_part2_example1() {
        let times: Vec<(u64, u64)> = vec![(0, 17), (2, 13), (3, 19)];
        assert_eq!(find_earliest(&times), 3417);
    }

    #[test]
    fn test_part2_example2() {
        let times: Vec<(u64, u64)> = vec![(0, 67), (1, 7), (2, 59), (3, 61)];
        assert_eq!(find_earliest(&times), 754018);
    }

    #[test]
    fn test_part2_example3() {
        let times: Vec<(u64, u64)> = vec![(0, 67), (2, 7), (3, 59), (4, 61)];
        assert_eq!(find_earliest(&times), 779210);
    }

    #[test]
    fn test_part2_example4() {
        let times: Vec<(u64, u64)> = vec![(0, 67), (1, 7), (3, 59), (4, 61)];
        assert_eq!(find_earliest(&times), 1261476);
    }

    #[test]
    fn test_part2_example5() {
        let times: Vec<(u64, u64)> = vec![(0, 1789), (1, 37), (2, 47), (3, 1889)];
        assert_eq!(find_earliest(&times), 1202161486);
    }
}
