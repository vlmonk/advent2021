use std::error::Error;

#[derive(Debug)]
struct Dice {
    current: usize,
}

impl Dice {
    pub fn new() -> Self {
        Self { current: 1 }
    }

    fn next(&mut self) -> usize {
        let value = self.current;

        if self.current >= 100 {
            self.current = 1
        } else {
            self.current += 1
        }

        value
    }
}

#[derive(Debug, PartialEq)]
enum Turn {
    A,
    B,
}

impl Turn {
    pub fn next(&mut self) {
        match self {
            Turn::A => std::mem::replace(self, Turn::B),
            Turn::B => std::mem::replace(self, Turn::A),
        };
    }
}

#[derive(Debug)]
struct Game {
    a_position: usize,
    b_position: usize,
    a_score: usize,
    b_score: usize,
    dice: Dice,
    turn: Turn,
    turns: usize,
}

impl Game {
    fn new(a_position: usize, b_position: usize) -> Self {
        let dice = Dice::new();
        let turn = Turn::A;

        Self {
            a_position,
            b_position,
            a_score: 0,
            b_score: 0,
            dice,
            turn,
            turns: 0,
        }
    }

    fn parse(input: &str) -> Option<Self> {
        let numbers = input
            .lines()
            .take(2)
            .map(|line| {
                line.find(':')
                    .and_then(|pos| line[pos + 2..].parse::<usize>().ok())
            })
            .collect::<Option<Vec<_>>>()?;

        let a = numbers.get(0).cloned()?;
        let b = numbers.get(1).cloned()?;

        Some(Self::new(a, b))
    }

    fn round(&mut self) {
        loop {
            self.tick();

            if self.a_score >= 1000 || self.b_score >= 1000 {
                break;
            }
        }
    }

    fn tick(&mut self) {
        let score = self.dice.next() + self.dice.next() + self.dice.next();

        match self.turn {
            Turn::A => {
                self.a_position = wrap10(self.a_position + score);
                self.a_score += self.a_position
            }
            Turn::B => {
                self.b_position = wrap10(self.b_position + score);
                self.b_score += self.b_position;
            }
        }

        self.turns += 1;
        self.turn.next();
    }

    fn result_a(&self) -> usize {
        match self.turn {
            Turn::A => self.a_score * self.turns * 3,
            Turn::B => self.b_score * self.turns * 3,
        }
    }
}

fn wrap10(score: usize) -> usize {
    (score - 1) % 10 + 1
}

fn main() -> Result<(), Box<dyn Error>> {
    let filename = std::env::args().nth(1).ok_or("Invalid input")?;
    let input = std::fs::read_to_string(filename)?;
    let mut game = Game::parse(&input).ok_or("Can't parse input")?;

    game.round();

    let result_a = game.result_a();
    let result_b = 0;

    println!("Task A: {}, Task B: {}", result_a, result_b);

    Ok(())
}
