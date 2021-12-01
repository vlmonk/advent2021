use std::error::Error;

fn main() -> Result<(), Box<dyn Error>> {
    let input = std::env::args().nth(1).ok_or("Invalid input")?;
    let content = std::fs::read_to_string(input)?;
    let numbers = content
        .lines()
        .map(|line| line.parse::<i32>())
        .collect::<Result<Vec<_>, _>>()?;
    let mut iter = numbers.into_iter().peekable();
    let mut result = 0;

    while let Some(current) = iter.next() {
        if let Some(next) = iter.peek() {
            if *next > current {
                result += 1
            }
        }
    }

    dbg!(result);
    Ok(())
}
