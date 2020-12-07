use std::collections::{HashMap, HashSet};
use std::fs;

#[macro_use]
extern crate lazy_static;

use regex::Regex;

type Rules = HashMap<String, Vec<(usize, String)>>;

fn parse_rules(input: &str) -> Rules {
    lazy_static! {
        static ref FULL_RE: Regex =
            Regex::new(r"^(\w+ \w+) bags contain (no other bags|.+)\.$").unwrap();
        static ref CHILD_RE: Regex = Regex::new(r"(\d+) (\w+ \w+)").unwrap();
    }

    let mut rules: Rules = HashMap::new();

    for rule in input.lines() {
        let groups = FULL_RE.captures(rule).unwrap();

        let mut children: Vec<(usize, String)> = Vec::new();

        if &groups[2] != "no other bags" {
            for cap in CHILD_RE.captures_iter(&groups[2]) {
                children.push((cap[1].parse().unwrap(), cap[2].into()));
            }
        }
        rules.insert(groups[1].into(), children);
    }

    rules
}

fn count_contains(rules: &Rules, target: &str) -> usize {
    // something like a reverse tree walk
    let mut seen = HashSet::new();
    let mut tocheck = vec![target];

    while let Some(target) = tocheck.pop() {
        for (colour, rule) in rules.iter() {
            // if rule hasn't been checked and target is a child of this rule
            if !seen.contains(colour) && rule.iter().any(|(_, n)| n == target) {
                tocheck.push(colour);
                seen.insert(colour);
            }
        }
    }
    seen.len()
}

fn _count_children(rules: &Rules, colour: &str) -> usize {
    rules
        .get(colour)
        .unwrap()
        .iter()
        .map(|(c, n)| c * _count_children(rules, n))
        .sum::<usize>()
        // + 1 for the bag itself
        + 1
}

fn count_children(rules: &Rules, colour: &str) -> usize {
    // -1 because we don't want to include the outer bag
    _count_children(rules, colour) - 1
}

fn main() {
    let input = fs::read_to_string("../inputs/day07.txt").unwrap();
    let rules = parse_rules(&input);

    //println!("{:?}", rules);

    println!("{}", count_contains(&rules, "shiny gold"));
    println!("{}", count_children(&rules, "shiny gold"));
}

mod test {
    use super::*;

    #[test]
    fn test_part1() {
        let input = fs::read_to_string("../inputs/day07-test.txt").unwrap();
        let rules = parse_rules(&input);
        assert_eq!(count_contains(&rules, "shiny gold"), 4);
    }

    #[test]
    fn test_part2() {
        let input = fs::read_to_string("../inputs/day07-test.txt").unwrap();
        let rules = parse_rules(&input);
        assert_eq!(count_children(&rules, "shiny gold"), 32);
    }
}
