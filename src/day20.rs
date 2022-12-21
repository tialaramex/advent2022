use jungle::readfile;

use std::collections::VecDeque;

type Num = i64;

struct List {
    v: VecDeque<(usize, Num)>,
}

impl List {
    fn new(input: Vec<isize>) -> Self {
        let v = input
            .into_iter()
            .enumerate()
            .map(|(u, i)| (u, i as Num))
            .collect();

        Self { v }
    }

    fn coords(&self) -> (Num, Num, Num) {
        let zero = self.v.iter().position(|&(_u, i)| i == 0).unwrap();
        let x = (zero + 1000) % self.v.len();
        let x = self.v[x];
        let y = (zero + 2000) % self.v.len();
        let y = self.v[y];
        let z = (zero + 3000) % self.v.len();
        let z = self.v[z];
        (x.1, y.1, z.1)
    }

    fn stir(&mut self, k: usize) {
        // Find where the k'th item is now
        let pos = self.v.iter().position(|&(u, _i)| u == k).unwrap();
        // Rotate left until that's at the front
        self.v.rotate_left(pos);
        // Pop Item from the front
        let item = self.v.pop_front().unwrap();
        // Rotate right correct amount
        let sz = self.v.len() as Num;
        let new = item.1.rem_euclid(sz) as usize;
        self.v.rotate_left(new);
        // Push Item onto the front
        self.v.push_front(item);
        // Don't bother rotating back since order has no particular meaning
    }

    fn mix(&mut self) {
        for k in 0..self.v.len() {
            self.stir(k);
        }
    }
}

pub fn a() {
    let ctxt = readfile("20");
    let input: Vec<isize> = ctxt.numbers().collect();
    let mut circle: List = List::new(input);
    circle.mix();

    let (x, y, z) = circle.coords();
    let sum = x + y + z;
    println!("Sum is {x} + {y} + {z} = {sum}");
}

pub fn b() {
    let ctxt = readfile("20");
    let input: Vec<isize> = ctxt.numbers().map(|n| n * 811589153).collect();
    let mut circle: List = List::new(input);
    for _ in 0..10 {
        circle.mix();
    }

    let (x, y, z) = circle.coords();
    let sum = x + y + z;
    println!("Sum is {x} + {y} + {z} = {sum}");
}
