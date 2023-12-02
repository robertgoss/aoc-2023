use std::fs::{read_to_string, File};
use std::io::BufReader;
use std::io::BufRead;


pub fn input_as_lines(day: i8) -> Vec<String> {
    let filename = format!("../data/day-{}.txt", day);
    let file = File::open(filename).expect("Issue opening file");
    let reader = BufReader::new(&file);
    reader.lines().map(
        |s| s.expect("Read failure")
    ).collect()
}

pub fn input_as_string(day: i8) -> String {
    let filename = format!("../data/day-{}.txt", day);
    read_to_string(filename).expect("Read failure")
}

pub fn input_as_line(day: i8) -> String {
    input_as_lines(day).into_iter().next().unwrap()
}
