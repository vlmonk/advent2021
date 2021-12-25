use std::error::Error;

#[derive(Debug)]
enum Reg {
    W,
    X,
    Y,
    Z,
}

impl Reg {
    pub fn parse(input: &str) -> Option<Self> {
        match input {
            "w" => Some(Self::W),
            "x" => Some(Self::X),
            "y" => Some(Self::Y),
            "z" => Some(Self::Z),
            _ => None,
        }
    }
}

#[derive(Debug)]
enum Src {
    Reg(Reg),
    Value(i32),
}

impl Src {
    pub fn parse(input: &str) -> Option<Self> {
        Reg::parse(input)
            .map(|r| Self::Reg(r))
            .or_else(|| input.parse::<i32>().ok().map(|value| Self::Value(value)))
    }
}

#[derive(Debug)]
enum Op {
    Inp(Reg),
    Add(Reg, Src),
    Mul(Reg, Src),
    Div(Reg, Src),
    Mod(Reg, Src),
    Eql(Reg, Src),
}

impl Op {
    pub fn parse(input: &str) -> Option<Self> {
        let mut items = input.split(' ');
        let op = items.next()?;
        let reg = items.next().and_then(Reg::parse);
        let src = items.next().and_then(Src::parse);
        // dbg!(op, &reg, &src);

        match op {
            "inp" => Some(Self::Inp(reg?)),
            "add" => Some(Self::Add(reg?, src?)),
            "mul" => Some(Self::Mul(reg?, src?)),
            "div" => Some(Self::Div(reg?, src?)),
            "mod" => Some(Self::Mod(reg?, src?)),
            "eql" => Some(Self::Eql(reg?, src?)),
            _ => None,
        }
    }
}

fn main() -> Result<(), Box<dyn Error>> {
    let filename = std::env::args().nth(1).ok_or("Invalid input")?;
    let input = std::fs::read_to_string(filename)?;
    let commands = input
        .lines()
        .map(Op::parse)
        .collect::<Option<Vec<_>>>()
        .ok_or("Can't parse");

    dbg!(&commands);

    let result_a = 0;
    let result_b = 0;

    println!("Task A: {}, Task B: {}", result_a, result_b);

    Ok(())
}
