// Advent of Code - Day 2
use std::io::prelude::*;
use std::io::BufReader;
use std::fs::File;
use std::string::String;

fn main() {    
    // let goal = "4570637"; // part one, still scans even though I know the input
    let goal = "19690720"; // part two

    for x in 0..99 {
        for y in 0..99 {
            let mut mem: Vec<String> = Vec::new();
            load_mem(&mut mem);
            let len = mem.len();            
            mem[1] = x.to_string();
            mem[2] = y.to_string();
            for z in (0..len-1).step_by(4) {                
                run_opcode(z, &mut mem);
            }            
            if mem[0] == goal {
                println!("Inputs {}, {} output {}", x, y, mem[0]);            
                return ()
            }
        }
    }
    // println!("{:?}", mem);
}

fn run_opcode(i: usize, mut m: &mut Vec<String>) {
    // copy the opcode to prevent write collision
    let oc = m[i].parse::<i32>().unwrap();
    let i1 = m[i+1].parse::<usize>().unwrap();
    let i2 = m[i+2].parse::<usize>().unwrap();
    let i3 = m[i+3].parse::<usize>().unwrap();
    match oc {
        // Match a single value
        1 => add(i1, i2, i3, &mut m),
        // Match several values
        2 => multiply(i1, i2, i3, &mut m),
        // Match an inclusive range
        99 => return (),
        // Handle the rest of cases
        _ => panic!("Weird opcode '{}' at block: {}", oc, i/4),
    }
}

fn add(i1: usize, i2: usize, i3: usize, m: &mut Vec<String>) {
    let v1 = m[i1].parse::<usize>().unwrap();
    let v2 = m[i2].parse::<usize>().unwrap();  
    let val = (v1 + v2).to_string();
    println!("ADD: [{}]:{} + [{}]:{} = [{}]:{}", i1, v1, i2, v2, i3, val);
    m[i3] = val;
}

fn multiply(i1: usize, i2: usize, i3: usize, m: &mut Vec<String>) {
    let v1 = m[i1].parse::<usize>().unwrap();
    let v2 = m[i2].parse::<usize>().unwrap();  
    let val = (v1 * v2).to_string();
    println!("MULTIPLY: [{}]:{} * [{}]:{} = [{}]:{}", i1, v1, i2, v2, i3, val);
    m[i3] = val;
}

fn load_mem(m: &mut Vec<String>) {
    for line_r in read_input().split(b',') {
        let line = line_r.unwrap();
        m.push(String::from_utf8_lossy(&line).to_string());
    }
}

fn read_input() -> BufReader<std::fs::File> {
    let file_in = File::open("./etc/two.txt").unwrap();
    let file_reader = BufReader::new(file_in);
    return file_reader
}
