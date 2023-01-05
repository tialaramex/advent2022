use jungle::readfile;

const WIDTH: usize = 143;
const HEIGHT: usize = 41;

struct Map {
    elevation: [[u8; WIDTH]; HEIGHT],
    dist: [[u16; WIDTH]; HEIGHT],
    done: [[bool; WIDTH]; HEIGHT],
    start: (usize, usize),
    end: (usize, usize),
}

impl Default for Map {
    fn default() -> Self {
        let elevation = [[0u8; WIDTH]; HEIGHT];
        let dist = [[0u16; WIDTH]; HEIGHT];
        let done = [[false; WIDTH]; HEIGHT];
        Self {
            elevation,
            dist,
            done,
            start: (0, 0),
            end: (0, 0),
        }
    }
}

use std::fmt;
impl fmt::Debug for Map {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str("===========================\n")?;
        for row in 0..HEIGHT {
            for col in 0..WIDTH {
                if (row, col) == self.start {
                    f.write_str("S")?
                } else if (row, col) == self.end {
                    f.write_str("E")?
                } else {
                    let c = self.elevation[row][col] as char;
                    f.write_fmt(format_args!("{}", c))?;
                }
            }
            f.write_str("\n")?;
        }
        f.write_str("---------------------------\n")?;
        for row in 0..HEIGHT {
            for col in 0..WIDTH {
                if (row, col) == self.start {
                    f.write_str("S")?
                } else if (row, col) == self.end {
                    f.write_str("E")?
                } else {
                    let c = char::from_u32(self.dist[row][col] as u32 + 60).unwrap();
                    f.write_fmt(format_args!("{}", c))?;
                }
            }
            f.write_str("\n")?;
        }
        Ok(())
    }
}

fn check_height(from: u8, to: u8) -> bool {
    from + 1 >= to
}

impl Map {
    fn read(text: &str) -> Self {
        let mut elevation = [[0u8; WIDTH]; HEIGHT];
        let mut start = (0, 0);
        let mut end = (0, 0);
        for (row, line) in text.lines().enumerate() {
            for (col, byte) in line.bytes().enumerate() {
                elevation[row][col] = match byte {
                    b'S' => {
                        start = (row, col);
                        b'a'
                    }
                    b'E' => {
                        end = (row, col);
                        b'z'
                    }
                    b'a'..=b'z' => byte,
                    _ => {
                        panic!("Unexpected map byte: {byte}")
                    }
                };
            }
        }
        Self {
            elevation,
            start,
            end,
            ..Default::default()
        }
    }

    fn height(&self, row: usize, col: usize) -> u8 {
        self.elevation[row][col]
    }

    fn step(&mut self, dist: u16, todo: Vec<(usize, usize)>) -> Vec<(usize, usize)> {
        let mut v = Vec::new();
        for (row, col) in todo {
            self.dist[row][col] = dist;
            self.done[row][col] = true;

            if row > 0
                && !self.done[row - 1][col]
                && check_height(self.height(row - 1, col), self.height(row, col))
            {
                v.push((row - 1, col));
            }
            if row + 1 < HEIGHT
                && !self.done[row + 1][col]
                && check_height(self.height(row + 1, col), self.height(row, col))
            {
                v.push((row + 1, col));
            }
            if col > 0
                && !self.done[row][col - 1]
                && check_height(self.height(row, col - 1), self.height(row, col))
            {
                v.push((row, col - 1));
            }
            if col + 1 < WIDTH
                && !self.done[row][col + 1]
                && check_height(self.height(row, col + 1), self.height(row, col))
            {
                v.push((row, col + 1));
            }
        }
        v
    }

    fn flood(&mut self) {
        let mut dist = 0;
        let mut v = Vec::new();
        v.push((self.end.0, self.end.1));
        while !v.is_empty() {
            v = self.step(dist, v);
            v.sort_unstable();
            v.dedup();
            dist += 1;
        }
    }

    fn shortest(self) -> u16 {
        self.dist[self.start.0][self.start.1]
    }

    fn best(self) -> u16 {
        let mut dist = u16::MAX;
        for row in 0..HEIGHT {
            for col in 0..WIDTH {
                if self.height(row, col) == b'a'
                    && self.done[row][col]
                    && self.dist[row][col] < dist
                {
                    dist = self.dist[row][col];
                }
            }
        }
        dist
    }
}

pub fn a() {
    let ctxt = readfile("12");
    let mut map = Map::read(&ctxt.text);
    map.flood();
    println!("Shortest route from S is: {} steps", map.shortest());
}

pub fn b() {
    let ctxt = readfile("12");
    let mut map = Map::read(&ctxt.text);
    map.flood();
    println!("Best distance is: {}", map.best());
}
