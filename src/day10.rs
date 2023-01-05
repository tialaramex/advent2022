use jungle::readfile;

fn run(lines: &mut dyn Iterator<Item = &str>, max: usize) -> Vec<i64> {
    let mut cycle = 0;
    let mut v = Vec::new();
    let mut addx: Option<i64> = None;
    let mut x: i64 = 1;
    loop {
        cycle += 1;

        v.push(x);

        if let Some(n) = addx {
            x += n;
            addx = None;
        } else if let Some(line) = lines.next() {
            if let Some((ins, n)) = line.split_once(' ') {
                /* Only addrx takes a parameter */
                if ins != "addx" {
                    panic!("Unexpected instruction {line}");
                }
                let n: i64 = n.parse().expect("addx should take an integer here");
                addx = Some(n);
            } else if line != "noop" {
                /* Only noop has no parameters */
                panic!("Unexpected instruction {line}");
            }
        }
        if cycle >= max {
            break;
        }
    }
    v
}

pub fn a() {
    fn strength(x: i64, cycle: usize) -> i64 {
        x * (cycle as i64)
    }

    let ctxt = readfile("10");
    let mut lines = ctxt.lines();
    let cycles = run(&mut lines, 220);

    let mut total: i64 = 0;
    for (cycle, &x) in cycles.iter().enumerate() {
        let cycle = cycle + 1;
        match cycle {
            20 | 60 | 100 | 140 | 180 | 220 => {
                total += strength(x, cycle);
            }
            _ => (),
        }
    }

    println!("Arbitrary total calculated was: {total}");
}

pub fn b() {
    let ctxt = readfile("10");
    let mut lines = ctxt.lines();
    let cycles = run(&mut lines, 240);

    for (cycle, &x) in cycles.iter().enumerate() {
        // Draw or don't draw sprite
        let col = (cycle % 40) as i64;
        if col + 1 >= x && col - 1 <= x {
            print!("█");
        } else {
            print!(" ");
        }
        if col == 39 {
            println!();
        }
    }
}
