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
    Add,        // add two values and Input at an int location
    Multiply,   // multiply two values and store at an int location
    Input,      // Take input and store at an int location
    Output,     // output one value
    Halt        // halt program
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
            99 => Command::Halt,
            _ => panic!("Unknown opcode {}", int)
        }
    }

    fn param_count(&self) -> i32 {
        match self {
            Command::Add => 3,
            Command::Multiply => 3,
            Command::Input => 1,
            Command::Output => 1,
            Command::Halt => 0
        }
    }
}

impl IntcodeMemory {

    fn  new() -> IntcodeMemory {
        IntcodeMemory{ints: read_input_to_ints(), cursor: 0}
    }

    fn next(&mut self) -> i32 {
        self.cursor+=1;
        self.read()
    }

    fn read(&self) -> i32 {
        self.ints[self.cursor]
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

    fn exec(&mut self, opm: OpcodeParamMode, params: Vec<i32>) -> usize {
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
        let opm = eval_opcode(mem.read_int(mem.cursor));
        // println!("Cursor[{}] - OpPm[{:?}]", mem.cursor, opm);
        let mut params: Vec<i32> = Vec::new();
        let pc = opm.command.param_count();
        // println!(" -- command parameter count: {}", pc);

        if pc > 0 {
            // >=1
            for _x in 0..pc {
                params.push(mem.next());
            }
        } else {
            // 0 params            
        }        
        // println!(" -- params to pass: {:?}", params);
        // pass parameters and execute command
        if mem.exec(opm, params) == 1 {
            break;
        }

        // Move to the next opcode        
        mem.next();
        // println!(" Current cursor: {}", mem.read());        
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