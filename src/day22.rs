use jungle::readfile;

const HEIGHT: usize = 200;
const WIDTH: usize = 150;

const OPEN: u8 = b'.';
const LEFT: u8 = b'<';
const DOWN: u8 = b'v';
const UP: u8 = b'^';
const RIGHT: u8 = b'>';
const SOLID: u8 = b'#';

#[derive(Copy, Clone, Debug, Default)]
enum Direction {
    #[default]
    Right,
    Down,
    Left,
    Up,
}

impl Direction {
    fn symbol(self) -> u8 {
        match self {
            Direction::Right => RIGHT,
            Direction::Down => DOWN,
            Direction::Left => LEFT,
            Direction::Up => UP,
        }
    }

    fn value(self) -> usize {
        match self {
            Direction::Right => 0,
            Direction::Down => 1,
            Direction::Left => 2,
            Direction::Up => 3,
        }
    }

    fn left(self) -> Self {
        match self {
            Direction::Right => Direction::Up,
            Direction::Down => Direction::Right,
            Direction::Left => Direction::Down,
            Direction::Up => Direction::Left,
        }
    }

    fn right(self) -> Self {
        match self {
            Direction::Right => Direction::Down,
            Direction::Down => Direction::Left,
            Direction::Left => Direction::Up,
            Direction::Up => Direction::Right,
        }
    }
}

struct Map {
    tiles: [[u8; WIDTH]; HEIGHT],
    left: [usize; HEIGHT],
    right: [usize; HEIGHT],
    top: [usize; WIDTH],
    bottom: [usize; WIDTH],
    row: usize,
    col: usize,
    facing: Direction,
}

use std::fmt;
impl fmt::Debug for Map {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        for row in 0..HEIGHT {
            for col in 0..WIDTH {
                if row == self.row && col == self.col {
                    f.write_str("!")?;
                } else {
                    let mut b = self.tiles[row][col];
                    if b == 0 {
                        b = b' ';
                    }
                    f.write_fmt(format_args!("{}", b as char))?;
                }
            }
            f.write_str("\n")?;
        }
        Ok(())
    }
}

impl Map {
    fn new() -> Map {
        let tiles = [[0; WIDTH]; HEIGHT];
        let left = [usize::MAX; HEIGHT];
        let right = [0; HEIGHT];
        let top = [usize::MAX; WIDTH];
        let bottom = [0; WIDTH];
        Map {
            tiles,
            left,
            right,
            top,
            bottom,
            row: 0,
            col: 0,
            facing: Default::default(),
        }
    }

    fn set(&mut self, row: usize, col: usize, byte: u8) {
        if self.left[row] > col {
            self.left[row] = col;
        }
        if self.right[row] < col {
            self.right[row] = col;
        }
        if self.top[col] > row {
            self.top[col] = row;
        }
        if self.bottom[col] < row {
            self.bottom[col] = row;
        }
        self.tiles[row][col] = byte;
    }

    fn line(&mut self, row: usize, line: &str) {
        let mut col = 0;
        for b in line.bytes() {
            match b {
                b' ' => {}
                OPEN => {
                    self.set(row, col, b);
                }
                SOLID => {
                    self.set(row, col, b);
                }
                _ => {
                    panic!("Unexpected item code {b} in maze row {row}");
                }
            }
            col += 1;
        }
    }

    fn start(&mut self) {
        self.row = 0;
        self.col = self.left[self.row];
        if self.row > self.bottom[self.col] || self.col > self.right[self.row] {
            panic!("Can't find the top-right start position, is map initialised properly?");
        }
    }

    fn report(&self) -> (usize, usize, Direction) {
        (self.row, self.col, self.facing)
    }

    fn forward(&mut self, n: usize) {
        for _ in 0..n {
            // Identify next tile
            let (nr, nc) = match self.facing {
                Direction::Left => {
                    if self.col == self.left[self.row] {
                        (self.row, self.right[self.row])
                    } else {
                        (self.row, self.col - 1)
                    }
                }
                Direction::Right => {
                    if self.col == self.right[self.row] {
                        (self.row, self.left[self.row])
                    } else {
                        (self.row, self.col + 1)
                    }
                }
                Direction::Up => {
                    if self.row == self.top[self.col] {
                        (self.bottom[self.col], self.col)
                    } else {
                        (self.row - 1, self.col)
                    }
                }
                Direction::Down => {
                    if self.row == self.bottom[self.col] {
                        (self.top[self.col], self.col)
                    } else {
                        (self.row + 1, self.col)
                    }
                }
            };
            // Either step into that tile or stop moving
            match self.tiles[nr][nc] {
                OPEN => {
                    self.row = nr;
                    self.col = nc;
                }
                SOLID => {
                    return;
                }
                _ => {
                    panic!("Should always be on the map");
                }
            }
        }
    }

    fn chase(&mut self, instructions: &str) {
        let steps = instructions.split_inclusive(['L', 'R']);
        for step in steps {
            if let Some(n) = step.strip_suffix('L') {
                let n: usize = n.parse().unwrap();
                self.forward(n);
                self.facing = self.facing.left();
            } else if let Some(n) = step.strip_suffix('R') {
                let n: usize = n.parse().unwrap();
                self.forward(n);
                self.facing = self.facing.right();
            } else {
                let n: usize = step.parse().unwrap();
                self.forward(n);
            }
        }
    }

