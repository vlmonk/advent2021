use std::convert::TryFrom;
use std::error::Error;
use std::fmt;

#[derive(Debug, PartialEq)]
enum Bit {
    Zero,
    One,
}

impl Bit {
    pub fn one(&self) -> bool {
        *self == Bit::One
    }

    pub fn zero(&self) -> bool {
        *self == Bit::Zero
    }

    pub fn to_char(&self) -> char {
        match self {
            Self::Zero => '0',
            Self::One => '1',
        }
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

#[derive(PartialEq)]
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

    pub fn at(&self, position: usize) -> &Bit {
        &self.bits[position]
    }
}

impl fmt::Debug for BitRow {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let bits = self.bits().map(|b| b.to_char()).collect::<String>();
        f.write_str(&format!(
            "BitRow {{ size: {}, bits: {} }}",
            self.size, &bits
        ))
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

fn find_value<'a, F>(input: &[&'a BitRow], position: usize, predicate: F) -> &'a BitRow
where
    F: Fn(usize, usize, &Bit) -> bool,
{
    let ones = input.iter().filter(|row| row.at(position).one()).count();
    let zeros = input.len() - ones;
    let selected = input
        .iter()
        .filter(|row| predicate(ones, zeros, row.at(position)))
        .copied()
        .collect::<Vec<_>>();

    match selected.len() {
        1 => selected[0],
        0 => panic!("Invalid input"),
        _ => find_value(&selected[..], position + 1, predicate),
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

    let all = content.iter().collect::<Vec<_>>();
    let oxy_predicate = |ones, zeros, bit: &Bit| if ones >= zeros { bit.one() } else { bit.zero() };
    let co2_predicate = |ones, zeros, bit: &Bit| if ones >= zeros { bit.zero() } else { bit.one() };
    let oxy = find_value(&all[..], 0, oxy_predicate).into_i32();
    let co2 = find_value(&all[..], 0, co2_predicate).into_i32();

    let result_a = gamma * epsilon;
    let result_b = oxy * co2;
    println!("Task A: {}\nTask B: {}", result_a, result_b);

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
