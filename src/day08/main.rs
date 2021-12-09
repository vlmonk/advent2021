use std::error::Error;

fn main() -> Result<(), Box<dyn Error>> {
    let filename = std::env::args().nth(1).ok_or("Invalid input")?;
    let raw = std::fs::read_to_string(filename)?;

    let result_a = raw
        .lines()
        .map(|l| l.split(" | ").skip(1).next().unwrap())
        .map(|l| l.split(" ").map(|p| p.len()))
        .flatten()
        .filter(|n| n == &2 || n == &4 || n == &3 || n == &7)
        .count();

    dbg!(result_a);

    Ok(())
}
