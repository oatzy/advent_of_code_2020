use std::fs;

mod ships;

use ships::{FacingShip, WayPointShip};

fn main() {
    let input = fs::read_to_string("../inputs/day12.txt").unwrap();

    let mut ship = FacingShip::new();
    ship.move_from_instructions(&input);
    println!("{}", ship.distance());

    let mut ship = WayPointShip::new();
    ship.move_from_instructions(&input);
    println!("{}", ship.distance());
}

mod test {
    use super::*;

    #[test]
    fn test_part1() {
        let input = fs::read_to_string("../inputs/day12-test.txt").unwrap();

        let mut ship = FacingShip::new();
        ship.move_from_instructions(&input);

        assert_eq!(ship.distance(), 25);
    }

    #[test]
    fn test_part2() {
        let input = fs::read_to_string("../inputs/day12-test.txt").unwrap();

        let mut ship = WayPointShip::new();
        ship.move_from_instructions(&input);

        assert_eq!(ship.distance(), 286);
    }
}
