use crate::ships::traits::Ship;
use crate::Error;

use anyhow::Result;

#[derive(Copy, Clone)]
enum Facing {
    North,
    South,
    East,
    West,
}

pub struct FacingShip {
    facing: Facing,
    east: isize,
    north: isize,
}

impl FacingShip {
    pub fn new() -> Self {
        FacingShip {
            facing: Facing::East,
            east: 0,
            north: 0,
        }
    }

    pub fn distance(&self) -> usize {
        (self.east.abs() + self.north.abs()) as usize
    }
}

impl Ship for FacingShip {
    fn north(&mut self, step: usize) {
        self.north += step as isize;
    }

    fn south(&mut self, step: usize) {
        self.north -= step as isize;
    }

    fn east(&mut self, step: usize) {
        self.east += step as isize;
    }

    fn west(&mut self, step: usize) {
        self.east -= step as isize;
    }

    fn left(&mut self, degrees: usize) -> Result<()> {
        self.facing = match (self.facing, degrees) {
            (Facing::North, 90) | (Facing::South, 270) | (Facing::East, 180) => Facing::West,
            (Facing::North, 180) | (Facing::West, 90) | (Facing::East, 270) => Facing::South,
            (Facing::North, 270) | (Facing::West, 180) | (Facing::South, 90) => Facing::East,
            (Facing::West, 270) | (Facing::South, 180) | (Facing::East, 90) => Facing::North,
            _ => return Err(Error::UnsupportedRotation(degrees).into()),
        };
        Ok(())
    }

    fn right(&mut self, degrees: usize) -> Result<()> {
        self.facing = match (self.facing, degrees) {
            (Facing::North, 270) | (Facing::South, 90) | (Facing::East, 180) => Facing::West,
            (Facing::North, 180) | (Facing::West, 270) | (Facing::East, 90) => Facing::South,
            (Facing::North, 90) | (Facing::West, 180) | (Facing::South, 270) => Facing::East,
            (Facing::West, 90) | (Facing::South, 180) | (Facing::East, 270) => Facing::North,
            _ => return Err(Error::UnsupportedRotation(degrees).into()),
        };
        Ok(())
    }

    fn forward(&mut self, step: usize) {
        match self.facing {
            Facing::North => self.north += step as isize,
            Facing::South => self.north -= step as isize,
            Facing::East => self.east += step as isize,
            Facing::West => self.east -= step as isize,
        }
    }
}
