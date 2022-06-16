use std::{error::Error, fmt::Display};

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

impl Display for Reg {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::W => write!(f, "w"),
            Self::X => write!(f, "x"),
            Self::Y => write!(f, "y"),
            Self::Z => write!(f, "z"),
        }
    }
}

#[derive(Debug)]
enum Src {
    Reg(Reg),
    Value(i64),
}

impl Src {
    pub fn parse(input: &str) -> Option<Self> {
        Reg::parse(input)
            .map(|r| Self::Reg(r))
            .or_else(|| input.parse::<i64>().ok().map(|value| Self::Value(value)))
    }
}

impl Display for Src {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Reg(reg) => write!(f, "{}", reg),
            Self::Value(val) => write!(f, "{}", val),
        }
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

impl Display for Op {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Inp(reg) => write!(f, "* inp {}", reg),
            Self::Add(reg, src) => write!(f, "add {} {}", reg, src),
            Self::Mul(reg, src) => write!(f, "mul {} {}", reg, src),
            Self::Div(reg, src) => write!(f, "div {} {}", reg, src),
            Self::Mod(reg, src) => write!(f, "mod {} {}", reg, src),
            Self::Eql(reg, src) => write!(f, "eql {} {}", reg, src),
        }
    }
}

#[derive(Debug)]
struct Computer {
    w: i64,
    x: i64,
    y: i64,
    z: i64,
}

impl Computer {
    pub fn new() -> Self {
        Self {
            w: 0,
            x: 0,
            y: 0,
            z: 0,
        }
    }

    pub fn run(&mut self, programm: &[Op], input: &[i64]) {
        let mut input = input.iter();

        for op in programm {
            match op {
                Op::Inp(reg) => {
                    let src = input.next().expect("Empty input");
                    let target = self.reg(reg);
                    *target = *src;
                }

                Op::Add(reg, src) => {
                    let src = self.src(src);
                    let target = self.reg(reg);
                    *target += src;
                }

                Op::Mul(reg, src) => {
                    let src = self.src(src);
                    let target = self.reg(reg);
                    *target *= src;
                }

                Op::Div(reg, src) => {
                    let src = self.src(src);
                    let target = self.reg(reg);
                    *target /= src;
                }

                Op::Mod(reg, src) => {
                    let src = self.src(src);
                    let target = self.reg(reg);
                    *target %= src;
                }

                Op::Eql(reg, src) => {
                    let src = self.src(src);
                    let target = self.reg(reg);

                    if *target == src {
                        *target = 1
                    } else {
                        *target = 0
                    }
                }
            }

            println!("{:10} -> {}", format!("{}", op), self);
        }
    }

    pub fn reg(&mut self, reg: &Reg) -> &mut i64 {
        match reg {
            Reg::W => &mut self.w,
            Reg::X => &mut self.x,
            Reg::Y => &mut self.y,
            Reg::Z => &mut self.z,
        }
    }

    pub fn value(&self, reg: &Reg) -> i64 {
        match reg {
            Reg::W => self.w,
            Reg::X => self.x,
            Reg::Y => self.y,
            Reg::Z => self.z,
        }
    }

    pub fn src(&self, src: &Src) -> i64 {
        match src {
            Src::Value(val) => *val,
            Src::Reg(reg) => self.value(reg),
        }
    }
}

impl Display for Computer {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "[w: {} x: {} y: {} z: {}]",
            self.w, self.x, self.y, self.z
        )
    }
}

pub struct Code {
    current: i64,
}

impl Code {
    pub fn new() -> Self {
        Self {
            // current: 99_999_999_999_999,
            current: 99_999_999_999_999,
        }
    }
}

impl Iterator for Code {
    type Item = Vec<i64>;

    fn next(&mut self) -> Option<Self::Item> {
        'outer: loop {
            if self.current == 0 {
                break;
            }

            let value = self.current;
            self.current -= 1;

            let printed = format!("{:014}", value);
            let digits = printed
                .chars()
                .map(|c| c.to_digit(10).map(|v| v as i64))
                .collect::<Option<Vec<_>>>()
                .expect("Invalid number");

            for d in &digits {
                if *d == 0 {
                    continue 'outer;
                }
            }

            // if digits[13] != 7 {
            //     continue;
            // }

            return Some(digits);
        }

        None
    }
}

fn main() -> Result<(), Box<dyn Error>> {
    let filename = std::env::args().nth(1).ok_or("Invalid input")?;
    let input = std::fs::read_to_string(filename)?;
    let commands = input
        .lines()
        .map(Op::parse)
        .collect::<Option<Vec<_>>>()
        .ok_or("Can't parse")?;

    // dbg!(&commands);

    let code = Code::new();
    for i in [1, 2, 3, 4, 5, 6, 7, 8, 9] {
        let mut computer = Computer::new();
        computer.run(&commands, &[i, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 4, 4]);

        println!("{} -> {}", i, computer.z);
    }
    // for (idx, c) in code.enumerate().take(65) {
    //     if idx % 1_000_000 == 0 {
    //         println!("I: {}", idx);
    //     }

    //     let mut computer = Computer::new();
    //     computer.run(&commands, &c);
    //     // dbg!(computer.z);
    //     let i = c
    //         .iter()
    //         .map(|c| format!("{}", c))
    //         .collect::<Vec<_>>()
    //         .join("");

    //     println!("{} -> {}", i, computer.z);

    //     if computer.z == 0 {
    //         println!("code {:?}", c);
    //         break;
    //     }
    // }

    let result_a = 0;
    let result_b = 0;

    println!("Task A: {}, Task B: {}", result_a, result_b);

    Ok(())
}
