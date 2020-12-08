use std::collections::HashSet;
use std::fs;

#[derive(Debug, Copy, Clone)]
enum Instruction {
    Acc(isize),
    Jmp(isize),
    Nop(isize),
}

#[derive(Debug, Clone)]
struct Computer {
    accumulator: isize,
    instructions: Vec<Instruction>,
}

#[derive(Debug, Eq, PartialEq)]
enum Exit {
    Loop,
    Ok,
}

impl Computer {
    fn reset(&mut self) {
        self.accumulator = 0;
    }

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

    fn run(&mut self) -> Exit {
        let mut seen = HashSet::new();
        let mut inx = 0;

        while !seen.contains(&inx) {
            if inx >= self.instructions.len() {
                return Exit::Ok;
            }

            seen.insert(inx);

            match self.instructions.get(inx as usize).unwrap() {
                Instruction::Nop(_) => inx += 1,
                Instruction::Acc(amount) => {
                    self.accumulator += amount;
                    inx += 1
                }
                Instruction::Jmp(step) => inx = (inx as isize + step) as usize,
            }
        }

        Exit::Loop
    }

    fn mutate_instruction(&self, inx: usize) -> Self {
        // return a clone of the computer which has the instruction at inx mutated
        // replaces Jmp <-> Nop, panics on Acc
        let mut mutated = self.clone();
        match mutated.instructions.get(inx).unwrap() {
            Instruction::Acc(_) => panic!("Can't mutate acc instructions"),
            Instruction::Nop(x) => {
                *(mutated.instructions.get_mut(inx).unwrap()) = Instruction::Jmp(*x)
            }
            Instruction::Jmp(x) => {
                *(mutated.instructions.get_mut(inx).unwrap()) = Instruction::Nop(*x)
            }
        }
        mutated
    }
}

fn find_instruction_fix_accumulator(computer: &Computer) -> isize {
    // try flipping Nop/Jmp instructions until we find one which doesn't loop
    // then returns the accumulator value for that mutant at exit
    for (inx, instruction) in computer.instructions.iter().enumerate() {
        if let Instruction::Acc(_) = instruction {
            continue;
        }

        let mut mutated = computer.mutate_instruction(inx);
        if mutated.run() == Exit::Ok {
            return mutated.accumulator;
        }
    }
    unreachable!("Didn't find any valid mutations");
}

fn main() {
    let input = fs::read_to_string("../inputs/day08.txt").unwrap();
    let mut computer = Computer::load_from_string(&input);

    let result = computer.run();
    assert_eq!(result, Exit::Loop);

    println!("{}", computer.accumulator);

    computer.reset();

    let part2 = find_instruction_fix_accumulator(&computer);
    println!("{}", part2);
}

mod test {
    use super::*;

    #[test]
    fn test_part1() {
        let input = fs::read_to_string("../inputs/day08-test.txt").unwrap();
        let mut computer = Computer::load_from_string(&input);

        computer.run();

        assert_eq!(computer.accumulator, 5);
    }

    #[test]
    fn test_part2() {
        let input = fs::read_to_string("../inputs/day08-test.txt").unwrap();
        let mut computer = Computer::load_from_string(&input);

        let result = find_instruction_fix_accumulator(&computer);

        assert_eq!(result, 8);
    }
}
