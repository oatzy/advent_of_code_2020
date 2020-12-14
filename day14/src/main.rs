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
}

fn set_value_v1(decoder: &mut Decoder, index: u64, value: u64) {
    decoder
        .memory
        .insert(index, apply_mask(&decoder.mask, value));
}

fn set_value_v2(decoder: &mut Decoder, index: u64, value: u64) {
    let index = apply_mask(&decoder.mask.replace("0", "X"), index);

    for inx in apply_variants(&decoder.mask, index) {
        decoder.memory.insert(inx, value);
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

fn apply_variants(mask: &String, value: u64) -> Vec<u64> {
    let mut values = vec![value];
    for (inx, _) in mask.chars().rev().enumerate().filter(|(_, c)| c == &'X') {
        let mut tmp = Vec::new();
        for v in values {
            tmp.push(v & !(1 << inx));
            tmp.push(v | (1 << inx));
        }
        values = tmp;
    }
    values
}

fn execute<F>(input: &str, set_value: F) -> u64
where
    F: Fn(&mut Decoder, u64, u64),
{
    let mut decoder = Decoder::new();

    for line in input.lines() {
        if line.starts_with("mem") {
            let parts: Vec<&str> = line[4..].split("] = ").collect();
            let index: u64 = parts[0].parse().unwrap();
            let value: u64 = parts[1].parse().unwrap();

            set_value(&mut decoder, index, value);
        } else {
            decoder.mask = line.split(" = ").skip(1).next().unwrap().into();
        }
    }

    decoder.memory.values().sum()
}

fn main() {
    let input = fs::read_to_string("../inputs/day14.txt").unwrap();

    println!("{}", execute(&input, set_value_v1));
    println!("{}", execute(&input, set_value_v2));
}

mod test {
    use super::*;

    #[test]
    fn test_part1() {
        let input = fs::read_to_string("../inputs/day14-test.txt").unwrap();

        assert_eq!(execute(&input, set_value_v1), 165);
    }

    #[test]
    fn test_part2() {
        let input = fs::read_to_string("../inputs/day14-test2.txt").unwrap();

        assert_eq!(execute(&input, set_value_v2), 208);
    }
}
