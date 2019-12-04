// Advent of Code - Day 1
use std::io::prelude::*;
use std::io::{self, BufReader};
use std::fs::File;

type Mass = f32;
type Fuel = f32;

fn main() {    
    let (total_mass, total_fuel) = read_input()
        .map(calc_totals)
        .unwrap();

    println!("Total mass: {}", total_mass);
    println!("Total fuel: {}", total_fuel);
}

fn read_input() -> io::Result<BufReader<File>> {
    let file_in = File::open("./etc/one.txt")?;
    Ok(BufReader::new(file_in)) // last argument is return value
}

fn calc_totals(input: BufReader<File>) -> (Mass, Fuel) {
    input
        .lines()
        .map(|line_r| {
            let line = line_r.unwrap();
            let mass = line.parse::<f32>().unwrap_or(0.0); // if we can't parse, assume 0
            (mass, calc_fuel(mass))
        })
        .fold((0.0, 0.0), |acc, curr| {
            let (t_mass, t_fuel) = acc;
            let (c_mass, c_fuel) = curr;
            (t_mass + c_mass, t_fuel + c_fuel)
        })
}


// calc fuel for mass
fn calc_fuel(val: f32) -> f32 {
    let fuel = (val / 3.0).floor() - 2.0;
    // prevent a negative return
    if fuel < 0.0 {
        0.0
    } else {
        // turtles are the way down
       fuel + calc_fuel(fuel)
    }
}