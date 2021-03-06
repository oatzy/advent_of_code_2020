use std::fs;
use std::collections::HashSet;

fn part1(report: &Vec<usize>) -> usize {
    for i in 0..report.len() {
        let x = report.get(i).unwrap();
        for j in (i + 1)..report.len() {
            let y = report.get(j).unwrap();
            if x + y == 2020 {
                return x * y;
            }
        }
    }
    panic!("no match found");
}

fn part1_alt(path: &str) -> usize {
    let mut seen: HashSet<usize> = HashSet::new();

    for x in fs::read_to_string(path).unwrap().lines().map(|x| x.parse::<usize>().unwrap()) {
        let y = 2020 - x;
        if seen.contains(&y) {
            return x * y;
        }
        seen.insert(x);
    }
    unreachable!("no match found");
}

fn part2(report: &Vec<usize>) -> usize {
    for i in 0..report.len() {
        let x = report.get(i).unwrap();
        for j in (i + 1)..report.len() {
            let y = report.get(j).unwrap();
            if x + y >= 2020 {
                // short-circuit
                continue;
            }
            for k in (j + 1)..report.len() {
                let z = report.get(k).unwrap();
                if x + y + z == 2020 {
                    return x * y * z;
                }
            }
        }
    }
    panic!("no match found");
}

fn main() {
    let input: Vec<usize> = fs::read_to_string("../inputs/day01.txt")
        .unwrap()
        .lines()
        .map(|x| x.parse().unwrap())
        .collect();

    println!("{}", part1(&input));
    println!("{}", part2(&input));

    //println!("{}", part1_alt("../inputs/day01.txt"));
}
