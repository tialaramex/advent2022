use jungle::readfile;

fn mistake(s: &str) -> u32 {
    match s {
        "A X" => 4,
        "B X" => 1,
        "C X" => 7,
        "A Y" => 8,
        "B Y" => 5,
        "C Y" => 2,
        "A Z" => 3,
        "B Z" => 9,
        "C Z" => 6,
        _ => {
            panic!("Impossible combination {s}");
        }
    }
}

fn correct(s: &str) -> u32 {
    match s {
        "A X" => 3,
        "B X" => 1,
        "C X" => 2,
        "A Y" => 4,
        "B Y" => 5,
        "C Y" => 6,
        "A Z" => 8,
        "B Z" => 9,
        "C Z" => 7,
        _ => {
            panic!("Impossible combination {s}");
        }
    }
}

pub fn a() {
    let ctxt = readfile("02");
    let total: u32 = ctxt.lines().map(mistake).sum();
    println!("Following the guide according to our mistaken understanding scores: {total}");
}

pub fn b() {
    let ctxt = readfile("02");
    let total: u32 = ctxt.lines().map(correct).sum();
    println!("Following the guide correctly scores: {total}");
}
