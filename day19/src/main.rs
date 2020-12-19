use std::collections::HashMap;
use std::fs;

extern crate nom;

use nom::{branch::alt, bytes::complete::tag, sequence::tuple, IResult};

struct Validator<'a> {
    rules: HashMap<&'a str, &'a str>,
}

impl<'a> Validator<'a> {
    fn from_rule_string(string: &'a str) -> Self {
        let mut rules = HashMap::new();

        for rule in string.lines() {
            let line: Vec<&str> = rule.splitn(2, ": ").collect();
            let id = line[0];
            let rule = line[1];

            rules.insert(id, rule);
        }

        Self { rules: rules }
    }

    fn seq<'b>(&self, rules: &str, string: &'b str) -> IResult<&'b str, bool> {
        // lazy - cover the cases that are 'expected'
        // based on the puzzle input
        let rules: Vec<&str> = rules.split(" ").collect();
        if rules.len() == 1 {
            return self.check_rule(rules[0], string);
        } else if rules.len() == 2 {
            let res = tuple((
                |s| self.check_rule(rules[0], s),
                |s| self.check_rule(rules[1], s),
            ))(string)?;
            return Ok((res.0, true));
        } else if rules.len() == 3 {
            let res = tuple((
                |s| self.check_rule(rules[0], s),
                |s| self.check_rule(rules[1], s),
                |s| self.check_rule(rules[2], s),
            ))(string)?;
            return Ok((res.0, true));
        }
        panic!("Got more sequence items than expected");
    }

    fn check_rule<'b>(&self, rule: &str, string: &'b str) -> IResult<&'b str, bool> {
        let rule = self.rules.get(rule).unwrap();
        if rule.contains("\"") {
            let res = tag(&rule[1..rule.len() - 1])(string)?;
            return Ok((res.0, true));
        }
        if rule.contains("|") {
            let rules: Vec<&str> = rule.split(" | ").collect();
            let res = alt((|s| self.seq(rules[0], s), |s| self.seq(rules[1], s)))(string)?;
            return Ok((res.0, true));
        }
        if rule.contains(" ") {
            return self.seq(rule, string);
        }
        self.check_rule(rule, string)
    }

    fn validate(&self, string: &str) -> bool {
        match self.check_rule("0", string) {
            Ok((s, _)) => s == "",
            Err(_) => false,
        }
    }
}

fn main() {
    let input = fs::read_to_string("../inputs/day19.txt").unwrap();
    let input: Vec<&str> = input.splitn(2, "\n\n").collect();
    let rules = input[0];
    let messages = input[1];

    let validator = Validator::from_rule_string(&rules);

    let valid = messages.lines().filter(|m| validator.validate(m)).count();
    println!("{}", valid);
}

mod test {
    use super::*;

    #[test]
    fn test_part1() {
        let input = fs::read_to_string("../inputs/day19-test.txt").unwrap();
        let input: Vec<&str> = input.splitn(2, "\n\n").collect();
        let rules = input[0];
        let messages = input[1];

        let validator = Validator::from_rule_string(&rules);

        let valid = messages.lines().filter(|m| validator.validate(m)).count();
        assert_eq!(valid, 2);
    }
}
