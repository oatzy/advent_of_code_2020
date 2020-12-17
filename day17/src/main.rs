use std::collections::HashSet;
use std::fs;

static STEPS: [(isize, isize, isize); 26] = [
    (-1, -1, -1),
    (-1, -1, 0),
    (-1, -1, 1),
    (-1, 0, -1),
    (-1, 0, 0),
    (-1, 0, 1),
    (-1, 1, -1),
    (-1, 1, 0),
    (-1, 1, 1),
    (0, -1, -1),
    (0, -1, 0),
    (0, -1, 1),
    (0, 0, -1),
    (0, 0, 1),
    (0, 1, -1),
    (0, 1, 0),
    (0, 1, 1),
    (1, -1, -1),
    (1, -1, 0),
    (1, -1, 1),
    (1, 0, -1),
    (1, 0, 0),
    (1, 0, 1),
    (1, 1, -1),
    (1, 1, 0),
    (1, 1, 1),
];

type P = (isize, isize, isize);

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

        Some((self.p.0 + step.0, self.p.1 + step.1, self.p.2 + step.2))
    }
}

fn adjacent(p: P) -> AdjacentIter {
    AdjacentIter { inx: 0, p: p }
}

#[derive(Debug, Clone)]
struct Cube {
    active: HashSet<P>,
    xrange: (isize, isize),
    yrange: (isize, isize),
    zrange: (isize, isize),
}

impl Cube {
    fn parse(initial: &str) -> Self {
        let mut xmax = 0;
        let mut ymax = 0;

        let mut active = HashSet::new();

        for (y, line) in initial.lines().enumerate() {
            xmax = line.len();
            for (x, char) in line.chars().enumerate() {
                if char == '#' {
                    active.insert((x as isize, y as isize, 0));
                }
            }
            ymax = y + 1;
        }

        Self {
            active: active,
            xrange: (0, xmax as isize + 1),
            yrange: (0, ymax as isize + 1),
            zrange: (0, 1),
        }
    }
}

fn iterate(cube: Cube) -> Cube {
    let mut active = HashSet::new();

    for x in (cube.xrange.0 - 1)..(cube.xrange.1 + 1) {
        for y in (cube.yrange.0 - 1)..(cube.yrange.1 + 1) {
            for z in (cube.zrange.0 - 1)..(cube.zrange.1 + 1) {
                let is_active = cube.active.contains(&(x, y, z));

                let active_count = adjacent((x, y, z))
                    .filter(|p| cube.active.contains(p))
                    .count();

                if active_count == 3 || (is_active && active_count == 2) {
                    active.insert((x, y, z));
                }
            }
        }
    }

    Cube {
        active: active,
        xrange: (cube.xrange.0 - 1, cube.xrange.1 + 1),
        yrange: (cube.yrange.0 - 1, cube.yrange.1 + 1),
        zrange: (cube.zrange.0 - 1, cube.zrange.1 + 1),
    }
}

fn part1(cube: Cube) -> usize {
    let mut cube = cube;

    for _ in 0..6 {
        cube = iterate(cube);
    }

    cube.active.len()
}

fn main() {
    let input = fs::read_to_string("../inputs/day17.txt").unwrap();
    let cube = Cube::parse(&input);

    println!("{}", part1(cube));
}

mod test {
    use super::*;

    #[test]
    fn test_part1() {
        let input = fs::read_to_string("../inputs/day17-test.txt").unwrap();
        let cube = Cube::parse(&input);
        //println!("{:?}", cube);

        assert_eq!(part1(cube), 112);
    }
}
