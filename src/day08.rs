use jungle::readfile;

const SIZE: usize = 99;

struct Grid {
    height: [[u8; SIZE]; SIZE],
}

impl Grid {
    fn read(text: &str) -> Grid {
        let mut height = [[0u8; SIZE]; SIZE];
        for (row, line) in text.lines().enumerate() {
            for (col, ch) in line.chars().enumerate() {
                height[row][col] = ch.to_digit(10).unwrap() as u8;
            }
        }
        Grid { height }
    }

    fn viz_up(&self, row: usize, col: usize) -> bool {
        let h = self.height[row][col];

        for r in (0..row).rev() {
            if self.height[r][col] >= h {
                return false;
            }
        }
        true
    }

    fn viz_dn(&self, row: usize, col: usize) -> bool {
        let h = self.height[row][col];

        for r in (row + 1)..SIZE {
            if self.height[r][col] >= h {
                return false;
            }
        }
        true
    }

    fn viz_lt(&self, row: usize, col: usize) -> bool {
        let h = self.height[row][col];

        for c in (0..col).rev() {
            if self.height[row][c] >= h {
                return false;
            }
        }
        true
    }

    fn viz_rt(&self, row: usize, col: usize) -> bool {
        let h = self.height[row][col];

        for c in (col + 1)..SIZE {
            if self.height[row][c] >= h {
                return false;
            }
        }
        true
    }

    fn visible(&self, row: usize, col: usize) -> bool {
        self.viz_up(row, col)
            || self.viz_dn(row, col)
            || self.viz_lt(row, col)
            || self.viz_rt(row, col)
    }

    fn scene_up(&self, row: usize, col: usize) -> u64 {
        let mut trees = 0;
        let h = self.height[row][col];

        for r in (0..row).rev() {
            trees += 1;
            if self.height[r][col] >= h {
                break;
            }
        }
        trees
    }

    fn scene_dn(&self, row: usize, col: usize) -> u64 {
        let mut trees = 0;
        let h = self.height[row][col];

        for r in (row + 1)..SIZE {
            trees += 1;
            if self.height[r][col] >= h {
                break;
            }
        }
        trees
    }

    fn scene_lt(&self, row: usize, col: usize) -> u64 {
        let mut trees = 0;
        let h = self.height[row][col];

        for c in (0..col).rev() {
            trees += 1;
            if self.height[row][c] >= h {
                break;
            }
        }
        trees
    }

    fn scene_rt(&self, row: usize, col: usize) -> u64 {
        let mut trees = 0;
        let h = self.height[row][col];

        for c in (col + 1)..SIZE {
            trees += 1;
            if self.height[row][c] >= h {
                break;
            }
        }
        trees
    }

    fn scenic(&self, row: usize, col: usize) -> u64 {
        self.scene_up(row, col)
            * self.scene_dn(row, col)
            * self.scene_lt(row, col)
            * self.scene_rt(row, col)
    }
}

pub fn a() {
    let ctxt = readfile("08");
    let grid = Grid::read(&ctxt.text);
    let mut visible = 0;
    for row in 0..SIZE {
        for col in 0..SIZE {
            if grid.visible(row, col) {
                visible += 1;
            }
        }
    }
    println!("{visible} trees visible");
}

pub fn b() {
    let ctxt = readfile("08");
    let grid = Grid::read(&ctxt.text);
    let mut best = 0;
    for row in 0..SIZE {
        for col in 0..SIZE {
            let s = grid.scenic(row, col);
            if s > best {
                best = s;
            }
        }
    }
    println!("Best scenic score was {best}");
}
