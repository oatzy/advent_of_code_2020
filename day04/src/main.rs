use std::collections::HashMap;
use std::fs;

use anyhow::Result;
use thiserror::Error;

static REQURIED_FIELDS: [&str; 7] = ["byr", "iyr", "eyr", "hgt", "hcl", "ecl", "pid"];
static VALID_EYE_COLOURS: [&str; 7] = ["amb", "blu", "brn", "gry", "grn", "hzl", "oth"];

#[derive(Error, Debug)]
pub enum Error<'a> {
    #[error("Field {0} wasn't found in password")]
    MissingField(&'a str),
}

#[derive(Debug)]
struct Passport {
    birth_year: usize,
    issue_year: usize,
    expire_year: usize,
    height: String,
    hair_colour: String,
    eye_colour: String,
    #[allow(dead_code)]
    country_id: Option<String>,
    passport_id: String,
}

impl Passport {
    fn parse(string: &str) -> Result<Self> {
        let mut temp: HashMap<&str, &str> = HashMap::new();

        for field in string.split_ascii_whitespace() {
            let entry: Vec<&str> = field.splitn(2, ":").collect();
            temp.insert(entry[0], entry[1]);
        }

        Ok(Passport {
            birth_year: try_get(&temp, "byr")?.parse()?,
            issue_year: try_get(&temp, "iyr")?.parse()?,
            expire_year: try_get(&temp, "eyr")?.parse()?,
            height: try_get(&temp, "hgt")?.into(),
            hair_colour: try_get(&temp, "hcl")?.into(),
            eye_colour: try_get(&temp, "ecl")?.into(),
            country_id: temp.get("cid").and_then(|&c| Some(c.into())),
            passport_id: try_get(&temp, "pid")?.into(),
        })
    }

    fn validate(&self) -> bool {
        valid_birth_year(self.birth_year)
            && valid_issue_year(self.issue_year)
            && valid_expire_year(self.expire_year)
            && valid_height(&self.height)
            && valid_hair_colour(&self.hair_colour)
            && valid_eye_colour(&self.eye_colour)
            && valid_passport_id(&self.passport_id)
    }
}

fn valid_birth_year(year: usize) -> bool {
    year >= 1920 && year <= 2002
}

fn valid_issue_year(year: usize) -> bool {
    year >= 2010 && year <= 2020
}

fn valid_expire_year(year: usize) -> bool {
    year >= 2020 && year <= 2030
}

fn valid_height(height: &str) -> bool {
    let unit = &height[height.len() - 2..];
    let height: usize = match height[..height.len() - 2].parse() {
        Ok(height) => height,
        Err(_) => return false, // parse error
    };

    match unit {
        "cm" => height >= 150 && height <= 193,
        "in" => height >= 59 && height <= 76,
        _ => false,
    }
}

fn valid_hair_colour(colour: &str) -> bool {
    &colour[..1] == "#"
        && colour[1..].len() == 6
        && colour[1..].chars().all(|c| c.is_ascii_hexdigit())
}

fn valid_eye_colour(colour: &str) -> bool {
    VALID_EYE_COLOURS.contains(&colour)
}

fn valid_passport_id(id: &str) -> bool {
    id.len() == 9 && id.chars().all(|c| c.is_ascii_digit())
}

fn try_get<'a, 'b>(map: &HashMap<&'a str, &'a str>, key: &'b str) -> Result<&'a str, Error<'b>> {
    // coerce an Option to a Result so we can use ?
    map.get(key)
        .and_then(|&v| Some(v))
        .ok_or(Error::MissingField(key))
}

fn part1(input: &str) -> usize {
    // compare for part1-oneliner.py
    input
        .split("\n\n")
        .filter(|&x| REQURIED_FIELDS.iter().all(|f| x.contains(f)))
        .count()
}

fn part2(input: &str) -> usize {
    input
        .split("\n\n")
        .map(Passport::parse)
        // any invalid due to missing fields will not return Ok
        .filter_map(Result::ok)
        .filter(|p| p.validate())
        .count()
}

fn main() -> Result<()> {
    let input = fs::read_to_string("../inputs/day04.txt")?;

    println!("{}", part1(&input));
    println!("{}", part2(&input));

    Ok(())
}

mod test {
    use super::*;

    #[test]
    fn test_part1() {
        let input = fs::read_to_string("../inputs/day04-test.txt").unwrap();
        assert_eq!(part1(&input), 2);
    }

    #[test]
    fn test_parse_valid() {
        let input = fs::read_to_string("../inputs/day04-test-valid.txt").unwrap();
        assert_eq!(part2(&input), 4);
    }

    #[test]
    fn test_part2_invalid() {
        let input = fs::read_to_string("../inputs/day04-test-invalid.txt").unwrap();
        assert_eq!(part2(&input), 0);
    }

    #[test]
    fn test_valid_birth_year() {
        assert!(valid_birth_year(2002));
        assert!(!valid_birth_year(2003));
    }

    #[test]
    fn test_valid_height() {
        assert!(valid_height("60in"));
        assert!(valid_height("190cm"));
        assert!(!valid_height("190in"));
        assert!(!valid_height("190"));
    }

    #[test]
    fn test_valid_hair_colour() {
        assert!(valid_hair_colour("#123abc"));
        assert!(!valid_hair_colour("#123abz"));
        assert!(!valid_hair_colour("123abc"));
    }

    #[test]
    fn test_valid_eye_colour() {
        assert!(valid_eye_colour("brn"));
        assert!(!valid_eye_colour("wat"));
    }

    #[test]
    fn test_valid_passport_id() {
        assert!(valid_passport_id("000000001"));
        assert!(!valid_passport_id("0123456789"));
    }
}
