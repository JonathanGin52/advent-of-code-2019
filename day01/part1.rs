use std::fs::File;
use std::io::{BufRead, BufReader};

pub fn main() {
    let mut result = 0;
    let file = File::open("input").expect("Unable to open file");
    let reader = BufReader::new(file);

    for line in reader.lines() {
        let value: i32 = line.unwrap().parse().unwrap();
        result += value / 3 - 2;
    }

    print!("{}", result);
}
