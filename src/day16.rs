use jungle::readfile;
use std::collections::HashMap;

#[derive(Copy, Clone, Eq, PartialEq, Hash, Ord, PartialOrd)]
struct ValveId(u16);

use std::str::FromStr;
impl FromStr for ValveId {
    type Err = &'static str;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let b = s.as_bytes();
        if b.len() < 2 {
            return Err("Valve label is too short");
        }
        if b.len() > 2 {
            return Err("Valve label is too long");
        }
        let a = (b[0] - b'A') as u16;
        let b = (b[1] - b'A') as u16;
        Ok(ValveId(26 * a + b))
    }
}

use std::fmt;
impl fmt::Debug for ValveId {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let a = (self.0 / 26) as u8 + b'A';
        let b = (self.0 % 26) as u8 + b'A';
        f.write_fmt(format_args!("{}{}", a as char, b as char))?;
        Ok(())
    }
}

impl ValveId {
    fn offset(&self) -> usize {
        self.0 as usize
    }
}

#[derive(Copy, Clone, Debug, Default, Eq, PartialEq, Hash, Ord, PartialOrd)]
struct Opened(u16);

impl Opened {
    fn is_closed(&self, c: u8) -> bool {
        self.0 & (1 << c) == 0
    }

    fn open(&mut self, c: u8) {
        self.0 |= 1 << c;
    }

    fn compatible(&self, other: Opened) -> bool {
        (self.0 & other.0) == 0
    }
}

type Flow = u16;

#[derive(Debug, Default)]
struct Valve {
    flow: Flow,
    openid: Option<u8>,
    to: Vec<ValveId>,
}

const VALVES: usize = 26 * 26;
const REMAINING: usize = 30;

struct Map {
    valves: [Option<Valve>; VALVES],
    remaining: [u32; REMAINING],
    flows: Vec<Flow>,
}

impl Default for Map {
    fn default() -> Self {
        Self {
            valves: [const { None }; VALVES],
            remaining: [0; REMAINING],
            flows: Vec::new(),
        }
    }
}

impl Map {
    fn add(&mut self, id: ValveId, v: Valve) {
        self.flows.push(v.flow);
        self.valves[id.offset()] = Some(v);
    }

    fn get(&self, id: ValveId) -> &Option<Valve> {
        &self.valves[id.offset()]
    }

    fn remainder(&self, t: u32) -> u32 {
        self.remaining[t as usize]
    }

    // Calculate maximum remaining pressure which could be released
    fn calculate(&mut self) {
        let mut v = self.flows.clone();
        v.sort_unstable();
        v.reverse();
        for t in 0..REMAINING {
            let mut remaining = 0;
            for k in 0..((t + 1) / 2) {
                if k < v.len() {
                    let since = t - (k * 2);
                    remaining += (v[k] as u32) * (since as u32);
                }
            }
            self.remaining[t] = remaining;
        }
    }
}

#[derive(Copy, Clone, Debug, Eq, PartialEq, Hash, Ord, PartialOrd)]
struct Visitor {
    released: u32,
    at: ValveId,
    opened: Opened,
}

impl Visitor {
    fn new() -> Self {
        Self {
            released: 0,
            at: "AA".parse().unwrap(),
            opened: Default::default(),
        }
    }

    fn next(&self, time: u32, at: &Valve) -> (Vec<Self>, Vec<Self>) {
        let mut out = Vec::new();
        let mut solutions = Vec::new();
        if let Some(n) = at.openid {
            if self.opened.is_closed(n) {
                let mut opener = *self;
                opener.opened.open(n);
                opener.released += time * (at.flow as u32);
                out.push(opener);
                solutions.push(opener);
            }
        }
        for exit in &at.to {
            let mut mover = *self;
            mover.at = *exit;
            out.push(mover);
        }
        (out, solutions)
    }
}

