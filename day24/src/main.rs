use std::collections::HashSet;
use std::fs;

static STEPS: [(isize, isize); 6] = [(1, -1), (0, -1), (-1, 0), (1, 0), (-1, 1), (0, 1)];

type P = (isize, isize);

struct AdjacentIter {
    inx: usize,
    p: P,
}

impl Iterator for AdjacentIter {
    type Item = P;

    fn next(&mut self) -> Option<Self::Item> {
        if self.inx >= STEPS.len() {
            return None;
        }

        let step = STEPS[self.inx];
        self.inx += 1;

        Some((self.p.0 + step.0, self.p.1 + step.1))
    }
}

fn adjacent(p: P) -> AdjacentIter {
    AdjacentIter { inx: 0, p: p }
}

struct Tiles {
    black: HashSet<P>,
    xrange: (isize, isize),
    yrange: (isize, isize),
}

impl Tiles {
    fn new() -> Self {
        Self {
            black: HashSet::new(),
            xrange: (0, 0),
            yrange: (0, 0),
        }
    }

    fn black_tiles(&self) -> usize {
        self.black.len()
    }

    fn flip_tile(&mut self, directions: &str) {
        let mut i = 0;
        let mut pos = (0, 0);

        while i < directions.len() {
            if &directions[i..i + 1] == "n" || &directions[i..i + 1] == "s" {
                match &directions[i..i + 2] {
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
                match &directions[i..i + 1] {
                    "e" => pos.0 += 1,
                    "w" => pos.0 -= 1,
                    d => panic!("unexpected direction {}", d),
                }
                i += 1;
            }
        }

        if !self.black.remove(&pos) {
            self.black.insert(pos);
        }

        self.xrange = max_range(self.xrange, pos.0);
        self.yrange = max_range(self.yrange, pos.1);
    }

    fn iterate(&mut self) {
        let mut new = HashSet::new();

        let mut xrange = self.xrange;
        let mut yrange = self.yrange;

        for x in (self.xrange.0 - 1)..=(self.xrange.1 + 1) {
            for y in (self.yrange.0 - 1)..=(self.yrange.1 + 1) {
                let is_black = self.black.contains(&(x, y));

                let black_count = adjacent((x, y)).filter(|p| self.black.contains(p)).count();

                if black_count == 2 || (is_black && black_count == 1) {
                    new.insert((x, y));

                    xrange = max_range(xrange, x);
                    yrange = max_range(yrange, y);
                }
            }
        }

        self.black = new;
        self.xrange = xrange;
        self.yrange = yrange;
    }
}

fn max_range(current: (isize, isize), new: isize) -> (isize, isize) {
    (current.0.min(new), current.1.max(new))
}

fn flip_tiles(input: &str) -> Tiles {
    let mut tiles = Tiles::new();

    for line in input.lines() {
        tiles.flip_tile(line);
    }
    tiles
}

fn main() {
    let input = fs::read_to_string("../inputs/day24.txt").unwrap();

    let mut tiles = flip_tiles(&input);
    println!("{}", tiles.black_tiles());

    for _ in 0..100 {
        tiles.iterate();
    }
    println!("{}", tiles.black_tiles());
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_part1() {
        let input = fs::read_to_string("../inputs/day24-test.txt").unwrap();

        let tiles = flip_tiles(&input);
        assert_eq!(tiles.black_tiles(), 10);
    }

    #[test]
    fn test_part2() {
        let input = fs::read_to_string("../inputs/day24-test.txt").unwrap();

        let mut tiles = flip_tiles(&input);

        for (day, expect) in vec![15, 12, 25, 14, 23, 28, 41, 37, 49, 37]
            .iter()
            .cloned()
            .enumerate()
        {
            tiles.iterate();
            assert_eq!(tiles.black_tiles(), expect, "day {}", day + 1);
        }

        for (day, expect) in vec![132, 259, 406, 566, 788, 1106, 1373, 1844, 2208]
            .iter()
            .cloned()
            .enumerate()
        {
            for _ in 0..10 {
                tiles.iterate();
            }
            assert_eq!(tiles.black_tiles(), expect, "day {}", 10 * day + 20);
        }
    }
}
