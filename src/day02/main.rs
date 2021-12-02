use lazy_static::lazy_static;
use regex::Regex;
use std::error::Error;

#[derive(Debug)]
enum Command {
    Forward(i32),
    Up(i32),
    Down(i32),
}

impl Command {
    pub fn parse(input: &str) -> Option<Self> {
        lazy_static! {
            static ref RE: Regex = Regex::new(r"^(up|down|forward)\s+(\d+)$").unwrap();
        }

        let caps = RE.captures(input)?;
        let direction = caps.get(1)?.as_str();
        let value = caps.get(2)?.as_str().parse::<i32>().ok()?;

        match direction {
            "up" => Some(Command::Up(value)),
            "down" => Some(Command::Down(value)),
            "forward" => Some(Command::Forward(value)),
            _ => None,
        }
    }
}

trait Boat {
    fn perform(&mut self, cmd: &Command);
    fn result(&self) -> i32;
}

struct SimpleBoat {
    horizontal: i32,
    depth: i32,
}

struct AdvancedBoat {
    horizontal: i32,
    depth: i32,
    aim: i32,
}

impl SimpleBoat {
    pub fn new() -> Self {
        Self {
            horizontal: 0,
            depth: 0,
        }
    }
}

impl AdvancedBoat {
    pub fn new() -> Self {
        Self {
            horizontal: 0,
            depth: 0,
            aim: 0,
        }
    }
}

impl Boat for SimpleBoat {
    fn perform(&mut self, cmd: &Command) {
        match cmd {
            Command::Up(value) => self.depth -= value,
            Command::Down(value) => self.depth += value,
            Command::Forward(value) => self.horizontal += value,
        }
    }

    fn result(&self) -> i32 {
        self.horizontal * self.depth
    }
}

impl Boat for AdvancedBoat {
    fn perform(&mut self, cmd: &Command) {
        match cmd {
            Command::Up(value) => self.aim -= value,
            Command::Down(value) => self.aim += value,
            Command::Forward(value) => {
                self.horizontal += value;
                self.depth += self.aim * value
            }
        }
    }

    fn result(&self) -> i32 {
        self.horizontal * self.depth
    }
}

fn main() -> Result<(), Box<dyn Error>> {
    let input = std::env::args().nth(1).ok_or("Invalid input")?;
    let content = std::fs::read_to_string(input)?;
    let lines = content
        .lines()
        .map(|line| Command::parse(line).ok_or_else(|| format!("Invalid line: {}", line)))
        .collect::<Result<Vec<_>, _>>()?;

    let mut simple_boat = SimpleBoat::new();
    let mut advanced_boat = AdvancedBoat::new();

    for command in lines {
        simple_boat.perform(&command);
        advanced_boat.perform(&command);
    }

    println!(
        "Task A: {}\nTask B: {}",
        simple_boat.result(),
        advanced_boat.result()
    );

    Ok(())
}
