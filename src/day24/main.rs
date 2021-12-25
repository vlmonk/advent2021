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
    Value(i64),
}

impl Src {
    pub fn parse(input: &str) -> Option<Self> {
        Reg::parse(input)
            .map(|r| Self::Reg(r))
            .or_else(|| input.parse::<i64>().ok().map(|value| Self::Value(value)))
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

pub struct Code {
    current: i64,
}

impl Code {
    pub fn new() -> Self {
        Self {
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
    for (idx, c) in code.enumerate() {
        if idx % 1_000_000 == 0 {
            println!("I: {}", idx);
        }

        let mut computer = Computer::new();
        computer.run(&commands, &c);
        // dbg!(computer.z);
        if computer.z == 0 {
            println!("code {:?}", c);
            break;
        }
    }

    let result_a = 0;
    let result_b = 0;

    println!("Task A: {}, Task B: {}", result_a, result_b);

    Ok(())
}
