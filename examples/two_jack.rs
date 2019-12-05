// Advent of Code - Day 2
use std::io::prelude::*;
use std::io::BufReader;
use std::fs::File;
use std::string::String;
use std::iter::Iterator;

fn main() {    
    let operations = load_operations(read_input());
    let mut op_iterator = operations.iter()
        .cloned()
        .cycle();

    let total = (0..99)
        .fold(0.0, |acc, num| {
            let operation = op_iterator.next().unwrap_or(Operation::NoOp);
            let result = operation.op(acc, num as f64);
            println!("{} {:?} {} = {}", acc, operation, num, result);
            result
        });

    println!("total: {}", total);
}

fn load_operations(reader: BufReader<File>) -> Vec<Operation> {
    reader
        .split(b',')
        .map(|line_result| {
            line_result
                .map(|line| {
                    let op_number = String::from_utf8_lossy(&line)
                        .to_string()
                        .parse::<usize>()
                        .unwrap_or(0);
                    Operation::from_usize(op_number)
                })
                .unwrap_or(Operation::NoOp)
        })
        .collect()
}

fn read_input() -> BufReader<File> {
    let file_in = File::open("./etc/two.txt").unwrap();
    let file_reader = BufReader::new(file_in);
    return file_reader
}

#[derive(Debug, Clone)]
enum Operation {
    Add,
    Multiply,
    NoOp
}

impl Operation {

    fn from_usize(u: usize) -> Self { 
        match u {
            1 => Self::Add,
            2 => Self::Multiply,
            _ => Self::NoOp
        }
    }

    fn op(&self, x: f64, y: f64) -> f64 {
        match self {
            Self::Add => x + y,
            Self::Multiply => x * y,
            Self::NoOp => x,
        }
    }
}