    fn cube_forward(&mut self, n: usize) {
        for _ in 0..n {
            // Identify next tile
            let (nr, nc, nf) = match self.facing {
                Direction::Left => {
                    if self.col == self.left[self.row] {
                        match self.row {
                            0..=49 => {
                                let row = 149 - self.row;
                                (row, 0, Direction::Right)
                            } // 4R
                            50..=99 => {
                                let col = self.row - 50;
                                (100, col, Direction::Down)
                            } // 4D
                            100..=149 => {
                                let row = 149 - self.row;
                                (row, 50, Direction::Right)
                            } // 1R
                            150..=199 => {
                                let col = self.row - 100;
                                (0, col, Direction::Down)
                            } // 1D
                            _ => {
                                panic!("Inconceivable");
                            }
                        }
                    } else {
                        (self.row, self.col - 1, self.facing)
                    }
                }
                Direction::Right => {
                    if self.col == self.right[self.row] {
                        match self.row {
                            0..=49 => {
                                let row = 149 - self.row;
                                (row, 99, Direction::Left)
                            } // 5L
                            50..=99 => {
                                let col = self.row + 50;
                                (49, col, Direction::Up)
                            } // 2U
                            100..=149 => {
                                let row = 149 - self.row;
                                (row, 149, Direction::Left)
                            } // 2L
                            150..=199 => {
                                let col = self.row - 100;
                                (149, col, Direction::Up)
                            } // 5U
                            _ => {
                                panic!("Inconceivable");
                            }
                        }
                    } else {
                        (self.row, self.col + 1, self.facing)
                    }
                }
                Direction::Up => {
                    if self.row == self.top[self.col] {
                        match self.col {
                            0..=49 => {
                                let row = self.col + 50;
                                (row, 50, Direction::Right)
                            } // 3R
                            50..=99 => {
                                let row = self.col + 100;
                                (row, 0, Direction::Right)
                            } // 6R
                            100..=149 => {
                                let col = self.col - 100;
                                (199, col, Direction::Up)
                            } // 6U
                            _ => {
                                panic!("Inconceivable");
                            }
                        }
                    } else {
                        (self.row - 1, self.col, self.facing)
                    }
                }
                Direction::Down => {
                    if self.row == self.bottom[self.col] {
                        match self.col {
                            0..=49 => {
                                let col = self.col + 100;
                                (0, col, Direction::Down)
                            } // 2D
                            50..=99 => {
                                let row = self.col + 100;
                                (row, 49, Direction::Left)
                            } // 6L
                            100..=149 => {
                                let row = self.col - 50;
                                (row, 99, Direction::Left)
                            } // 3L
                            _ => {
                                panic!("Inconceivable");
                            }
                        }
                    } else {
                        (self.row + 1, self.col, self.facing)
                    }
                }
            };
            // Either step into that tile or stop moving
            match self.tiles[nr][nc] {
                OPEN | LEFT | RIGHT | UP | DOWN => {
                    self.set(self.row, self.col, self.facing.symbol());
                    self.row = nr;
                    self.col = nc;
                    self.facing = nf;
                }
                SOLID => {
                    return;
                }
                _ => {
                    panic!("Should always be on the map");
                }
            }
        }
    }

    fn cube_chase(&mut self, instructions: &str) {
        let steps = instructions.split_inclusive(['L', 'R']);
        for step in steps {
            if let Some(n) = step.strip_suffix('L') {
                let n: usize = n.parse().unwrap();
                self.cube_forward(n);
                self.facing = self.facing.left();
            } else if let Some(n) = step.strip_suffix('R') {
                let n: usize = n.parse().unwrap();
                self.cube_forward(n);
                self.facing = self.facing.right();
            } else {
                let n: usize = step.parse().unwrap();
                self.cube_forward(n);
            }
        }
    }
}

fn password(position: (usize, usize, Direction)) -> usize {
    1000 * (position.0 + 1) + 4 * (position.1 + 1) + position.2.value()
}

pub fn a() {
    let ctxt = readfile("22");
    let mut map = Map::new();
    let mut lines = ctxt.lines();
    let mut row = 0;
    loop {
        let line = lines.next().unwrap();
        if line.is_empty() {
            break;
        }
        map.line(row, line);
        row += 1;
    }
    map.start();
    let steps = lines.next().unwrap();
    map.chase(steps);
    let position = map.report();
    let password = password(position);
    println!("Password is apparently {password}");
}

pub fn b() {
    let ctxt = readfile("22");
    let mut map = Map::new();
    let mut lines = ctxt.lines();
    let mut row = 0;
    loop {
        let line = lines.next().unwrap();
        if line.is_empty() {
            break;
        }
        map.line(row, line);
        row += 1;
    }
    map.start();
    let steps = lines.next().unwrap();
    map.cube_chase(steps);
    let position = map.report();
    let password = password(position);
    println!("Password is apparently {password}");
}
