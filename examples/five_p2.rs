// Advent of Code - Day 5 part 1
extern crate num_digitize;

use num_digitize::ToDigits;
use std::io::prelude::*;
use std::io::BufReader;
use std::fs::File;
use std::string::String;
use std::io;

#[derive(Debug)]
struct IntcodeMemory {
    ints: Vec<i32>,
    cursor: usize
}

#[derive(Debug)]
struct OpcodeParamMode {
    command: Command,
    pmode1: ParameterMode,
    pmode2: ParameterMode,
    pmode3: ParameterMode
}

impl OpcodeParamMode {
    
}

#[derive(Debug)]
enum Command {
    Add,            // add two values and Input at an int location
    Multiply,       // multiply two values and store at an int location
    Input,          // Take input and store at an int location
    Output,         // output one value
    JumpIfTrue,     // Jump to instruction pointer value if a value is true
    JumpIfFalse,    // Jump to instruction pointer value if a value is false
    LessThan,       // Store value if one value is less than another
    Equals,         // Store value if one value equal to another
    Halt            // halt program
}

#[derive(Debug)]
enum ParameterMode {
    Position,
    Immediate
}

impl ParameterMode {
    fn match_param_mode(int: i32) -> ParameterMode {
        match int {
            0 => ParameterMode::Position,
            1 => ParameterMode::Immediate,
            _ => panic!("Unknown param mode {}", int)
        }
    }
}

impl Command {

    fn match_opcode(int: i32) -> Command {
        match int {
            1 => Command::Add,
            2 => Command::Multiply,
            3 => Command::Input,
            4 => Command::Output,
            5 => Command::JumpIfTrue,
            6 => Command::JumpIfFalse,
            7 => Command::LessThan,
            8 => Command::Equals,
            99 => Command::Halt,
            _ => panic!("Unknown opcode {}", int)
        }
    }

    fn param_count(&self) -> usize {
        match self {
            Command::Add => 3,
            Command::Multiply => 3,
            Command::Input => 1,
            Command::Output => 1,
            Command::JumpIfTrue => 2,
            Command::JumpIfFalse => 2,
            Command::LessThan => 3,
            Command::Equals => 3,
            Command::Halt => 0
        }
    }
}

impl IntcodeMemory {

    fn  new() -> IntcodeMemory {
        IntcodeMemory{ints: read_input_to_ints(), cursor: 0}
    }

    fn move_cursor(&mut self, i: usize) {
        self.cursor= i;
    }

    fn next_by(&mut self, i: usize) {
        self.cursor+=i;
    }

    fn read_offset(&self, incr: usize) -> i32 {
        self.ints[self.cursor+incr]
    }

    fn read_int(&self, incr: usize) -> i32 {
        self.ints[incr]
    }

    fn write_int(&mut self, i: usize, v: i32) {
        self.ints[i] = v;
    }

    fn len(&self) -> usize {
        self.ints.len()
    }

