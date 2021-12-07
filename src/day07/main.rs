use std::error::Error;

#[derive(Debug)]
struct Game {
    crabs: Vec<i32>,
}

impl Game {
    pub fn parse(input: &str) -> Result<Self, Box<dyn Error>> {
        let crabs = input
            .split(",")
            .map(|n| n.parse::<i32>())
            .collect::<Result<Vec<_>, _>>()?;

        Ok(Self { crabs })
    }

    pub fn fuel_to<F>(&self, target: i32, fuel_fx: F) -> i32
    where
        F: Fn(i32, i32) -> i32,
    {
        self.crabs.iter().map(|c| fuel_fx(*c, target)).sum()
    }

    pub fn max(&self) -> Option<i32> {
        self.crabs.iter().max().copied()
    }

    pub fn min(&self) -> Option<i32> {
        self.crabs.iter().min().copied()
    }
}

fn main() -> Result<(), Box<dyn Error>> {
    let filename = std::env::args().nth(1).ok_or("Invalid input")?;
    let game = std::fs::read_to_string(filename)?
        .lines()
        .nth(0)
        .ok_or_else(|| "Empth input".into())
        .and_then(Game::parse)?;

    let min = game.min().ok_or_else(|| "Empty input")?;
    let max = game.max().ok_or_else(|| "Empty input")?;

    let fuel_a = |a: i32, b: i32| (a - b).abs();
    let fuel_b = |a: i32, b: i32| (a - b).abs() * ((a - b).abs() + 1) / 2;

    let task_a = (min..=max).map(|v| game.fuel_to(v, fuel_a)).min().unwrap();
    let task_b = (min..=max).map(|v| game.fuel_to(v, fuel_b)).min().unwrap();

    println!("Task A: {}, Task B: {}", task_a, task_b);

    Ok(())
}
