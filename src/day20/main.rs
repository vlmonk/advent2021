use std::collections::HashSet;
use std::error::Error;

struct Rules {
    points: HashSet<usize>,
}

impl Rules {
    fn new(input: &str) -> Self {
        let mut points = HashSet::new();
        input.chars().enumerate().for_each(|(idx, c)| {
            if c == '#' {
                points.insert(idx);
            }
        });

        Self { points }
    }

    fn get(&self, idx: usize) -> bool {
        self.points.contains(&idx)
    }
}

#[derive(PartialEq, Debug)]
struct BoundingBox {
    xmin: i32,
    xmax: i32,
    ymin: i32,
    ymax: i32,
}

impl BoundingBox {
    fn new(xmin: i32, xmax: i32, ymin: i32, ymax: i32) -> Self {
        Self {
            xmin,
            xmax,
            ymin,
            ymax,
        }
    }

    fn from_points(points: &HashSet<(i32, i32)>) -> Option<Self> {
        let mut result = None;
        for (x, y) in points {
            result = match result {
                None => Some(BoundingBox::from_point(*x, *y)),
                Some(bb) => Some(bb.extend(*x, *y)),
            }
        }

        return result;
    }

    fn from_point(x: i32, y: i32) -> Self {
        Self {
            xmin: x,
            xmax: x,
            ymin: y,
            ymax: y,
        }
    }

    fn extend(&self, x: i32, y: i32) -> Self {
        Self {
            xmin: if self.xmin < x { self.xmin } else { x },
            xmax: if self.xmax > x { self.xmax } else { x },
            ymin: if self.ymin < y { self.ymin } else { y },
            ymax: if self.ymax > y { self.ymax } else { y },
        }
    }

    fn within(&self, x: i32, y: i32) -> bool {
        x >= self.xmin && x <= self.xmax && y >= self.ymin && y <= self.ymax
    }
}

struct Field {
    points: HashSet<(i32, i32)>,
    bb: Option<BoundingBox>,
    fill: bool,
}

impl Field {
    fn new(input: &str) -> Self {
        let mut points = HashSet::new();
        input.lines().enumerate().for_each(|(y, line)| {
            line.chars().enumerate().for_each(|(x, c)| {
                if c == '#' {
                    let pair = (x as i32, y as i32);
                    points.insert(pair);
                }
            });
        });

        let bb = BoundingBox::from_points(&points);

        Self {
            points,
            bb,
            fill: false,
        }
    }

    fn rule_index(&self, x: i32, y: i32) -> usize {
        (y - 1..=y + 1)
            .map(|y| (x - 1..=x + 1).map(move |x| if self.get(x, y) { 1 } else { 0 }))
            .flatten()
            .enumerate()
            .map(|(idx, v)| v << (8 - idx))
            .sum()
    }

    fn step(&self, rules: &Rules) -> Self {
        let mut points = HashSet::new();
        if let Some(bb) = &self.bb {
            for y in (bb.ymin - 1)..=(bb.ymax + 1) {
                for x in (bb.xmin - 1)..=(bb.xmax + 1) {
                    let idx = self.rule_index(x, y);
                    let z = rules.get(idx);
                    // dbg!(x, y, idx, z);
                    if z {
                        points.insert((x, y));
                    }
                }
            }
        }
        let bb = BoundingBox::from_points(&points);
        let fill = if self.fill {
            rules.get(511)
        } else {
            rules.get(0)
        };

        Self { points, bb, fill }
    }

    fn pixels(&self) -> usize {
        self.points.len()
    }

    fn get(&self, x: i32, y: i32) -> bool {
        let within = if let Some(bb) = &self.bb {
            bb.within(x, y)
        } else {
            false
        };

        if within {
            self.points.contains(&(x, y))
        } else {
            self.fill
        }
    }
}

impl std::fmt::Display for Field {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        if let Some(bb) = &self.bb {
            write!(f, "fill: {}\n\n", if self.fill { "#" } else { "." })?;
            for y in bb.ymin..=bb.ymax {
                for x in bb.xmin..=bb.xmax {
                    if self.points.contains(&(x, y)) {
                        write!(f, "#")?
                    } else {
                        write!(f, ".")?
                    }
                }

                write!(f, "\n")?
            }
        }
        Ok(())
    }
}

fn main() -> Result<(), Box<dyn Error>> {
    let filename = std::env::args().nth(1).ok_or("Missing input filename")?;
    let data = std::fs::read_to_string(filename)?;
    let mut raw = data.split("\n\n");

    let rules = raw.next().map(Rules::new).ok_or("Invalid input")?;
    let mut field = raw.next().map(Field::new).ok_or("Invalid input")?;

    (0..2).for_each(|_| field = field.step(&rules));
    let pixels_a = field.pixels();

    (0..48).for_each(|_| field = field.step(&rules));
    let pixels_b = field.pixels();

    println!("Result A: {}\nResult B: {}", pixels_a, pixels_b);
    Ok(())
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_rule() {
        let input = "..##..#";
        let rule = Rules::new(&input);
        assert_eq!(true, rule.get(2));
        assert_eq!(false, rule.get(4));
        assert_eq!(false, rule.get(4));
        assert_eq!(true, rule.get(6));
        assert_eq!(false, rule.get(979));
    }

    #[test]
    fn test_bounding_box() {
        let input = "#.#\n...\n#..";
        let field = Field::new(&input);
        let expected = Some(BoundingBox::new(0, 2, 0, 2));
        assert_eq!(expected, field.bb);
    }

    #[test]
    fn text_rule_index() {
        let input = "#..\n...\n.#.";
        let field = Field::new(&input);
        assert_eq!(0b000010000, field.rule_index(0, 0));
        assert_eq!(0b000000001, field.rule_index(-1, -1));
        assert_eq!(0b100000010, field.rule_index(1, 1));
    }
}
