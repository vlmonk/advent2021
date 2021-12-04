use std::error::Error;

const SIZE: usize = 5;

fn index(row: usize, col: usize) -> usize {
    row * SIZE + col
}

#[derive(Debug)]
struct Card {
    numbers: Vec<(i32, bool)>,
}

impl Card {
    fn parse(input: &str) -> Result<Self, Box<dyn Error>> {
        let numbers = input
            .split(&[' ', '\n'][..])
            .filter(|v| v != &"")
            .map(|s| s.parse::<i32>().map(|n| (n, false)))
            .collect::<Result<Vec<_>, _>>()?;

        Ok(Self { numbers })
    }

    fn mark(&mut self, number: i32) {
        for (n, checked) in self.numbers.iter_mut() {
            if *n == number {
                *checked = true
            }
        }
    }

    fn sum_unmarked(&self) -> i32 {
        self.numbers
            .iter()
            .filter(|(_, checked)| !checked)
            .map(|(n, _)| n)
            .sum()
    }

    fn ready(&self) -> bool {
        (0..5)
            .map(|row| self.ready_row(row))
            .chain((0..5).map(|col| self.ready_col(col)))
            .any(|v| v)
    }

    fn ready_row(&self, row: usize) -> bool {
        (0..5)
            .map(|col| index(row, col))
            .all(|idx| self.numbers[idx].1)
    }

    fn ready_col(&self, col: usize) -> bool {
        (0..5)
            .map(|row| index(row, col))
            .all(|idx| self.numbers[idx].1)
    }
}

fn main() -> Result<(), Box<dyn Error>> {
    let input = std::env::args().nth(1).ok_or("Invalid input")?;
    let raw = std::fs::read_to_string(input)?;
    let mut parts = raw.split("\n\n");

    let numbers_raw = parts.nth(0).ok_or("invalid input")?;
    let numbers = numbers_raw
        .split(",")
        .map(|p| p.parse::<i32>())
        .collect::<Result<Vec<_>, _>>()?;

    let mut cards = parts.map(Card::parse).collect::<Result<Vec<_>, _>>()?;
    let founed = numbers
        .iter()
        .filter_map(|n| {
            cards.iter_mut().for_each(|c| c.mark(*n));
            let ready = cards.iter().find(|c| c.ready());
            ready.map(|c| c.sum_unmarked()).map(|sum| sum * n)
        })
        .nth(0)
        .ok_or_else(|| "Result not found!")?;

    println!("Task A: {}", founed);
    Ok(())
}
