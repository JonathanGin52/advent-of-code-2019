use std::env;
use std::cmp;
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
            _ => exit(&format!("Invalid parameter mode: {}", v)),
        }
    }
}

#[derive(Debug)]
struct Instruction {
    opcode: i32,
    pointer_increment: usize,
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
    let mut phase_settings = [0, 1, 2, 3, 4];
    let size = phase_settings.len();
    let permutations = Vec::new();
    let mut max = -1;

    generate_permutations(&mut phase_settings, size, &permutations);

    for permutation in permutations {
        println!("{:?}", permutation);
    }
    println!("{}", max);
}

fn generate_permutations(mut phase_settings: &mut [i32], size: usize, permutations: &Vec<&mut[i32]>) {
    if size == 1 {
        permutations.push(phase_settings);
        return;
    }
    for i in 0..size {
        generate_permutations(&mut phase_settings, size - 1, permutations);
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

fn run(mut codes: &mut Vec<i32>, mut input: &mut Vec<i32>) -> Vec<i32> {
    let mut instruction_pointer = 0;
    let mut output: Vec<i32> = Vec::new();

    while instruction_pointer < codes.len() && codes[instruction_pointer] != 99 {
        // println!("{:?}", codes);
        // println!("{}", instruction_pointer);
        let instruction = create_instruction(&codes, instruction_pointer);
        execute_instruction(&mut codes, &instruction, &mut instruction_pointer, &mut input, &mut output);
    }

    output
}

fn execute_instruction(
    codes: &mut Vec<i32>,
    instruction: &Instruction,
    instruction_pointer: &mut usize,
    input: &mut Vec<i32>,
    output: &mut Vec<i32>,
) {
    // println!("{:?}", instruction);
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
        3 => codes[parameters[0].data as usize] = input.remove(0),
        4 => {
            let out = fetch_param(codes, &parameters[0]);
            output.push(out);
            // println!("{}", out);
        }
        5 => {
            if fetch_param(codes, &parameters[0]) != 0 {
                *instruction_pointer = fetch_param(codes, &parameters[1]) as usize;
                *instruction_pointer -= 3
            }
        }
        6 => {
            if fetch_param(codes, &parameters[0]) == 0 {
                *instruction_pointer = fetch_param(codes, &parameters[1]) as usize;
                *instruction_pointer -= 3
            }
        }
        7 => {
            codes[parameters[2].data as usize] =
                (fetch_param(codes, &parameters[0]) < fetch_param(codes, &parameters[1])) as i32
        }
        8 => {
            codes[parameters[2].data as usize] =
                (fetch_param(codes, &parameters[0]) == fetch_param(codes, &parameters[1])) as i32
        }
        _ => exit(&"Opcode not implemented: {}"),
    };
    *instruction_pointer += instruction.pointer_increment;
}

fn create_instruction(codes: &Vec<i32>, instruction_pointer: usize) -> Instruction {
    let input = codes[instruction_pointer];
    let opcode = input % 100;
    let pointer_increment: usize;
    let mut parameters: [Parameter; 3] = Default::default();

    match opcode {
        1 | 2 | 7 | 8 => {
            pointer_increment = 4;
            for i in 0..3 {
                let val = codes[(instruction_pointer + i + 1) as usize];
                let parameter_mode = ParameterMode::new(get_digit(input / 100, i as u32));
                parameters[i as usize].mode = parameter_mode;
                parameters[i as usize].data = val;
            }
        }
        3 | 4 => {
            pointer_increment = 2;
            let val = codes[(instruction_pointer + 1) as usize];
            let parameter_mode = ParameterMode::new(get_digit(input / 100, 0_u32));
            parameters[0].mode = parameter_mode;
            parameters[0].data = val;
        }
        5 | 6 => {
            pointer_increment = 3;
            for i in 0..2 {
                let val = codes[(instruction_pointer + i + 1) as usize];
                let parameter_mode = ParameterMode::new(get_digit(input / 100, i as u32));
                parameters[i as usize].mode = parameter_mode;
                parameters[i as usize].data = val;
            }
        }
        _ => exit(&format!("Invalid opcode: {}", opcode)),
    };

    Instruction {
        opcode,
        pointer_increment,
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
        Err(_) => exit("Malformed input"),
    }
}

// Gets the nth digit starting from the right, 0-indexed
fn get_digit(number: i32, n: u32) -> i32 {
    number % 10_i32.pow(n + 1) / 10_i32.pow(n)
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
