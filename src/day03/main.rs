use std::convert::TryFrom;
use std::error::Error;

#[derive(Debug, PartialEq)]
enum Bit {
    Zero,
    One,
}

impl Bit {
    pub fn one(&self) -> bool {
        *self == Bit::One
    }
}

impl TryFrom<char> for Bit {
    type Error = &'static str;

    fn try_from(value: char) -> Result<Self, Self::Error> {
        match value {
            '0' => Ok(Bit::Zero),
            '1' => Ok(Bit::One),
            _ => Err("Invalid input"),
        }
    }
}

#[derive(Debug, PartialEq)]
struct BitRow {
    size: usize,
    bits: Vec<Bit>,
}

impl BitRow {
    pub fn parse(input: &str) -> Result<Self, Box<dyn Error>> {
        let size = input.len();
        let bits = input
            .chars()
            .map(Bit::try_from)
            .collect::<Result<Vec<_>, _>>()?;

        Ok(Self { size, bits })
    }

    pub fn from_bits(bits: Vec<Bit>) -> Self {
        let size = bits.len();
        Self { size, bits }
    }

    pub fn into_i32(&self) -> i32 {
        self.bits
            .iter()
            .enumerate()
            .map(|(i, bit)| {
                let idx = self.size - i - 1;
                if bit.one() {
                    1 << idx
                } else {
                    0
                }
            })
            .reduce(|acc, e| e + acc)
            .unwrap_or(0)
    }

    pub fn bits(&self) -> impl Iterator<Item = &Bit> {
        self.bits.iter()
    }
}

#[derive(Debug, PartialEq)]
enum Common {
    Empty,
    Filled {
        size: usize,
        count: Vec<usize>,
        total: usize,
    },
}

impl Common {
    fn new() -> Self {
        Common::Empty
    }

    fn add(self, row: &BitRow) -> Result<Self, Box<dyn Error>> {
        match self {
            Self::Empty => {
                let size = row.size;
                let count = row.bits().map(|b| if b.one() { 1 } else { 0 }).collect();
                let total = 1;

                Ok(Common::Filled { size, count, total })
            }

            Self::Filled {
                size,
                mut count,
                total,
            } => {
                if size != row.size {
                    return Err("Invalid input size".into());
                }

                row.bits().enumerate().for_each(|(i, bit)| {
                    if bit.one() {
                        count[i] += 1
                    }
                });

                let total = total + 1;

                Ok(Common::Filled { size, count, total })
            }
        }
    }

    fn most_common(&self) -> Result<BitRow, Box<dyn Error>> {
        match &self {
            &Self::Empty => Err("Empty common".into()),
            &Self::Filled { total, count, .. } => {
                let bits = count
                    .iter()
                    .map(|i| if i * 2 >= *total { Bit::One } else { Bit::Zero })
                    .collect::<Vec<_>>();

                Ok(BitRow::from_bits(bits))
            }
        }
    }

    fn least_common(&self) -> Result<BitRow, Box<dyn Error>> {
        match &self {
            &Self::Empty => Err("Empty common".into()),
            &Self::Filled { total, count, .. } => {
                let bits = count
                    .iter()
                    .map(|i| if i * 2 < *total { Bit::One } else { Bit::Zero })
                    .collect::<Vec<_>>();

                Ok(BitRow::from_bits(bits))
            }
        }
    }
}

fn main() -> Result<(), Box<dyn Error>> {
    let input = std::env::args().nth(1).ok_or("Invalid input")?;
    let raw = std::fs::read_to_string(input)?;
    let content = raw
        .lines()
        .map(BitRow::parse)
        .collect::<Result<Vec<_>, _>>()?;

    let common = content
        .iter()
        .try_fold(Common::new(), |acc, e| acc.add(e))?;

    let gamma = common.most_common()?.into_i32();
    let epsilon = common.least_common()?.into_i32();

    let result_a = gamma * epsilon;
    println!("Task A: {}", result_a);

    Ok(())
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_parse_bitrow() {
        let row = BitRow::parse("1011").unwrap();
        let expected = BitRow {
            size: 4,
            bits: vec![Bit::One, Bit::Zero, Bit::One, Bit::One],
        };
        assert_eq!(row, expected);
    }

    #[test]
    fn test_add() {
        let common = Common::new();
        let common = common.add(&BitRow::parse("1100").unwrap()).unwrap();
        let common = common.add(&BitRow::parse("0100").unwrap()).unwrap();

        let expected = Common::Filled {
            size: 4,
            total: 2,
            count: vec![1, 2, 0, 0],
        };

        assert_eq!(common, expected);
    }

    #[test]
    fn test_most_common() {
        let common = Common::Filled {
            size: 4,
            total: 5,
            count: vec![4, 3, 2, 1],
        };

        let expected = BitRow::parse("1100").unwrap();
        assert_eq!(common.most_common().unwrap(), expected);
    }

    #[test]
    fn test_least_common() {
        let common = Common::Filled {
            size: 5,
            total: 6,
            count: vec![4, 3, 2, 1, 6],
        };

        let expected = BitRow::parse("00110").unwrap();
        assert_eq!(common.least_common().unwrap(), expected);
    }

    #[test]
    fn test_into_i32() {
        let row = BitRow::parse("01100").unwrap();
        assert_eq!(row.into_i32(), 12);
    }
}
