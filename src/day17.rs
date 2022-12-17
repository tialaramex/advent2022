use jungle::readfile;

type Line = [u8; 7];

use std::collections::VecDeque;

#[derive(Clone)]
struct Chamber {
    tall: usize,
    extra: usize,
    zap: usize,
    layers: VecDeque<Line>,
}

const ROCK: u8 = b'#';
const SPACE: u8 = b' ';

impl Chamber {
    fn new() -> Self {
        Self {
            tall: 0,
            extra: 0,
            zap: 0,
            layers: VecDeque::new(),
        }
    }

    fn fix(&mut self) {
        if self.zap > 0 {
            self.layers.drain(0..self.zap);
            self.extra += self.zap;
            self.tall -= self.zap;
            self.zap = 0;
        }

        for _ in self.layers.len()..(self.tall + 4) {
            let line = [SPACE; 7];
            self.layers.push_back(line);
        }
    }

    fn hit(&mut self, height: usize, x: usize) {
        if height >= self.tall {
            self.tall = height + 1;
        }

        self.layers[height][x] = ROCK;
        if self.layers[height] == [ROCK; 7] {
            self.zap = height + 1;
        }
    }

    fn empty(&self, height: usize, x: usize) -> bool {
        height > self.tall || self.layers[height][x] == SPACE
    }

    fn height(&self) -> usize {
        self.tall
    }

    fn true_height(&self) -> usize {
        self.tall + self.extra
    }
}

enum Rock {
    Minus,
    Plus,
    Ell,
    Aye,
    Ball,
}

use Rock::{Aye, Ball, Ell, Minus, Plus};

impl Rock {
    fn new(n: usize) -> Self {
        match n % 5 {
            0 => Minus,
            1 => Plus,
            2 => Ell,
            3 => Aye,
            4 => Ball,
            _ => { panic!("That's not how modulo works"); }
        }
    }
}

struct Dropper {
    rock: Rock,
    bottom: usize,
    left: usize,
}

impl Dropper {
    fn new(rock: Rock, bottom: usize) -> Self {
        Self {
            rock,
            bottom,
            left: 2,
        }
    }

    fn check_left(&self, chamber: &Chamber) -> bool {
        if self.left == 0 {
            return false;
        }
        match self.rock {
            Minus => chamber.empty(self.bottom, self.left - 1),
            Plus => {
                chamber.empty(self.bottom, self.left)
                    && chamber.empty(self.bottom + 1, self.left - 1)
                    && chamber.empty(self.bottom + 2, self.left)
            }
            Ell => {
                chamber.empty(self.bottom, self.left - 1)
                    && chamber.empty(self.bottom + 1, self.left + 1)
                    && chamber.empty(self.bottom + 2, self.left + 1)
            }
            Aye => {
                chamber.empty(self.bottom, self.left - 1)
                    && chamber.empty(self.bottom + 1, self.left - 1)
                    && chamber.empty(self.bottom + 2, self.left - 1)
                    && chamber.empty(self.bottom + 3, self.left - 1)
            }
            Ball => {
                chamber.empty(self.bottom, self.left - 1)
                    && chamber.empty(self.bottom + 1, self.left - 1)
            }
        }
    }

    fn check_right(&self, chamber: &Chamber) -> bool {
        let right = self.left
            + match self.rock {
                Minus => 3,
                Plus => 2,
                Ell => 2,
                Aye => 0,
                Ball => 1,
            };

        if right == 6 {
            return false;
        }
        match self.rock {
            Minus => chamber.empty(self.bottom, self.left + 4),
            Plus => {
                chamber.empty(self.bottom, self.left + 2)
                    && chamber.empty(self.bottom + 1, self.left + 3)
                    && chamber.empty(self.bottom + 2, self.left + 2)
            }
            Ell => {
                chamber.empty(self.bottom, self.left + 3)
                    && chamber.empty(self.bottom + 1, self.left + 3)
                    && chamber.empty(self.bottom + 2, self.left + 3)
            }
            Aye => {
                chamber.empty(self.bottom, self.left + 1)
                    && chamber.empty(self.bottom + 1, self.left + 1)
                    && chamber.empty(self.bottom + 2, self.left + 1)
                    && chamber.empty(self.bottom + 3, self.left + 1)
            }
            Ball => {
                chamber.empty(self.bottom, self.left + 2)
                    && chamber.empty(self.bottom + 1, self.left + 2)
            }
        }
    }

