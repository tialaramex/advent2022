use jungle::readfile;

type Number = u8;

#[derive(Copy, Clone, Debug)]
struct Blueprint {
    num: Number,
    ore: Number,
    clay: Number,
    obsidian: (Number, Number),
    geode: (Number, Number),
    most_ore: Number,
}

use std::str::FromStr;
impl FromStr for Blueprint {
    type Err = &'static str;

    fn from_str(line: &str) -> Result<Self, Self::Err> {
        let line = line.strip_prefix("Blueprint ").unwrap();
        let (num, rest) = line.split_once(": Each ore robot costs ").unwrap();
        let num: Number = num.parse().unwrap();
        let (ore, rest) = rest.split_once(" ore. Each clay robot costs ").unwrap();
        let ore: Number = ore.parse().unwrap();
        let (clay, rest) = rest.split_once(" ore. Each obsidian robot costs ").unwrap();
        let clay: Number = clay.parse().unwrap();
        let (obs1, rest) = rest.split_once(" ore and ").unwrap();
        let (obs2, rest) = rest.split_once(" clay. Each geode robot costs ").unwrap();
        let obsidian: (Number, Number) = (obs1.parse().unwrap(), obs2.parse().unwrap());
        let (geo1, rest) = rest.split_once(" ore and ").unwrap();
        let geo2 = rest.strip_suffix(" obsidian.").unwrap();
        let geode: (Number, Number) = (geo1.parse().unwrap(), geo2.parse().unwrap());
        let most_ore = Number::max(Number::max(ore, clay), Number::max(obsidian.0, geode.0));

        Ok(Blueprint {
            num,
            ore,
            clay,
            obsidian,
            geode,
            most_ore,
        })
    }
}

#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
struct Me {
    geode: Number,
    geode_robot: Number,
    obsidian: Number,
    obsidian_robot: Number,
    clay: Number,
    clay_robot: Number,
    ore: Number,
    ore_robot: Number,
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

    fn harvest(&mut self) {
        self.ore += self.ore_robot;
        self.clay += self.clay_robot;
        self.obsidian += self.obsidian_robot;
        self.geode += self.geode_robot;
    }

    fn next(&self, print: &Blueprint) -> Vec<Self> {
        let mut out = Vec::new();

        // Make an ore robot if we have enough resource and we may need more ore
        if self.ore >= print.ore && self.ore_robot < print.most_ore {
            let mut make = *self;
            make.ore -= print.ore;
            make.harvest();
            make.ore_robot += 1;
            out.push(make);
        }

        // Make a clay robot if we have enough resource and we may need more clay
        if self.ore >= print.clay && self.clay_robot < print.obsidian.1 {
            let mut make = *self;
            make.ore -= print.clay;
            make.harvest();
            make.clay_robot += 1;
            out.push(make);
        }

        // Make an obsidian robot if we have enough resource and we may need more obsidian
        if self.ore >= print.obsidian.0
            && self.clay >= print.obsidian.1
            && self.obsidian_robot < print.geode.1
        {
            let mut make = *self;
            make.ore -= print.obsidian.0;
            make.clay -= print.obsidian.1;
            make.harvest();
            make.obsidian_robot += 1;
            out.push(make);
        }

        // Definitely make a geode robot if we can
        if self.ore >= print.geode.0 && self.obsidian >= print.geode.1 {
            let mut make = *self;
            make.ore -= print.geode.0;
            make.obsidian -= print.geode.1;
            make.harvest();
            make.geode_robot += 1;
            out.push(make);
        } else {
            // If we can't yet afford a geode robot, try saving up
            let mut mined = *self;
            mined.harvest();
            out.push(mined);
        }

        out
    }
}

// How many extra geodes could possibly get opened
// Assume each day one extra robot is available (no more could possibly be made)
fn extra(mut n: Number, t: Number) -> Number {
    let mut sum = 0;
    for _ in 0..t {
        sum += n;
        n += 1;
    }
    sum
}

fn run(print: &Blueprint, mut time: u8) -> Number {
    let mut current: Vec<Me> = Vec::new();
    current.push(Me::new());

    loop {
        time -= 1;

        let mut next: Vec<Me> = Vec::with_capacity(current.len());
        for maybe in current {
            next.append(&mut maybe.next(print));
        }
        next.select_nth_unstable_by(0, |a, b| b.cmp(a));
        let best = next.first().unwrap();
        if time == 0 {
            return best.geode;
        }
        let target = best.geode + (best.geode_robot * time);
        if target > 0 {
            next.retain(|&maybe| maybe.geode + extra(maybe.geode_robot, time) >= target);
        }

        next.sort_unstable();
        next.dedup();
        current = next;
    }
}

fn quality(print: &Blueprint) -> u32 {
    run(print, 24) as u32 * print.num as u32
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
        let best = run(&print, 32) as u32;
        product *= best;
    }
    println!("Product of best geode production for three blueprints: {product}");
}
