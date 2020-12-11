use std::collections::HashMap;
use std::fs;

static STEPS: [(isize, isize); 8] = [
    (-1, -1),
    (-1, 0),
    (-1, 1),
    (0, -1),
    (0, 1),
    (1, -1),
    (1, 0),
    (1, 1),
];

type Seats = HashMap<P, Seat>;

#[derive(Debug, Clone, Eq, PartialEq)]
struct SeatMap {
    seats: Seats,
    width: usize,
    height: usize,
}

impl SeatMap {
    fn first_in_path(&self, origin: P, step: &(isize, isize)) -> Seat {
        let (mut x, mut y) = (origin.0 as isize + step.0, origin.1 as isize + step.1);

        while x >= 0 && y >= 0 && x <= self.width as isize && y <= self.height as isize {
            let p = P(x as usize, y as usize);
            if self.seats.contains_key(&p) {
                return *self.seats.get(&p).unwrap();
            }
            x += step.0;
            y += step.1;
        }

        Seat::Empty
    }
}

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

fn read_seats(input: &str) -> SeatMap {
    let mut seats = HashMap::new();
    let mut width = 0;
    let mut height = 0;

    for (y, line) in input.lines().enumerate() {
        width = line.len();
        for (x, char) in line.chars().enumerate() {
            // insert returns Option type
            let _ = match char {
                '.' => None,
                'L' => seats.insert(P(x, y), Seat::Empty),
                '#' => seats.insert(P(x, y), Seat::Occupied),
                c => unreachable!("Got unexpected char {}", c),
            };
        }
        height = y;
    }

    SeatMap {
        seats: seats,
        width: width,
        height: height,
    }
}

fn iterate_adjacent(seats: &Seats) -> Seats {
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

fn iterate_path(seats: &SeatMap) -> SeatMap {
    let new = seats
        .seats
        .iter()
        .map(|(&p, seat)| {
            let occupied = STEPS
                .iter()
                .filter(|x| seats.first_in_path(p, *x) == Seat::Occupied)
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
                        if occupied >= 5 {
                            Seat::Empty
                        } else {
                            Seat::Occupied
                        }
                    }
                },
            )
        })
        .collect();

    SeatMap {
        seats: new,
        width: seats.width,
        height: seats.height,
    }
}

fn find_fixed_point_adjacent(seats: &Seats) -> Seats {
    let mut old: Seats = seats.to_owned();

    loop {
        let new = iterate_adjacent(&old);
        if new == old {
            return new;
        }
        old = new;
    }
}

fn find_fixed_point_path(seats: &SeatMap) -> SeatMap {
    let mut old: SeatMap = seats.to_owned();

    loop {
        let new = iterate_path(&old);
        if new == old {
            return new;
        }
        old = new;
    }
}

fn part1(seats: &Seats) -> usize {
    find_fixed_point_adjacent(&seats)
        .iter()
        .filter(|(_, &s)| s == Seat::Occupied)
        .count()
}

fn part2(seats: &SeatMap) -> usize {
    find_fixed_point_path(&seats)
        .seats
        .iter()
        .filter(|(_, &s)| s == Seat::Occupied)
        .count()
}

fn main() {
    let input = fs::read_to_string("../inputs/day11.txt").unwrap();
    let seats = read_seats(&input);

    println!("{}", part1(&seats.seats));

    println!("{}", part2(&seats));
}

mod test {
    use super::*;

    #[test]
    fn test_part1() {
        let input = fs::read_to_string("../inputs/day11-test.txt").unwrap();
        let seats = read_seats(&input);

        assert_eq!(part1(&seats.seats), 37);
    }

    #[test]
    fn test_part2() {
        let input = fs::read_to_string("../inputs/day11-test.txt").unwrap();
        let seats = read_seats(&input);

        assert_eq!(part2(&seats), 26);
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
