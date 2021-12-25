use std::{collections::HashMap, error::Error, fmt::Display};

#[derive(Debug, Clone, Copy)]
enum Cucumber {
    East,
    South,
}

#[derive(Debug)]
pub struct Field {
    width: usize,
    height: usize,
    points: HashMap<(usize, usize), Cucumber>,
}

impl Field {
    pub fn parse(input: &str) -> Option<Self> {
        let mut width: Option<usize> = None;
        let mut height = 0;
        let mut points = HashMap::new();

        for line in input.lines() {
            if let Some(width) = width {
                assert!(line.len() == width)
            } else {
                width = Some(line.len())
            }

            for (x, c) in line.chars().enumerate() {
                match c {
                    '>' => {
                        points.insert((x, height), Cucumber::East);
                    }
                    'v' => {
                        points.insert((x, height), Cucumber::South);
                    }
                    '.' => {}
                    _ => return None,
                }
            }

            height += 1;
        }

        if let Some(width) = width {
            Some(Self {
                width,
                height,
                points,
            })
        } else {
            None
        }
    }

    fn get(&self, x: usize, y: usize) -> Option<Cucumber> {
        self.points.get(&(x, y)).copied()
    }
}

impl Display for Field {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for y in 0..self.height {
            for x in 0..self.width {
                match self.get(x, y) {
                    Some(Cucumber::East) => write!(f, ">")?,
                    Some(Cucumber::South) => write!(f, "v")?,
                    _ => write!(f, ".")?,
                }
            }

            write!(f, "\n")?;
        }
        Ok(())
    }
}

fn main() -> Result<(), Box<dyn Error>> {
    let filename = std::env::args().nth(1).ok_or("Invalid input")?;
    let input = std::fs::read_to_string(filename)?;
    let field = Field::parse(&input).ok_or("Can't parse input")?;

    println!("{}", field);

    let result_a = 0;
    let result_b = 0;

    println!("Task A: {}, Task B: {}", result_a, result_b);

    Ok(())
}
