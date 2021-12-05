use lazy_static::lazy_static;
use regex::Regex;
use std::collections::HashMap;
use std::error::Error;

struct Game {
    grid: HashMap<(i32, i32), usize>,
}

impl Game {
    pub fn new() -> Self {
        Self {
            grid: HashMap::new(),
        }
    }

    fn add_line(&mut self, line: &Line) {
        let (x0, y0) = line.start();
        let (dx, dy) = line.step();

        for n in 0..line.size() {
            let (x, y) = (x0 + n * dx, y0 + n * dy);
            self.add_point(x, y);
        }
    }

    fn result(&self) -> usize {
        self.grid.values().filter(|v| **v >= 2).count()
    }

    fn add_point(&mut self, x: i32, y: i32) {
        let value = self.grid.entry((x, y)).or_insert(0);
        *value += 1;
    }
}

#[derive(Debug)]
enum Line {
    Horizontal { x1: i32, x2: i32, y: i32 },
    Vertical { x: i32, y1: i32, y2: i32 },
    Diagonal { x1: i32, y1: i32, x2: i32, y2: i32 },
}

impl Line {
    pub fn parse(input: &str) -> Option<Self> {
        lazy_static! {
            static ref RE: Regex = Regex::new(r"^(\d+),(\d+)\s+->\s+(\d+),(\d+)$").unwrap();
        }

        let caps = RE.captures(input)?;

        let x1 = caps.get(1)?.as_str().parse::<i32>().ok()?;
        let y1 = caps.get(2)?.as_str().parse::<i32>().ok()?;
        let x2 = caps.get(3)?.as_str().parse::<i32>().ok()?;
        let y2 = caps.get(4)?.as_str().parse::<i32>().ok()?;

        if x1 == x2 {
            let x = x1;
            Some(Line::Vertical { x, y1, y2 })
        } else if y1 == y2 {
            let y = y1;
            Some(Line::Horizontal { x1, x2, y })
        } else if (x1 - x2).abs() == (y1 - y2).abs() {
            Some(Line::Diagonal { x1, y1, x2, y2 })
        } else {
            None
        }
    }

    fn diagonal(&self) -> bool {
        match self {
            Self::Diagonal { .. } => true,
            _ => false,
        }
    }

    pub fn size(&self) -> i32 {
        match self {
            Self::Horizontal { x1, x2, .. } => (x1 - x2).abs() + 1,
            Self::Vertical { y1, y2, .. } => (y1 - y2).abs() + 1,
            Self::Diagonal { x1, x2, .. } => (x1 - x2).abs() + 1,
        }
    }

    pub fn step(&self) -> (i32, i32) {
        match self {
            Self::Horizontal { x1, x2, .. } => {
                let dx = if x2 > x1 { 1 } else { -1 };
                (dx, 0)
            }
            Self::Vertical { y1, y2, .. } => {
                let dy = if y2 > y1 { 1 } else { -1 };
                (0, dy)
            }

            Self::Diagonal { x1, y1, x2, y2 } => {
                let dx = if x2 > x1 { 1 } else { -1 };
                let dy = if y2 > y1 { 1 } else { -1 };
                (dx, dy)
            }
        }
    }

    pub fn start(&self) -> (i32, i32) {
        match self {
            &Self::Horizontal { x1, y, .. } => (x1, y),
            &Self::Vertical { x, y1, .. } => (x, y1),
            &Self::Diagonal { x1, y1, .. } => (x1, y1),
        }
    }
}

fn main() -> Result<(), Box<dyn Error>> {
    let filename = std::env::args().nth(1).ok_or("Invalid input")?;
    let input = std::fs::read_to_string(filename)?;
    let lines = input
        .lines()
        .map(Line::parse)
        .collect::<Option<Vec<_>>>()
        .ok_or_else(|| "Invalid input")?;

    let mut game = Game::new();

    for line in lines.iter().filter(|l| !l.diagonal()) {
        game.add_line(line);
    }

    let result_a = game.result();

    for line in lines.iter().filter(|l| l.diagonal()) {
        game.add_line(line);
    }

    let result_b = game.result();

    println!("Task A: {}\nTask B: {}", result_a, result_b);

    Ok(())
}
