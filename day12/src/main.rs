use std::fs;

use anyhow;
use thiserror::Error;

mod ships;

use ships::{move_from_instructions, FacingShip, WayPointShip};

#[derive(Error, Debug)]
pub enum Error {
    #[error("Got invalid instruction '{0}'")]
    InvalidInstruction(String),
    #[error("Unsupported rotation {0} degrees")]
    UnsupportedRotation(usize),
}

fn main() -> anyhow::Result<()> {
    let input = fs::read_to_string("../inputs/day12.txt")?;

    let mut ship = FacingShip::new();
    move_from_instructions(&mut ship, &input)?;
    println!("{}", ship.distance());

    let mut ship = WayPointShip::new();
    move_from_instructions(&mut ship, &input)?;
    println!("{}", ship.distance());

    Ok(())
}

mod test {
    use super::*;

    #[test]
    fn test_part1() {
        let input = fs::read_to_string("../inputs/day12-test.txt").unwrap();

        let mut ship = FacingShip::new();
        move_from_instructions(&mut ship, &input).unwrap();

        assert_eq!(ship.distance(), 25);
    }

    #[test]
    fn test_part2() {
        let input = fs::read_to_string("../inputs/day12-test.txt").unwrap();

        let mut ship = WayPointShip::new();
        move_from_instructions(&mut ship, &input).unwrap();

        assert_eq!(ship.distance(), 286);
    }
}
