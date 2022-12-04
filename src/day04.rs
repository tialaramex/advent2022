use jungle::readfile;

fn assignment(s: &str) -> (u32, u32) {
    let (a,b) = s.split_once('-').unwrap();
    let a = str::parse(a).unwrap();
    let b = str::parse(b).unwrap();
    (a,b)
}

fn contains(a: (u32, u32), b: (u32, u32)) -> bool {
    a.0 <= b.0 && a.1 >= b.1
}

pub fn a() {
    let ctxt = readfile("04");
    let mut overlaps = 0;
    for line in ctxt.lines() {
        let (a,b) = line.split_once(',').unwrap();
        let a = assignment(a);
        let b = assignment(b);
        if contains(a,b) || contains(b,a) {
            overlaps += 1;
        }
    }
    println!("{overlaps} pairs fully overlapped");
}

fn unrelated(a: (u32, u32), b: (u32, u32)) -> bool {
    a.1 < b.0 || a.0 > b.1
}

pub fn b() {
    let ctxt = readfile("04");
    let mut overlaps = 0;
    for line in ctxt.lines() {
        let (a,b) = line.split_once(',').unwrap();
        let a = assignment(a);
        let b = assignment(b);
        if !unrelated(a,b) {
            overlaps += 1;
        }
    }
    println!("{overlaps} pairs overlapped at least partially");
}
