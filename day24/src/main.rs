use std::collections::HashSet;
use std::fs;

fn flip_tiles(input: &str) -> HashSet<(isize, isize)> {
    let mut tiles = HashSet::new();

    for line in input.lines() {
        let mut i = 0;
        let mut pos = (0, 0);

        while i < line.len() {
            if &line[i..i + 1] == "n" || &line[i..i + 1] == "s" {
                match &line[i..i + 2] {
                    "nw" => {
                        pos.0 -= 1;
                        pos.1 += 1
                    }
                    "ne" => pos.1 += 1,
                    "sw" => pos.1 -= 1,
                    "se" => {
                        pos.0 += 1;
                        pos.1 -= 1;
                    }
                    d => panic!("unexpected direction {}", d),
                }
                i += 2;
            } else {
                match &line[i..i + 1] {
                    "e" => pos.0 += 1,
                    "w" => pos.0 -= 1,
                    d => panic!("unexpected direction {}", d),
                }
                i += 1;
            }
        }

        if !tiles.remove(&pos) {
            tiles.insert(pos);
        }
    }

    tiles
}

fn main() {
    let input = fs::read_to_string("../inputs/day24.txt").unwrap();

    let tiles = flip_tiles(&input);
    println!("{}", tiles.len());
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_part1() {
        let input = fs::read_to_string("../inputs/day24-test.txt").unwrap();

        let tiles = flip_tiles(&input);
        assert_eq!(tiles.len(), 10);
    }
}
