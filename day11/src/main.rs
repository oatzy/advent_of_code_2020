use std::collections::HashMap;
use std::fs;

type Seats = HashMap<P, Seat>;

#[derive(Debug, Copy, Clone, Eq, PartialEq)]
enum Seat {
    Empty,
    Occupied,
}

#[derive(Debug, Copy, Clone, Eq, PartialEq, Hash)]
struct P(usize, usize);

impl P {
    fn adjacent(&self) -> AdjacentIter {
        self.into()
    }
}

struct AdjacentIter {
    dx: isize,
    dy: isize,
    p: (isize, isize),
}

impl From<&P> for AdjacentIter {
    fn from(item: &P) -> Self {
        AdjacentIter {
            dx: -1,
            dy: -1,
            p: (item.0 as isize, item.1 as isize),
        }
    }
}

impl Iterator for AdjacentIter {
    type Item = P;

    fn next(&mut self) -> Option<Self::Item> {
        if self.dy > 1 {
            return None;
        }
        if self.p.0 + self.dx < 0 {
            self.dx += 1
        }
        if self.p.1 + self.dy < 0 {
            self.dy += 1
        }
        if self.dx == 0 && self.dy == 0 {
            self.dx += 1
        }

        let x = self.p.0 + self.dx;
        let y = self.p.1 + self.dy;

        self.dx += 1;
        if self.dx > 1 {
            self.dx = -1;
            self.dy += 1;
        }

        Some(P(x as usize, y as usize))
    }
}

fn read_seats(input: &str) -> Seats {
    let mut seats = HashMap::new();

    for (y, line) in input.lines().enumerate() {
        for (x, char) in line.chars().enumerate() {
            // insert returns Option type
            let _ = match char {
                '.' => None,
                'L' => seats.insert(P(x, y), Seat::Empty),
                '#' => seats.insert(P(x, y), Seat::Occupied),
                c => unreachable!("Got unexpected char {}", c),
            };
        }
    }

    seats
}

fn iterate(seats: &Seats) -> Seats {
    seats
        .iter()
        .map(|(&p, seat)| {
            let occupied = p
                .adjacent()
                .filter(|x| seats.get(x) == Some(&Seat::Occupied))
                .count();

            (
                p,
                match seat {
                    Seat::Empty => {
                        if occupied == 0 {
                            Seat::Occupied
                        } else {
                            Seat::Empty
                        }
                    }
                    Seat::Occupied => {
                        if occupied >= 4 {
                            Seat::Empty
                        } else {
                            Seat::Occupied
                        }
                    }
                },
            )
        })
        .collect()
}

fn find_fixed_point(seats: &Seats) -> Seats {
    let mut old: Seats = seats.to_owned();

    loop {
        let new = iterate(&old);
        if new == old {
            return new;
        }
        old = new;
    }
}

fn part1(seats: &Seats) -> usize {
    find_fixed_point(&seats)
        .iter()
        .filter(|(_, &s)| s == Seat::Occupied)
        .count()
}

fn main() {
    let input = fs::read_to_string("../inputs/day11.txt").unwrap();
    let seats = read_seats(&input);

    println!("{}", part1(&seats));
}

mod test {
    use super::*;

    #[test]
    fn test_part1() {
        let input = fs::read_to_string("../inputs/day11-test.txt").unwrap();
        let seats = read_seats(&input);

        assert_eq!(part1(&seats), 37);
    }

    #[test]
    fn test_adjacent() {
        assert_eq!(
            P(0, 0).adjacent().collect::<Vec<P>>(),
            vec![P(1, 0), P(0, 1), P(1, 1)]
        );
        assert_eq!(
            P(2, 2).adjacent().collect::<Vec<P>>(),
            vec![
                P(1, 1),
                P(2, 1),
                P(3, 1),
                P(1, 2),
                P(3, 2),
                P(1, 3),
                P(2, 3),
                P(3, 3)
            ]
        );
    }
}
