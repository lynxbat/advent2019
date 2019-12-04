// Advent of Code - Day 1
use std::io::prelude::*;
use std::io::BufReader;
use std::fs::File;

static INPUT_FILE: &str = "./input.txt";

fn main() {
    println!("{}", INPUT_FILE);
    let file_in = File::open(INPUT_FILE).unwrap();
    let file_reader = BufReader::new(file_in);

    let mut total_mass = 0.0;
    let mut total_fuel = 0.0;

    for line_r in file_reader.lines() {
        let line = line_r.unwrap();
        let mass = line.parse::<f32>().unwrap();    
        total_mass += mass;
        let fuel = calc_fuel(mass);
        total_fuel += fuel;      
    }

    println!("Total mass: {}", total_mass);
    println!("Total fuel: {}", total_fuel);
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
