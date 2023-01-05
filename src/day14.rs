use jungle::readfile;

type Symbol = u8;

const ROCK: Symbol = b'#';
const AIR: Symbol = b'.';
const SAND: Symbol = b'o';

const HEIGHT: usize = 1000;
const WIDTH: usize = 1000;

struct Cave {
    map: [[Symbol; WIDTH]; HEIGHT],
    depth: usize,
    left: usize,
    right: usize,
}

impl Default for Cave {
    fn default() -> Self {
        Cave {
            map: [[AIR; WIDTH]; HEIGHT],
            depth: 0,
            left: 499,
            right: 501,
        }
    }
}

use std::fmt;
impl fmt::Debug for Cave {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str("===========================\n")?;
        for row in 0..=self.depth {
            for col in self.left..=self.right {
                if row == 0 && col == 500 {
                    f.write_str("+")?;
                } else {
                    let c = self.map[row][col] as char;
                    f.write_fmt(format_args!("{}", c))?;
                }
            }
            f.write_str("\n")?;
        }
        f.write_str("---------------------------\n")?;
        Ok(())
    }
}

fn numeric(s: &str) -> (usize, usize) {
    if let Some((x, y)) = s.split_once(',') {
        let x: usize = x.parse().expect("Point x should be an integer");
        let y: usize = y.parse().expect("Point y should be an integer");
        (x, y)
    } else {
        panic!("Invalid map point {s}");
    }
}

impl Cave {
    fn draw(&mut self, line: &str) {
        let mut points = line.split(" -> ").map(numeric);
        let (mut x, mut y) = points.next().unwrap();
        self.rock(x, y);
        for (tox, toy) in points {
            if tox < x {
                loop {
                    x -= 1;
                    self.rock(x, y);
                    if x == tox {
                        break;
                    }
                }
            } else if tox > x {
                loop {
                    x += 1;
                    self.rock(x, y);
                    if x == tox {
                        break;
                    }
                }
            } else if toy < y {
                loop {
                    y -= 1;
                    self.rock(x, y);
                    if y == toy {
                        break;
                    }
                }
            } else if toy > y {
                loop {
                    y += 1;
                    self.rock(x, y);
                    if y == toy {
                        break;
                    }
                }
            } else {
                panic!("The cave readings should all be horizontal or vertical lines");
            }
        }
    }

    fn rock(&mut self, x: usize, y: usize) {
        self.map[y][x] = ROCK;
        if y + 1 > self.depth {
            self.depth = y + 1;
        }
        if x - 1 < self.left {
            self.left = x - 1;
        }
        if x + 1 > self.right {
            self.right = x + 1;
        }
    }

    fn draw_base(&mut self) {
        let y = self.depth + 1;
        let left = 499 - self.depth;
        let right = 501 + self.depth;
        for x in left..=right {
            self.rock(x, y);
        }
    }

    // Try to drop One unit of sand,
    // if it falls into the abyss that's false (for part 1)
    // if it blocks the source that's also false (for part 2)
    fn grain(&mut self) -> bool {
        let mut x = 500;
        let mut y = 0;

        if self.map[y][x] == SAND {
            return false;
        }

        loop {
            // Fell into abyss
            if y > self.depth || x < self.left || x > self.right {
                return false;
            }

            if self.map[y + 1][x] == AIR {
                y += 1;
                continue;
            }
            if self.map[y + 1][x - 1] == AIR {
                y += 1;
                x -= 1;
                continue;
            }
            if self.map[y + 1][x + 1] == AIR {
                y += 1;
                x += 1;
                continue;
            }
            break;
        }
        self.map[y][x] = SAND;
        true
    }

    // Fill with sand until we can't
    fn fill(&mut self) -> usize {
        let mut unit = 0;
        loop {
            if !self.grain() {
                break;
            }
            unit += 1;
        }
        unit
    }
}

pub fn a() {
    let ctxt = readfile("14");
    let mut cave: Cave = Default::default();
    for line in ctxt.lines() {
        cave.draw(line);
    }
    let count = cave.fill();
    println!("{count} units of sand will come to rest");
}

pub fn b() {
    let ctxt = readfile("14");
    let mut cave: Cave = Default::default();
    for line in ctxt.lines() {
        cave.draw(line);
    }
    cave.draw_base();
    let count = cave.fill();
    println!("{count} units of sand will come to rest");
}
