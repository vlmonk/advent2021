use std::{collections::VecDeque, error::Error};

enum Info {
    Wrong(char),
    Missing(Vec<char>),
}

fn matched(input: char) -> char {
    match input {
        '[' => ']',
        '(' => ')',
        '<' => '>',
        '{' => '}',
        _ => panic!("Invalid char: {}", input),
    }
}

fn parse(input: &str) -> Info {
    let mut buffer = VecDeque::new();
    for char in input.chars() {
        match char {
            '[' | '(' | '<' | '{' => buffer.push_back(char),
            ']' | ')' | '>' | '}' => match buffer.pop_back() {
                Some(c) if matched(c) == char => {}
                _ => return Info::Wrong(char),
            },
            _ => panic!("Invalid char: {}", char),
        }
    }

    let mut missing = vec![];

    while let Some(c) = buffer.pop_back() {
        missing.push(matched(c))
    }

    Info::Missing(missing)
}

fn score_missing(input: &char) -> i64 {
    match input {
        ')' => 1,
        ']' => 2,
        '}' => 3,
        '>' => 4,
        _ => panic!("Invalid char: {}", input),
    }
}

fn calculate_missing(input: &[i64]) -> i64 {
    input.iter().fold(0, |a, e| a * 5 + e)
}

fn main() -> Result<(), Box<dyn Error>> {
    let filename = std::env::args().nth(1).ok_or("Invalid input")?;
    let raw = std::fs::read_to_string(filename)?;
    let parsed = raw.lines().map(parse).collect::<Vec<_>>();

    let wrong = parsed
        .iter()
        .filter_map(|info| {
            if let Info::Wrong(c) = info {
                Some(c)
            } else {
                None
            }
        })
        .collect::<Vec<_>>();

    let result_a: isize = wrong
        .iter()
        .map(|c| match c {
            ')' => 3,
            ']' => 57,
            '}' => 1197,
            '>' => 25137,
            _ => panic!("invalid char: {}", c),
        })
        .sum();

    let mut missing = parsed
        .iter()
        .filter_map(|info| {
            if let Info::Missing(m) = info {
                let scored = m.iter().map(score_missing).collect::<Vec<_>>();
                let total = calculate_missing(&scored);
                Some(total)
            } else {
                None
            }
        })
        .collect::<Vec<_>>();

    missing.sort();

    let idx = (missing.len() - 1) / 2;
    let middle = missing[idx];

    println!("Task A: {}\nTask B: {}", result_a, middle);

    Ok(())
}
