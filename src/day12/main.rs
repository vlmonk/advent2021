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
    visited: HashSet<Cave>,
    visited_twice: bool,
}

impl Path {
    pub fn new(path: Vec<Cave>) -> Self {
        let mut visited = HashSet::new();
        let mut visited_twice = false;

        for c in path.iter() {
            if let Cave::Small(_) = c {
                let new_entry = visited.insert(c.clone());
                visited_twice |= !new_entry;
            }
        }

        Self {
            path,
            visited,
            visited_twice,
        }
    }

    pub fn start() -> Self {
        Self::new(vec![Cave::start()])
    }

    pub fn last(&self) -> &Cave {
        self.path.last().unwrap()
    }

    pub fn add(&self, c: Cave) -> Path {
        let mut path = self.path.clone();
        path.push(c);

        Self::new(path)
    }

    pub fn completed(&self) -> bool {
        if let Cave::End = self.last() {
            true
        } else {
            false
        }
    }

    pub fn allow(&self, added: &Cave, strict: bool) -> bool {
        match added {
            Cave::Large(_) | Cave::End => true,
            Cave::Small(_) => {
                if strict {
                    !self.visited.contains(added)
                } else {
                    !self.visited.contains(added) || !self.visited_twice
                }
            }
            _ => false,
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

    pub fn all_paths(&self, strict: bool) -> Vec<Path> {
        let mut result = Vec::new();

        let mut queue: VecDeque<Path> = VecDeque::new();
        queue.push_back(Path::start());

        while let Some(path) = queue.pop_front() {
            for node in self.next_nodes(&path, strict) {
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

    pub fn next_nodes(&self, p: &Path, strict: bool) -> Vec<Cave> {
        self.next_cave(p)
            .filter(|c| p.allow(c, strict))
            .cloned()
            .collect()
    }

    pub fn next_cave<'a>(&'a self, p: &'a Path) -> impl Iterator<Item = &'a Cave> + 'a {
        let last = p.last();

        self.paths.iter().filter_map(move |c| {
            if c.0 == *last {
                Some(&c.1)
            } else if c.1 == *last {
                Some(&c.0)
            } else {
                None
            }
        })
    }
}
fn main() -> Result<(), Box<dyn Error>> {
    let filename = std::env::args().nth(1).ok_or("Invalid input")?;
    let raw = std::fs::read_to_string(filename)?;
    let game = Game::parse(&raw)?;

    let result_a = game.all_paths(true).len();
    let result_b = game.all_paths(false).len();

    println!("Task A: {}\nTask B: {}", result_a, result_b);
    Ok(())
}