    fn exec(&mut self, opm: OpcodeParamMode, params: Vec<i32>) -> i32 {
        // Run commands
        match opm.command {
            Command::Add => {
                let v1 = self.pmode_read(opm.pmode1, params[0]);
                let v2 = self.pmode_read(opm.pmode2, params[1]);
                let i = params[2] as usize;
                // println!("*  Adding {} to {} = {}, storing at {}", v1, v2, v1+v2, i);
                self.write_int(i, v1+v2);
            },       
            Command::Multiply => {
                let v1 = self.pmode_read(opm.pmode1, params[0]);
                let v2 = self.pmode_read(opm.pmode2, params[1]);
                let i = params[2] as usize;
                // println!("*  Multiplying {} to {} = {}, storing at {}", v1, v2, v1*v2, i);
                self.write_int(i, v1*v2);
            },
            Command::Input => {                
                println!("Please provide input: ");
                let i = params[0] as usize;
                let mut input = String::new();
                match io::stdin().read_line(&mut input) {
                    Ok(_n) => {}
                    Err(error) => println!("error: {}", error),
                }
                let v = match input.trim().parse::<i32>() {
                    Ok(i) => i,
                    Err(e) => panic!("Cannot parse '{}' into an int - {}", input, e),
                    
                };
                // println!("*  Storing input {} at {}", v, i);
                self.write_int(i, v);
            },
            Command::Output => {                
                let v = self.pmode_read(opm.pmode1, params[0]);                
                println!("Cursor[{}] ****** OUTPUT [{}] *******", self.cursor, v);
            },
            Command::JumpIfTrue => {
                let v1 = self.pmode_read(opm.pmode1, params[0]);
                let v2 = self.pmode_read(opm.pmode2, params[1]);
                match v1 {
                    0 => {
                        // println!("*  JumpIfTrue is not true (no jump)");
                        return 0
                    },
                    _ => {
                        // println!("*  JumpIfTrue is true [{}] jumping to [{}]", v1, v2);
                        return v2
                    }
                }
            },
            Command::JumpIfFalse => {
                let v1 = self.pmode_read(opm.pmode1, params[0]);
                let v2 = self.pmode_read(opm.pmode2, params[1]);
                match v1 {
                    0 => {
                        // println!("*  JumpIfFalse is true [{}] jumping to [{}]", v1, v2);
                        return v2
                    },
                    _ => {
                        // println!("*  JumpIfFalse is not false [{}] (no jump)", v1);
                        return 0
                    },
                }
            },
            Command::LessThan => {                
                let v1 = self.pmode_read(opm.pmode1, params[0]);
                let v2 = self.pmode_read(opm.pmode2, params[1]);
                let i = params[2] as usize;
                if v1 < v2 {    
                    // println!("*  {} is less than {}, storing 1 in {}", v1, v2, i);
                    self.write_int(i, 1);
                } else {
                    // println!("*  {} is not less than {}, storing 0 in {}", v1, v2, i);
                    self.write_int(i, 0);
                }
            },
            Command::Equals => {
                let v1 = self.pmode_read(opm.pmode1, params[0]);
                let v2 = self.pmode_read(opm.pmode2, params[1]);
                let i = params[2] as usize;
                if v1 == v2 {    
                    // println!("*  {} equals {}, storing 1 in {}", v1, v2, i);
                    self.write_int(i, 1);
                } else {
                    // println!("*  {} does not equal {}, storing 0 in {}", v1, v2, i);
                    self.write_int(i, 0);
                }
            },
            Command::Halt => {
                // println!("*  Halting");
                return 1
            }
        }
        return 0
    }

    fn pmode_read(&self, pm: ParameterMode, p: i32) -> i32 {        
        match pm {
            ParameterMode::Position => {                
                let i = p as usize;
                let v = self.read_int(i);
                // println!(" * Read '{}' at {}", v, i);
                v
            },
            ParameterMode::Immediate => {
                // println!(" * Returning immediate '{}'", p);
                p
            },
        }
    }

}

fn main() {
    // Our sequence of ints
    let mut mem = IntcodeMemory::new();
    // compute(mem);

    // eval at cursor, retrieve command type and parameter mode
    while mem.cursor < mem.len() {
        // println!("[{}]", mem.cursor); 
        let opm = eval_opcode(mem.read_int(mem.cursor));
        // println!("Cursor[{}] - OpPm[{:?}]", mem.cursor, opm);
        let mut params: Vec<i32> = Vec::new();
        let pc = opm.command.param_count();
        // println!(" -- command parameter count: {}", pc);

        if pc > 0 {
            // >=1
            for x in 1..pc+1 {
                params.push(mem.read_offset(x));
            }
        } else {
            // 0 params            
        }        
        // println!(" -- params to pass: {:?}", params);
        // pass parameters and execute command
        // return code determines where the instruction pointer goes next or if we halt
        let r = mem.exec(opm, params) as usize;
        match r {
            0 => mem.next_by(pc+1),
            1 => break,
            _ => mem.move_cursor(r)
        };               
    };
    
}

fn eval_opcode(int: i32) -> OpcodeParamMode {
    let mut y: Vec<i32> = int.to_digits().iter().map(|i| i32::from(*i)).collect();
    // Ensure the vec is five elements
    while y.len() < 5 {
        y.insert(0, 0);
    }

    OpcodeParamMode{
        command: Command::match_opcode(y[4]+(y[3]*10)),
        pmode1: ParameterMode::match_param_mode(y[2]),
        pmode2: ParameterMode::match_param_mode(y[1]),
        pmode3: ParameterMode::match_param_mode(y[0]),
    }
}

fn read_input_to_ints() -> Vec<i32> {
    let file_in = File::open("./etc/five.txt").unwrap();    
    let l: Vec<String> = BufReader::new(file_in).lines().map(|l| l.unwrap()).collect();
    let mut ints: Vec<i32> =  Vec::new();
    for o in l[0].split(",") {
        let i = match o.parse::<i32>() {
            Ok(i) => i,
            Err(e) => panic!(e),
        };
        ints.push(i);        
    }
    ints
}