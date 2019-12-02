use std::fs::File;
use std::io::{BufRead, BufReader};

pub fn main() {
    let mut result = 0;
    let file = File::open("input").expect("Unable to open file");
    let reader = BufReader::new(file);

    for line in reader.lines() {
        let mut value: i32 = line.unwrap().parse::<i32>().unwrap() / 3 - 2;
        while value > 0 {
            result += value;
            value = value / 3 - 2;
        }
    }

    print!("{}", result);
}
