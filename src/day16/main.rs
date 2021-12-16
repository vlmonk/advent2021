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
    Operator(Vec<Packet>, u8),
}

impl Payload {
    pub fn literal(input: u64) -> Self {
        Self::Literal(input)
    }

    pub fn operator(input: Vec<Packet>, typeid: u8) -> Self {
        Self::Operator(input, typeid)
    }

    fn parse_literal<T>(input: &mut T) -> Option<(Self, usize)>
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
        Some((Self::literal(value), parts.len() * 5))
    }

    fn version_sum(&self) -> usize {
        match self {
            Self::Literal(_) => 0,
            Self::Operator(packets, _) => packets.iter().map(|p| p.version_sum()).sum(),
        }
    }
}

#[derive(Debug, PartialEq)]
struct Packet {
    version: u8,
    payload: Payload,
}

impl Packet {
    fn new(version: u8, payload: Payload) -> Self {
        Self { version, payload }
    }

    pub fn parse<T>(input: &mut T) -> Option<(Self, usize)>
    where
        T: BitInput,
    {
        let version = input.take(3)? as u8;
        let typeid = input.take(3)? as u8;

        match typeid {
            4 => {
                let (payload, size) = Payload::parse_literal(input)?;
                let packet = Packet::new(version, payload);
                Some((packet, size + 6))
            }
            _ => {
                let lengthid = input.take(1)?;
                match lengthid {
                    0 => {
                        let length = input.take(15)? as usize;
                        let mut rest = length;
                        let mut packets = vec![];
                        while rest > 0 {
                            let (packet, size) = Self::parse(input)?;
                            packets.push(packet);
                            rest -= size;
                        }
                        let payload = Payload::operator(packets, typeid);
                        let packet = Self::new(version, payload);

                        Some((packet, length + 22))
                    }
                    1 => {
                        let mut count = input.take(11)? as usize;
                        let mut length = 0;
                        let mut packets = vec![];
                        while count > 0 {
                            let (packet, size) = Self::parse(input)?;
                            packets.push(packet);
                            length += size;
                            count -= 1;
                        }
                        let payload = Payload::operator(packets, typeid);
                        let packet = Self::new(version, payload);

                        Some((packet, length + 18))
                    }
                    _ => todo!(),
                }
            }
        }
    }

    pub fn version_sum(&self) -> usize {
        self.version as usize + self.payload.version_sum()
    }
}

fn main() -> Result<(), Box<dyn Error>> {
    let filename = std::env::args().nth(1).ok_or("Invalid input")?;
    let input = std::fs::read_to_string(filename)?;
    let mut raw = RawData::parse(&input).ok_or_else(|| "Can't parse input")?;

    let (packet, _) = Packet::parse(&mut raw).ok_or_else(|| "Can't parse packet")?;

    let result_a = packet.version_sum();
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
        let (packet, size) = Packet::parse(&mut raw).unwrap();
        let payload = Payload::literal(2021);
        let expected = Packet::new(6, payload);

        assert_eq!(packet, expected);
        assert_eq!(size, 21);
    }

    #[test]
    fn test_operator_1() {
        let input = "38006F45291200";
        let mut raw = RawData::parse(&input).unwrap();
        let (packet, size) = Packet::parse(&mut raw).unwrap();
        let a = Packet::new(6, Payload::literal(10));
        let b = Packet::new(2, Payload::literal(20));
        let payload = Payload::operator(vec![a, b], 6);
        let expected = Packet::new(1, payload);

        assert_eq!(packet, expected);
        assert_eq!(size, 49);
    }
}
