use std::collections::HashSet;
use std::fs;

struct Field {
    name: String,
    lower: (usize, usize),
    upper: (usize, usize),
}

impl Field {
    fn parse(string: &str) -> Self {
        let mut parts = string.split(": ");
        let class = parts.next().unwrap().to_owned();

        let mut ranges = parts.next().unwrap().split(" or ");
        let lower: Vec<usize> = ranges
            .next()
            .unwrap()
            .split("-")
            .map(|x| x.parse().unwrap())
            .collect();

        let upper: Vec<usize> = ranges
            .next()
            .unwrap()
            .split("-")
            .map(|x| x.parse().unwrap())
            .collect();

        Self {
            name: class,
            lower: (lower[0], lower[1]),
            upper: (upper[0], upper[1]),
        }
    }
}

struct TicketInfo {
    fields: Vec<Field>,
    my_ticket: Vec<usize>,
    tickets: Vec<Vec<usize>>,
}

impl TicketInfo {
    fn parse(string: &str) -> Self {
        let mut sections = string.split("\n\n");

        let fields: Vec<Field> = sections
            .next()
            .unwrap()
            .lines()
            .map(|f| Field::parse(f))
            .collect();

        let mine: Vec<usize> = sections
            .next()
            .unwrap()
            .lines()
            .skip(1)
            .next()
            .unwrap()
            .split(",")
            .map(|v| v.parse().unwrap())
            .collect();

        let tickets: Vec<Vec<usize>> = sections
            .next()
            .unwrap()
            .lines()
            .skip(1)
            .map(|t| t.split(",").map(|v| v.parse().unwrap()).collect())
            .collect();

        Self {
            fields: fields,
            my_ticket: mine,
            tickets: tickets,
        }
    }
}

fn error_rate(info: &TicketInfo) -> usize {
    // set of all value that fall in any of the valid ranges
    let mut valid = HashSet::new();
    for field in info.fields.iter() {
        for i in field.lower.0..=field.lower.1 {
            valid.insert(i);
        }
        for i in field.upper.0..=field.upper.1 {
            valid.insert(i);
        }
    }

    info.tickets
        .iter()
        .flat_map(|t| t.iter().filter(|v| !valid.contains(v)))
        .sum()
}

fn main() {
    let input = fs::read_to_string("../inputs/day16.txt").unwrap();
    let info = TicketInfo::parse(&input);

    println!("{}", error_rate(&info));
}

mod test {
    use super::*;

    #[test]
    fn test_part1() {
        let input = fs::read_to_string("../inputs/day16-test.txt").unwrap();
        let info = TicketInfo::parse(&input);

        assert_eq!(error_rate(&info), 71);
    }
}
