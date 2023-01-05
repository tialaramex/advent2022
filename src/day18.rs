use jungle::readfile;

fn parse(line: &str) -> (usize, usize, usize) {
    let c: [&str; 3] = line
        .split(',')
        .collect::<Vec<_>>()
        .try_into()
        .expect("3D co-ordinates should have exactly three elements");
    let x: usize = c[0].parse().unwrap();
    let y: usize = c[1].parse().unwrap();
    let z: usize = c[2].parse().unwrap();
    (x, y, z)
}

fn extents(text: &str) -> (usize, usize, usize) {
    let mut max_x = 0;
    let mut max_y = 0;
    let mut max_z = 0;
    for line in text.lines() {
        let (x, y, z) = parse(line);
        if x > max_x {
            max_x = x;
        }
        if y > max_y {
            max_y = y;
        }
        if z > max_z {
            max_z = z;
        }
    }
    (max_x, max_y, max_z)
}

const GRID_SIZE: usize = 32;

type Coord3 = (usize, usize, usize);

struct Crude {
    grid: [[[bool; GRID_SIZE]; GRID_SIZE]; GRID_SIZE],
    water: [[[bool; GRID_SIZE]; GRID_SIZE]; GRID_SIZE],
}

impl Default for Crude {
    fn default() -> Self {
        let empty = [[[false; GRID_SIZE]; GRID_SIZE]; GRID_SIZE];
        Crude {
            grid: empty,
            water: empty,
        }
    }
}

impl Crude {
    fn set(&mut self, x: usize, y: usize, z: usize) {
        self.grid[x][y][z] = true;
    }

    fn big_enough(&self, x: usize, y: usize, z: usize) -> bool {
        x + 2 < GRID_SIZE && y + 2 < GRID_SIZE && z + 2 < GRID_SIZE
    }

    fn count(&self, x: usize, y: usize, z: usize) -> usize {
        if self.check(x, y, z) {
            let mut count = 0;
            count += if self.check(x, y, z - 1) { 0 } else { 1 };
            count += if self.check(x, y - 1, z) { 0 } else { 1 };
            count += if self.check(x - 1, y, z) { 0 } else { 1 };
            count += if self.check(x, y, z + 1) { 0 } else { 1 };
            count += if self.check(x, y + 1, z) { 0 } else { 1 };
            count += if self.check(x + 1, y, z) { 0 } else { 1 };
            count
        } else {
            0
        }
    }

    // For symmetry with the count method write it this way
    #[allow(clippy::bool_to_int_with_if)]
    fn count_water(&self, x: usize, y: usize, z: usize) -> usize {
        if self.check(x, y, z) {
            let mut count = 0;
            count += if self.water[x][y][z - 1] { 1 } else { 0 };
            count += if self.water[x][y - 1][z] { 1 } else { 0 };
            count += if self.water[x - 1][y][z] { 1 } else { 0 };
            count += if self.water[x][y][z + 1] { 1 } else { 0 };
            count += if self.water[x][y + 1][z] { 1 } else { 0 };
            count += if self.water[x + 1][y][z] { 1 } else { 0 };

            count
        } else {
            0
        }
    }

    fn check(&self, x: usize, y: usize, z: usize) -> bool {
        self.grid[x][y][z]
    }

    fn surface(&self) -> usize {
        let mut count = 0;
        for x in 1..(GRID_SIZE - 1) {
            for y in 1..(GRID_SIZE - 1) {
                for z in 1..(GRID_SIZE - 1) {
                    count += self.count(x, y, z);
                }
            }
        }
        count
    }

    fn exterior(&self) -> usize {
        let mut count = 0;
        for x in 1..(GRID_SIZE - 1) {
            for y in 1..(GRID_SIZE - 1) {
                for z in 1..(GRID_SIZE - 1) {
                    count += self.count_water(x, y, z);
                }
            }
        }
        count
    }

    fn flood(&mut self) {
        let mut queue: Vec<Coord3> = Vec::new();
        queue.push((0, 0, 0));
        while !queue.is_empty() {
            let mut next: Vec<Coord3> = Vec::new();
            for (x, y, z) in queue {
                if !self.check(x, y, z) && !self.water[x][y][z] {
                    self.water[x][y][z] = true;
                    if x > 0 {
                        next.push((x - 1, y, z));
                    }
                    if y > 0 {
                        next.push((x, y - 1, z));
                    }
                    if z > 0 {
                        next.push((x, y, z - 1));
                    }
                    if x + 1 < GRID_SIZE {
                        next.push((x + 1, y, z));
                    }
                    if y + 1 < GRID_SIZE {
                        next.push((x, y + 1, z));
                    }
                    if z + 1 < GRID_SIZE {
                        next.push((x, y, z + 1));
                    }
                }
            }
            queue = next;
        }
    }
}

pub fn a() {
    let ctxt = readfile("18");
    let (mx, my, mz) = extents(ctxt.value());
    let mut crude: Crude = Default::default();
    if !crude.big_enough(mx, my, mz) {
        panic!("Input extent {mx},{my},{mz} is too large for our grid");
    }
    for line in ctxt.lines() {
        let (x, y, z) = parse(line);
        crude.set(x + 1, y + 1, z + 1);
    }

    println!("Surface area calculated as: {}", crude.surface());
}

pub fn b() {
    let ctxt = readfile("18");
    let (mx, my, mz) = extents(ctxt.value());
    let mut crude: Crude = Default::default();
    if !crude.big_enough(mx, my, mz) {
        panic!("Input extent {mx},{my},{mz} is too large for our grid");
    }
    for line in ctxt.lines() {
        let (x, y, z) = parse(line);
        crude.set(x + 1, y + 1, z + 1);
    }

    crude.flood();
    println!("Exterior surface area calculated as: {}", crude.exterior());
}
