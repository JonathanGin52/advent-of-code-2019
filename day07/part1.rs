use std::env;
use std::fs::File;
use std::io::{BufRead, BufReader};
mod intcode_machine;

use intcode_machine::*;

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() != 2 {
        exit(&"No input file specified");
    };
    // let mut phase_settings = [0, 1, 2, 3, 4];
    // let length = phase_settings.len();
    let codes = load_file(&args[1]);

    let intcode_machine = IntcodeMachine::new(codes);
}

fn permute(mut phase_settings: &mut [i32], size: usize, mut max: &mut i32) {
    if size == 1 {
        return;
    }
    for i in 0..size {
        permute(&mut phase_settings, size - 1, &mut max);
        if size % 2 == 1 {
            let temp = phase_settings[0];
            phase_settings[0] = phase_settings[size - 1];
            phase_settings[size - 1] = temp;
        } else {
            let temp = phase_settings[i];
            phase_settings[i] = phase_settings[size - 1];
            phase_settings[size - 1] = temp;
        }
    }
}

fn exit(message: &str) -> ! {
    println!("{}", message);
    std::process::exit(1);
}

fn load_file(file_path: &str) -> Vec<i32> {
    let mut input = Vec::new();

    let file = File::open(file_path).expect("Unable to open file");
    let mut reader = BufReader::new(file);

    let mut line = String::new();
    reader.read_line(&mut line).expect("Unable to read line");

    for code in line.split(",") {
        input.push(match code.trim().parse() {
            Ok(n) => n,
            Err(e) => {
                println!("{}", e);
                continue;
            }
        });
    }

    input
}
