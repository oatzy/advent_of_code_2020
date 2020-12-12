use crate::Error;
use anyhow::Result;

pub trait Ship {
    fn north(&mut self, step: usize);

    fn south(&mut self, step: usize);

    fn east(&mut self, step: usize);

    fn west(&mut self, step: usize);

    fn left(&mut self, degrees: usize) -> Result<()>;

    fn right(&mut self, degrees: usize) -> Result<()>;

    fn forward(&mut self, step: usize);
}

pub fn move_from_instructions<S: Ship>(ship: &mut S, instructions: &str) -> Result<()> {
    for i in instructions.lines() {
        let size: usize = (&i[1..]).parse()?;

        match &i[..1] {
            "N" => ship.north(size),
            "S" => ship.south(size),
            "E" => ship.east(size),
            "W" => ship.west(size),
            "L" => ship.left(size)?,
            "R" => ship.right(size)?,
            "F" => ship.forward(size),
            _ => return Err(Error::InvalidInstruction(i.to_owned()).into()),
        }
    }
    Ok(())
}
