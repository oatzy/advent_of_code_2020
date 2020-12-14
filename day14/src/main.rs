use std::collections::HashMap;
use std::fs;

struct Decoder {
    mask: String,
    memory: HashMap<u64, u64>,
}

impl Decoder {
    fn new() -> Self {
        Self {
            mask: "X".repeat(36),
            memory: HashMap::new(),
        }
    }

    fn change_mask<S: Into<String>>(&mut self, mask: S) {
        self.mask = mask.into();
    }

    fn set(&mut self, index: u64, value: u64) {
        self.memory.insert(index, apply_mask(&self.mask, value));
    }
}

fn apply_mask(mask: &String, value: u64) -> u64 {
    let mut value: u64 = value;
    for (inx, c) in mask.chars().rev().enumerate().filter(|(_, c)| c != &'X') {
        value = match c {
            '0' => value & !(1 << inx),
            '1' => value | (1 << inx),
            _ => panic!("Got unexpected bit '{}'", c),
        }
    }
    value
}

fn part1(input: &str) -> u64 {
    let mut decoder = Decoder::new();

    for line in input.lines() {
        if line.starts_with("mem") {
            let parts: Vec<&str> = line[4..].split("] = ").collect();
            let index: u64 = parts[0].parse().unwrap();
            let value: u64 = parts[1].parse().unwrap();

            decoder.set(index, value);
        } else {
            decoder.change_mask(line.split(" = ").skip(1).next().unwrap());
        }
    }

    decoder.memory.values().sum()
}

fn main() {
    let input = fs::read_to_string("../inputs/day14.txt").unwrap();

    println!("{}", part1(&input));
}

mod test {
    use super::*;

    #[test]
    fn test_part1() {
        let input = fs::read_to_string("../inputs/day14-test.txt").unwrap();

        assert_eq!(part1(&input), 165);
    }
}
