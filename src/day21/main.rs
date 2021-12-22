use std::{collections::HashMap, error::Error, fmt::Display};

const QUANTUM_LIMIT: usize = 21;

fn wrap10(score: usize) -> usize {
    (score - 1) % 10 + 1
}
fn parse_input(input: &str) -> Option<(usize, usize)> {
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

    Some((a, b))
}

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

#[derive(Debug, PartialEq, Eq, Hash)]
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

impl Display for Turn {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::A => write!(f, "A"),
            Self::B => write!(f, "B"),
        }
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

#[derive(Hash, PartialEq, Eq)]
enum GameResult {
    InProgress,
    WinA,
    WinB,
}

impl Display for GameResult {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::InProgress => write!(f, "[-]"),
            Self::WinA => write!(f, "[A]"),
            Self::WinB => write!(f, "[B]"),
        }
    }
}

#[derive(Hash, PartialEq, Eq, Clone)]
struct Player {
    score: usize,
    position: usize,
}

impl Player {
    pub fn new(position: usize) -> Self {
        Self { position, score: 0 }
    }

    fn move_by(&mut self, roll: usize) {
        self.position = wrap10(self.position + roll);
        self.score += self.position
    }
}

impl Display for Player {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:02}/{:02}", self.position, self.score)
    }
}

#[derive(Hash, PartialEq, Eq)]
struct State {
    a: Player,
    b: Player,
    turn: Turn,
    result: GameResult,
}

impl State {
    pub fn new(a: usize, b: usize) -> Self {
        let a = Player::new(a);
        let b = Player::new(b);

        Self {
            a,
            b,
            turn: Turn::A,
            result: GameResult::InProgress,
        }
    }

    pub fn tick(&self, roll: usize) -> State {
        let mut a = self.a.clone();
        let mut b = self.b.clone();

        match self.turn {
            Turn::A => a.move_by(roll),
            Turn::B => b.move_by(roll),
        }

        let turn = match self.turn {
            Turn::A => Turn::B,
            Turn::B => Turn::A,
        };

        // let result = if a.score >= QUANTUM_LIMIT && b.score >= QUANTUM_LIMIT {
        //     if a.score > b.score {
        //         GameResult::WinA
        //     } else {
        //         GameResult::WinB
        //     }
        // } else {
        //     GameResult::InProgress
        // };

        let result = if a.score >= QUANTUM_LIMIT {
            GameResult::WinA
        } else if b.score >= QUANTUM_LIMIT {
            GameResult::WinB
        } else {
            GameResult::InProgress
        };

        Self { a, b, turn, result }
    }

    fn finished(&self) -> bool {
        match self.result {
            GameResult::InProgress => false,
            _ => true,
        }
    }
}

impl Display for State {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "A: {}, B: {}, Turn: {}, Result: {}",
            self.a, self.b, self.turn, self.result
        )
    }
}

struct QuantumGame {
    world: HashMap<State, usize>,
}

impl QuantumGame {
    fn new(a: usize, b: usize) -> Self {
        let single = State::new(a, b);
        let mut world = HashMap::new();
        world.insert(single, 1);

        Self { world }
    }

    fn tick(&mut self) -> bool {
        let mut changed = 0;
        let mut next_world = HashMap::new();

        for (state, count) in self.world.drain() {
            if state.finished() {
                next_world
                    .entry(state)
                    .and_modify(|c| *c += count)
                    .or_insert(count);
            } else {
                for roll_a in [1, 2, 3] {
                    for roll_b in [1, 2, 3] {
                        for roll_c in [1, 2, 3] {
                            let next_state = state.tick(roll_a + roll_b + roll_c);
                            next_world
                                .entry(next_state)
                                .and_modify(|c| *c += count)
                                .or_insert(count);
                        }
                    }
                }

                changed += 1;
            }
        }

        self.world = next_world;
        changed > 0
    }

    fn run(&mut self) {
        loop {
            let changed = self.tick();
            if !changed {
                break;
            }
        }
    }

    fn result_b(&self) -> usize {
        let (mut win_a, mut win_b) = (0, 0);
        for (state, count) in self.world.iter() {
            if state.result == GameResult::WinA {
                win_a += *count
            } else if state.result == GameResult::WinB {
                win_b += count
            }
        }

        win_a.max(win_b)
    }
}

impl Display for QuantumGame {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for (state, count) in self.world.iter() {
            write!(f, "{:05} - {}\n", count, state)?;
        }

        Ok(())
    }
}

fn main() -> Result<(), Box<dyn Error>> {
    let filename = std::env::args().nth(1).ok_or("Invalid input")?;
    let input = std::fs::read_to_string(filename)?;
    let (a, b) = parse_input(&input).ok_or("Can't parse input")?;

    let mut game = Game::new(a, b);
    game.round();
    let result_a = game.result_a();

    let mut quantum_game = QuantumGame::new(a, b);
    quantum_game.run();
    let result_b = quantum_game.result_b();

    println!("Task A: {}, Task B: {}", result_a, result_b);

    Ok(())
}
