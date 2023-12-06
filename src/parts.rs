use std::collections::HashSet;

type Point = (usize, usize);

#[derive(Clone, Debug)]
struct PartNumber {
    base: Point,
    length: u32,
    value: u32,
}

#[derive(Debug)]
struct Gear {
    nums: (PartNumber, PartNumber),
}

#[derive(Debug)]
pub struct Engine {
    part_numbers: Vec<PartNumber>,
    gears: Vec<Gear>,
}

fn is_symbol(ch: &char) -> bool {
    !ch.is_ascii_digit() && *ch != '.'
}

fn symbols_from_line(line: &str, i: usize) -> Vec<Point> {
    line.chars()
        .enumerate()
        .filter(|(_, ch)| is_symbol(ch))
        .map(|(j, _)| (i, j))
        .collect()
}

fn star_from_line(line: &str, i: usize) -> Vec<Point> {
    line.chars()
        .enumerate()
        .filter(|(_, ch)| *ch == '*')
        .map(|(j, _)| (i, j))
        .collect()
}

fn part_nums_from_line(line: &str, i: usize) -> Vec<PartNumber> {
    let mut nums: Vec<PartNumber> = Vec::new();
    let mut opt_num: Option<PartNumber> = None;
    for (j, ch) in line.chars().enumerate() {
        if let Some(val) = super::calibration::is_num(ch) {
            if let Some(num) = &mut opt_num {
                num.extend(val);
            } else {
                opt_num = Some(PartNumber::one(val, i, j))
            }
        } else {
            if let Some(num) = &opt_num {
                nums.push(num.clone());
                opt_num = None;
            }
        }
    }
    if let Some(num) = &opt_num {
        nums.push(num.clone());
    }
    nums
}

impl PartNumber {
    fn one(val: u8, i: usize, j: usize) -> PartNumber {
        PartNumber {
            base: (i, j),
            length: 1,
            value: val as u32,
        }
    }

    fn extend(&mut self, val: u8) {
        self.length += 1;
        self.value = (self.value * 10) + (val as u32);
    }

    fn adjacent_pt(&self, pt: &Point) -> bool {
        let dx = pt.0 as i32 - self.base.0 as i32;
        if dx < -1 || dx > 1 {
            return false;
        }
        let dy = pt.1 as i32 - self.base.1 as i32;
        dy >= -1 && dy <= (self.length as i32)
    }

    fn adjacent_symbol(&self, symbols: &HashSet<Point>) -> bool {
        for i in -1..=1 {
            for j in -1..=(self.length as i32) {
                if self.adjacent_symbol_offset(i, j, symbols) {
                    return true;
                }
            }
        }
        false
    }

    fn adjacent_symbol_offset(&self, i: i32, j: i32, symbols: &HashSet<Point>) -> bool {
        let x = i + self.base.0 as i32;
        let y = j + self.base.1 as i32;
        if x < 0 || y < 0 {
            false
        } else {
            symbols.contains(&(x as usize, y as usize))
        }
    }
}

impl Gear {
    fn from_pt(pt: Point, parts: &Vec<PartNumber>) -> Option<Gear> {
        let adj: Vec<PartNumber> = parts
            .iter()
            .filter(|part| part.adjacent_pt(&pt))
            .cloned()
            .collect();
        if adj.len() == 2 {
            Some(Gear {
                nums: (adj[0].clone(), adj[1].clone()),
            })
        } else {
            None
        }
    }
}

impl Engine {
    pub fn from_lines(lines: &Vec<String>) -> Engine {
        let symbols: HashSet<Point> = lines
            .iter()
            .enumerate()
            .map(|(i, line)| symbols_from_line(line, i))
            .flatten()
            .collect();
        let numbers: Vec<PartNumber> = lines
            .iter()
            .enumerate()
            .map(|(i, line)| part_nums_from_line(line, i))
            .flatten()
            .collect();
        let part_numbers = numbers
            .into_iter()
            .filter(|number| number.adjacent_symbol(&symbols))
            .collect();
        let stars: Vec<Point> = lines
            .iter()
            .enumerate()
            .map(|(i, line)| star_from_line(line, i))
            .flatten()
            .collect();
        let gears = stars
            .into_iter()
            .filter_map(|pt| Gear::from_pt(pt, &part_numbers))
            .collect();
        Engine {
            part_numbers,
            gears,
        }
    }

    pub fn sum_part_numbers(&self) -> usize {
        self.part_numbers
            .iter()
            .map(|number| number.value as usize)
            .sum()
    }

    pub fn sum_gears(&self) -> usize {
        self.gears
            .iter()
            .map(|gear| gear.nums.0.value as usize * gear.nums.1.value as usize)
            .sum()
    }
}
