use std::error::Error;

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
enum Number {
    Single(i32),
    Pair(Box<Number>, Box<Number>),
}

impl Number {
    pub fn parse(input: &str) -> Option<Self> {
        let mut tokens = Tokenizer::new(input);
        Number::parse_next(&mut tokens)
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
}

fn main() -> Result<(), Box<dyn Error>> {
    let filename = std::env::args().nth(1).ok_or("Invalid input")?;
    let input = std::fs::read_to_string(filename)?;
    let number = Number::parse(&input);
    dbg!(number);

    Ok(())
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_parse_single_num() {
        let input = "12";
        let expected = Number::Single(12);
        let result = Number::parse(input).unwrap();
        assert_eq!(result, expected);
    }

    #[test]
    fn test_simple() {
        let input = "[1,2]";
        let expected = Number::pair(Number::single(1), Number::Single(2));
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
