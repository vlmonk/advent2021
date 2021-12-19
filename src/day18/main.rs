use std::{error::Error, fmt};

#[derive(Debug, PartialEq)]
enum Token {
    Open,
    Close,
    Comma,
    Num(i32),
}

struct Tokenizer<'a> {
    position: usize,
    input: &'a str,
}

impl<'a> Tokenizer<'a> {
    pub fn new(input: &'a str) -> Self {
        Self { input, position: 0 }
    }
}

impl<'a> Iterator for Tokenizer<'a> {
    type Item = Token;

    fn next(&mut self) -> Option<Self::Item> {
        let mut chars = self.input.get(self.position..)?.chars();
        while let Some(c) = chars.next() {
            match c {
                '[' => {
                    self.position += c.len_utf8();
                    return Some(Token::Open);
                }
                ']' => {
                    self.position += c.len_utf8();
                    return Some(Token::Close);
                }
                ',' => {
                    self.position += c.len_utf8();
                    return Some(Token::Comma);
                }
                c if c.is_digit(10) => {
                    let next_chars_len: usize = chars
                        .take_while(|c| c.is_digit(10))
                        .map(|c| c.len_utf8())
                        .sum();
                    let len = next_chars_len + c.len_utf8();
                    let slice = self.input.get(self.position..self.position + len)?;
                    let value = slice.parse::<i32>().ok()?;
                    self.position += len;
                    return Some(Token::Num(value));
                }
                c => {
                    self.position += c.len_utf8();
                }
            }
        }
        None
    }
}

#[derive(Debug, PartialEq)]
struct ExplodePosition<'a> {
    left: Option<&'a mut i32>,
    pair: &'a mut Number,
    right: Option<&'a mut i32>,
}

impl<'a> ExplodePosition<'a> {
    fn new(left: Option<&'a mut i32>, pair: &'a mut Number, right: Option<&'a mut i32>) -> Self {
        Self { left, pair, right }
    }
}

#[derive(Debug, PartialEq)]
enum Number {
    Single(i32),
    Pair(Box<Number>, Box<Number>),
}

impl Number {
    pub fn parse(input: &str) -> Option<Self> {
        let mut tokens = Tokenizer::new(input);
        Number::parse_next(&mut tokens)
    }

    pub fn value(&self) -> Option<i32> {
        match self {
            Self::Single(v) => Some(*v),
            _ => None,
        }
    }

    pub fn a(&self) -> Option<i32> {
        match self {
            Self::Pair(a, _) => match **a {
                Self::Single(v) => Some(v),
                _ => None,
            },
            _ => None,
        }
    }

    pub fn b(&self) -> Option<i32> {
        match self {
            Self::Pair(_, b) => match **b {
                Self::Single(v) => Some(v),
                _ => None,
            },
            _ => None,
        }
    }

    pub fn single(input: i32) -> Self {
        Self::Single(input)
    }

    pub fn pair(a: Number, b: Number) -> Self {
        let a = Box::new(a);
        let b = Box::new(b);
        Self::Pair(a, b)
    }

    pub fn parse_next(tokens: &mut Tokenizer) -> Option<Number> {
        let token = tokens.next()?;
        match token {
            Token::Num(v) => Some(Number::single(v)),
            Token::Open => {
                let a = Number::parse_next(tokens)?;
                if let Some(Token::Comma) = tokens.next() {
                } else {
                    return None;
                }

                let b = Number::parse_next(tokens)?;

                if let Some(Token::Close) = tokens.next() {
                } else {
                    return None;
                }

                Some(Number::pair(a, b))
            }
            _ => None,
        }
    }

    pub fn explode(&mut self) {
        if let Some(ExplodePosition { left, pair, right }) = self.find_pair(4) {
            if let (Some(left), Some(value)) = (left, pair.a()) {
                *left += value
            }

            if let (Some(right), Some(value)) = (right, pair.b()) {
                *right += value
            }

            // if let (Some(left), Some(value)) = (left, pair.a().and_then(|n| n.value())) {
            //     *left += value
            // }
            let mut number = Number::single(0);
            std::mem::swap(pair, &mut number);
        }
        // let left: Option<&Number> = None;
        // if let Some(position) = self.find_pair(4, None, None) {
        //     let mut number = Number::single(0);
        //     std::mem::swap(pair, &mut number);
        // }
    }

