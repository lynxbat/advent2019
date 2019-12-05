// Advent of Code - Day 1
use std::io::prelude::*;
use std::io::BufReader;
use std::fs::File;

fn main() {    
    let mut total_mass = 0.0;
    let mut total_fuel = 0.0;

    for line_r in read_input().lines() {
        let line = line_r.unwrap();
        let mass = line.parse::<f32>().unwrap();    
        total_mass += mass;
        let fuel = calc_fuel(mass);
        total_fuel += fuel;      
    }

    println!("Total mass: {}", total_mass);
    println!("Total fuel: {}", total_fuel);
}

fn read_input() -> BufReader<std::fs::File> {
    let file_in = File::open("./etc/one.txt").unwrap();
    let file_reader = BufReader::new(file_in);
    return file_reader
}

// calc fuel for mass
fn calc_fuel(val: f32) -> f32 {
    let mut fuel = (val / 3.0).floor()-2.0;
    // prevent a negative return
    if fuel < 0.0 {
        fuel = 0.0;
    } else {
        // turtles are the way down
        fuel = fuel + calc_fuel(fuel);
    }
    return fuel
}
