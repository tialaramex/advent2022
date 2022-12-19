use jungle::readfile;

#[derive(Copy, Clone, Debug)]
struct Blueprint {
    num: usize,
    ore: usize,
    clay: usize,
    obsidian: (usize, usize),
    geode: (usize, usize),
}

use std::str::FromStr;
impl FromStr for Blueprint {
    type Err = &'static str;

    fn from_str(line: &str) -> Result<Self, Self::Err> {
        let line = line.strip_prefix("Blueprint ").unwrap();
        let (num, rest) = line.split_once(": Each ore robot costs ").unwrap();
        let num: usize = num.parse().unwrap();
        let (ore, rest) = rest.split_once(" ore. Each clay robot costs ").unwrap();
        let ore: usize = ore.parse().unwrap();
        let (clay, rest) = rest.split_once(" ore. Each obsidian robot costs ").unwrap();
        let clay: usize = clay.parse().unwrap();
        let (obs1, rest) = rest.split_once(" ore and ").unwrap();
        let (obs2, rest) = rest.split_once(" clay. Each geode robot costs ").unwrap();
        let obsidian: (usize, usize) = (obs1.parse().unwrap(), obs2.parse().unwrap());
        let (geo1, rest) = rest.split_once(" ore and ").unwrap();
        let geo2 = rest.strip_suffix(" obsidian.").unwrap();
        let geode: (usize, usize) = (geo1.parse().unwrap(), geo2.parse().unwrap());

        Ok(Blueprint {
            num,
            ore,
            clay,
            obsidian,
            geode,
        })
    }
}

#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
struct Me {
    geode: usize,
    geode_robot: usize,
    obsidian: usize,
    obsidian_robot: usize,
    clay: usize,
    clay_robot: usize,
    ore: usize,
    ore_robot: usize,
}

impl Me {
    fn new() -> Self {
        Self {
            ore_robot: 1,
            ore: 0,
            clay_robot: 0,
            clay: 0,
            obsidian_robot: 0,
            obsidian: 0,
            geode_robot: 0,
            geode: 0,
        }
    }

    fn next(&self, print: &Blueprint) -> Vec<Self> {

        let mut out = Vec::new();
        if self.ore >= print.ore {
            let mut make = self.clone();
            make.ore -= print.ore;
            make.ore += self.ore_robot;
            make.clay += self.clay_robot;
            make.obsidian += self.obsidian_robot;
            make.geode += self.geode_robot;
            make.ore_robot += 1;
            out.push(make);
        }
        if self.ore >= print.clay {
            let mut make = self.clone();
            make.ore -= print.clay;
            make.ore += self.ore_robot;
            make.clay += self.clay_robot;
            make.obsidian += self.obsidian_robot;
            make.geode += self.geode_robot;
            make.clay_robot += 1;
            out.push(make);
        }
        if self.ore >= print.obsidian.0 && self.clay >= print.obsidian.1 {
            let mut make = self.clone();
            make.ore -= print.obsidian.0;
            make.clay -= print.obsidian.1;
            make.ore += self.ore_robot;
            make.clay += self.clay_robot;
            make.obsidian += self.obsidian_robot;
            make.geode += self.geode_robot;
            make.obsidian_robot += 1;
            out.push(make);
        }
        if self.ore >= print.geode.0 && self.obsidian >= print.geode.1 {
            let mut make = self.clone();
            make.ore -= print.geode.0;
            make.obsidian -= print.geode.1;
            make.ore += self.ore_robot;
            make.clay += self.clay_robot;
            make.obsidian += self.obsidian_robot;
            make.geode += self.geode_robot;
            make.geode_robot += 1;
            out.push(make);
        }
        // Otherwise just keep the resources
        let mut mined = self.clone();
        mined.ore += self.ore_robot;
        mined.clay += self.clay_robot;
        mined.obsidian += self.obsidian_robot;
        mined.geode += self.geode_robot;
        out.push(mined);

        out
    }
}

fn extra(mut n: usize, t: usize) -> usize {
    let mut sum = 0;
    for _ in 0..t {
        sum += n;
        n += 1;
    }
    sum
}

fn run(print: &Blueprint) -> usize {
    let mut time = 24;
    let mut current: Vec<Me> = Vec::new();
    current.push(Me::new());

    loop {
        time -= 1;

//println!("{time} .. {}", current.len());
        let mut next: Vec<Me> = Vec::new();
        for maybe in current {
            next.append(&mut maybe.next(print));
        }
        next.sort_unstable();
        let best = next.last().unwrap();
//println!("best is {best:?}");
        let target = best.geode;
        if best.geode > 0 {
            next.retain(|&maybe| maybe.geode + extra(maybe.geode_robot, time) >= target);
        }

        next.dedup();
        current = next;
        if time == 0 {
            break;
        }
    }
    current.last().unwrap().geode
}

fn quality(print: &Blueprint) -> usize {
    run(print) * print.num
}

fn part2(print: &Blueprint) -> usize {
    let mut time = 32;
    let mut current: Vec<Me> = Vec::new();
    current.push(Me::new());

    loop {
        time -= 1;

//println!("{time} .. {}", current.len());
        let mut next: Vec<Me> = Vec::new();
        for maybe in current {
            next.append(&mut maybe.next(print));
        }
        next.sort_unstable();
        let best = next.last().unwrap();
//println!("best is {best:?}");
        let target = best.geode;
        if best.geode > 0 {
            next.retain(|&maybe| maybe.geode + extra(maybe.geode_robot, time) >= target);
        }

        next.dedup();
        current = next;
        if time == 0 {
            break;
        }
    }
    current.last().unwrap().geode
}


pub fn a() {
    let ctxt = readfile("19");
    let mut sum = 0;
    for line in ctxt.lines() {
        let print: Blueprint = line.parse().unwrap();
        let q = quality(&print);
        sum += q;
    }
    println!("Adding up quality for all blueprints: {sum}");
}

pub fn b() {
    let ctxt = readfile("19");
    let mut product = 1;
    for line in ctxt.lines().take(3) {
        let print: Blueprint = line.parse().unwrap();
        let best = part2(&print);
        product *= best;
    }
    println!("Product of best geode production for three blueprints: {product}");
}
