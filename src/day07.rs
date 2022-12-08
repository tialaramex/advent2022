use jungle::readfile;

fn log2dirs(lines: &mut dyn Iterator<Item = &str>) -> Vec<usize> {
    let mut v = Vec::new();

    let mut sizes: Vec<usize> = Vec::new();
    let mut du: usize = 0;
    for line in lines {
        if let Some(command) = line.strip_prefix("$ ") {
            // Command input
            let two = command.split_once(' ');
            if let Some((command, dir)) = two {
                match (command, dir) {
                    ("cd", "..") => {
                        v.push(du);
                        du += sizes.pop().unwrap();
                    }
                    ("cd", _) => {
                        sizes.push(du);
                        du = 0;
                    }
                    (_, _) => {
                        panic!("Unanticipated command {command}");
                    }
                }
            } else if command != "ls" {
                panic!("Unanticipated command {command}");
            }
        } else {
            // Directory output
            let (size, _) = line.split_once(' ').unwrap();
            if size != "dir" {
                let size: usize = size.parse().unwrap();
                du += size;
            }
        }
    }
    loop {
        if sizes.is_empty() {
            break;
        }
        v.push(du);
        du += sizes.pop().unwrap();
    }
    v
}

pub fn a() {
    let ctxt = readfile("07");
    let dirs = log2dirs(&mut ctxt.lines());
    let sum: usize = dirs.into_iter().filter(|&v| v <= 100_000).sum();
    println!("{sum}");
}

const SPACE: usize = 70_000_000;
const NEED: usize = 30_000_000;

pub fn b() {
    let ctxt = readfile("07");
    let dirs = log2dirs(&mut ctxt.lines());
    let clear = dirs.last().unwrap() + NEED - SPACE;
    let least = dirs.into_iter().filter(|&v| v >= clear).min().unwrap();
    println!("Clearing away the smallest directory of at least {clear} saves {least} bytes");
}
