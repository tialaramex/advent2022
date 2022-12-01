use jungle::readfile;
use jungle::Contents;

fn calories(input: Contents) -> Vec<u64> {
    let mut elves = Vec::new();
    let mut total: u64 = 0;
    for calorie in input.lines() {
        if let Ok(c) = u64::from_str_radix(calorie, 10) {
            total += c;
        } else {
            elves.push(total);
            total = 0;
        }
    }
    elves
}

pub fn a() {
    let ctxt = readfile("01");
    let mut elves = calories(ctxt);
    let (_, most, _) = elves.select_nth_unstable_by(0, |a, b| b.cmp(a));
    println!("Elf with most calories has: {most}");
}

pub fn b() {
    let ctxt = readfile("01");
    let mut elves = calories(ctxt);
    elves.select_nth_unstable_by(2, |a, b| b.cmp(a));
    let top = &elves[0..3];
    let total: u64 = top.iter().sum();
    println!("Top three elves have {total}");
}
