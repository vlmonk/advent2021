use std::{collections::HashMap, error::Error, fmt::Display, iter::repeat};

#[derive(Debug, Clone, PartialEq, Copy)]
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

    fn east<'a>(&'a self) -> impl Iterator<Item = (usize, usize)> + 'a {
        self.points
            .iter()
            .filter(|(_, c)| **c == Cucumber::East)
            .map(|(xy, _)| xy)
            .copied()
    }

    fn south<'a>(&'a self) -> impl Iterator<Item = (usize, usize)> + 'a {
        self.points
            .iter()
            .filter(|(_, c)| **c == Cucumber::South)
            .map(|(xy, _)| xy)
            .copied()
    }

    fn step_east(&mut self) -> usize {
        let mut points = HashMap::new();
        let mut moved = 0;

        for (x, y) in self.east() {
            let (nx, ny) = ((x + 1) % self.width, y);
            match self.get(nx, ny) {
                Some(_) => {
                    points.insert((x, y), Cucumber::East);
                }
                None => {
                    points.insert((nx, ny), Cucumber::East);
                    moved += 1
                }
            };
        }

        for pos in self.south() {
            points.insert(pos, Cucumber::South);
        }

        self.points = points;

        moved
    }

    fn step_south(&mut self) -> usize {
        let mut points = HashMap::new();
        let mut moved = 0;

        for (x, y) in self.south() {
            let (nx, ny) = (x, (y + 1) % self.height);
            match self.get(nx, ny) {
                Some(_) => {
                    points.insert((x, y), Cucumber::South);
                }
                None => {
                    points.insert((nx, ny), Cucumber::South);
                    moved += 1
                }
            };
        }

        for pos in self.east() {
            points.insert(pos, Cucumber::East);
        }

        self.points = points;

        moved
    }

    fn step(&mut self) -> usize {
        self.step_east() + self.step_south()
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
    let mut field = Field::parse(&input).ok_or("Can't parse input")?;
    let result_a = repeat(()).take_while(|_| field.step() > 0).count() + 1;
    let result_b = 0;

    println!("Task A: {}, Task B: {}", result_a, result_b);

    Ok(())
}
