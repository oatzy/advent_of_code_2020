use std::collections::HashSet;
use std::fs;

use anyhow::Result;

static PATHS: [(usize, usize); 5] = [(1, 1), (3, 1), (5, 1), (7, 1), (1, 2)];

struct Map {
    trees: HashSet<(usize, usize)>,
    width: usize,
    height: usize,
}

fn parse_map(map: &str) -> Map {
    let mut trees = HashSet::new();
    let mut width = 0;
    let mut height = 0;

    for (y, line) in map.lines().enumerate() {
        width = line.len();
        for (x, char) in line.chars().enumerate() {
            if char == '#' {
                trees.insert((x, y));
            }
        }
        height = y; // cheeky
    }

    Map {
        trees: trees,
        width: width,
        height: height,
    }
}

fn trees_in_path(map: &Map, right: usize, down: usize) -> usize {
    let mut total = 0;

    let mut x = 0;
    let mut y = 0;

    while y <= map.height {
        if map.trees.contains(&(x, y)) {
            total += 1;
        }
        x = (x + right) % map.width;
        y += down;
    }

    total
}

fn product_of_paths(map: &Map) -> usize {
    PATHS
        .iter()
        .map(|(right, down)| trees_in_path(&map, *right, *down))
        .product()
}

fn main() -> Result<()> {
    let input = fs::read_to_string("../inputs/day03.txt")?;
    let map = parse_map(&input);

    let part1 = trees_in_path(&map, 3, 1);
    println!("{}", part1);

    let part2 = product_of_paths(&map);
    println!("{}", part2);

    Ok(())
}

mod test {

    use super::*;

    #[test]
    fn test_part1() {
        let input = fs::read_to_string("../inputs/day03-test.txt").unwrap();
        let map = parse_map(&input);

        let trees = trees_in_path(&map, 3, 1);
        assert_eq!(trees, 7);
    }

    #[test]
    fn test_part2() {
        let input = fs::read_to_string("../inputs/day03-test.txt").unwrap();
        let map = parse_map(&input);

        let result = product_of_paths(&map);
        assert_eq!(result, 336);
    }
}
