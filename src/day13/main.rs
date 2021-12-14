use lazy_static::lazy_static;
use regex::Regex;
use std::{collections::HashSet, error::Error, fmt};

#[derive(Debug)]
struct Field {
    dots: HashSet<(i32, i32)>,
}

impl Field {
    pub fn parse(input: &str) -> Option<Self> {
        let dots = input
            .lines()
            .map(|line| {
                let mut numbers = line.split(',').filter_map(|n| n.parse::<i32>().ok());
                if let (Some(x), Some(y)) = (numbers.next(), numbers.next()) {
                    Some((x, y))
                } else {
                    None
                }
            })
            .collect::<Option<HashSet<_>>>()?;

        Some(Self { dots })
    }

    pub fn fold(self, rule: &Rule) -> Self {
        let dots = self
            .dots
            .iter()
            .copied()
            .filter_map(|(x, y)| match rule {
                &Rule::Horizontal(line) => match y {
                    y if y < line => Some((x, y)),
                    y if y > line => Some((x, 2 * line - y)),
                    _ => None,
                },
                &Rule::Vertical(line) => match x {
                    x if x < line => Some((x, y)),
                    x if x > line => Some((2 * line - x, y)),
                    _ => None,
                },
            })
            .collect::<HashSet<_>>();

        Self { dots }
    }

    pub fn size(&self) -> usize {
        self.dots.len()
    }
}

impl fmt::Display for Field {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let xmin = self.dots.iter().cloned().map(|(x, _)| x).min().unwrap_or(0);
        let xmax = self.dots.iter().cloned().map(|(x, _)| x).max().unwrap_or(0);

        let ymin = self.dots.iter().cloned().map(|(_, y)| y).min().unwrap_or(0);
        let ymax = self.dots.iter().cloned().map(|(_, y)| y).max().unwrap_or(0);

        for y in ymin..=ymax {
            for x in xmin..=xmax {
                if self.dots.contains(&(x, y)) {
                    write!(f, "█")?;
                } else {
                    write!(f, " ")?;
                }
            }

            write!(f, "\n")?;
        }

        Ok(())
    }
}

#[derive(Debug)]
enum Rule {
    Horizontal(i32),
    Vertical(i32),
}

impl Rule {
    pub fn parse(input: &str) -> Option<Self> {
        lazy_static! {
            static ref RE: Regex = Regex::new(r"fold along (x|y)=(\d+)").unwrap();
        }

        let matches = RE.captures(input)?;
        let value = matches
            .get(2)
            .and_then(|s| s.as_str().parse::<i32>().ok())?;
        let axis = matches.get(1)?.as_str();

        match axis {
            "x" => Some(Self::Vertical(value)),
            "y" => Some(Self::Horizontal(value)),
            _ => None,
        }
    }
}

fn main() -> Result<(), Box<dyn Error>> {
    let filename = std::env::args().nth(1).ok_or("Invalid input")?;
    let raw = std::fs::read_to_string(filename)?;

    let field = raw.split("\n\n").nth(0).and_then(Field::parse).unwrap();
    let rules = raw
        .split("\n\n")
        .nth(1)
        .and_then(|raw| raw.lines().map(Rule::parse).collect::<Option<Vec<_>>>())
        .unwrap();

    let field = field.fold(&rules[0]);
    let result_a = field.size();
    let result_b = rules[1..].iter().fold(field, |f, r| f.fold(r));

    println!("Task A: {}\nTask B:\n{}", result_a, result_b);

    Ok(())
}
