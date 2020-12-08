use std::collections::HashSet;
use std::fs;

#[derive(Debug, Copy, Clone)]
enum Instruction {
    Acc(isize),
    Jmp(isize),
    Nop(isize),
}

struct Computer {
    accumulator: isize,
    instructions: Vec<Instruction>,
}

impl Computer {
    fn load_from_string(input: &str) -> Self {
        let mut instructions = Vec::new();

        for line in input.lines() {
            let fields: Vec<&str> = line.splitn(2, " ").collect();
            let value: isize = fields[1].parse().unwrap();

            instructions.push(match fields[0] {
                "nop" => Instruction::Nop(value),
                "acc" => Instruction::Acc(value),
                "jmp" => Instruction::Jmp(value),
                _ => unreachable!(format!("Got unexpected instruction {}", line)),
            });
        }

        Computer {
            accumulator: 0,
            instructions: instructions,
        }
    }

    fn run_until_loop(&mut self) {
        let mut seen = HashSet::new();
        let mut inx: isize = 0;

        while !seen.contains(&inx) {
            seen.insert(inx);
            match self.instructions.get(inx as usize).unwrap() {
                Instruction::Nop(_) => inx += 1,
                Instruction::Acc(amount) => {
                    self.accumulator += amount;
                    inx += 1
                }
                Instruction::Jmp(step) => inx += step,
            }
        }
    }
}

fn main() {
    let input = fs::read_to_string("../inputs/day08.txt").unwrap();
    let mut computer = Computer::load_from_string(&input);

    computer.run_until_loop();

    println!("{}", computer.accumulator);
}

mod test {
    use super::*;

    #[test]
    fn test_part1() {
        let input = fs::read_to_string("../inputs/day08-test.txt").unwrap();
        let mut computer = Computer::load_from_string(&input);

        computer.run_until_loop();

        assert_eq!(computer.accumulator, 5);
    }
}
