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

    fn left(&mut self, degrees: usize) {
        self.facing = match (self.facing, degrees) {
            (Facing::North, 90) | (Facing::South, 270) | (Facing::East, 180) => Facing::West,
            (Facing::North, 180) | (Facing::West, 90) | (Facing::East, 270) => Facing::South,
            (Facing::North, 270) | (Facing::West, 180) | (Facing::South, 90) => Facing::East,
            (Facing::West, 270) | (Facing::South, 180) | (Facing::East, 90) => Facing::North,
            _ => panic!("Unexpected left rotation {}", degrees),
        };
    }

    fn right(&mut self, degrees: usize) {
        self.facing = match (self.facing, degrees) {
            (Facing::North, 270) | (Facing::South, 90) | (Facing::East, 180) => Facing::West,
            (Facing::North, 180) | (Facing::West, 270) | (Facing::East, 90) => Facing::South,
            (Facing::North, 90) | (Facing::West, 180) | (Facing::South, 270) => Facing::East,
            (Facing::West, 90) | (Facing::South, 180) | (Facing::East, 270) => Facing::North,
            _ => panic!("Unexpected right rotation {}", degrees),
        };
    }

    fn forward(&mut self, step: usize) {
        match self.facing {
            Facing::North => self.north += step as isize,
            Facing::South => self.north -= step as isize,
            Facing::East => self.east += step as isize,
            Facing::West => self.east -= step as isize,
        }
    }

    pub fn distance(&self) -> usize {
        (self.east.abs() + self.north.abs()) as usize
    }

    pub fn move_from_instructions(&mut self, instructions: &str) {
        for i in instructions.lines() {
            let size: usize = (&i[1..]).parse().unwrap();

            match &i[..1] {
                "N" => self.north(size),
                "S" => self.south(size),
                "E" => self.east(size),
                "W" => self.west(size),
                "L" => self.left(size),
                "R" => self.right(size),
                "F" => self.forward(size),
                _ => panic!("got unexpected instruction {}", i),
            }
        }
    }
}
