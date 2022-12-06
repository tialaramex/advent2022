use jungle::readfile;

fn repeats(bytes: &[u8]) -> bool {
    let mut found = [false; 256];
    for b in bytes {
        if found[*b as usize] {
            return true;
        }
        found[*b as usize] = true;
    }
    false
}

fn start(bytes: &[u8], count: usize) -> usize {
    let mut offset = 0;
    for w in bytes.windows(count) {
        if !repeats(w) {
            return offset + count;
        }
        offset += 1;
    }
    panic!("Input data contains no start marker");
}

pub fn a() {
    let ctxt = readfile("06");
    let packet = ctxt.value().as_bytes();
    let offset = start(packet, 4);
    println!("{offset} characters processed before the first start-of-packet marker is detected");
}

pub fn b() {
    let ctxt = readfile("06");
    let packet = ctxt.value().as_bytes();
    let offset = start(packet, 14);
    println!("{offset} characters processed before the first start-of-message marker is detected");
}
