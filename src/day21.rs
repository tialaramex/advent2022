use jungle::readfile;

type Num = i64;

#[derive(Copy, Clone, Eq, PartialEq, Hash)]
struct MonkeyId(u32);

use std::str::FromStr;
impl FromStr for MonkeyId {
    type Err = &'static str;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let bytes = s.as_bytes();
        if bytes.len() < 4 {
            return Err("Monkey label is too short");
        }
        if bytes.len() > 4 {
            return Err("Monkey label is too long");
        }
        let a = bytes[0] as u32;
        let b = (bytes[1] as u32) << 8;
        let c = (bytes[2] as u32) << 16;
        let d = (bytes[3] as u32) << 24;

        Ok(MonkeyId(a + b + c + d))
    }
}

use std::fmt;
impl fmt::Debug for MonkeyId {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let a = (self.0 & 0xFF) as u8;
        let b = ((self.0 >> 8) & 0xFF) as u8;
        let c = ((self.0 >> 16) & 0xFF) as u8;
        let d = (self.0 >> 24) as u8;
        f.write_fmt(format_args!(
            "{}{}{}{}",
            a as char, b as char, c as char, d as char
        ))?;
        Ok(())
    }
}

#[derive(Copy, Clone, Debug)]
enum Monkey {
    Human,
    Value(Num),
    Equal(MonkeyId, MonkeyId),
    Add(MonkeyId, MonkeyId),
    Sub(MonkeyId, MonkeyId),
    Mul(MonkeyId, MonkeyId),
    Div(MonkeyId, MonkeyId),
}

impl Monkey {
    fn a(&self) -> MonkeyId {
        match *self {
            Monkey::Add(a, _b) => a,
            Monkey::Sub(a, _b) => a,
            Monkey::Mul(a, _b) => a,
            Monkey::Div(a, _b) => a,
            Monkey::Equal(a, _b) => a,
            _ => {
                panic!("Monkey {self:?} does not have 'a'");
            }
        }
    }

    fn b(&self) -> MonkeyId {
        match *self {
            Monkey::Add(_a, b) => b,
            Monkey::Sub(_a, b) => b,
            Monkey::Mul(_a, b) => b,
            Monkey::Div(_a, b) => b,
            Monkey::Equal(_a, b) => b,
            _ => {
                panic!("Monkey {self:?} does not have 'b'");
            }
        }
    }
}

use std::collections::HashMap;

fn parse(line: &str) -> (MonkeyId, Monkey) {
    let (id, rest) = line.split_once(": ").unwrap();
    let id: MonkeyId = id.parse().unwrap();
    if let Ok(n) = rest.parse::<Num>() {
        (id, Monkey::Value(n))
    } else {
        let mut expr = rest.split(' ');
        let a = expr.next().unwrap();
        let a: MonkeyId = a.parse().unwrap();
        let op = expr.next().unwrap();
        let b = expr.next().unwrap();
        let b: MonkeyId = b.parse().unwrap();
        match op {
            "+" => (id, Monkey::Add(a, b)),
            "-" => (id, Monkey::Sub(a, b)),
            "*" => (id, Monkey::Mul(a, b)),
            "/" => (id, Monkey::Div(a, b)),
            _ => panic!("This monkey has unexpected expression: {line}"),
        }
    }
}

struct Troupe {
    monkeys: HashMap<MonkeyId, Monkey>,
    values: HashMap<MonkeyId, Num>,
}

impl Troupe {
    fn new() -> Self {
        Troupe {
            monkeys: HashMap::new(),
            values: HashMap::new(),
        }
    }

    fn add(&mut self, line: &str) {
        let (id, monkey) = parse(line);
        self.monkeys.insert(id, monkey);
        if let Monkey::Value(n) = monkey {
            self.values.insert(id, n);
        }
    }

    fn get(&self, id: MonkeyId) -> Option<Num> {
        self.values.get(&id).copied()
    }

    fn zap(&mut self, human: MonkeyId, root: MonkeyId) {
        self.values.remove(&human);
        let humn = self.monkeys.get_mut(&human).unwrap();
        *humn = Monkey::Human;

        let root = self.monkeys.get_mut(&root).unwrap();
        let a = root.a();
        let b = root.b();
        *root = Monkey::Equal(a, b);
    }

