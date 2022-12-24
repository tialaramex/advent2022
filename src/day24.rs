use jungle::readfile;

#[derive(Copy, Clone, Debug)]
struct Blizzard {
    start: usize,
}

struct Valley {
    width: usize,
    height: usize,
    up: Vec<Vec<Blizzard>>,
    dn: Vec<Vec<Blizzard>>,
    lt: Vec<Vec<Blizzard>>,
    rt: Vec<Vec<Blizzard>>,
}

impl Valley {
    fn new(width: usize, height: usize) -> Self {
        let mut up = Vec::new();
        let mut dn = Vec::new();
        for _ in 0..width {
            up.push(Vec::new());
            dn.push(Vec::new());
        }
        let mut lt = Vec::new();
        let mut rt = Vec::new();
        for _ in 0..width {
            lt.push(Vec::new());
            rt.push(Vec::new());
        }
        Self {
            width,
            height,
            up,
            dn,
            lt,
            rt,
        }
    }

    fn add_up(&mut self, col: usize, addition: Blizzard) {
        self.up[col].push(addition);
    }

    fn add_dn(&mut self, col: usize, addition: Blizzard) {
        self.dn[col].push(addition);
    }

    fn add_lt(&mut self, row: usize, addition: Blizzard) {
        self.lt[row].push(addition);
    }

    fn add_rt(&mut self, row: usize, addition: Blizzard) {
        self.rt[row].push(addition);
    }

    fn safe(&self, tick: usize, row: usize, col: usize) -> bool {
        for b in self.up[col].iter() {
            if (row + tick) % self.height == b.start {
                return false;
            }
        }
        for b in self.dn[col].iter() {
            if (b.start + tick) % self.height == row {
                return false;
            }
        }
        for b in self.lt[row].iter() {
            if (col + tick) % self.width == b.start {
                return false;
            }
        }
        for b in self.rt[row].iter() {
            if (b.start + tick) % self.width == col {
                return false;
            }
        }
        true
    }
}

#[derive(Copy, Clone, Debug, Eq, PartialEq, Hash)]
struct Expedition {
    row: usize,
    col: usize,
}

use std::collections::HashSet;

impl Expedition {
    fn new() -> Self {
        Self { row: 0, col: 1 }
    }

    fn at_exit(&self, valley: &Valley) -> bool {
        self.row == valley.height && self.col == valley.width
    }

    fn at_entrance(&self, _valley: &Valley) -> bool {
        self.row == 1 && self.col == 1
    }

    fn adapt(valley: &Valley, tick: usize, row: usize, col: usize) -> bool {
        if row == 0 || col == 0 || row > valley.height || col > valley.width {
            return false;
        }

        valley.safe(tick, row - 1, col - 1)
    }

    // Simulate valley in this tick and identify adjacent safe spaces
    fn next(self, tick: usize, valley: &Valley) -> Vec<Self> {
        let mut v = Vec::new();
        // At the start...
        if self.row == 0 {
            if Expedition::adapt(valley, tick, self.row + 1, self.col) {
                v.push(Self {
                    row: self.row + 1,
                    col: self.col,
                });
            }
            // Remain outside waiting
            v.push(self);
            return v;
        }

        // At the end... (coming back?)
        if self.row == valley.height + 1 {
            if Expedition::adapt(valley, tick, self.row - 1, self.col) {
                v.push(Self {
                    row: self.row - 1,
                    col: self.col,
                });
            }
            // Remain outside waiting
            v.push(self);
            return v;
        }

        // In the valley, dodging blizzards
        // Wait?
        if Expedition::adapt(valley, tick, self.row, self.col) {
            v.push(self.clone());
        }

        // Up
        if Expedition::adapt(valley, tick, self.row - 1, self.col) {
            v.push(Self {
                row: self.row - 1,
                col: self.col,
            });
        }

        // Left
        if Expedition::adapt(valley, tick, self.row, self.col - 1) {
            v.push(Self {
                row: self.row,
                col: self.col - 1,
            });
        }

        // Down
        if Expedition::adapt(valley, tick, self.row + 1, self.col) {
            v.push(Self {
                row: self.row + 1,
                col: self.col,
            });
        }

        // Right
        if Expedition::adapt(valley, tick, self.row, self.col + 1) {
            v.push(Self {
                row: self.row,
                col: self.col + 1,
            });
        }

        v
    }

