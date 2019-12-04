// Advent of Code - Day 1
use std::io::prelude::*;
use std::io::{self, BufReader};
use std::fs::File;

type Mass = f32;
type Fuel = f32;

fn main() {    
    let result = read_input()
        .map(calc_totals);

    // perform pattern matching on the result
    // display the totals if it is Ok
    // exit with an error message and non zero exit code on error
    match result {
        Ok((total_mass, total_fuel)) => {
            println!("Total mass: {}", total_mass);
            println!("Total fuel: {}", total_fuel);
        }
        Err(e) => {
            eprintln!("An error occurred: {}", e);
            std::process::exit(1);
        }
    }
}

fn read_input() -> io::Result<BufReader<File>> {
    // the "?" macro is short and for return Err 
    let file_in = File::open("./etc/one.txt")?;
    // last argument is return value, since we are returning it in a Result wrap it in Ok
    Ok(BufReader::new(file_in)) 
}

fn calc_totals(input: BufReader<File>) -> (Mass, Fuel) {
    input
        .lines()
        // map is used to manipulate each entry in an iterator
        .map(|line_r| {
            let line = line_r.unwrap();
            let mass = line.parse::<f32>().unwrap_or(0.0); // if we can't parse, assume 0
            (mass, calc_fuel(mass))
        })
        // fold allows you to reduce an iterator to a single value
        // The first parameter is the initializer
        // the second paramter is a function with the accumulator (total) and the current entry
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
    // if an else return a value
    if fuel < 0.0 {
        0.0
    } else {
        // turtles are the way down
       fuel + calc_fuel(fuel)
    }
}