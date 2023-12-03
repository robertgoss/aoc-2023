use std::collections::HashSet;

type Point = (usize, usize);

#[derive(Clone, Debug)]
struct PartNumber {
    base: Point,
    length: u32,
    value: u32,
}

#[derive(Debug)]
pub struct Engine {
    symbols: HashSet<Point>,
    part_numbers: Vec<PartNumber>,
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
        Engine {
            symbols,
            part_numbers,
        }
    }

    pub fn sum_part_numbers(&self) -> usize {
        self.part_numbers
            .iter()
            .map(|number| number.value as usize)
            .sum()
    }
}
