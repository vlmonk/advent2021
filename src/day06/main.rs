use std::error::Error;

fn step(fish: &[u8]) -> Vec<u8> {
    let mut added = 0;
    let mut next = fish
        .iter()
        .map(|cur| match cur {
            0 => {
                added += 1;
                6
            }
            n => n - 1,
        })
        .collect::<Vec<_>>();

    for _ in 0..added {
        next.push(8);
    }

    next
}

fn main() -> Result<(), Box<dyn Error>> {
    let filename = std::env::args().nth(1).ok_or("Invalid input")?;
    let mut fish = std::fs::read_to_string(filename)?
        .lines()
        .nth(0)
        .ok_or_else(|| "Empth input")?
        .split(",")
        .map(|p| p.parse::<u8>())
        .collect::<Result<Vec<_>, _>>()?;

    for n in 0..80 {
        fish = step(&fish);
    }

    let r = fish.len();
    println!("Task A: {}", r);

    Ok(())
}
