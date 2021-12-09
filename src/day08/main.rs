use std::{
    collections::{BTreeSet, HashMap},
    convert::{TryFrom, TryInto},
    error::Error,
    ops::{Rem, Sub},
};

#[derive(PartialEq, Eq, Hash, Debug, Clone, Copy, Ord, PartialOrd)]
enum Digit {
    A,
    B,
    C,
    D,
    E,
    F,
    G,
}

impl TryFrom<char> for Digit {
    type Error = &'static str;

    fn try_from(value: char) -> Result<Self, Self::Error> {
        match value {
            'a' => Ok(Self::A),
            'b' => Ok(Self::B),
            'c' => Ok(Self::C),
            'd' => Ok(Self::D),
            'e' => Ok(Self::E),
            'f' => Ok(Self::F),
            'g' => Ok(Self::G),
            _ => Err("Invalid character"),
        }
    }
}

#[derive(Debug, PartialEq, Eq, Hash, Clone)]
struct Pattern {
    inner: BTreeSet<Digit>,
}

impl Pattern {
    pub fn len(&self) -> usize {
        self.inner.len()
    }
}

impl Sub for &Pattern {
    type Output = Pattern;

    fn sub(self, rhs: Self) -> Self::Output {
        let diff = self.inner.difference(&rhs.inner).copied().collect();
        Pattern { inner: diff }
    }
}

impl Rem for &Pattern {
    type Output = usize;

    fn rem(self, rhs: Self) -> Self::Output {
        self.inner.intersection(&rhs.inner).count()
    }
}

impl TryFrom<&str> for Pattern {
    type Error = &'static str;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        let mut inner = BTreeSet::new();
        for c in value.chars() {
            inner.insert(c.try_into()?);
        }

        Ok(Self { inner })
    }
}

#[derive(Debug)]
struct Input {
    left: Vec<Pattern>,
    right: Vec<Pattern>,
    decoder: HashMap<Pattern, usize>,
}

impl TryFrom<&str> for Input {
    type Error = &'static str;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        let mut parts = value.split(" | ").map(|part| {
            part.split(" ")
                .map(Pattern::try_from)
                .collect::<Result<Vec<_>, _>>()
        });

        let left = parts.next().unwrap_or_else(|| Err("Left part not found"))?;
        let right = parts
            .next()
            .unwrap_or_else(|| Err("Right part not found"))?;

        let decoder = decode(left.as_ref()).ok_or_else(|| "Can't decode")?;

        Ok(Self {
            left,
            right,
            decoder,
        })
    }
}

impl Input {
    pub fn decoded(&self) -> Option<usize> {
        let len = self.right.len();
        let digits = self
            .right
            .iter()
            .map(|p| self.decoder.get(p).copied())
            .collect::<Option<Vec<_>>>()?;

        let result: usize = digits
            .iter()
            .enumerate()
            .map(|(idx, d)| *d * (10usize.pow(len as u32 - idx as u32 - 1)))
            .sum();

        Some(result)
    }
}

fn decode(input: &[Pattern]) -> Option<HashMap<Pattern, usize>> {
    let simple = |target| input.iter().find(|p| p.len() == target);

    let one = simple(2)?;
    let four = simple(4)?;
    let seven = simple(3)?;
    let eight = simple(7)?;

    let test = four - one;
    let advanced = |a, b, c| {
        input
            .iter()
            .find(|p| p.len() == a && *p % &test == b && *p % one == c)
    };

    let zero = advanced(6, 1, 2)?;
    let two = advanced(5, 1, 1)?;
    let three = advanced(5, 1, 2)?;
    let five = advanced(5, 2, 1)?;
    let six = advanced(6, 2, 1)?;
    let nine = advanced(6, 2, 2)?;

    let mut decoded = HashMap::new();
    let ordered = [zero, one, two, three, four, five, six, seven, eight, nine];

    for (idx, pat) in ordered.iter().cloned().cloned().enumerate() {
        decoded.insert(pat, idx);
    }

    Some(decoded)
}

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

    let input = raw
        .lines()
        .map(Input::try_from)
        .collect::<Result<Vec<_>, _>>()?;

    let decoded = input
        .iter()
        .map(|l| l.decoded())
        .collect::<Option<Vec<_>>>()
        .ok_or_else(|| "error")?;

    let result_b: usize = decoded.iter().sum();
    dbg!(result_b);

    Ok(())
}
