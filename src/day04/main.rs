use std::error::Error;

#[derive(Debug)]
enum Num {
    Checked(i32),
    Unchecked(i32),
}

impl Num {
    fn checked(&self) -> bool {
        match self {
            Num::Checked(_) => true,
            Num::Unchecked(_) => false,
        }
    }

    fn val(&self) -> i32 {
        match self {
            Num::Checked(n) => *n,
            Num::Unchecked(n) => *n,
        }
    }
}

const SIZE: usize = 5;

fn index(row: usize, col: usize) -> usize {
    row * SIZE + col
}

#[derive(Debug)]
struct Card {
    numbers: Vec<Num>,
}

impl Card {
    fn parse(input: &str) -> Result<Self, Box<dyn Error>> {
        let numbers = input
            .split(&[' ', '\n'][..])
            .filter(|v| v != &"")
            .map(|s| s.parse::<i32>().map(|n| Num::Unchecked(n)))
            .collect::<Result<Vec<_>, _>>()?;

        Ok(Self { numbers })
    }

    fn check(&mut self, number: i32) {
        for n in self.numbers.iter_mut() {
            match n {
                Num::Unchecked(k) if *k == number => {
                    let value = Num::Checked(number);
                    let _ = std::mem::replace(n, value);
                }
                _ => {}
            }
        }
    }

    fn sum_unmarked(&self) -> i32 {
        self.numbers
            .iter()
            .filter(|v| !v.checked())
            .map(|v| v.val())
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
            .all(|idx| self.numbers[idx].checked())
    }

    fn ready_col(&self, col: usize) -> bool {
        (0..5)
            .map(|row| index(row, col))
            .all(|idx| self.numbers[idx].checked())
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
        .inspect(|n| println!("N: {}", n))
        .filter_map(|n| {
            cards.iter_mut().for_each(|c| c.check(*n));
            let ready = cards.iter().find(|c| c.ready());
            ready.map(|c| c.sum_unmarked()).map(|sum| sum * n)
        })
        .nth(0);

    dbg!(founed);
    // dbg!(cards);

    Ok(())
}
