use std::error::Error;

fn main() -> Result<(), Box<dyn Error>> {
    let input = std::env::args().nth(1).ok_or("Invalid input")?;
    let content = std::fs::read_to_string(input)?;
    let numbers = content
        .lines()
        .map(|line| line.parse::<i32>())
        .collect::<Result<Vec<_>, _>>()?;

    let result_a = &numbers[..].windows(2).filter(|w| w[1] > w[0]).count();

    let window = (2..numbers.len())
        .map(|i| numbers[i - 2] + numbers[i - 1] + numbers[i])
        .collect::<Vec<_>>();

    let result_b = (1..window.len())
        .map(|i| (window[i - 1], window[i]))
        .filter(|(a, b)| b > a)
        .count();

    println!("Task A: {}\nTask B: {}", result_a, result_b);
    Ok(())
}
