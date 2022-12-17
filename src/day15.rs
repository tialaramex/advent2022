use jungle::readfile;

#[derive(Copy, Clone, Debug)]
struct Coord {
    x: isize,
    y: isize,
}

use std::num::ParseIntError;
use std::str::FromStr;

impl FromStr for Coord {
    type Err = ParseIntError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (x, y) = s.split_once(", ").unwrap();
        let x = x.strip_prefix("x=").unwrap();
        let y = y.strip_prefix("y=").unwrap();
        let x: isize = x.parse()?;
        let y: isize = y.parse()?;
        Ok(Coord { x, y })
    }
}

fn parse(s: &str) -> (Coord, Coord) {
    let (sensor, beacon) = s.split_once(": ").unwrap();
    let sensor = sensor.strip_prefix("Sensor at ").unwrap();
    let sensor: Coord = sensor.parse().unwrap();
    let beacon = beacon.strip_prefix("closest beacon is at ").unwrap();
    let beacon: Coord = beacon.parse().unwrap();
    (sensor, beacon)
}

fn manhattan(a: &Coord, b: &Coord) -> usize {
    a.x.abs_diff(b.x) + a.y.abs_diff(b.y)
}

fn consider(sensor: &Coord, beacon: &Coord, y: isize) -> (isize, isize) {
    let nearest = Coord { x: sensor.x, y };
    let radius = manhattan(sensor, beacon) as isize;
    let r2 = manhattan(sensor, &nearest) as isize;
    if r2 >= radius {
        (0, 0)
    } else {
        let diff = radius - r2;
        (sensor.x - diff, sensor.x + diff)
    }
}

use std::collections::VecDeque;

#[derive(Copy, Clone, Debug)]
enum Where {
    Before(usize),
    During(usize),
    After(usize),
}

impl Where {
    fn touch(pos: isize, left: isize, right: isize, k: usize) -> Self {
        if pos < left {
            Before(k)
        } else if pos > right {
            After(k)
        } else {
            During(k)
        }
    }
}

use Where::{After, Before, During};

#[derive(Debug)]
struct Overlap {
    v: VecDeque<(isize, isize)>,
}

impl Overlap {
    fn new() -> Self {
        Self { v: VecDeque::new() }
    }

    fn add(&mut self, left: isize, right: isize) {
        // Easy case, there is no previous range
        if self.v.is_empty() {
            self.v.push_back((left, right));
            return;
        }

        let mut l = After(0);
        let mut r = After(0);

        for (k, &(p1, p2)) in self.v.iter().enumerate() {
            if let After(_n) = l {
                l = Where::touch(left, p1, p2, k);
            }
            if let After(_n) = r {
                r = Where::touch(right, p1, p2, k);
            }
        }

        match (l, r) {
            (After(l), After(r)) => {
                if l == r {
                    self.v.insert(l + 1, (left, right));
                } else {
                    panic!("Shouldn't see After({l}) After({r})");
                }
            }
            (Before(l), After(r)) => {
                if l == r {
                    self.v[l] = (left, right);
                } else {
                    panic!("Tricky Before({l}) After({r})");
                }
            }
            (Before(l), Before(r)) => {
                if l == r {
                    self.v.insert(l, (left, right));
                } else {
                    panic!("Tricky Before({l}) Before({r})");
                }
            }
            (Before(l), During(r)) => {
                for _ in l..r {
                    self.v.remove(l);
                }
                self.v[l].0 = left;
            }
            (During(l), After(r)) => {
                let tmp = self.v[l].0;
                for _ in l..r {
                    self.v.remove(l);
                }
                self.v[l].0 = tmp;
                self.v[l].1 = right;
            }
            (During(l), Before(r)) => {
                let tmp = self.v[l].0;
                for _ in l..(r - 1) {
                    self.v.remove(l);
                }
                self.v[l].0 = tmp;
                self.v[l].1 = right;
            }
            (During(l), During(r)) => {
                let tmp = self.v[l].0;
                for _ in l..r {
                    self.v.remove(l);
                }
                self.v[l].0 = tmp;
            }
            (_, _) => {
                panic!("Unhandled {l:?} {r:?}");
            }
        }
    }

    fn hole(&self, mut left: isize, right: isize) -> Option<isize> {
        for &(p1, p2) in self.v.iter() {
            if p1 > left {
                return Some(left);
            }
            if p2 >= right {
                return None;
            }
            left = p2 + 1;
        }
        Some(left)
    }
}

const DISTANCE: isize = 2_000_000;

pub fn a() {
    let ctxt = readfile("15");

    let mut overlap = Overlap::new();
    for line in ctxt.lines() {
        let (sensor, beacon) = parse(line);
        let (left, right) = consider(&sensor, &beacon, DISTANCE);
        overlap.add(left, right);
    }
    let mut misses = 0;
    for x in overlap.v {
        misses += x.1 - x.0 + 1;
    }
    println!(
        "There are {} positions which cannot contain a beacon",
        misses - 1
    );
}

const MAX_RANGE: isize = 4_000_000;

pub fn b() {
    let ctxt = readfile("15");

    let mut v = Vec::new();

    for line in ctxt.lines() {
        let (sensor, beacon) = parse(line);
        v.push((sensor, beacon));
    }
    for y in 0..MAX_RANGE {
        let mut overlap = Overlap::new();
        for (sensor, beacon) in v.iter() {
            let (left, right) = consider(sensor, beacon, y);
            if (left, right) != (0, 0) {
                overlap.add(left, right);
            }
        }
        if let Some(x) = overlap.hole(0, MAX_RANGE) {
            println!("x = {x}, y = {y}");
            println!("Therefore frequency = {}", (4000000 * x) + y);
            return;
        }
    }
}
