use jungle::readfile;

use core::cmp::Ordering;

#[derive(Copy, Clone, Debug)]
enum Distress<'t> {
    List(&'t str),
    Integer(&'t str),
}

fn item(mut s: &str) -> Option<(Distress, &str)> {
    const OPEN: u8 = b'[';
    const CLOSE: u8 = b']';
    const COMMA: u8 = b',';

    if s.is_empty() {
        return None;
    }
    if s.starts_with(',') {
        s = &s[1..];
    }
    if s.starts_with('[') {
        // List
        let b = s.as_bytes();
        let mut inner: u8 = 0;
        for k in 0..b.len() {
            if b[k] == OPEN {
                inner += 1;
            }
            if b[k] == CLOSE {
                inner -= 1;
                if inner == 0 {
                    let substr = &s[1..k];
                    let rest = &s[k + 1..];
                    return Some((Distress::List(substr), rest));
                }
            }
        }
        panic!("Couldn't find end of list in {s}");
    } else {
        // Integer
        let b = s.as_bytes();
        for k in 0..b.len() {
            if b[k] == COMMA {
                let substr = &s[0..k];
                let rest = &s[k..];
                return Some((Distress::Integer(substr), rest));
            }
        }
        return Some((Distress::Integer(s), ""));
    }
}

fn compare(mut left: &str, mut right: &str) -> Ordering {
    loop {
        let lnext = item(left);
        let rnext = item(right);
        match (lnext, rnext) {
            (None, None) => return Ordering::Equal,
            (_, None) => return Ordering::Greater,
            (None, _) => return Ordering::Less,
            (Some((Distress::List(l), lrest)), Some((Distress::List(r), rrest))) => {
                let o = compare(l, r);
                if o != Ordering::Equal {
                    return o;
                }
                left = lrest;
                right = rrest;
            }
            (Some((Distress::Integer(l), lrest)), Some((Distress::List(r), rrest))) => {
                let o = compare(l, r);
                if o != Ordering::Equal {
                    return o;
                }
                left = lrest;
                right = rrest;
            }
            (Some((Distress::List(l), lrest)), Some((Distress::Integer(r), rrest))) => {
                let o = compare(l, r);
                if o != Ordering::Equal {
                    return o;
                }
                left = lrest;
                right = rrest;
            }
            (Some((Distress::Integer(l), lrest)), Some((Distress::Integer(r), rrest))) => {
                let l: u8 = l.parse().unwrap();
                let r: u8 = r.parse().unwrap();
                let o = l.cmp(&r);
                if o != Ordering::Equal {
                    return o;
                }
                left = lrest;
                right = rrest;
            }
        }
    }
}

pub fn a() {
    let ctxt = readfile("13");
    let mut lines = ctxt.lines();
    let mut sum = 0;
    let mut group = 0;
    loop {
        group += 1;
        let left = lines.next().unwrap();
        let right = lines.next().unwrap();

        if compare(left, right) == Ordering::Less {
            sum += group;
        }

        if let Some(s) = lines.next() {
            if !s.is_empty() {
                panic!("Expected every third line to be blank");
            }
        } else {
            break;
        }
    }
    println!("Sum of indices of correctly ordered packets is {sum}");
}

pub fn b() {
    const DIV_A: &str = "[[2]]";
    const DIV_B: &str = "[[6]]";

    let ctxt = readfile("13");

    let mut v: Vec<&str> = ctxt.lines().filter(|l| !l.is_empty()).collect();
    v.push(DIV_A);
    v.push(DIV_B);
    v.sort_by(|a, b| compare(a, b));

    let mut count = 0;
    let mut pos_a = 0;
    let mut pos_b = 0;
    for line in v {
        count += 1;
        if line == DIV_A {
            pos_a = count;
        }
        if line == DIV_B {
            pos_b = count;
        }
    }
    println!("Decoder key is: {}", pos_a * pos_b);
}
