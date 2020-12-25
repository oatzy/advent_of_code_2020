use std::collections::VecDeque;
use std::fmt;

struct Cups {
    cups: VecDeque<usize>,
    hand: [usize; 3],
    last_insert: usize,
}

impl From<Vec<usize>> for Cups {
    fn from(input: Vec<usize>) -> Self {
        Self {
            cups: input.into(),
            hand: [0; 3],
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

        for i in self.hand.iter() {
            self.cups.insert((position + 1) % self.cups.len(), *i)
        }

        self.last_insert = position;
    }

    fn playn(&mut self, n: usize) {
        for _ in 0..n {
            self.play1();
        }
    }
}

/// doubly linked list, implemented as a Vec
/// Concept stolen from the advent of code reddit.

struct Cup {
    label: usize,
    prev_inx: usize,
    next_inx: usize,
}

struct LLCups {
    cups: Vec<Cup>,
    cur_inx: usize,
    size: usize,
}

impl LLCups {
    fn with_size(n: usize) -> Self {
        let mut cups = Vec::with_capacity(n);

        for i in 0..n {
            cups.push(Cup {
                label: i + 1,
                next_inx: i + 1,
                prev_inx: i,
            })
        }

        cups[0].prev_inx = n - 1;
        cups[n - 1].next_inx = 0;

        Self {
            cups: cups,
            cur_inx: 0,
            size: n,
        }
    }

    fn next_three_contains(&self, inx: usize, label: usize) -> bool {
        let mut inx = inx;
        for _ in 0..3 {
            let cup = self.cups.get(inx).unwrap();
            if cup.label == label {
                return true;
            }
            inx = cup.next_inx;
        }
        false
    }

    fn find_cup(&self, label: usize) -> usize {
        // everything after the first 10 is label == inx + 1
        if label >= 10 {
            return label - 1;
        }

        for i in 0..10 {
            let cup = self.cups.get(i).unwrap();
            if cup.label == label {
                return i;
            }
        }
        panic!("cup with label {} wasn't found", label);
    }

    fn play1(&mut self) {
        let (current, hand_inx) = {
            let current = self.cups.get(self.cur_inx).unwrap();
            (current.label, current.next_inx)
        };

        let mut dest = if current > 1 { current - 1 } else { self.size };
        while self.next_three_contains(hand_inx, dest) {
            dest = if dest > 1 { dest - 1 } else { self.size };
        }

        let dest_inx = self.find_cup(dest);

        let (tail_inx, tail_next) = {
            let mid_inx = self.cups.get(hand_inx).unwrap().next_inx;
            let tail_inx = self.cups.get(mid_inx).unwrap().next_inx;
            let tail_next = self.cups.get(tail_inx).unwrap().next_inx;
            (tail_inx, tail_next)
        };

        self.cups.get_mut(self.cur_inx).unwrap().next_inx = tail_next;
        self.cups.get_mut(tail_next).unwrap().prev_inx = self.cur_inx;

        let dest_next = self.cups.get(dest_inx).unwrap().next_inx;
        self.cups.get_mut(tail_inx).unwrap().next_inx = dest_next;
        self.cups.get_mut(dest_next).unwrap().prev_inx = tail_inx;

        self.cups.get_mut(dest_inx).unwrap().next_inx = hand_inx;
        self.cups.get_mut(hand_inx).unwrap().prev_inx = dest_inx;

        self.cur_inx = self.cups.get(self.cur_inx).unwrap().next_inx;
    }

    fn playn(&mut self, n: usize) {
        for _ in 0..n {
            self.play1();
        }
    }

    fn find_stars(&self) -> (usize, usize) {
        let oneinx = self.find_cup(1);
        let one_cup = self.cups.get(oneinx).unwrap();
        let next_cup = self.cups.get(one_cup.next_inx).unwrap();
        (
            next_cup.label,
            self.cups.get(next_cup.next_inx).unwrap().label,
        )
    }
}

fn main() {
    // part1
    {
        let mut cups: Cups = vec![9, 6, 2, 7, 1, 3, 8, 5, 4].into();
        cups.playn(100);
        println!("{}", cups);
    }

    // part2
    {
        let mut cups = LLCups::with_size(1_000_000);
        for (i, x) in vec![9, 6, 2, 7, 1, 3, 8, 5, 4].iter().cloned().enumerate() {
            cups.cups.get_mut(i).unwrap().label = x;
        }

        cups.playn(10_000_000);
        let stars = cups.find_stars();

        println!("{:?}", stars);
        println!("{}", stars.0 * stars.1);
    }
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
    fn test_part2() {
        let mut cups = LLCups::with_size(1_000_000);
        for (i, x) in vec![3, 8, 9, 1, 2, 5, 4, 6, 7].iter().cloned().enumerate() {
            cups.cups.get_mut(i).unwrap().label = x;
        }

        cups.playn(10_000_000);
        let stars = cups.find_stars();

        assert_eq!(stars, (934001, 159792));
        assert_eq!(stars.0 * stars.1, 149245887792);
    }
}
