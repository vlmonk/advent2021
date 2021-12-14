use lazy_static::lazy_static;
use regex::Regex;
use std::{collections::HashMap, error::Error};

#[derive(Debug)]
struct Polymer {
    items: HashMap<(char, char), u64>,
}

impl Polymer {
    pub fn parse(input: &str) -> Self {
        let mut items = HashMap::new();
        let mut iter = input.chars().peekable();

        while let (Some(a), Some(b)) = (iter.next(), iter.peek()) {
            let value = items.entry((a, *b)).or_insert(0);
            *value += 1
        }

        Self { items }
    }

    pub fn new(items: HashMap<(char, char), u64>) -> Self {
        Self { items }
    }

    pub fn iter<'a>(&'a self) -> impl Iterator<Item = (char, char, u64)> + 'a {
        self.items
            .iter()
            .map(|(pair, value)| (pair.0, pair.1, *value))
    }
}

// impl fmt::Display for Polymer {
//     fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
//         write!(f, "Polymer: {}", self.items.iter().collect::<String>())
//     }
// }

#[derive(Debug)]
struct Rule {
    from: (char, char),
    to: char,
}

impl Rule {
    pub fn parse(input: &str) -> Option<Self> {
        lazy_static! {
            static ref RE: Regex = Regex::new(r"(\w)(\w) -> (\w)").unwrap();
        }

        let matches = RE.captures(input)?;
        let a = matches.get(1)?.as_str().chars().nth(0)?;
        let b = matches.get(2)?.as_str().chars().nth(0)?;
        let to = matches.get(3)?.as_str().chars().nth(0)?;

        Some(Self { from: (a, b), to })
    }
}

#[derive(Debug)]
struct Rules {
    rules: Vec<Rule>,
}

impl Rules {
    pub fn parse(input: &str) -> Option<Self> {
        let rules = input.lines().map(Rule::parse).collect::<Option<Vec<_>>>()?;
        Some(Self { rules })
    }

    pub fn generate(&self, a: &char, b: &char) -> Option<char> {
        self.rules
            .iter()
            .find(|r| &r.from.0 == a && &r.from.1 == b)
            .map(|r| r.to)
    }
}

#[derive(Debug)]
struct Game {
    polymer: Polymer,
    rules: Rules,
    first: char,
    last: char,
}

impl Game {
    pub fn parse(input: &str) -> Option<Self> {
        let mut parts = input.split("\n\n");
        let polymer_part = parts.next()?;

        let first = polymer_part.chars().nth(0)?;
        let last = polymer_part.chars().last()?;
        let polymer = Polymer::parse(polymer_part);

        let rules = parts.next().and_then(Rules::parse)?;
        Some(Self {
            polymer,
            rules,
            first,
            last,
        })
    }

    pub fn step(&mut self) {
        let mut next = HashMap::new();

        for (a, b, count) in self.polymer.iter() {
            if let Some(middle) = self.rules.generate(&a, &b) {
                let i1 = (a, middle);
                let i2 = (middle, b);

                let i1_value = next.entry(i1).or_insert(0);
                *i1_value += count;

                let i2_value = next.entry(i2).or_insert(0);
                *i2_value += count;
            }
        }

        self.polymer = Polymer::new(next)
    }

    pub fn result(&self) -> Option<u64> {
        let mut hash = HashMap::new();
        for (a, b, count) in self.polymer.iter() {
            let value_a = hash.entry(a).or_insert(0);
            *value_a += count;

            let value_b = hash.entry(b).or_insert(0);
            *value_b += count
        }

        let first = hash.entry(self.first).or_insert(0);
        *first += 1;

        let last = hash.entry(self.last).or_insert(0);
        *last += 1;

        let max = hash.values().max()? / 2;
        let min = hash.values().min()? / 2;

        Some(max - min)
    }
}

fn main() -> Result<(), Box<dyn Error>> {
    let filename = std::env::args().nth(1).ok_or("Invalid input")?;
    let raw = std::fs::read_to_string(filename)?;

    let mut game = Game::parse(&raw).expect("Invalid input");

    for _ in 0..10 {
        game.step()
    }

    let result_a = game.result().expect("Result A not found");

    for _ in 10..40 {
        game.step()
    }

    let result_b = game.result().expect("Result B not found");

    println!("Task A: {}\nTask B: {}", result_a, result_b);

    Ok(())
}
