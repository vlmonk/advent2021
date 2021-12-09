use std::collections::{HashSet, VecDeque};
use std::error::Error;

pub struct Point {
    x: usize,
    y: usize,
    value: u32,
}

pub struct Game {
    xsize: usize,
    ysize: usize,
    field: Vec<u32>,
}

impl Game {
    pub fn parse(input: &str) -> Option<Self> {
        let mut field = vec![];
        let mut lines = input.lines().peekable();
        let first_line = lines.peek()?;
        let xsize = first_line.len();
        let mut ysize = 0;

        for line in lines {
            let mut points = line
                .chars()
                .map(|c| c.to_digit(10))
                .collect::<Option<Vec<_>>>()?;
            assert_eq!(points.len(), xsize);
            field.append(&mut points);
            ysize += 1;
        }

        Some(Self {
            field,
            xsize,
            ysize,
        })
    }

    pub fn points<'a>(&'a self) -> impl Iterator<Item = Point> + 'a {
        (0..self.ysize)
            .map(move |y| {
                (0..self.xsize).map(move |x| {
                    let value = self.point(x, y);
                    Point { x, y, value }
                })
            })
            .flatten()
    }

    pub fn lowest<'a>(&'a self) -> impl Iterator<Item = Point> + 'a {
        self.points().filter(move |p| {
            let around = self
                .around(p.x, p.y)
                .map(|(x, y)| self.point(x, y))
                .collect::<Vec<_>>();

            around.into_iter().all(|v| p.value < v)
        })
    }

    pub fn point(&self, x: usize, y: usize) -> u32 {
        assert!(x < self.xsize);
        assert!(y < self.ysize);

        self.field[y * self.xsize + x]
    }

    fn within(&self, x: usize, y: usize) -> bool {
        x < self.xsize && y < self.ysize
    }

    fn around<'a>(&'a self, x: usize, y: usize) -> impl Iterator<Item = (usize, usize)> + 'a {
        let p = match (x, y) {
            (0, 0) => vec![(0, 1), (1, 0)],
            (0, y) => vec![(0, y - 1), (0, y + 1), (1, y)],
            (x, 0) => vec![(x, 1), (x - 1, 0), (x + 1, 0)],
            (x, y) => vec![(x, y - 1), (x, y + 1), (x - 1, y), (x + 1, y)],
        };

        p.into_iter().filter(move |(x, y)| self.within(*x, *y))
    }

    fn area(&self, x: usize, y: usize) -> usize {
        let mut visited: HashSet<(usize, usize)> = HashSet::new();
        let mut queue = VecDeque::new();
        queue.push_back((x, y));
        while let Some((x, y)) = queue.pop_front() {
            if let None = visited.get(&(x, y)) {
                visited.insert((x, y));

                self.around(x, y).for_each(|(x, y)| {
                    let value = self.point(x, y);
                    if value < 9 {
                        queue.push_back((x, y));
                    }
                })
            }
        }

        visited.len()
    }
}

fn main() -> Result<(), Box<dyn Error>> {
    let filename = std::env::args().nth(1).ok_or("Invalid input")?;
    let raw = std::fs::read_to_string(filename)?;
    let game = Game::parse(&raw).ok_or("Parser error")?;
    let result_a: u32 = game.lowest().map(|p| p.value + 1).sum();

    let mut areas = game
        .lowest()
        .map(|p| game.area(p.x, p.y))
        .collect::<Vec<_>>();

    areas.sort();
    areas.reverse();

    let result_b = areas[0..3].into_iter().fold(1, |a, e| a * e);

    println!("Task A: {}\nTask B: {}\n", result_a, result_b);

    Ok(())
}
