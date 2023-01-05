use jungle::readfile;

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
}

type Flow = u16;

#[derive(Debug, Default)]
struct Valve {
    flow: Flow,
    openid: Option<u8>,
    to: Vec<ValveId>,
}

const VALVES: usize = 26 * 26;

struct Map {
    valves: [Option<Valve>; VALVES],
    remaining: [u32; 30],
    flows: Vec<Flow>,
}

use std::mem::{self, MaybeUninit};

impl Default for Map {
    fn default() -> Self {
        Self {
            valves: {
                let mut valves: [MaybeUninit<Option<Valve>>; VALVES] =
                    unsafe { MaybeUninit::uninit().assume_init() };

                for valve in &mut valves[..] {
                    valve.write(None);
                }

                unsafe { mem::transmute::<_, [Option<Valve>; VALVES]>(valves) }
            },
            remaining: [0; 30],
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

    // Calculate maximum remaining value
    fn calculate(&mut self) {
        let mut v = self.flows.clone();
        v.sort_unstable();
        v.reverse();
        for t in 0..30 {
            let mut remaining = 0;
            for k in 0..((t + 1) / 2) {
                if k < v.len() {
                    let since = t - (k * 2);
                    remaining += (v[k] as u32) * (since as u32);
                }
            }
            self.remaining[t] = remaining;
        }
        println!("{:?}", self.remaining);
    }

    // Calculate maximum remaining value with an elephant
    fn calculate_elephant(&mut self) {
        let mut v = self.flows.clone();
        v.sort_unstable();
        v.reverse();
        for t in 0..26 {
            let mut remaining = 0;

            let mut t1 = 0;
            let mut k = 0;
            let mut moving: bool = false;
            loop {
                if t1 >= t {
                    break;
                }
                if moving {
                    // Can't open valves because we're moving
                    moving = false;
                } else {
                    // I open the best valve, elephant opens the next best valve
                    if k < v.len() {
                        let since = t - t1;
                        remaining += (v[k] as u32) * (since as u32);
                        k += 1;
                        if k < v.len() {
                            remaining += (v[k] as u32) * (since as u32);
                            k += 1;
                        }
                    }
                    moving = true;
                }
                t1 += 1;
            }
            self.remaining[t] = remaining;
        }
    }
}

#[derive(Copy, Clone, Debug, Eq, PartialEq, Hash, Ord, PartialOrd)]
struct Me {
    pressure: u32,
    at: ValveId,
    opened: Opened,
}

impl Me {
    fn new() -> Self {
        Self {
            pressure: 0,
            at: "AA".parse().unwrap(),
            opened: Default::default(),
        }
    }

    fn next(&self, time: u32, at: &Valve) -> Vec<Self> {
        let mut out = Vec::new();
        if let Some(n) = at.openid {
            if self.opened.is_closed(n) {
                let mut opener = *self;
                opener.opened.open(n);
                opener.pressure += time * (at.flow as u32);
                out.push(opener);
            }
        }
        for exit in &at.to {
            let mut mover = *self;
            mover.at = *exit;
            out.push(mover);
        }
        out
    }
}

#[derive(Copy, Clone, Debug, Eq, PartialEq, Hash, Ord, PartialOrd)]
struct Us {
    pressure: u32,
    im_at: ValveId,
    el_at: ValveId,
    opened: Opened,
}

impl Us {
    fn new() -> Self {
        Self {
            pressure: 0,
            im_at: "AA".parse().unwrap(),
            el_at: "AA".parse().unwrap(),
            opened: Default::default(),
        }
    }

    fn next_me(&self, time: u32, at: &Valve) -> Vec<Self> {
        let mut out = Vec::new();
        if let Some(n) = at.openid {
            if self.opened.is_closed(n) {
                let mut opener = *self;
                opener.opened.open(n);
                opener.pressure += time * (at.flow as u32);
                out.push(opener);
            }
        }
        for exit in &at.to {
            let mut mover = *self;
            mover.im_at = *exit;
            out.push(mover);
        }
        out
    }

    fn next_elephant(&self, time: u32, at: &Valve) -> Vec<Self> {
        let mut out = Vec::new();
        if let Some(n) = at.openid {
            if self.opened.is_closed(n) {
                let mut opener = *self;
                opener.opened.open(n);
                opener.pressure += time * (at.flow as u32);
                out.push(opener);
            }
        }
        for exit in &at.to {
            let mut mover = *self;
            mover.el_at = *exit;
            out.push(mover);
        }
        out
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
    let mut current: Vec<Me> = vec![Me::new()];

    loop {
        time -= 1;
        let mut next: Vec<Me> = Vec::new();
        //println!("{time} .. {}", current.len());

        for possible in current {
            let x = map.get(possible.at);
            if let Some(valve) = x {
                for maybe in possible.next(time, valve) {
                    next.push(maybe);
                }
            } else {
                panic!("Somehow {:?} is not on the map", possible.at);
            }
        }

        next.sort_unstable();
        let best = next.last().unwrap().pressure;
        let remain = map.remainder(time);
        let need = if remain > best { 0 } else { best - remain };

        // Eliminate possibilities that can't get enough pressure to beat the leader
        next.retain(|&maybe| maybe.pressure >= need);
        next.dedup();

        current = next;
        if time == 0 {
            break;
        }
    }

    current.last().unwrap().pressure
}

fn part2(map: &Map) -> u32 {
    // 4 minutes to teach an elephant about valves
    let mut time = 26;
    let mut current: Vec<Us> = vec![Us::new()];

    loop {
        time -= 1;
        let mut elxt: Vec<Us> = Vec::new();

        for possible in current {
            let x = map.get(possible.im_at);
            if let Some(valve) = x {
                for maybe in possible.next_me(time, valve) {
                    elxt.push(maybe);
                }
            } else {
                panic!("Somehow {:?} is not on the map", possible.im_at);
            }
        }

        elxt.sort_unstable();
        elxt.dedup();

        let mut next: Vec<Us> = Vec::new();

        for possible in elxt {
            let x = map.get(possible.el_at);
            if let Some(valve) = x {
                for maybe in possible.next_elephant(time, valve) {
                    next.push(maybe);
                }
            } else {
                panic!("Somehow {:?} is not on the map", possible.el_at);
            }
        }

        next.select_nth_unstable_by(0, |a, b| b.cmp(a));

        let best = next.first().unwrap().pressure;
        let remain = map.remainder(time);
        let need = if remain > best { 0 } else { best - remain };

        // Eliminate possibilities that can't get enough pressure to beat the leader
        next.retain(|&maybe| maybe.pressure >= need);
        next.sort_unstable();
        next.dedup();

        current = next;
        if time == 0 {
            break;
        }
    }

    current.last().unwrap().pressure
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
    map.calculate_elephant();
    let n = part2(&map);
    println!("Working with an elephant, most pressure released is {n}");
}