    fn find_left(&mut self) -> &mut i32 {
        match self {
            Number::Single(value) => value,
            Number::Pair(a, _) => a.find_left(),
        }
    }

    fn find_right(&mut self) -> &mut i32 {
        match self {
            Number::Single(value) => value,
            Number::Pair(_, b) => b.find_right(),
        }
    }

    fn has_pair(&self, depth: usize) -> bool {
        if depth > 0 {
            match self {
                Number::Single(_) => false,
                Number::Pair(a, b) => a.has_pair(depth - 1) || b.has_pair(depth - 1),
            }
        } else {
            match self {
                Number::Single(_) => false,
                Number::Pair(_, _) => true,
            }
        }
    }

    fn find_pair(&mut self, depth: usize) -> Option<ExplodePosition> {
        if depth > 0 {
            dbg!(depth);
            match self {
                Number::Single(_) => None,
                Number::Pair(a, b) => {
                    let path_a = a.has_pair(depth - 1);
                    let path_b = b.has_pair(depth - 1);

                    if path_a {
                        if let Some(ExplodePosition { left, pair, right }) = a.find_pair(depth - 1)
                        {
                            let right = match right {
                                Some(right) => Some(right),
                                None => Some(b.find_left()),
                            };

                            return Some(ExplodePosition::new(left, pair, right));
                        }
                    } else if path_b {
                        if let Some(ExplodePosition { left, pair, right }) = b.find_pair(depth - 1)
                        {
                            let left = match left {
                                Some(left) => Some(left),
                                None => Some(a.find_right()),
                            };

                            return Some(ExplodePosition::new(left, pair, right));
                        }
                    }

                    None
                }
            }
        } else {
            match self {
                Number::Single(_) => None,
                Number::Pair(_, _) => Some(ExplodePosition::new(None, self, None)),
            }
        }
    }
}

impl fmt::Display for Number {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Number::Single(v) => write!(f, "{}", v),
            Number::Pair(a, b) => write!(f, "[{},{}]", a, b),
        }
    }
}

fn main() -> Result<(), Box<dyn Error>> {
    let filename = std::env::args().nth(1).ok_or("Invalid input")?;
    let input = std::fs::read_to_string(filename)?;
    let mut number = Number::parse(&input).ok_or("Can't parse input")?;
    println!("{}", number);
    number.explode();
    println!("{}", number);

    Ok(())
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_parse_single_num() {
        let input = "12";
        let expected = Number::single(12);
        let result = Number::parse(input).unwrap();
        assert_eq!(result, expected);
    }

    #[test]
    fn test_simple() {
        let input = "[1,2]";
        let expected = Number::pair(Number::single(1), Number::single(2));
        let result = Number::parse(input).unwrap();
        assert_eq!(result, expected);
    }

    #[test]
    fn test_tokenizer() {
        let input = "[],";
        let mut tokenizer = Tokenizer::new(input);
        assert_eq!(tokenizer.next(), Some(Token::Open));
        assert_eq!(tokenizer.next(), Some(Token::Close));
        assert_eq!(tokenizer.next(), Some(Token::Comma));
        assert_eq!(tokenizer.next(), None);
    }

    #[test]
    fn test_tokenizer_num() {
        let input = "9923,[119";
        let mut tokenizer = Tokenizer::new(input);
        assert_eq!(tokenizer.next(), Some(Token::Num(9923)));
        assert_eq!(tokenizer.next(), Some(Token::Comma));
        assert_eq!(tokenizer.next(), Some(Token::Open));
        assert_eq!(tokenizer.next(), Some(Token::Num(119)));
        assert_eq!(tokenizer.next(), None);
    }

    #[test]
    fn test_tokenizer_skip_space() {
        let input = " 999 , 111 ";
        let mut tokenizer = Tokenizer::new(input);
        assert_eq!(tokenizer.next(), Some(Token::Num(999)));
        assert_eq!(tokenizer.next(), Some(Token::Comma));
        assert_eq!(tokenizer.next(), Some(Token::Num(111)));
        assert_eq!(tokenizer.next(), None);
    }
}
