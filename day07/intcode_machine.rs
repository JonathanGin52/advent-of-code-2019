use std::io;

pub struct IntcodeMachine {
    tape: Vec<i32>,
    instruction_pointer: usize,
    input: Vec<i32>,
    output: Vec<i32>,
}

impl IntcodeMachine {
    pub fn new(tape: Vec<i32>) -> IntcodeMachine {
        IntcodeMachine {
            tape,
            instruction_pointer: 0,
            input: Vec::new(),
            output: Vec::new(),
        }
    }

    fn next_instruction(&self, instruction_pointer: usize) -> Instruction {
        let input = self.tape[instruction_pointer];
        let opcode = input % 100;
        let mut pointer_increment: usize = 0;
        let mut parameters: [Parameter; 3] = Default::default();

        let mut parse_instruction = |&instruction_size| {
            pointer_increment = instruction_size;
            for i in 0..(instruction_size - 1) {
                let val = self.tape[(instruction_pointer + i + 1)];
                let parameter_mode = ParameterMode::new(Self::get_digit(input / 100, i as u32));
                parameters[i].mode = parameter_mode;
                parameters[i].data = val;
            }
        };

        match opcode {
            1 | 2 | 7 | 8 => parse_instruction(&4_usize),
            3 | 4 => parse_instruction(&2_usize),
            5 | 6 => parse_instruction(&3_usize),
            _ => exit(&format!("Invalid opcode: {}", opcode)),
        };

        Instruction {
            opcode,
            pointer_increment,
            parameters,
        }
    }

    fn move_instruction_pointer(&mut self, delta: usize) {
        self.instruction_pointer += delta;
    }

    // Gets the nth digit starting from the right, 0-indexed
    fn get_digit(number: i32, n: u32) -> i32 {
        number % 10_i32.pow(n + 1) / 10_i32.pow(n)
    }
}
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

#[derive(Debug)]
enum InstructionResult {
    Pending,
    Success,
    Halt,
}

#[derive(Debug)]
struct Instruction {
    opcode: i32,
    pointer_increment: usize,
    parameters: [Parameter; 3],
}

impl Instruction {
    pub fn execute(&self, mut machine: IntcodeMachine) -> InstructionResult {
        let parameters = &self.parameters;
        match self.opcode {
            1 => {
                machine.tape[parameters[2].data as usize] =
                    self.fetch_param(&machine.tape, &parameters[0]) + self.fetch_param(&machine.tape, &parameters[1])
            }
            2 => {
                machine.tape[parameters[2].data as usize] =
                    self.fetch_param(&machine.tape, &parameters[0]) * self.fetch_param(&machine.tape, &parameters[1])
            }
            3 => machine.tape[parameters[0].data as usize] = machine.input.remove(0),
            4 => {
                let out = self.fetch_param(&machine.tape, &parameters[0]);
                machine.output.push(out);
                // println!("{}", out);
            }
            5 => {
                if self.fetch_param(&machine.tape, &parameters[0]) != 0 {
                    machine.instruction_pointer = self.fetch_param(&machine.tape, &parameters[1]) as usize;
                    return InstructionResult::Success;
                }
            }
            6 => {
                if self.fetch_param(&machine.tape, &parameters[0]) == 0 {
                    machine.instruction_pointer = self.fetch_param(&machine.tape, &parameters[1]) as usize;
                    return InstructionResult::Success;
                }
            }
            7 => {
                machine.tape[parameters[2].data as usize] =
                    (self.fetch_param(&machine.tape, &parameters[0]) < self.fetch_param(&machine.tape, &parameters[1])) as i32
            }
            8 => {
                machine.tape[parameters[2].data as usize] =
                    (self.fetch_param(&machine.tape, &parameters[0]) == self.fetch_param(&machine.tape, &parameters[1])) as i32
            }
            99 => return InstructionResult::Halt,
            _ => exit(&"Opcode not implemented: {}"),
        };
        machine.move_instruction_pointer(self.pointer_increment);
        InstructionResult::Success
    }

    fn fetch_param(&self, tape: &Vec<i32>, parameter: &Parameter) -> i32 {
        match parameter.mode {
            ParameterMode::Position => tape[parameter.data as usize],
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
}
 

fn exit(message: &str) -> ! {
    println!("{}", message);
    std::process::exit(1);
}
