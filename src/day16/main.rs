use std::error::Error;

trait BitInput {
    fn take(&mut self, n: usize) -> Option<u64>;
}

#[derive(Debug)]
struct RawData {
    data: String,
    position: usize,
}

impl RawData {
    pub fn parse(input: &str) -> Option<Self> {
        let data = input
            .chars()
            .filter(|c| c.is_digit(16))
            .map(|c| c.to_digit(16).map(|d| format!("{:04b}", d)))
            .collect::<Option<Vec<_>>>()?
            .join("");

        Some(Self { data, position: 0 })
    }
}

impl BitInput for RawData {
    fn take(&mut self, n: usize) -> Option<u64> {
        let part = &self.data[self.position..self.position + n];

        if part.len() == n {
            let value = u64::from_str_radix(part, 2).ok()?;
            self.position += n;
            Some(value)
        } else {
            None
        }
    }
}

#[derive(Debug, PartialEq)]
enum Payload {
    Literal(u64),
}

impl Payload {
    pub fn literal(input: u64) -> Self {
        Self::Literal(input)
    }

    fn parse_literal<T>(input: &mut T) -> Option<Self>
    where
        T: BitInput,
    {
        let mut parts = vec![];
        let mut is_next = 1;

        while is_next > 0 {
            let part = input.take(5)?;
            is_next = (part & 0b10000) >> 4;
            let bin_part = format!("{:04b}", (part & 0b1111));
            parts.push(bin_part);
        }

        let value = u64::from_str_radix(&parts.join(""), 2).ok()?;
        Some(Self::literal(value))
    }
}

#[derive(Debug, PartialEq)]
struct Packet {
    version: u8,
    typeid: u8,
    payload: Box<Payload>,
}

impl Packet {
    pub fn parse<T>(input: &mut T) -> Option<Self>
    where
        T: BitInput,
    {
        let version = input.take(3)? as u8;
        let typeid = input.take(3)? as u8;

        match typeid {
            4 => {
                let payload = Payload::parse_literal(input)?;
                Some(Self {
                    version,
                    typeid,
                    payload: Box::new(payload),
                })
            }
            n => panic!("Invalid typeid: {}", n),
        }
    }
}

fn main() -> Result<(), Box<dyn Error>> {
    let filename = std::env::args().nth(1).ok_or("Invalid input")?;
    let input = std::fs::read_to_string(filename)?;
    let mut raw = RawData::parse(&input).ok_or_else(|| "Can't parse input")?;

    let x = Packet::parse(&mut raw);
    dbg!(x);

    let result_a = 0;
    let result_b = 0;
    println!("Task A: {}\nTask B: {}", result_a, result_b);

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_literal() {
        let input = "D2FE28";
        let mut raw = RawData::parse(&input).unwrap();
        let packet = Packet::parse(&mut raw).unwrap();
        let payload = Box::new(Payload::literal(2021));
        let expected = Packet {
            version: 6,
            typeid: 4,
            payload,
        };

        assert_eq!(packet, expected);
    }
}
