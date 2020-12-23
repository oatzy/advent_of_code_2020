use std::collections::VecDeque;
use std::fmt;

struct Cups {
    cups: VecDeque<usize>,
    hand: [usize; 3],
}

impl From<Vec<usize>> for Cups {
    fn from(input: Vec<usize>) -> Self {
        Self {
            cups: input.into(),
            hand: [0, 0, 0],
        }
    }
}

impl fmt::Display for Cups {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut temp = self.cups.clone();
        while temp.front().unwrap() != &1 {
            temp.rotate_left(1);
        }
        write!(
            f,
            "{}",
            temp.iter()
                .skip(1)
                .map(|x| x.to_string())
                .collect::<Vec<String>>()
                .join("")
        )
    }
}

impl Cups {
    fn play1(&mut self) {
        let current = *self.cups.front().unwrap();

        // take 3 after current
        self.cups.rotate_left(1);
        for i in 0..3 {
            self.hand[i] = self.cups.pop_front().unwrap();
        }

        // find current - 1 (or next available)
        let mut dest = if current > 1 { current - 1 } else { 9 };
        while !self.cups.contains(&dest) {
            dest = if dest > 1 { dest - 1 } else { 9 };
        }

        while self.cups.back().unwrap() != &dest {
            self.cups.rotate_left(1)
        }

        // insert hand after dest
        for x in self.hand.iter() {
            self.cups.push_back(*x);
        }

        // rotate to one after current
        while self.cups.back().unwrap() != &current {
            self.cups.rotate_left(1);
        }
    }
}

fn main() {
    let mut cups: Cups = vec![9, 6, 2, 7, 1, 3, 8, 5, 4].into();

    for _ in 0..100 {
        cups.play1();
    }

    println!("{}", cups);
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_part1() {
        let mut cups: Cups = vec![3, 8, 9, 1, 2, 5, 4, 6, 7].into();

        for _ in 0..10 {
            cups.play1();
        }

        assert_eq!(format!("{}", cups), "92658374");

        for _ in 10..100 {
            cups.play1();
        }

        assert_eq!(format!("{}", cups), "67384529");
    }
}
