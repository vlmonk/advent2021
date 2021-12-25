use std::error::Error;

fn main() -> Result<(), Box<dyn Error>> {
    let filename = std::env::args().nth(1).ok_or("Invalid input")?;
    let input = std::fs::read_to_string(filename)?;
    dbg!(input);

    let result_a = 0;
    let result_b = 0;

    println!("Task A: {}, Task B: {}", result_a, result_b);

    Ok(())
}
