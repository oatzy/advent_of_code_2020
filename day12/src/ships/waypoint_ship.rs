use crate::Error;
use anyhow::Result;

struct WayPoint {
    east: isize,
    north: isize,
}

pub struct WayPointShip {
    wp: WayPoint,
    east: isize,
    north: isize,
}

impl WayPointShip {
    pub fn new() -> Self {
        WayPointShip {
            wp: WayPoint { east: 10, north: 1 },
            east: 0,
            north: 0,
        }
    }

    fn north(&mut self, step: usize) {
        self.wp.north += step as isize;
    }

    fn south(&mut self, step: usize) {
        self.wp.north -= step as isize;
    }

    fn east(&mut self, step: usize) {
        self.wp.east += step as isize;
    }

    fn west(&mut self, step: usize) {
        self.wp.east -= step as isize;
    }

    fn left(&mut self, degrees: usize) -> Result<()> {
        self.wp = match degrees {
            90 => WayPoint {
                north: self.wp.east,
                east: -self.wp.north,
            },
            180 => WayPoint {
                north: -self.wp.north,
                east: -self.wp.east,
            },
            270 => WayPoint {
                north: -self.wp.east,
                east: self.wp.north,
            },
            _ => return Err(Error::UnsupportedRotation(degrees).into()),
        };
        Ok(())
    }

    fn right(&mut self, degrees: usize) -> Result<()> {
        self.wp = match degrees {
            270 => WayPoint {
                north: self.wp.east,
                east: -self.wp.north,
            },
            180 => WayPoint {
                north: -self.wp.north,
                east: -self.wp.east,
            },
            90 => WayPoint {
                north: -self.wp.east,
                east: self.wp.north,
            },
            _ => return Err(Error::UnsupportedRotation(degrees).into()),
        };
        Ok(())
    }

    fn forward(&mut self, step: usize) {
        self.east += self.wp.east * step as isize;
        self.north += self.wp.north * step as isize;
    }

    pub fn distance(&self) -> usize {
        (self.east.abs() + self.north.abs()) as usize
    }

    pub fn move_from_instructions(&mut self, instructions: &str) -> Result<()> {
        for i in instructions.lines() {
            let size: usize = (&i[1..]).parse()?;

            match &i[..1] {
                "N" => self.north(size),
                "S" => self.south(size),
                "E" => self.east(size),
                "W" => self.west(size),
                "L" => self.left(size)?,
                "R" => self.right(size)?,
                "F" => self.forward(size),
                _ => return Err(Error::InvalidInstruction(i.to_owned()).into()),
            }
        }

        Ok(())
    }
}
