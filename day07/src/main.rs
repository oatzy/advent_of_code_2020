use std::collections::HashMap;
use std::fs;

#[macro_use]
extern crate lazy_static;

use regex::Regex;

type Rules = HashMap<String, Vec<(usize, String)>>;

fn parse_rules(input: &str) -> Rules {

    lazy_static! {
        static ref FULL_RE: Regex = Regex::new(r"^(\w+ \w+) bags contain (no other bags|.+)\.$").unwrap();
        static ref CHILD_RE: Regex = Regex::new(r"(\d+) (\w+ \w+)").unwrap();
    }

    let mut rules: Rules = HashMap::new();

    for rule in input.lines() {
        let groups = FULL_RE.captures(rule).unwrap();

        let mut children: Vec<(usize, String)> = Vec::new();

        if &groups[2] != "no other bags" {
            for cap in CHILD_RE.captures_iter(&groups[2]) {
                children.push((cap[1].parse().unwrap(), cap[1].into()));
            }
        }
        rules.insert(groups[1].into(), children);
    }

    rules
}

fn main() {
    let input = fs::read_to_string("../inputs/day07.txt").unwrap();
    let rules = parse_rules(&input);
}