    fn once(&mut self) -> usize {
        let mut count = 0;
        for (&id, monkey) in self.monkeys.iter() {
            if self.values.contains_key(&id) {
                continue;
            } else {
                match monkey {
                    Monkey::Add(a, b) => {
                        let a = self.get(*a);
                        let b = self.get(*b);
                        if let (Some(a), Some(b)) = (a, b) {
                            self.values.insert(id, a + b);
                            count += 1;
                        }
                    }
                    Monkey::Sub(a, b) => {
                        let a = self.get(*a);
                        let b = self.get(*b);
                        if let (Some(a), Some(b)) = (a, b) {
                            self.values.insert(id, a - b);
                            count += 1;
                        }
                    }
                    Monkey::Mul(a, b) => {
                        let a = self.get(*a);
                        let b = self.get(*b);
                        if let (Some(a), Some(b)) = (a, b) {
                            self.values.insert(id, a * b);
                            count += 1;
                        }
                    }
                    Monkey::Div(a, b) => {
                        let a = self.get(*a);
                        let b = self.get(*b);
                        if let (Some(a), Some(b)) = (a, b) {
                            self.values.insert(id, a / b);
                            count += 1;
                        }
                    }
                    _ => {}
                }
            }
        }
        count
    }

    fn trace(&self, id: MonkeyId) -> Num {
        let mut next = id;
        let mut target = 0;
        loop {
            let monkey = self.monkeys.get(&next).unwrap();
            match *monkey {
                Monkey::Value(_) => {
                    panic!("Unexpected value while tracing root back");
                }
                Monkey::Human => return target,
                Monkey::Equal(a, b) => {
                    let na = self.get(a);
                    let nb = self.get(b);
                    if let Some(n) = na {
                        target = n;
                        next = b;
                    } else if let Some(n) = nb {
                        target = n;
                        next = a;
                    } else {
                        panic!("Impossible to trace from this scenario");
                    }
                }
                Monkey::Add(a, b) => {
                    let na = self.get(a);
                    let nb = self.get(b);
                    if let Some(n) = na {
                        target -= n;
                        next = b;
                    } else if let Some(n) = nb {
                        target -= n;
                        next = a;
                    } else {
                        panic!("Impossible to trace from this scenario");
                    }
                }
                Monkey::Sub(a, b) => {
                    let na = self.get(a);
                    let nb = self.get(b);
                    if let Some(n) = na {
                        target = -(target - n);
                        next = b;
                    } else if let Some(n) = nb {
                        target += n;
                        next = a;
                    } else {
                        panic!("Impossible to trace from this scenario");
                    }
                }
                Monkey::Mul(a, b) => {
                    let na = self.get(a);
                    let nb = self.get(b);
                    if let Some(n) = na {
                        target /= n;
                        next = b;
                    } else if let Some(n) = nb {
                        target /= n;
                        next = a;
                    } else {
                        panic!("Impossible to trace from this scenario");
                    }
                }
                Monkey::Div(a, b) => {
                    let na = self.get(a);
                    let nb = self.get(b);
                    if let Some(_) = na {
                        panic!("It's tricky to handle the other side of division so I did not");
                    } else if let Some(n) = nb {
                        target *= n;
                        next = a;
                    } else {
                        panic!("Impossible to trace from this scenario");
                    }
                }
            }
        }
    }
}

pub fn a() {
    let ctxt = readfile("21");
    let mut troupe = Troupe::new();
    for line in ctxt.lines() {
        troupe.add(line);
    }
    loop {
        if troupe.once() == 0 {
            break;
        }
    }
    let root: MonkeyId = "root".parse().unwrap();
    println!("Root monkey number is: {}", troupe.get(root).unwrap());
}

pub fn b() {
    let ctxt = readfile("21");
    let mut troupe = Troupe::new();
    for line in ctxt.lines() {
        troupe.add(line);
    }
    let humn: MonkeyId = "humn".parse().unwrap();
    let root: MonkeyId = "root".parse().unwrap();
    troupe.zap(humn, root);

    loop {
        if troupe.once() == 0 {
            break;
        }
    }

    let input = troupe.trace(root);
    println!("We should yell {input} to pass the equality test");
}