    fn route(valley: &Valley) -> usize {
        let mut current: Vec<Expedition> = Vec::new();
        let start = Expedition::new();
        current.push(start);
        let mut tick = 0;
        loop {
            let mut next: Vec<Expedition> = Vec::new();
            let mut seen: HashSet<Expedition> = HashSet::new();

            tick += 1;
            for c in current {
                for x in c.next(tick, &valley) {
                    if !seen.contains(&x) {
                        seen.insert(x);
                        next.push(x);
                        if x.at_exit(valley) {
                            // It takes one minute to actually leave the valley
                            return tick + 1;
                        }
                    }
                }
            }
            current = next;
        }
    }

    // There, and back again, and there again
    fn hobbit(valley: &Valley) -> usize {
        let mut current: Vec<Expedition> = Vec::new();
        let start = Expedition::new();
        current.push(start);
        let mut tick = 0;
        'first: loop {
            let mut next: Vec<Expedition> = Vec::new();
            let mut seen: HashSet<Expedition> = HashSet::new();

            tick += 1;
            for c in current {
                for x in c.next(tick, &valley) {
                    if !seen.contains(&x) {
                        seen.insert(x);
                        next.push(x);
                        if x.at_exit(valley) {
                            // It takes one minute to actually leave the valley
                            tick += 1;
                            break 'first;
                        }
                    }
                }
            }
            current = next;
        }

        let end = Expedition {
            row: valley.height + 1,
            col: valley.width,
        };
        current = Vec::new();
        current.push(end);
        'second: loop {
            let mut next: Vec<Expedition> = Vec::new();
            let mut seen: HashSet<Expedition> = HashSet::new();

            tick += 1;
            for c in current {
                for x in c.next(tick, &valley) {
                    if !seen.contains(&x) {
                        seen.insert(x);
                        next.push(x);
                        if x.at_entrance(valley) {
                            // It takes one minute to actually leave the valley
                            tick += 1;
                            break 'second;
                        }
                    }
                }
            }
            current = next;
        }

        current = Vec::new();
        current.push(start);
        loop {
            let mut next: Vec<Expedition> = Vec::new();
            let mut seen: HashSet<Expedition> = HashSet::new();

            tick += 1;
            for c in current {
                for x in c.next(tick, &valley) {
                    if !seen.contains(&x) {
                        seen.insert(x);
                        next.push(x);
                        if x.at_exit(valley) {
                            // It takes one minute to actually leave the valley
                            return tick + 1;
                        }
                    }
                }
            }

            current = next;
        }
    }
}

fn read_map(filename: &str) -> Valley {
    let ctxt = readfile(filename);
    let height = ctxt.lines().count() - 2;
    let width = ctxt.lines().next().unwrap().len() - 2;
    let mut valley = Valley::new(width, height);

    let mut row = 0;
    for line in ctxt.lines() {
        let mut col = 0;
        for b in line.bytes() {
            match b {
                b'#' => {
                    if row != 0 && col != 0 && row != height + 1 && col != width + 1 {
                        panic!("Wall in the middle of the valley is unexpected");
                    }
                }
                b'.' => { /* empty, do nothing */ }
                b'<' => {
                    valley.add_lt(row - 1, Blizzard { start: col - 1 });
                }
                b'>' => {
                    valley.add_rt(row - 1, Blizzard { start: col - 1 });
                }
                b'^' => {
                    valley.add_up(col - 1, Blizzard { start: row - 1 });
                }
                b'v' => {
                    valley.add_dn(col - 1, Blizzard { start: row - 1 });
                }
                _ => {
                    panic!("Map input should only mark blizzards, etc.");
                }
            }
            col += 1;
        }
        row += 1;
    }
    valley
}

pub fn a() {
    let map: Valley = read_map("24");
    let ticks = Expedition::route(&map);
    println!("Expedition took {ticks} minutes to cross valley");
}

pub fn b() {
    let map: Valley = read_map("24");
    let ticks = Expedition::hobbit(&map);
    println!("I took {ticks} minutes to cross valley, go back for snacks, and return");
}
