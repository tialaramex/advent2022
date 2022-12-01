use jungle::readfile;

pub fn a() {
    let ctxt = readfile("01");
    let mut elves = Vec::new();
    let mut total: u64 = 0;
    for calorie in ctxt.lines() {
        if let Ok(c) = u64::from_str_radix(calorie, 10) {
            total += c;
        } else {
            elves.push(total);
            total = 0;
        }
    }
    elves.sort();
    println!("Elf with most calories has: {}", elves.last().unwrap());
}

pub fn b() {
    let ctxt = readfile("01");
    let mut elves = Vec::new();
    let mut total: u64 = 0;
    for calorie in ctxt.lines() {
        if let Ok(c) = u64::from_str_radix(calorie, 10) {
            total += c;
        } else {
            elves.push(total);
            total = 0;
        }
    }
    elves.sort();
    elves.reverse();
    let top = &elves[0..3];
    let total: u64 = top.iter().sum();
    println!("Top three elves have {total}");
}
