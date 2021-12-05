use std::collections::VecDeque;
use std::error::Error;

const SIZE: usize = 5;

fn index(row: usize, col: usize) -> usize {
    row * SIZE + col
}

struct Game {
    numbers: VecDeque<i32>,
    cards: Vec<(Card, bool)>,
    keep: Option<i32>,
}

impl Game {
    fn parse(input: &str) -> Result<Self, Box<dyn Error>> {
        let mut parts = input.split("\n\n");

        let numbers_raw = parts.nth(0).ok_or("invalid input")?;
        let numbers = numbers_raw
            .split(",")
            .map(|p| p.parse::<i32>())
            .collect::<Result<VecDeque<_>, _>>()?;

        let cards = parts.map(Card::parse).collect::<Result<Vec<_>, _>>()?;
        let cards = cards.into_iter().map(|c| (c, false)).collect();
        let keep = None;

        Ok(Self {
            numbers,
            cards,
            keep,
        })
    }

    fn find_and_mark(&mut self) -> Option<i32> {
        self.cards
            .iter_mut()
            .filter(|(c, used)| c.ready() && !used)
            .map(|(c, used)| {
                *used = true;
                c.sum_unmarked()
            })
            .nth(0)
    }

    fn mark(&mut self, k: i32) {
        self.cards.iter_mut().for_each(|(c, _)| c.mark(k));
    }
}

impl Iterator for Game {
    type Item = i32;

    fn next(&mut self) -> Option<Self::Item> {
        if let Some(v) = self.find_and_mark() {
            match self.keep {
                Some(k) => return Some(v * k),
                None => panic!("Unexpected"),
            }
        }

        while let Some(k) = self.numbers.pop_front() {
            self.keep = Some(k);
            self.mark(k);
            if let Some(v) = self.find_and_mark() {
                return Some(v * k);
            }
        }

        None
    }
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
    let filename = std::env::args().nth(1).ok_or("Invalid input")?;
    let input = std::fs::read_to_string(filename)?;
    let mut game = Game::parse(&input)?;

    let result_a = game.next().ok_or_else(|| "Result A not found!")?;
    let result_b = game.last().ok_or_else(|| "Result B not found!")?;
    println!("Task A: {}\nTask B: {}", result_a, result_b);

    Ok(())
}
