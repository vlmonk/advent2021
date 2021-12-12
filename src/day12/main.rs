use std::{
    collections::{HashSet, VecDeque},
    error::Error,
    fmt::Display,
};

fn is_upper(input: &str) -> bool {
    input
        .chars()
        .nth(0)
        .map(|c| c.is_uppercase())
        .unwrap_or(false)
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
enum Cave {
    Start,
    End,
    Small(String),
    Large(String),
}

impl Cave {
    pub fn new(input: &str) -> Self {
        match input {
            "start" => Cave::Start,
            "end" => Cave::End,
            cave if is_upper(cave) => Cave::Large(cave.to_owned()),
            cave => Cave::Small(cave.to_owned()),
        }
    }

    pub fn start() -> Self {
        Self::Start
    }
}

impl Display for Cave {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Cave::Start => write!(f, "[S]"),
            Cave::End => write!(f, "[E]"),
            Cave::Small(s) | Cave::Large(s) => write!(f, "{}", s),
        }
    }
}

type Connection = (Cave, Cave);

struct Path {
    path: Vec<Cave>,
}

impl Path {
    pub fn start() -> Self {
        Self {
            path: vec![Cave::start()],
        }
    }

    pub fn last(&self) -> &Cave {
        self.path.last().unwrap()
    }

    pub fn add(&self, c: Cave) -> Path {
        let mut path = self.path.clone();
        path.push(c);

        Self { path }
    }

    pub fn completed(&self) -> bool {
        if let Cave::End = self.last() {
            true
        } else {
            false
        }
    }
}

impl Display for Path {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let caves = self
            .path
            .iter()
            .map(|p| format!("{}", p))
            .collect::<Vec<_>>()
            .join(" -> ");

        write!(f, "{}", caves)
    }
}

#[derive(Debug)]
struct Game {
    paths: Vec<Connection>,
}

impl Game {
    pub fn parse(input: &str) -> Result<Self, Box<dyn Error>> {
        let lines = input.lines();
        let paths = lines
            .map(|l| {
                let mut entries = l.split('-');
                let a = entries.next().expect("invalid input");
                let b = entries.next().expect("invalid input");

                (Cave::new(a), Cave::new(b))
            })
            .collect::<Vec<_>>();

        Ok(Self { paths })
    }

    pub fn all_paths(&self) -> Vec<Path> {
        let mut result = Vec::new();

        let mut queue: VecDeque<Path> = VecDeque::new();
        queue.push_back(Path::start());

        while let Some(path) = queue.pop_front() {
            for node in self.next_nodes(&path) {
                let path = path.add(node);
                if path.completed() {
                    result.push(path)
                } else {
                    queue.push_back(path)
                }
            }
        }

        result
    }

    pub fn next_nodes(&self, p: &Path) -> Vec<Cave> {
        let mut visited = HashSet::new();
        let last = p.last();

        for path in p.path.iter() {
            if let Cave::Small(_) = path {
                visited.insert(path);
            };
        }

        let caves = self
            .paths
            .iter()
            .filter_map(|c| {
                if c.0 == *last {
                    Some(c.1.clone())
                } else if c.1 == *last {
                    Some(c.0.clone())
                } else {
                    None
                }
            })
            .collect::<Vec<_>>();

        caves
            .into_iter()
            .filter_map(|c| match c {
                Cave::Large(_) | Cave::End => Some(c),
                Cave::Small(_) if !visited.contains(&c) => Some(c),
                _ => None,
            })
            .collect()
    }
}
fn main() -> Result<(), Box<dyn Error>> {
    let filename = std::env::args().nth(1).ok_or("Invalid input")?;
    let raw = std::fs::read_to_string(filename)?;
    let game = Game::parse(&raw)?;
    let paths = game.all_paths();
    for p in paths.iter() {
        println!("{}", p)
    }
    let result_a = paths.len();

    println!("Task A: {}", result_a);

    Ok(())
}
