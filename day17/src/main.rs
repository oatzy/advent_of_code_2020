use std::fs;

mod cubes;

use cubes::{Cube, HyperCube};

fn part1(input: &str) -> usize {
    let mut cube = Cube::parse(input);

    for _ in 0..6 {
        cube = cube.iterate();
    }

    cube.energy()
}

fn part2(input: &str) -> usize {
    let mut cube = HyperCube::parse(input);

    for _ in 0..6 {
        cube = cube.iterate();
    }

    cube.energy()
}

fn main() {
    let input = fs::read_to_string("../inputs/day17.txt").unwrap();

    println!("{}", part1(&input));
    println!("{}", part2(&input));
}

mod test {
    use super::*;

    #[test]
    fn test_part1() {
        let input = fs::read_to_string("../inputs/day17-test.txt").unwrap();
        assert_eq!(part1(&input), 112);
    }

    #[test]
    fn test_part2() {
        let input = fs::read_to_string("../inputs/day17-test.txt").unwrap();
        assert_eq!(part2(&input), 848);
    }
}