    fn check_down(&self, chamber: &Chamber) -> bool {
        if self.bottom == 0 {
            return false;
        }
        match self.rock {
            Minus => {
                for x in 0..4 {
                    if !chamber.empty(self.bottom - 1, self.left + x) {
                        return false;
                    }
                }
                true
            }
            Plus => {
                chamber.empty(self.bottom - 1, self.left + 1)
                    && chamber.empty(self.bottom, self.left)
                    && chamber.empty(self.bottom, self.left + 2)
            }
            Ell => {
                chamber.empty(self.bottom - 1, self.left)
                    && chamber.empty(self.bottom - 1, self.left + 1)
                    && chamber.empty(self.bottom - 1, self.left + 2)
            }
            Aye => chamber.empty(self.bottom - 1, self.left),
            Ball => {
                chamber.empty(self.bottom - 1, self.left)
                    && chamber.empty(self.bottom - 1, self.left + 1)
            }
        }
    }

    fn stop(&self, chamber: &mut Chamber) {
        match self.rock {
            Minus => {
                for x in 0..4 {
                    chamber.hit(self.bottom, self.left + x);
                }
            }
            Plus => {
                chamber.hit(self.bottom, self.left + 1);
                chamber.hit(self.bottom + 1, self.left);
                chamber.hit(self.bottom + 1, self.left + 1);
                chamber.hit(self.bottom + 1, self.left + 2);
                chamber.hit(self.bottom + 2, self.left + 1);
            }
            Ell => {
                chamber.hit(self.bottom, self.left);
                chamber.hit(self.bottom, self.left + 1);
                chamber.hit(self.bottom, self.left + 2);
                chamber.hit(self.bottom + 1, self.left + 2);
                chamber.hit(self.bottom + 2, self.left + 2);
            }
            Aye => {
                for y in 0..4 {
                    chamber.hit(self.bottom + y, self.left);
                }
            }
            Ball => {
                chamber.hit(self.bottom, self.left);
                chamber.hit(self.bottom, self.left + 1);
                chamber.hit(self.bottom + 1, self.left);
                chamber.hit(self.bottom + 1, self.left + 1);
            }
        }
    }

    fn drop(&mut self, chamber: &mut Chamber, motion: &mut dyn Iterator<Item = u8>) -> usize {
        let mut count = 0;
        loop {
            count += 1;
            let jet = motion.next().unwrap();
            match jet {
                LEFT => {
                    if self.check_left(chamber) {
                        self.left -= 1;
                    }
                }
                RIGHT => {
                    if self.check_right(chamber) {
                        self.left += 1;
                    }
                }
                _ => {
                    panic!("Jet should not push other directions");
                }
            }
            if self.check_down(chamber) {
                self.bottom -= 1;
            } else {
                self.stop(chamber);
                break count;
            }
        }
    }
}

use std::fmt;
impl fmt::Debug for Chamber {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let l = self.layers.len();
        let n = if l > 20 { l - 20 } else { 0 };
        for row in (n..l).rev() {
            f.write_str("|")?;
            for col in 0..7 {
                f.write_fmt(format_args!("{}", self.layers[row][col] as char))?;
            }
            f.write_str("|\n")?;
        }
        if n == 0 {
            f.write_str("\\-------/\n")?;
        } else {
            f.write_str("?--?-?--?\n")?;
        }
        Ok(())
    }
}

const LEFT: u8 = b'<';
const RIGHT: u8 = b'>';

