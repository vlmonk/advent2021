use std::{
    collections::{HashSet, VecDeque},
    error::Error,
    fmt,
};

#[derive(Debug)]
struct Grid {
    xsize: usize,
    ysize: usize,
    storage: Vec<u32>,
}

impl Grid {
    pub fn parse(input: &str) -> Result<Self, Box<dyn Error>> {
        let xsize = input.lines().nth(0).ok_or_else(|| "Empty input")?.len();
        let storage: Vec<u32> = input.chars().filter_map(|c| c.to_digit(10)).collect();
        let ysize = storage.len() / xsize;
        Ok(Self {
            xsize,
            ysize,
            storage,
        })
    }

    pub fn flash(&mut self) -> usize {
        let mut queue = VecDeque::new();
        let mut flashed = HashSet::new();

        for (x, y) in self.points() {
            let value = self.inc(x, y);
            if value > 9 {
                flashed.insert((x, y));
                queue.push_back((x, y));
            }
        }

        while let Some((x, y)) = queue.pop_front() {
            for (x, y) in self.around(x, y) {
                let value = self.inc(x, y);
                if value > 9 && !flashed.contains(&(x, y)) {
                    flashed.insert((x, y));
                    queue.push_back((x, y));
                }
            }
        }

        for (x, y) in flashed.iter() {
            self.set(*x, *y, 0)
        }

        flashed.len()
    }

    pub fn size(&self) -> usize {
        self.xsize * self.ysize
    }

    fn points(&self) -> impl Iterator<Item = (i32, i32)> {
        let (xsize, ysize) = (self.xsize, self.ysize);

        (0..ysize)
            .map(move |y| (0..xsize).map(move |x| (x as i32, y as i32)))
            .flatten()
    }

    fn around(&self, x: i32, y: i32) -> impl Iterator<Item = (i32, i32)> {
        let (xsize, ysize) = (self.xsize as i32, self.ysize as i32);

        (-1..=1)
            .map(|dy| (-1..=1).map(move |dx| (dx, dy)))
            .flatten()
            .filter(|(dx, dy)| *dx != 0 || *dy != 0)
            .map(move |(dx, dy)| (x + dx, y + dy))
            .filter(move |(x, y)| *x >= 0 && *x < xsize && *y >= 0 && *y < ysize)
    }

    fn inc(&mut self, x: i32, y: i32) -> u32 {
        let idx = (x as usize) + (y as usize) * self.xsize;
        self.storage[idx] += 1;
        self.storage[idx]
    }

    fn set(&mut self, x: i32, y: i32, value: u32) {
        let idx = (x as usize) + (y as usize) * self.xsize;
        self.storage[idx] = value;
    }
}

impl fmt::Display for Grid {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        for y in 0..self.ysize {
            let idx = (y as usize) * self.xsize;
            let xsize = self.xsize as usize;
            let row = (idx..idx + xsize)
                .map(|p| self.storage[p])
                .map(|x| format!("{}", x))
                .collect::<String>();
            write!(f, "{}\n", row)?
        }

        Ok(())
    }
}

fn main() -> Result<(), Box<dyn Error>> {
    let filename = std::env::args().nth(1).ok_or("Invalid input")?;
    let raw = std::fs::read_to_string(filename)?;
    let mut grid = Grid::parse(&raw)?;

    let result_a: usize = (0..100).map(|_| grid.flash()).sum();
    let result_b = (101..)
        .skip_while(|_| grid.flash() != grid.size())
        .nth(0)
        .ok_or("Not found")?;

    println!("Task A: {}, Task B: {}", result_a, result_b);
    Ok(())
}
