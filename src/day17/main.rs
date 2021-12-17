use lazy_static::lazy_static;
use regex::Regex;
use std::error::Error;

#[derive(Debug)]
struct Target {
    xmin: i32,
    xmax: i32,
    ymin: i32,
    ymax: i32,
}

impl Target {
    pub fn parse(input: &str) -> Option<Self> {
        lazy_static! {
            static ref RE: Regex = Regex::new(r"x=(-?\d+)..(-?\d+),\s+y=(-?\d+)..(-?\d+)").unwrap();
        }
        let captures = RE.captures(input)?;
        if let &[xmin, xmax, ymin, ymax] = (1..=4)
            .map(|i| captures.get(i).and_then(|c| c.as_str().parse::<i32>().ok()))
            .collect::<Option<Vec<_>>>()?
            .as_slice()
        {
            let (ymax, ymin) = if ymax > ymin {
                (ymax, ymin)
            } else {
                (ymin, ymax)
            };

            Some(Self {
                xmin,
                xmax,
                ymin,
                ymax,
            })
        } else {
            None
        }
    }

    pub fn hit(&self, x: i32, y: i32) -> bool {
        let &Target {
            xmin,
            xmax,
            ymin,
            ymax,
        } = self;

        if x >= xmin && x <= xmax && y >= ymin && y <= ymax {
            true
        } else {
            false
        }
    }
}

pub struct Game {
    target: Target,
}

impl Game {
    pub fn parse(input: &str) -> Option<Self> {
        let target = Target::parse(input)?;
        Some(Self { target })
    }

    fn hit(&self, mut dx: i32, mut dy: i32) -> Option<i32> {
        let mut ymax = 0;
        let mut x = 0;
        let mut y = 0;

        loop {
            if self.target.hit(x, y) {
                return Some(ymax);
            }

            if x > self.target.xmax || y < self.target.ymin {
                return None;
            }

            x += dx;
            y += dy;
            ymax = ymax.max(y);

            if dx > 0 {
                dx -= 1
            } else if dx < 0 {
                dx += 1
            }

            dy -= 1;
        }
    }

    pub fn results<'a>(&'a self) -> impl Iterator<Item = i32> + 'a {
        let x_limit = self.target.xmax;
        let y_limit = self.target.ymin.abs();
        (-y_limit..=y_limit)
            .rev()
            .map(move |dy| (0..=x_limit).map(move |dx| (dx, dy)))
            .flatten()
            .filter_map(move |(dx, dy)| match self.hit(dx, dy) {
                Some(height) => Some(height),
                None => None,
            })
    }
}

fn main() -> Result<(), Box<dyn Error>> {
    let filename = std::env::args().nth(1).ok_or("Invalid input")?;
    let input = std::fs::read_to_string(filename)?;

    let game = Game::parse(&input).ok_or("Can't parse input")?;
    let mut results = game.results();

    let result_a = results.next().ok_or("Can't find result A")?;
    let result_b = results.count() + 1;

    println!("Task A: {}\nTask B: {}", result_a, result_b);

    Ok(())
}
