use std::fs::File;
use std::io::{prelude::*, BufReader};
pub mod day_1;
pub mod day_2;
pub mod day_3;
pub mod day_4;
pub mod day_5;
pub mod day_6;
pub mod day_7;

fn main() {
    day_7::part_1();
}
pub fn read_input(f: &str) -> Vec<String> {
    let file = File::open(f).unwrap();
    let reader = BufReader::new(file);
    reader.lines().map(|a| a.unwrap()).collect()
}
