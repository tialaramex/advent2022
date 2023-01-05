use jungle::readfile;

const STACKS: usize = 9;
const EMPTY: Vec<u8> = Vec::new();

fn interpret(text: &str) -> ([Vec<u8>; STACKS], Vec<&str>) {
    let mut lines = text.lines();
    let mut stacks = [EMPTY; STACKS];

    loop {
        let line = lines
            .next()
            .expect("Should be both list of crates and instructions in file");
        if line.len() != (4 * STACKS) - 1 {
            break;
        }
        let bytes = line.as_bytes();
        for (k, bytes) in bytes.chunks(4).enumerate() {
            if bytes[0] == b'[' && bytes[2] == b']' {
                stacks[k].push(bytes[1]);
            }
        }
    }

    // Stacks are the other way up
    for v in stacks.iter_mut() {
        v.reverse();
    }

    let mut instructions = Vec::<&str>::new();
    for line in lines {
        instructions.push(line);
    }

    (stacks, instructions)
}

fn step(line: &str) -> (usize, usize, usize) {
    let line = line
        .strip_prefix("move ")
        .expect("Should be move instruction");
    let (count, rest) = line.split_once(" from ").unwrap();
    let count: usize = count.parse().unwrap();
    let (from, to) = rest.split_once(" to ").unwrap();
    // The stack numbers count from 1 instead of 0
    let from: usize = from.parse().unwrap();
    let to: usize = to.parse().unwrap();
    (count, from - 1, to - 1)
}

pub fn a() {
    let ctxt = readfile("05");
    let (mut stacks, ins) = interpret(&ctxt.text);
    for i in ins {
        let (count, from, to) = step(i);
        for _ in 0..count {
            let top = stacks[from]
                .pop()
                .expect("Stacks we're taking from shouldn't be empty");
            stacks[to].push(top);
        }
    }
    for stack in stacks.iter() {
        let top = stack.last().unwrap();
        print!("{}", *top as char);
    }
    println!(" is the order of crates on top");
}

pub fn b() {
    let ctxt = readfile("05");
    let (mut stacks, ins) = interpret(&ctxt.text);
    for i in ins {
        let (count, from, to) = step(i);
        let mut tmp = Vec::new();
        for _ in 0..count {
            let top = stacks[from]
                .pop()
                .expect("Stacks we're taking from shouldn't be empty");
            tmp.push(top);
        }
        for _ in 0..count {
            let top = tmp.pop().expect("Temporary stack shouldn't be empty");
            stacks[to].push(top);
        }
    }
    for stack in stacks.iter() {
        let top = stack.last().unwrap();
        print!("{}", *top as char);
    }
    println!(" is the order of crates on top");
}
