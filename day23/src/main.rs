use std::collections::VecDeque;
use std::fmt;

struct Cups {
    cups: VecDeque<usize>,
    hand: [usize; 3],
    round: usize,
    last_insert: usize,
}

impl From<Vec<usize>> for Cups {
    fn from(input: Vec<usize>) -> Self {
        Self {
            cups: input.into(),
            hand: [0; 3],
            round: 0,
            last_insert: 0,
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
    fn extend_to(&mut self, n: usize) {
        for i in (self.cups.len() + 1)..=n {
            self.cups.push_back(i);
        }
    }

    fn play1(&mut self) {
        let current = *self.cups.front().unwrap();

        // take 3 after current
        self.cups.rotate_left(1);
        for i in 0..3 {
            self.hand[2 - i] = self.cups.pop_front().unwrap();
        }

        // find current - 1 (or next available)
        let mut dest = if current > 1 {
            current - 1
        } else {
            self.cups.len() + 3
        };
        while self.hand.contains(&dest) {
            dest = if dest > 1 {
                dest - 1
            } else {
                self.cups.len() + 3
            };
        }

        let mut position = self.last_insert;

        while self.cups[position] != dest {
            position = if position > 0 {
                position - 1
            } else {
                self.cups.len() - 1
            }
        }

        //println!("{}", position);
        for i in self.hand.iter() {
            self.cups.insert((position + 1) % self.cups.len(), *i)
        }

        self.last_insert = position;
    }

    fn playn(&mut self, n: usize) {
        for i in 0..n {
            //println!("round {}", i);
            if i % 10_000 == 0 {
                println!("round {}", i);
                println!("{}", self.last_insert);
                // println!("{:?}", self.hand);
                // println!("{:?}", self.cups.iter().take(10).collect::<Vec<_>>());
                println!("{:?}", self.find_stars());
            }
            self.play1();
        }
    }

    fn find_stars(&self) -> (usize, usize) {
        let mut inx = 0;
        while self.cups[inx] != 1 {
            inx += 1;
        }
        println!("{}", inx);
        (
            self.cups[(inx + 1) % self.cups.len()],
            self.cups[(inx + 2) % self.cups.len()],
        )
    }
}

struct VecCups {
    cups: Vec<usize>,
    hand: [usize; 3],
    head: usize,
}

impl fmt::Display for VecCups {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut i = 0;
        while self.cups[i] != 1 {
            i = (i + 1) % self.cups.len();
        }
        i = (i + 1) % self.cups.len();
        let values = self
            .cups
            .iter()
            .skip(i)
            .chain(self.cups.iter().take(if i == 0 { 0 } else { i - 1 }))
            .map(|x| x.to_string())
            .collect::<Vec<String>>()
            .join("");

        write!(f, "{}", values)
    }
}

impl VecCups {
    fn new() -> Self {
        Self {
            cups: Vec::new(),
            hand: [0, 0, 0],
            head: 0,
        }
    }

    fn play1(&mut self) {
        let current = self.cups[self.head];

        let len = self.cups.len();
        let mut offset = (self.head + 1) % len;

        // take 3 after current
        for i in 0..3 {
            self.hand[i] = self.cups[(offset + i) % len];
        }

        // find current - 1 (or next available)
        let mut dest = if current > 1 { current - 1 } else { len };
        while self.hand.contains(&dest) {
            dest = if dest > 1 { dest - 1 } else { len };
        }

        // shift everything right 3 until we find dest
        while offset != self.head {
            self.cups[offset] = self.cups[(offset + 3) % len];
            if self.cups[offset] == dest {
                break;
            }
            offset = (offset + 1) % len;
        }
        assert!(offset != self.head);

        // insert hand after dest
        for (i, x) in self.hand.iter().enumerate() {
            self.cups[(offset + i + 1) % len] = *x;
        }

        // move to one after current
        self.head = (self.head + 1) % len;
    }

    fn playn(&mut self, n: usize) {
        for i in 0..n {
            if i % 10_000 == 0 {
                println!("round {}", i);
                // println!("{:?}", self.hand);
                // println!("{:?}", self.cups.iter().take(10).collect::<Vec<_>>());
                println!("{:?}", self.find_stars());
            }
            self.play1();
        }
    }

    fn find_stars(&self) -> (usize, usize) {
        let mut inx = 0;
        while self.cups[inx] != 1 {
            inx += 1;
        }
        println!("{}", inx);
        (
            self.cups[(inx + 1) % self.cups.len()],
            self.cups[(inx + 2) % self.cups.len()],
        )
    }
}

fn main() {
    // part1
    {
        let mut cups: VecCups = VecCups::new();
        cups.cups.extend(vec![9, 6, 2, 7, 1, 3, 8, 5, 4]);
        cups.playn(100);
        println!("{}", cups);
    }

    // part2
    // {
    //     let mut cups: Cups = vec![9, 6, 2, 7, 1, 3, 8, 5, 4].into();
    //     cups.extend_to(1_000_000);

    //     cups.playn(10_000_000);
    //     let stars = cups.find_stars();

    //     println!("{:?}", stars);
    //     println!("{}", stars.0 * stars.1);
    // }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_part1() {
        let mut cups: Cups = vec![3, 8, 9, 1, 2, 5, 4, 6, 7].into();

        cups.playn(10);
        assert_eq!(format!("{}", cups), "92658374");

        cups.playn(90);
        assert_eq!(format!("{}", cups), "67384529");
    }

    #[test]
    fn test_part1_vec() {
        let mut cups = VecCups::new();
        cups.cups.extend(vec![3, 8, 9, 1, 2, 5, 4, 6, 7]);

        cups.playn(10);
        assert_eq!(format!("{}", cups), "92658374");

        cups.playn(90);
        assert_eq!(format!("{}", cups), "67384529");
    }

    #[test]
    fn test_part2() {
        let mut cups: Cups = vec![3, 8, 9, 1, 2, 5, 4, 6, 7].into();
        cups.extend_to(1_000_000);

        cups.playn(10_000_000);
        let stars = cups.find_stars();

        assert_eq!(stars, (934001, 159792));
        assert_eq!(stars.0 * stars.1, 149245887792);
    }

    #[test]
    fn test_part2_vec() {
        let mut cups = VecCups::new();
        cups.cups.extend(vec![3, 8, 9, 1, 2, 5, 4, 6, 7]);
        cups.cups.extend(10..=1_000_000);

        cups.playn(10_000_000);
        let stars = cups.find_stars();

        assert_eq!(stars, (934001, 159792));
        assert_eq!(stars.0 * stars.1, 149245887792);
    }
}
