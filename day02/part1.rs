use std::fs::File;
use std::io::{BufRead, BufReader};
use std::env;

fn load_file(file_path: &str) -> Vec<i32> {
    let mut input = Vec::new();

    let file = File::open(file_path).expect("Unable to open file");
    let mut reader = BufReader::new(file);

    let mut line = String::new();
    reader.read_line(&mut line).expect("Unable to read line");

    for code in line.split(",") {
        input.push(match code.trim().parse() {
            Ok(n) => n,
            Err(_) => {
                println!("Invalid string: {}", code);
                continue
            },
        });
    }

    input
}

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() != 2 {
        std::process::exit(1);
    };
    let mut codes: Vec<i32> = load_file(&args[1]);
    codes[1] = 12;
    codes[2] = 2;
    for i in (0..codes.len()).step_by(4) {
        let opcode = codes[i];
        if opcode == 99 { break; }
        let param1 = codes[i + 1] as usize;
        let param2 = codes[i + 2] as usize;
        let param3 = codes[i + 3] as usize;
        match opcode {
            1 => codes[param3] = codes[param1] + codes[param2],
            2 => codes[param3] = codes[param1] * codes[param2],
            _ => {
                println!("Invalid opcode: {}", opcode);
                break;
            }
        }
    }
    println!("{}", codes[0]);
}
