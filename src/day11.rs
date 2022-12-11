use jungle::readfile;

use std::collections::VecDeque;

type Worry = u64;

struct Monkey {
    items: VecDeque<Worry>,
    op: Box<dyn Fn(Worry) -> Worry>,
    divisor: Worry,
    if_true: usize,
    if_false: usize,
    inspections: usize,
}

fn make_op(s: &str) -> Box<dyn Fn(Worry) -> Worry> {
    let op = s
        .strip_prefix("  Operation: new = old ")
        .expect("Not an operation that I understand");
    let (f, n) = op.split_once(' ').unwrap();
    if n == "old" {
        match f {
            "*" => return Box::new(|x| x * x),
            "+" => return Box::new(|x| x + x),
            _ => {
                panic!("Operation uses operator {f}");
            }
        }
    }
    let n: Worry = n.parse().unwrap();
    match f {
        "*" => Box::new(move |x| x * n),
        "+" => Box::new(move |x| x + n),
        _ => {
            panic!("Operation uses operator {f}");
        }
    }
}

fn make_test(s: &str) -> Worry {
    let test = s
        .strip_prefix("  Test: divisible by ")
        .expect("Not an operation that I understand");
    test.parse().unwrap()
}

impl Monkey {
    fn read(lines: &mut dyn Iterator<Item = &str>) -> Self {
        let id = lines.next().unwrap();
        if !id.starts_with("Monkey ") {
            panic!("Not a monkey: {}", id);
        }
        let items = lines.next().unwrap();
        let items = items
            .strip_prefix("  Starting items: ")
            .expect("Not a list of starting items");
        let items: VecDeque<Worry> = items
            .split(", ")
            .map(|n| Worry::from_str_radix(n, 10).unwrap())
            .collect();
        let op = lines.next().unwrap();
        let op = make_op(op);
        let test = lines.next().unwrap();
        let divisor = make_test(test);
        let if_true = lines.next().unwrap();
        let if_true: usize = if_true
            .strip_prefix("    If true: throw to monkey ")
            .expect("Should specify what happens if test is true")
            .parse()
            .unwrap();
        let if_false = lines.next().unwrap();
        let if_false: usize = if_false
            .strip_prefix("    If false: throw to monkey ")
            .expect("Should specify what happens if test is false")
            .parse()
            .unwrap();
        Monkey {
            items,
            op,
            divisor,
            if_true,
            if_false,
            inspections: 0,
        }
    }

    // Returns which monkey gets the item and the new item worry level
    fn process(&self, n: Worry) -> (usize, Worry) {
        let mut m = (self.op)(n);
        // Monkey gets bored
        m /= 3;

        match m % self.divisor == 0 {
            true => (self.if_true, m),
            false => (self.if_false, m),
        }
    }

    fn do_list(&mut self) -> Vec<(usize, Worry)> {
        let mut v = Vec::new();
        let mut items = VecDeque::new();
        std::mem::swap(&mut items, &mut self.items);
        for item in items.drain(..) {
            self.inspections += 1;
            v.push(self.process(item));
        }
        v
    }

    // Returns which monkey gets the item and the new item worry level
    fn alt_process(&self, n: Worry, reduce: Worry) -> (usize, Worry) {
        // No monkey boredom, so reduce by a chosen factor
        let m = (self.op)(n) % reduce;

        match m % self.divisor == 0 {
            true => (self.if_true, m),
            false => (self.if_false, m),
        }
    }

    fn alt_do_list(&mut self, reduce: Worry) -> Vec<(usize, Worry)> {
        let mut v = Vec::new();
        let mut items = VecDeque::new();
        std::mem::swap(&mut items, &mut self.items);
        for item in items.drain(..) {
            self.inspections += 1;
            v.push(self.alt_process(item, reduce));
        }
        v
    }

    fn factor(&self) -> Worry {
        self.divisor
    }

    fn activity(&self) -> usize {
        self.inspections
    }

    fn give(&mut self, item: Worry) {
        self.items.push_back(item);
    }
}

pub fn a() {
    let ctxt = readfile("11");
    let mut monkeys = Vec::new();
    let mut lines = ctxt.lines();
    loop {
        monkeys.push(Monkey::read(&mut lines));
        if lines.next().is_none() {
            break;
        }
    }
    let count = monkeys.len();
    for _ in 0..20 {
        for k in 0..count {
            let split = monkeys[k].do_list();
            for (m, item) in split {
                monkeys[m].give(item);
            }
        }
    }
    let mut v: Vec<usize> = monkeys.into_iter().map(|m| m.activity()).collect();
    v.select_nth_unstable_by(1, |a, b| b.cmp(a));
    let business = v[0] * v[1];
    println!("Level of monkey business is {business}");
}

pub fn b() {
    let ctxt = readfile("11");
    let mut monkeys = Vec::new();
    let mut lines = ctxt.lines();
    loop {
        monkeys.push(Monkey::read(&mut lines));
        if lines.next().is_none() {
            break;
        }
    }
    let reduce: Worry = monkeys.iter().map(|m| Monkey::factor(m)).product();
    let count = monkeys.len();
    for _ in 0..10_000 {
        for k in 0..count {
            let split = monkeys[k].alt_do_list(reduce);
            for (m, item) in split {
                monkeys[m].give(item);
            }
        }
    }
    let mut v: Vec<usize> = monkeys.into_iter().map(|m| m.activity()).collect();
    v.select_nth_unstable_by(1, |a, b| b.cmp(a));
    let business = v[0] * v[1];
    println!("Level of monkey business is {business}");
}
