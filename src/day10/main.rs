use std::{collections::VecDeque, error::Error};

fn find_wrong(input: &str) -> Option<char> {
    let mut buffer = VecDeque::new();
    for char in input.chars() {
        match char {
            '[' | '(' | '<' | '{' => buffer.push_back(char),
            '}' => {
                if let Some('{') = buffer.pop_back() {
                } else {
                    return Some(char);
                }
            }
            '>' => {
                if let Some('<') = buffer.pop_back() {
                } else {
                    return Some(char);
                }
            }
            ']' => {
                if let Some('[') = buffer.pop_back() {
                } else {
                    return Some(char);
                }
            }
            ')' => {
                if let Some('(') = buffer.pop_back() {
                } else {
                    return Some(char);
                }
            }
            _ => panic!("Invalid char: {}", char),
        }
    }
    None
}

fn main() -> Result<(), Box<dyn Error>> {
    let filename = std::env::args().nth(1).ok_or("Invalid input")?;
    let raw = std::fs::read_to_string(filename)?;

    let chars = raw.lines().filter_map(find_wrong).collect::<Vec<_>>();
    let score: isize = chars
        .iter()
        .map(|c| match c {
            ')' => 3,
            ']' => 57,
            '}' => 1197,
            '>' => 25137,
            _ => panic!("invalid char: {}", c),
        })
        .sum();

    dbg!(score);

    Ok(())
}
