use jungle::readfile;

fn p(c: char) -> u32 {
    let priority = match c {
        'a'..='z' => 1 + (c as u8) - b'a',
        'A'..='Z' => 27 + (c as u8) - b'A',
        _ => {
            panic!("Unexpected item {c} in backpack");
        }
    };
    priority as u32
}

fn priority(s: &str) -> u32 {
    let (a, b) = s.split_at(s.len() / 2);
    for c in a.chars() {
        if b.contains(c) {
            return p(c);
        }
    }
    panic!("Somehow this bag's compartments are correct");
}

pub fn a() {
    let ctxt = readfile("03");
    let total: u32 = ctxt.lines().map(priority).sum();
    println!("Sum of priorities is {total}");
}

fn badge(s1: &str, s2: &str, s3: &str) -> u32 {
    for c in s1.chars() {
        if s2.contains(c) {
            if s3.contains(c) {
                return p(c);
            }
        }
    }
    panic!("Somehow the elves have no items in common");
}

pub fn b() {
    let ctxt = readfile("03");
    let mut lines = ctxt.lines();
    let mut total: u32 = 0;
    loop {
        if let Some(elf1) = lines.next() {
            let elf2 = lines.next().unwrap();
            let elf3 = lines.next().unwrap();
            total += badge(elf1, elf2, elf3);
        } else {
            break;
        }
    }
    println!("The total of priorities is {total}");
}