pub fn a() {
    let ctxt = readfile("17");
    let mut chamber = Chamber::new();

    let mut bytes = ctxt.text.trim().bytes().cycle();
    for k in 0..2022 {
        chamber.fix();
        let start = chamber.height() + 3;
        let mut dropper = Dropper::new(Rock::new(k), start);
        dropper.drop(&mut chamber, &mut bytes);
    }
    println!("Tower of rocks is {} units tall", chamber.true_height());
}

const TARGET: usize = 1_000_000_000_000;

pub fn b() {
    let ctxt = readfile("17");
    let mut chamber = Chamber::new();

    let jet_cycle = ctxt.text.trim();
    let jet_cycle_len = jet_cycle.len();

    let mut bytes = jet_cycle.bytes().cycle();

    let mut clock = 0;
    let mut rocks = 0;
    loop {
        chamber.fix();
        let start = chamber.height() + 3;
        let mut dropper = Dropper::new(Rock::new(rocks), start);
        clock += dropper.drop(&mut chamber, &mut bytes);

        rocks +=1;
        if clock >= jet_cycle_len * 10 && (rocks % 5) == 0 {
            break;
        }
    }

    let r1 = rocks;
    let a = chamber.true_height();

    clock = 0;
    loop {
        chamber.fix();
        let start = chamber.height() + 3;
        let mut dropper = Dropper::new(Minus, start);
        clock += dropper.drop(&mut chamber, &mut bytes);
        chamber.fix();
        let start = chamber.height() + 3;
        let mut dropper = Dropper::new(Plus, start);
        clock += dropper.drop(&mut chamber, &mut bytes);
        chamber.fix();
        let start = chamber.height() + 3;
        let mut dropper = Dropper::new(Ell, start);
        clock += dropper.drop(&mut chamber, &mut bytes);
        chamber.fix();
        let start = chamber.height() + 3;
        let mut dropper = Dropper::new(Aye, start);
        clock += dropper.drop(&mut chamber, &mut bytes);
        chamber.fix();
        let start = chamber.height() + 3;
        let mut dropper = Dropper::new(Ball, start);
        clock += dropper.drop(&mut chamber, &mut bytes);

        rocks +=5;
        if clock % jet_cycle_len == 0 {
            break;
        }
    }

    let r2 = rocks;
    let b = chamber.true_height();

    clock = 0;
    loop {
        chamber.fix();
        let start = chamber.height() + 3;
        let mut dropper = Dropper::new(Minus, start);
        clock += dropper.drop(&mut chamber, &mut bytes);
        chamber.fix();
        let start = chamber.height() + 3;
        let mut dropper = Dropper::new(Plus, start);
        clock += dropper.drop(&mut chamber, &mut bytes);
        chamber.fix();
        let start = chamber.height() + 3;
        let mut dropper = Dropper::new(Ell, start);
        clock += dropper.drop(&mut chamber, &mut bytes);
        chamber.fix();
        let start = chamber.height() + 3;
        let mut dropper = Dropper::new(Aye, start);
        clock += dropper.drop(&mut chamber, &mut bytes);
        chamber.fix();
        let start = chamber.height() + 3;
        let mut dropper = Dropper::new(Ball, start);
        clock += dropper.drop(&mut chamber, &mut bytes);

        rocks +=5;
        if clock % jet_cycle_len == 0 {
            break;
        }
    }

    let r3 = rocks;
    let c = chamber.true_height();

    let r_inc = r3 - r2;
    let h_inc = c - b;
    if r_inc != r2 - r1 || h_inc != b - a {
        panic!("The pattern matching failed, giving up");
    }

    let skips = (TARGET - rocks) / r_inc;
    let skipped_height = h_inc * skips;
    rocks += r_inc * skips;

    loop {
        chamber.fix();
        let start = chamber.height() + 3;
        let mut dropper = Dropper::new(Rock::new(rocks), start);
        clock += dropper.drop(&mut chamber, &mut bytes);

        rocks +=1;
        if rocks == TARGET {
            break;
        }
    }
    println!("After all {TARGET} rocks, the tower ends up {} tall", skipped_height + chamber.true_height());
}
