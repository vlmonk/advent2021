use std::{collections::HashMap, error::Error, fmt};

#[derive(Debug, PartialEq, Eq, Hash, Clone, Copy)]
pub struct Point {
    x: i32,
    y: i32,
}

impl Point {
    pub fn new(x: i32, y: i32) -> Self {
        Self { x, y }
    }

    pub fn up(&self) -> Self {
        Self {
            x: self.x,
            y: self.y - 1,
        }
    }

    pub fn down(&self) -> Self {
        Self {
            x: self.x,
            y: self.y + 1,
        }
    }

    pub fn right(&self) -> Self {
        Self {
            x: self.x + 1,
            y: self.y,
        }
    }

    pub fn left(&self) -> Self {
        Self {
            x: self.x - 1,
            y: self.y,
        }
    }

    pub(crate) fn distanct(&self, to: Point) -> i32 {
        (self.x - to.x).abs() + (self.y - to.y).abs()
    }
}

impl fmt::Display for Point {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "({},{})", self.x, self.y)
    }
}

pub trait Searchable {
    fn get(&self, xy: Point) -> Option<(Point, i32)>;
    fn around(&self, xy: &Point) -> Vec<(Point, i32)> {
        [
            self.get(xy.up()),
            self.get(xy.down()),
            self.get(xy.left()),
            self.get(xy.right()),
        ]
        .iter()
        .filter_map(|v| *v)
        .collect()
    }
}

#[derive(Debug)]
struct Grid {
    points: HashMap<Point, i32>,
    xsize: usize,
    ysize: usize,
}

impl fmt::Display for Grid {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        for y in 0..self.ysize {
            for x in 0..self.xsize {
                write!(f, "{}", self.get(Point::new(x as i32, y as i32)).unwrap().1)?
            }
            write!(f, "\n")?
        }

        Ok(())
    }
}

impl Grid {
    pub fn parse(input: &str) -> Option<Self> {
        let (mut xsize, mut ysize) = (0usize, 0usize);
        let mut points = HashMap::new();

        for (y, line) in input.lines().enumerate() {
            for (x, ch) in line.chars().enumerate() {
                xsize = xsize.max(x + 1);

                let point = Point::new(x as i32, y as i32);
                let value = ch.to_digit(10)? as i32;
                points.insert(point, value);
            }

            ysize = ysize.max(y + 1);
        }

        Some(Self {
            points,
            xsize,
            ysize,
        })
    }

    pub(crate) fn target(&self) -> Point {
        dbg!(self.xsize, self.ysize);
        Point::new(self.xsize as i32 - 1, self.ysize as i32 - 1)
    }

    pub fn multiple(&self) -> Self {
        let mut points = HashMap::new();
        for (p, v) in self.points.iter() {
            for dy in 0..5 {
                for dx in 0..5 {
                    let x = p.x + dx * self.xsize as i32;
                    let y = p.y + dy * self.ysize as i32;
                    let value = (v + dx + dy - 1) % 9 + 1;
                    let point = Point::new(x, y);
                    points.insert(point, value);
                }
            }
        }

        Self {
            points,
            xsize: self.xsize * 5,
            ysize: self.ysize * 5,
        }
    }
}

impl Searchable for Grid {
    fn get(&self, xy: Point) -> Option<(Point, i32)> {
        self.points.get(&xy).map(|v| (xy, *v))
    }
}

#[derive(Debug, Clone)]
pub struct Path {
    xy: Point,
    g: i32,
    h: i32,
    f: i32,
}

impl Path {
    pub fn new(xy: Point, g: i32, h: i32) -> Self {
        Self { xy, g, h, f: g + h }
    }
}

impl fmt::Display for Path {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "xy: {}, g: {}, h: {}, f: {}",
            self.xy, self.g, self.h, self.f
        )
    }
}

pub fn astar<T>(grid: &T, from: Point, to: Point) -> Path
where
    T: Searchable,
{
    let mut open: HashMap<Point, Path> = HashMap::new();
    let mut closed: HashMap<Point, Path> = HashMap::new();

    open.insert(from, Path::new(from, 0, from.distanct(to)));

    while open.len() > 0 {
        // println!("O: {}, C: {}", open.len(), closed.len());
        // println!("Open:");
        // for (k, v) in open.iter() {
        //     println!("{} -> {}", k, v)
        // }

        // println!("Closed:");
        // for (k, v) in closed.iter() {
        //     println!("{} -> {}", k, v)
        // }

        let (point, path) = open
            .iter()
            .fold(
                None,
                |current: Option<(&Point, &Path)>, (point, path)| match current {
                    Some((_, current_path)) if current_path.f < path.f => current,
                    _ => Some((point, path)),
                },
            )
            .expect("min not found");

        let point = point.clone();
        let path = path.clone();

        // println!("Selected: {} -> {}", point, path);

        let _ = open.remove(&point).unwrap();

        for (next_point, value) in grid.around(&point) {
            let next_path = Path::new(next_point, path.g + value, next_point.distanct(to));
            // println!("Next: {} -> {}", next_point, next_path);

            if next_path.xy == to {
                // println!("Target");
                return next_path;
            }

            if let Some(z) = open.get(&next_point) {
                // println!("open found");
                if z.f <= next_path.f {
                    // println!("open with lower z found, skip");
                    continue;
                }
            }

            if let Some(z) = closed.get(&next_point) {
                // println!("close found");
                if z.f <= next_path.f {
                    // println!("closed with lower z found, skip");
                    continue;
                }
            }

            // println!("OK, add next_path to open");
            open.insert(next_point, next_path);
        }

        closed.insert(point, path);
    }

    panic!("NO FOUN");
}

fn main() -> Result<(), Box<dyn Error>> {
    let filename = std::env::args().nth(1).ok_or("Invalid input")?;
    let raw = std::fs::read_to_string(filename)?;
    let grid = Grid::parse(&raw).expect("Can't parse grid");
    let target = grid.target();
    let result_a = astar(&grid, Point::new(0, 0), target);

    let large = grid.multiple();
    // println!("{}", large);
    let large_target = large.target();
    // dbg!(large_target);
    let result_b = astar(&large, Point::new(0, 0), large_target);

    println!("Task A: {}\nTask B: {}", result_a.f, result_b.f);

    Ok(())
}