fn tunnels(s: &str) -> Vec<ValveId> {
    let mut v = Vec::new();
    if let Some(valve) = s.strip_prefix("tunnel leads to valve ") {
        v.push(valve.parse().unwrap());
    } else {
        let valves = s.strip_prefix("tunnels lead to valves ").unwrap();
        for valve in valves.split(", ") {
            v.push(valve.parse().unwrap());
        }
    }
    v
}

fn parse(s: &str) -> (ValveId, Valve) {
    let s = s.strip_prefix("Valve ").unwrap();
    let (v, rest) = s.split_once(" has flow rate=").unwrap();
    let v: ValveId = v.parse().unwrap();
    let (flow, rest) = rest.split_once("; ").unwrap();
    let flow: Flow = flow.parse().unwrap();

    let to = tunnels(rest);

    let valve = Valve {
        flow,
        openid: None,
        to,
    };
    (v, valve)
}

fn part1(map: &Map) -> u32 {
    let mut time = 30;
    let mut current: Vec<Visitor> = vec![Visitor::new()];

    loop {
        time -= 1;
        let mut next: Vec<Visitor> = Vec::new();

        for possible in current {
            let x = map.get(possible.at);
            if let Some(valve) = x {
                let (onward, solutions) = possible.next(time, valve);
                for maybe in onward {
                    next.push(maybe);
                }
                for maybe in solutions {
                    next.push(maybe);
                }
            } else {
                panic!("Somehow {:?} is not on the map", possible.at);
            }
        }

        next.sort_unstable();
        let best = next.last().unwrap().released;
        let remain = map.remainder(time);
        let need = if remain > best { 0 } else { best - remain };

        // Eliminate possibilities that can't get enough pressure to beat the leader
        next.retain(|&maybe| maybe.released >= need);
        next.dedup();

        current = next;
        if time == 0 {
            break;
        }
    }

    current.last().unwrap().released
}

fn part2(map: &Map) -> u32 {
    // 4 minutes to teach an elephant about valves
    let mut time = 26;
    let mut routes: HashMap<Opened, u32> = HashMap::new();
    let mut current: Vec<Visitor> = vec![Visitor::new()];

    loop {
        time -= 1;
        let mut next: Vec<Visitor> = Vec::new();

        for possible in current {
            let x = map.get(possible.at);
            if let Some(valve) = x {
                let (onward, solutions) = possible.next(time, valve);
                for maybe in onward {
                    next.push(maybe);
                }
                for maybe in solutions {
                    next.push(maybe);
                    let current = routes.entry(maybe.opened).or_insert(maybe.released);
                    if *current < maybe.released {
                        *current = maybe.released;
                    }
                }
            } else {
                panic!("Somehow {:?} is not on the map", possible.at);
            }
        }

        // De-duplication requires a sorted slice
        next.sort_unstable();
        next.dedup();

        current = next;
        if time == 0 {
            break;
        }
    }

    let mut routes: Vec<(Opened, u32)> = routes.into_iter().collect();
    routes.sort_unstable_by_key(|route| route.1);
    routes.reverse();
    let base = routes.clone();

    let mut best = 0;
    for (opened, released) in base {
        for alt in &routes {
            if opened.compatible(alt.0) {
                if released + alt.1 > best {
                    best = released + alt.1;
                }
                break;
            }
        }
    }

    best
}

pub fn a() {
    let mut openable = 0;
    let mut map: Map = Default::default();
    let ctxt = readfile("16");
    for line in ctxt.lines() {
        let (id, mut valve) = parse(line);
        if valve.flow > 0 {
            valve.openid = Some(openable);
            openable += 1;
        }
        map.add(id, valve);
    }
    map.calculate();
    let n = part1(&map);
    println!("Most pressure released is {n}");
}

pub fn b() {
    let mut openable = 0;
    let mut map: Map = Default::default();
    let ctxt = readfile("16");
    for line in ctxt.lines() {
        let (id, mut valve) = parse(line);
        if valve.flow > 0 {
            valve.openid = Some(openable);
            openable += 1;
        }
        map.add(id, valve);
    }
    map.calculate();
    let n = part2(&map);
    println!("Working with an elephant, most pressure released is {n}");
}
