use std::collections::HashMap;
use std::fs::{read_to_string, File};
use std::io::BufRead;
use std::io::BufReader;

pub fn input_as_lines(day: i8) -> Vec<String> {
    let filename = format!("data/day-{}.txt", day);
    let file = File::open(filename).expect("Issue opening file");
    let reader = BufReader::new(&file);
    reader.lines().map(|s| s.expect("Read failure")).collect()
}

pub fn input_as_string(day: i8) -> String {
    let filename = format!("data/day-{}.txt", day);
    read_to_string(filename).expect("Read failure")
}

pub fn input_as_grid(day: i8) -> HashMap<(i64, i64), char> {
    let mut grid: HashMap<(i64, i64), char> = HashMap::new();
    for (i, line) in input_as_lines(day).into_iter().enumerate() {
        for (j, ch) in line.chars().enumerate() {
            grid.insert((i as i64, j as i64), ch);
        }
    }
    grid
}
