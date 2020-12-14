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

// struct VariantIter {
//     head: Vec<String>,
//     tail: Option<Box<VariantIter>>,
//     inx: usize,
// }

// impl VariantIter {
//     fn new(s: String) -> Self {
//         if s.len() == 0 {
//             panic!("Got empty string");
//         }
//         let head = &s[..1];
//         let head = if head != "X" {
//             vec![head.to_string()]
//         } else {
//             vec!["0".to_string(), "1".to_string()]
//         };
//         let tail = if s[1..].len() == 0 {
//             None
//         } else {
//             Some(Box::new(VariantIter::new(s[1..].to_string())))
//         };
//         Self {
//             head: head,
//             tail: tail,
//             inx: 0,
//         }
//     }
// }

// impl Iterator for VariantIter {
//     type Item = String;

//     fn next(&mut self) -> Option<Self::Item> {
//         if self.inx >= self.head.len() {
//             return None;
//         }

//         let c = self.head.get(self.inx).unwrap();

//         if self.tail.is_none() {
//             self.inx += 1;
//             return Some(c.to_owned());
//         }

//         let t = self.tail.as_mut().unwrap().next();
//         if t.is_none() {
//             self.inx += 1;
//             if self.inx >= self.head.len() {
//                 return None;
//             }
//             return self.next();
//         }

//         Some(c.to_string() + &(t.unwrap()))
//     }
// }

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

    // #[test]
    // fn test_variant_iter() {
    //     assert_eq!(
    //         VariantIter::new("0".to_string()).collect::<Vec<String>>(),
    //         vec!["0"]
    //     );

    //     assert_eq!(
    //         VariantIter::new("1".to_string()).collect::<Vec<String>>(),
    //         vec!["1"]
    //     );

    //     assert_eq!(
    //         VariantIter::new("X".to_string()).collect::<Vec<String>>(),
    //         vec!["0", "1"]
    //     );

    //     assert_eq!(
    //         VariantIter::new("X1".to_string()).collect::<Vec<String>>(),
    //         vec!["01", "11"]
    //     );

    //     assert_eq!(
    //         VariantIter::new("0X".to_string()).collect::<Vec<String>>(),
    //         vec!["00", "01"]
    //     );
    // }
}
