use std::collections::HashSet;

static STEPS: [(isize, isize, isize, isize); 80] = [
    (-1, -1, -1, -1),
    (-1, -1, -1, 0),
    (-1, -1, -1, 1),
    (-1, -1, 0, -1),
    (-1, -1, 0, 0),
    (-1, -1, 0, 1),
    (-1, -1, 1, -1),
    (-1, -1, 1, 0),
    (-1, -1, 1, 1),
    (-1, 0, -1, -1),
    (-1, 0, -1, 0),
    (-1, 0, -1, 1),
    (-1, 0, 0, -1),
    (-1, 0, 0, 0),
    (-1, 0, 0, 1),
    (-1, 0, 1, -1),
    (-1, 0, 1, 0),
    (-1, 0, 1, 1),
    (-1, 1, -1, -1),
    (-1, 1, -1, 0),
    (-1, 1, -1, 1),
    (-1, 1, 0, -1),
    (-1, 1, 0, 0),
    (-1, 1, 0, 1),
    (-1, 1, 1, -1),
    (-1, 1, 1, 0),
    (-1, 1, 1, 1),
    (0, -1, -1, -1),
    (0, -1, -1, 0),
    (0, -1, -1, 1),
    (0, -1, 0, -1),
    (0, -1, 0, 0),
    (0, -1, 0, 1),
    (0, -1, 1, -1),
    (0, -1, 1, 0),
    (0, -1, 1, 1),
    (0, 0, -1, -1),
    (0, 0, -1, 0),
    (0, 0, -1, 1),
    (0, 0, 0, -1),
    (0, 0, 0, 1),
    (0, 0, 1, -1),
    (0, 0, 1, 0),
    (0, 0, 1, 1),
    (0, 1, -1, -1),
    (0, 1, -1, 0),
    (0, 1, -1, 1),
    (0, 1, 0, -1),
    (0, 1, 0, 0),
    (0, 1, 0, 1),
    (0, 1, 1, -1),
    (0, 1, 1, 0),
    (0, 1, 1, 1),
    (1, -1, -1, -1),
    (1, -1, -1, 0),
    (1, -1, -1, 1),
    (1, -1, 0, -1),
    (1, -1, 0, 0),
    (1, -1, 0, 1),
    (1, -1, 1, -1),
    (1, -1, 1, 0),
    (1, -1, 1, 1),
    (1, 0, -1, -1),
    (1, 0, -1, 0),
    (1, 0, -1, 1),
    (1, 0, 0, -1),
    (1, 0, 0, 0),
    (1, 0, 0, 1),
    (1, 0, 1, -1),
    (1, 0, 1, 0),
    (1, 0, 1, 1),
    (1, 1, -1, -1),
    (1, 1, -1, 0),
    (1, 1, -1, 1),
    (1, 1, 0, -1),
    (1, 1, 0, 0),
    (1, 1, 0, 1),
    (1, 1, 1, -1),
    (1, 1, 1, 0),
    (1, 1, 1, 1),
];

type P = (isize, isize, isize, isize);

struct AdjacentIter {
    inx: usize,
    p: P,
}

impl Iterator for AdjacentIter {
    type Item = P;

    fn next(&mut self) -> Option<Self::Item> {
        if self.inx >= STEPS.len() {
            return None;
        }

        let step = STEPS[self.inx];
        self.inx += 1;

        Some((
            self.p.0 + step.0,
            self.p.1 + step.1,
            self.p.2 + step.2,
            self.p.3 + step.3,
        ))
    }
}

fn adjacent(p: P) -> AdjacentIter {
    AdjacentIter { inx: 0, p: p }
}

#[derive(Debug, Clone)]
pub struct HyperCube {
    active: HashSet<P>,
    xrange: (isize, isize),
    yrange: (isize, isize),
    zrange: (isize, isize),
    wrange: (isize, isize),
}

impl HyperCube {
    pub fn parse(initial: &str) -> Self {
        let mut xmax = 0;
        let mut ymax = 0;

        let mut active = HashSet::new();

        for (y, line) in initial.lines().enumerate() {
            xmax = line.len();
            for (x, char) in line.chars().enumerate() {
                if char == '#' {
                    active.insert((x as isize, y as isize, 0, 0));
                }
            }
            ymax = y + 1;
        }

        Self {
            active: active,
            xrange: (0, xmax as isize),
            yrange: (0, ymax as isize),
            zrange: (0, 0),
            wrange: (0, 0),
        }
    }

    pub fn iterate(self) -> Self {
        let mut active = HashSet::new();

        let mut xrange = self.xrange;
        let mut yrange = self.yrange;
        let mut zrange = self.zrange;
        let mut wrange = self.wrange;

        for x in (self.xrange.0 - 1)..=(self.xrange.1 + 1) {
            for y in (self.yrange.0 - 1)..=(self.yrange.1 + 1) {
                for z in (self.zrange.0 - 1)..=(self.zrange.1 + 1) {
                    for w in (self.wrange.0 - 1)..=(self.wrange.1 + 1) {
                        let is_active = self.active.contains(&(x, y, z, w));

                        let active_count = adjacent((x, y, z, w))
                            .filter(|p| self.active.contains(p))
                            .count();

                        if active_count == 3 || (is_active && active_count == 2) {
                            active.insert((x, y, z, w));

                            if x < xrange.0 {
                                xrange.0 = x
                            }
                            if x > xrange.1 {
                                xrange.1 = x
                            }
                            if y < yrange.0 {
                                yrange.0 = y
                            }
                            if y > yrange.1 {
                                yrange.1 = y
                            }
                            if z < zrange.0 {
                                zrange.0 = z
                            }
                            if z > zrange.1 {
                                zrange.1 = z
                            }
                            if w < wrange.0 {
                                wrange.0 = w
                            }
                            if w > wrange.1 {
                                wrange.1 = w
                            }
                        }
                    }
                }
            }
        }

        HyperCube {
            active: active,
            xrange: xrange,
            yrange: yrange,
            zrange: zrange,
            wrange: wrange,
        }
    }

    pub fn energy(&self) -> usize {
        self.active.len()
    }
}
