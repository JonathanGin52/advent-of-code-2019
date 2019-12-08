use std::env;
use std::fmt;
use std::fs::File;
use std::io;
use std::io::{BufRead, BufReader};

#[derive(Debug)]
enum ParameterMode {
    Position,
    Immediate,
}

impl ParameterMode {
    fn new(v: i32) -> ParameterMode {
        match v {
            0 => ParameterMode::Position,
            1 => ParameterMode::Immediate,
            _ => {
                exit(&format!("Invalid parameter mode: {}", v));
                ParameterMode::Position
            }
        }
    }
}

impl fmt::Display for ParameterMode {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let output = match *self {
            ParameterMode::Position => "Position",
            ParameterMode::Immediate => "Immediate",
        };
        write!(f, "Parameter mode: {}", output)
    }
}

#[derive(Debug)]
struct Instruction {
    opcode: i32,
    program_counter_increment: usize,
    parameters: [Parameter; 3],
}

#[derive(Debug)]
struct Parameter {
    mode: ParameterMode,
    data: i32,
}

impl Default for Parameter {
    fn default() -> Self {
        Parameter {
            data: -1,
            mode: ParameterMode::Position,
        }
    }
}

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() != 2 {
        exit(&"No input file specified");
    };
    let mut codes: Vec<i32> = load_file(&args[1]);
    let mut program_counter = 0;

    while program_counter < codes.len() {
        // println!("{:?}", codes);
        let opcode = codes[program_counter];
        if opcode == 99 {
            break;
        }
        let instruction = create_instruction(&codes, program_counter);
        execute_instruction(&mut codes, &instruction);
        program_counter += instruction.program_counter_increment;
    }
    println!("{}", codes[0]);
}

fn execute_instruction(codes: &mut Vec<i32>, instruction: &Instruction) {
    let parameters = &instruction.parameters;
    match instruction.opcode {
        1 => {
            codes[parameters[2].data as usize] =
                fetch_param(codes, &parameters[0]) + fetch_param(codes, &parameters[1])
        }
        2 => {
            codes[parameters[2].data as usize] =
                fetch_param(codes, &parameters[0]) * fetch_param(codes, &parameters[1])
        }
        3 => codes[parameters[0].data as usize] = get_input(),
        4 => println!("{}", fetch_param(codes, &parameters[0])),
        _ => exit(&"Not yet implemented"),
    };
    //println!("{:?}", instruction);
}

fn create_instruction(codes: &Vec<i32>, program_counter: usize) -> Instruction {
    let input = codes[program_counter];
    let opcode = input % 100;
    let mut program_counter_increment: usize = 0;
    let mut parameters: [Parameter; 3] = Default::default();

    match opcode {
        1 | 2 => {
            program_counter_increment = 4;
            for i in 0..3 {
                let val = codes[(program_counter + i + 1) as usize];
                let parameter_mode = ParameterMode::new(get_digit(input / 100, i as u32));
                parameters[i as usize].mode = parameter_mode;
                parameters[i as usize].data = val;
            }
        }
        3 | 4 => {
            program_counter_increment = 2;
            let val = codes[(program_counter + 1) as usize];
            let parameter_mode = ParameterMode::new(get_digit(input / 100, 0_u32));
            parameters[0].mode = parameter_mode;
            parameters[0].data = val;
        }
        _ => exit(&format!("Invalid opcode: {}", opcode)),
    };

    Instruction {
        opcode,
        program_counter_increment,
        parameters,
    }
}

fn fetch_param(codes: &Vec<i32>, parameter: &Parameter) -> i32 {
    match parameter.mode {
        ParameterMode::Position => codes[parameter.data as usize],
        ParameterMode::Immediate => parameter.data,
    }
}

fn get_input() -> i32 {
    let mut input = String::new();
    io::stdin()
        .read_line(&mut input)
        .expect("Failed to read from stdin");

    match input.trim().parse::<i32>() {
        Ok(i) => i,
        Err(_) => {
            exit("Foo");
            -1
        }
    }
}

// Gets the nth digit starting from the right, 0-indexed
fn get_digit(number: i32, n: u32) -> i32 {
    number % 10_i32.pow(n + 1) / 10_i32.pow(n)
}

fn exit(message: &str) {
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
