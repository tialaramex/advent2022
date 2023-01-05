use jungle::map::Map;
use jungle::readfile;

#[derive(Copy, Clone, Default, Eq, PartialEq)]
enum Elf {
    #[default]
    Empty,
    Here,
    North,
    South,
    West,
    East,
    Nope,
}

use Elf::{Empty, Here};

use std::fmt;
impl fmt::Debug for Elf {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_fmt(format_args!(
            "{}",
            match self {
                Empty => '.',
                Here => '#',
                Elf::North => '^',
                Elf::South => 'v',
                Elf::West => '<',
                Elf::East => '>',
                Elf::Nope => 'X',
            }
        ))?;
        Ok(())
    }
}

type Ground = Map<Elf>;

fn crowded(map: &Ground, x: isize, y: isize) -> bool {
    map.read(x - 1, y - 1).unwrap_or_default() == Here
        || map.read(x, y - 1).unwrap_or_default() == Here
        || map.read(x + 1, y - 1).unwrap_or_default() == Here
        || map.read(x - 1, y).unwrap_or_default() == Here
        || map.read(x + 1, y).unwrap_or_default() == Here
        || map.read(x - 1, y + 1).unwrap_or_default() == Here
        || map.read(x, y + 1).unwrap_or_default() == Here
        || map.read(x + 1, y + 1).unwrap_or_default() == Here
}

fn direction(map: &mut Ground, x: isize, y: isize, d: Elf, dx: isize, dy: isize) -> bool {
    if map.read(x + dx, y + dy).unwrap_or_default() == Here
        || map.read(x - dx, y - dy).unwrap_or_default() == Here
    {
        return false;
    }

    match map.read(x, y).unwrap_or_default() {
        Here => false,
        Empty => {
            map.write(x, y, d);
            true
        }
        // If any other elf is proposing this location nobody moves there
        _ => {
            map.write(x, y, Elf::Nope);
            true
        }
    }
}

fn propose(map: &mut Ground, offset: usize) {
    let elves = map.find(|elf| elf == Here);
    for (x, y) in elves {
        if crowded(map, x, y) {
            for k in 0..4 {
                match (k + offset) % 4 {
                    0 => {
                        if direction(map, x, y - 1, Elf::North, 1, 0) {
                            break;
                        }
                    }
                    1 => {
                        if direction(map, x, y + 1, Elf::South, 1, 0) {
                            break;
                        }
                    }
                    2 => {
                        if direction(map, x - 1, y, Elf::West, 0, 1) {
                            break;
                        }
                    }
                    3 => {
                        if direction(map, x + 1, y, Elf::East, 0, 1) {
                            break;
                        }
                    }
                    _ => {
                        panic!("Inconceivable!");
                    }
                }
            }
        }
    }
}

fn motion(map: &mut Ground) -> usize {
    let mut count = 0;
    let elves = map.find(|elf| elf != Empty && elf != Here);
    for (x, y) in elves {
        match map.read(x, y) {
            Some(Elf::North) => {
                map.write(x, y, Here);
                map.write(x, y + 1, Empty);
                count += 1;
            }
            Some(Elf::South) => {
                map.write(x, y, Here);
                map.write(x, y - 1, Empty);
                count += 1;
            }
            Some(Elf::West) => {
                map.write(x, y, Here);
                map.write(x + 1, y, Empty);
                count += 1;
            }
            Some(Elf::East) => {
                map.write(x, y, Here);
                map.write(x - 1, y, Empty);
                count += 1;
            }
            Some(Elf::Nope) => {
                map.write(x, y, Empty);
            }
            _ => {
                panic!("Unexpected situation during motion");
            }
        }
    }
    count
}

fn step(map: &mut Ground, offset: usize) -> bool {
    propose(map, offset);
    motion(map) > 0
}

fn read_map(filename: &str) -> Ground {
    let ctxt = readfile(filename);
    let mut map = Map::new();
    for (y, line) in ctxt.lines().enumerate() {
        for (x, byte) in line.bytes().enumerate() {
            match byte {
                b'#' => {
                    map.write(x as isize, y as isize, Here);
                }
                b'.' => {
                    map.write(x as isize, y as isize, Empty);
                }
                _ => {
                    panic!("Map input should only have elves");
                }
            }
        }
    }
    map
}

pub fn a() {
    let mut map: Ground = read_map("23");
    // Number of elves doesn't change
    let n = map.count(|&&elf| elf == Here) as isize;

    for k in 0..10 {
        step(&mut map, k);
    }

    let elves = map.find(|elf| elf != Empty);
    let mut min_x = isize::MAX;
    let mut max_x = isize::MIN;
    let mut min_y = isize::MAX;
    let mut max_y = isize::MIN;
    for (x, y) in elves {
        if x < min_x {
            min_x = x;
        }
        if y < min_y {
            min_y = y;
        }
        if y > max_y {
            max_y = y;
        }
        if x > max_x {
            max_x = x;
        }
    }

    let width = (max_x - min_x) + 1;
    let height = (max_y - min_y) + 1;
    println!(
        "{} Empty ground tiles in the smallest containing rectangle",
        (width * height) - n
    );
}

pub fn b() {
    let mut map: Ground = read_map("23");

    let mut k = 0;
    let steps = loop {
        if !step(&mut map, k) {
            break k + 1;
        }
        k += 1;
    };

    println!("Elves don't go anywhere on move #{steps}");
}
