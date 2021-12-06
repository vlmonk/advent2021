use std::error::Error;

const T1: usize = 80;
const T2: usize = 256;

struct Game {
    n8: usize,
    n7: usize,
    n6: usize,
    n5: usize,
    n4: usize,
    n3: usize,
    n2: usize,
    n1: usize,
    n0: usize,
}

impl Game {
    pub fn parse(input: &str) -> Result<Self, Box<dyn Error>> {
        let mut game = Game::empty();
        for n in input.split(',') {
            match n {
                "8" => game.n8 += 1,
                "7" => game.n7 += 1,
                "6" => game.n6 += 1,
                "5" => game.n5 += 1,
                "4" => game.n4 += 1,
                "3" => game.n3 += 1,
                "2" => game.n2 += 1,
                "1" => game.n1 += 1,
                "0" => game.n0 += 1,
                _ => return Err(format!("Invalid value: {}", n).into()),
            }
        }

        Ok(game)
    }

    fn empty() -> Self {
        Self {
            n8: 0,
            n7: 0,
            n6: 0,
            n5: 0,
            n4: 0,
            n3: 0,
            n2: 0,
            n1: 0,
            n0: 0,
        }
    }

    fn step(&mut self) {
        let n8 = self.n0;
        let n7 = self.n8;
        let n6 = self.n7 + self.n0;
        let n5 = self.n6;
        let n4 = self.n5;
        let n3 = self.n4;
        let n2 = self.n3;
        let n1 = self.n2;
        let n0 = self.n1;

        self.n8 = n8;
        self.n7 = n7;
        self.n6 = n6;
        self.n5 = n5;
        self.n4 = n4;
        self.n3 = n3;
        self.n2 = n2;
        self.n1 = n1;
        self.n0 = n0;
    }

    fn size(&self) -> usize {
        self.n8 + self.n7 + self.n6 + self.n5 + self.n4 + self.n3 + self.n2 + self.n1 + self.n0
    }
}

fn main() -> Result<(), Box<dyn Error>> {
    let filename = std::env::args().nth(1).ok_or("Invalid input")?;
    let mut game = std::fs::read_to_string(filename)?
        .lines()
        .nth(0)
        .ok_or_else(|| "Empth input".into())
        .and_then(Game::parse)?;

    for _ in 0..T1 {
        game.step()
    }

    let r1 = game.size();

    for _ in 0..(T2 - T1) {
        game.step()
    }

    let r2 = game.size();

    println!("Task A: {}\nTask B: {}", r1, r2);

    Ok(())
}
