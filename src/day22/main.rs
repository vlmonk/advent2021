use lazy_static::lazy_static;
use regex::Regex;
use std::{collections::HashSet, error::Error};

fn segments(amin: i64, amax: i64, bmin: i64, bmax: i64) -> impl Iterator<Item = (i64, i64)> {
    let p0 = amin.min(bmin);
    let p1 = amin.max(bmin);
    let p2 = amax.min(bmax);
    let p3 = amax.max(bmax);

    let left = (p0, p1 - 1);
    let middle = (p1, p2);
    let right = (p2 + 1, p3);

    vec![left, middle, right].into_iter().filter(|r| r.0 <= r.1)
}

fn parts(a: &Cuboid, b: &Cuboid) -> impl Iterator<Item = Cuboid> {
    let all = segments(a.zmin, a.zmax, b.zmin, b.zmax)
        .map(|(za, zb)| {
            segments(a.ymin, a.ymax, b.ymin, b.ymax)
                .map(move |(ya, yb)| {
                    segments(a.xmin, a.xmax, b.xmin, b.xmax)
                        .map(move |(xa, xb)| Cuboid::new(xa, xb, ya, yb, za, zb))
                })
                .flatten()
        })
        .flatten()
        .collect::<Vec<_>>();

    all.into_iter()
}

// return parts of A not contains in B
fn not(a: &Cuboid, b: &Cuboid) -> impl Iterator<Item = Cuboid> {
    if a.intersect(b) {
        let collected = parts(a, b)
            .filter(|part| part.intersect(a) && !part.intersect(b))
            .collect::<Vec<_>>();

        collected.into_iter()
    } else {
        vec![a.clone()].into_iter()
    }
}

#[derive(Debug, Hash, Eq, PartialEq, Clone)]
pub struct Cuboid {
    xmin: i64,
    xmax: i64,
    ymin: i64,
    ymax: i64,
    zmin: i64,
    zmax: i64,
}

impl Cuboid {
    pub fn new(xmin: i64, xmax: i64, ymin: i64, ymax: i64, zmin: i64, zmax: i64) -> Self {
        Self {
            xmin,
            xmax,
            ymin,
            ymax,
            zmin,
            zmax,
        }
    }

    pub fn intersect(&self, b: &Cuboid) -> bool {
        let x = b.xmin <= self.xmax && b.xmax >= self.xmin;
        let y = b.ymin <= self.ymax && b.ymax >= self.ymin;
        let z = b.zmin <= self.zmax && b.zmax >= self.zmin;

        x && y && z
    }

    fn size(&self) -> i64 {
        (self.xmax - self.xmin + 1) * (self.ymax - self.ymin + 1) * (self.zmax - self.zmin + 1)
    }

    fn limit(&self, limit: i64) -> Option<Cuboid> {
        let xmin = self.xmin.max(limit * -1);
        let xmax = self.xmax.min(limit);

        let ymin = self.ymin.max(limit * -1);
        let ymax = self.ymax.min(limit);

        let zmin = self.zmin.max(limit * -1);
        let zmax = self.zmax.min(limit);

        if self.xmin >= -limit
            && self.xmax <= limit
            && self.ymin >= -limit
            && self.ymax <= limit
            && self.zmin >= -limit
            && self.zmax <= limit
        {
            Some(Self {
                xmin,
                xmax,
                ymin,
                ymax,
                zmin,
                zmax,
            })
        } else {
            None
        }
    }
}

#[derive(Debug, Clone)]
enum Action {
    On,
    Off,
}

impl Action {
    pub fn parse(input: &str) -> Option<Self> {
        match input {
            "on" => Some(Self::On),
            "off" => Some(Self::Off),
            _ => None,
        }
    }
}

#[derive(Debug)]
struct Rule {
    cuboid: Cuboid,
    action: Action,
}

impl Rule {
    pub fn parse(input: &str) -> Option<Self> {
        lazy_static! {
            static ref RE: Regex =
                Regex::new(r"(on|off) x=(-?\d+)..(-?\d+),y=(-?\d+)..(-?\d+),z=(-?\d+)..(-?\d+)")
                    .unwrap();
        }

        let caps = RE.captures(input)?;
        let action = caps
            .get(1)
            .map(|cap| cap.as_str())
            .and_then(Action::parse)?;

        let nums = (2..=7)
            .map(|idx| {
                caps.get(idx)
                    .and_then(|raw| raw.as_str().parse::<i64>().ok())
            })
            .collect::<Option<Vec<_>>>()?;

        match *nums.as_slice() {
            [xmin, xmax, ymin, ymax, zmin, zmax] => {
                let cuboid = Cuboid::new(xmin, xmax, ymin, ymax, zmin, zmax);
                Some(Self { cuboid, action })
            }
            _ => None,
        }
    }

    pub fn limit(&self, limit: i64) -> Option<Self> {
        Some(Self {
            action: self.action.clone(),
            cuboid: self.cuboid.limit(limit)?,
        })
    }
}

#[derive(Debug)]
struct Reactor {
    cubes: HashSet<Cuboid>,
}

impl Reactor {
    pub fn new() -> Self {
        let cubes = HashSet::new();
        Self { cubes }
    }

    pub fn process(&mut self, rule: &Rule) {
        match (self.cubes.len(), &rule.action) {
            (0, Action::On) => {
                self.cubes.insert(rule.cuboid.clone());
            }
            (0, Action::Off) => {}
            (_, Action::On) => {
                let mut inserted = vec![rule.cuboid.clone()];

                for cube in &self.cubes {
                    let mut next_inserted = vec![];

                    for i in inserted.iter() {
                        for next_part in not(&i, &cube) {
                            next_inserted.push(next_part);
                        }
                    }

                    inserted = next_inserted
                }

                for i in inserted {
                    self.cubes.insert(i);
                }
            }
            (_, Action::Off) => {
                let mut next = HashSet::new();

                for cube in &self.cubes {
                    for p in not(cube, &rule.cuboid) {
                        next.insert(p);
                    }
                }

                self.cubes = next;
            }
        }
    }

    pub fn enabled(&self) -> i64 {
        self.cubes.iter().map(|c| c.size()).sum()
    }
}

fn main() -> Result<(), Box<dyn Error>> {
    let filename = std::env::args().nth(1).ok_or("Invalid input")?;
    let input = std::fs::read_to_string(filename)?;

    let rules = input
        .lines()
        .map(Rule::parse)
        .collect::<Option<Vec<_>>>()
        .ok_or("Can't parse input")?;

    let limited = rules.iter().filter_map(|r| r.limit(50)).collect::<Vec<_>>();

    let mut reactor = Reactor::new();

    for rule in limited {
        reactor.process(&rule);
    }

    let result_a = reactor.enabled();

    let mut reactor = Reactor::new();

    for rule in rules {
        reactor.process(&rule)
    }

    let result_b = reactor.enabled();

    println!("Task A: {}, Task B: {}", result_a, result_b);

    Ok(())
}
