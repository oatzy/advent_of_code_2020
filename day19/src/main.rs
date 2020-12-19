use std::collections::HashMap;
use std::fs;
use std::iter::repeat;

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
        let mut s = string;
        for rule in rules.split(" ") {
            let res = self.check_rule(rule, s)?;
            s = res.0;
        }
        Ok((s, true))
    }

    fn vec_seq<'b>(&self, rules: &Vec<&str>, string: &'b str) -> IResult<&'b str, bool> {
        let mut s = string;
        for rule in rules {
            let res = self.check_rule(rule, s)?;
            s = res.0;
        }
        Ok((s, true))
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

    fn rule11_recursive<'b>(&self, string: &'b str) -> IResult<&'b str, bool> {
        let mut i = 1;
        loop {
            let res = self.vec_seq(&repeat("42").take(i).collect::<Vec<&str>>(), string)?;
            let res = self.vec_seq(&repeat("31").take(i).collect::<Vec<&str>>(), res.0);
            if res.is_ok() && res.unwrap().0 == "" {
                return Ok(("", true));
            };
            i += 1;
        }
    }

    fn _validate_part2<'b>(&self, string: &'b str) -> IResult<&'b str, bool> {
        let mut i = 1;
        loop {
            let res = self.vec_seq(&repeat("42").take(i).collect::<Vec<&str>>(), string)?;
            let res = self.rule11_recursive(res.0);
            if res.is_ok() && res.unwrap().0 == "" {
                return Ok(("", true));
            };
            i += 1;
        }
    }

    fn validate_part2(&self, string: &str) -> bool {
        match self._validate_part2(string) {
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

    let valid = messages
        .lines()
        .filter(|m| validator.validate_part2(m))
        .count();
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

    #[test]
    fn test_part2() {
        let input = fs::read_to_string("../inputs/day19-test2.txt").unwrap();
        let input: Vec<&str> = input.splitn(2, "\n\n").collect();
        let rules = input[0];
        let messages = input[1];

        let validator = Validator::from_rule_string(&rules);

        let valid = messages
            .lines()
            .filter(|m| validator.validate_part2(m))
            .count();
        assert_eq!(valid, 12);
    }
}
