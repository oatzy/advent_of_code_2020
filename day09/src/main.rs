use std::fs;

fn sum_in_list(values: &[usize], target: usize) -> bool {
    for (i, x) in values.iter().enumerate() {
        for y in values.iter().skip(i + 1) {
            if x + y == target {
                return true;
            }
        }
    }
    false
}

fn find_first_invalid(values: &Vec<usize>, size: usize) -> Option<usize> {
    for (i, x) in values.iter().skip(size).enumerate() {
        if !sum_in_list(&values[i..i + size], *x) {
            return Some(*x);
        }
    }
    None
}

fn find_contiguous_sum(values: &Vec<usize>, target: usize) -> Option<(usize, usize)> {
    let mut start = 0;
    let mut end = 1;

    while end < values.len() {
        let sum: usize = values[start..end].iter().sum();

        if sum < target {
            end += 1;
        } else if sum > target {
            start += 1;
        } else {
            return Some((start, end));
        }
    }
    None
}

fn minmax_sum(values: &[usize]) -> usize {
    values.iter().max().unwrap() + values.iter().min().unwrap()
}

fn main() {
    let input: Vec<usize> = fs::read_to_string("../inputs/day09.txt")
        .unwrap()
        .lines()
        .map(|x| x.parse().unwrap())
        .collect();

    let invalid = find_first_invalid(&input, 25).unwrap();

    println!("{}", invalid);

    let (start, end) = find_contiguous_sum(&input, invalid).unwrap();
    let result = minmax_sum(&input[start..end]);

    println!("{}", result);
}

mod test {
    use super::*;

    fn load_test_input() -> Vec<usize> {
        fs::read_to_string("../inputs/day09-test.txt")
            .unwrap()
            .lines()
            .map(|x| x.parse().unwrap())
            .collect()
    }

    #[test]
    fn part1() {
        let input = load_test_input();
        let invalid = find_first_invalid(&input, 5).unwrap();

        assert_eq!(invalid, 127);
    }

    #[test]
    fn test_part2() {
        let input = load_test_input();
        let (s, e) = find_contiguous_sum(&input, 127).unwrap();

        let result = minmax_sum(&input[s..e]);

        assert_eq!(result, 62);
    }
}
