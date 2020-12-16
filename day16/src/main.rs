use std::collections::{HashMap, HashSet};
use std::fs;

#[derive(Debug)]
struct Field {
    name: String,
    lower: (usize, usize),
    upper: (usize, usize),
    index: Option<usize>,
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
            index: None,
        }
    }

    fn is_valid(&self, value: usize) -> bool {
        (value >= self.lower.0 && value <= self.lower.1)
            || (value >= self.upper.0 && value <= self.upper.1)
    }
}

#[derive(Debug)]
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

fn find_field_mapping(info: &mut TicketInfo) {
    // remove invlid tickets
    let tickets = info
        .tickets
        .iter()
        .filter(|t| t.iter().all(|v| info.fields.iter().any(|f| f.is_valid(*v))));

    // build a map of fields to possible columns
    let count = info.fields.len();
    let mut possibles: HashMap<usize, HashSet<usize>> =
        (0..count).map(|i| (i, (0..count).collect())).collect();

    for ticket in tickets {
        for (inx, v) in ticket.iter().enumerate() {
            for (jnx, f) in info.fields.iter().enumerate() {
                if !f.is_valid(*v) {
                    possibles.get_mut(&jnx).unwrap().remove(&inx);
                }
            }
        }
    }

    // identify and set fields which have only one match
    let mut all_set = false;
    while !all_set {
        let mut new = possibles.clone();
        all_set = true;

        for (f, pos) in possibles.iter() {
            if pos.len() == 1 {
                let index = pos.iter().next().unwrap();
                info.fields[*f].index = Some(*index);

                for (_, qos) in new.iter_mut() {
                    qos.remove(index);
                }
            } else if pos.len() != 0 {
                all_set = false;
            }
        }

        possibles = new;
    }
}

fn main() {
    let input = fs::read_to_string("../inputs/day16.txt").unwrap();
    let mut info = TicketInfo::parse(&input);

    println!("{}", error_rate(&info));

    find_field_mapping(&mut info);

    let part2: usize = info
        .fields
        .iter()
        .filter(|f| f.name.starts_with("departure"))
        .map(|f| info.my_ticket[f.index.unwrap()])
        .product();

    println!("{}", part2);
}

mod test {
    use super::*;

    #[test]
    fn test_part1() {
        let input = fs::read_to_string("../inputs/day16-test.txt").unwrap();
        let info = TicketInfo::parse(&input);

        assert_eq!(error_rate(&info), 71);
    }

    #[test]
    fn test_part2() {
        let input = fs::read_to_string("../inputs/day16-test2.txt").unwrap();
        let mut info = TicketInfo::parse(&input);

        find_field_mapping(&mut info);

        assert_eq!(info.fields[1].index, Some(0));
        assert_eq!(info.fields[0].index, Some(1));
        assert_eq!(info.fields[2].index, Some(2));
    }
}
