// Advent of Code - Day 4 part 1
use std::io::prelude::*;
// use std::io::BufReader;
// use std::fs::File;
// use std::string::String;
// use std::fmt;

fn main() {
    let range_start = 138241;
    let range_end = 674034;
    // let range_start = 1234;
    // let range_end = 2345;

    let mut works: Vec<i32> = Vec::new();
    for r in range_start..range_end {        
        if has_double(r) && no_decr(r) {
            works.push(r);
        };
    }
    println!("{:?}", works.len());
}


fn has_double(num: i32) -> bool {
    let sd: String = format!("{}", num);
    let mut lc = "0".to_owned().chars().next().unwrap();
    for c in sd.chars() {
        if lc == c {
            return true
        };
        lc = c;
    }
    false
}

fn no_decr(num: i32) -> bool {
    let s = format!("{}", num);
    let mut sd = s.chars();
    let mut lc = sd.next().unwrap();
    
    for c in sd {   
        let c1 = lc.to_digit(10);
        let c2 = c.to_digit(10);
        if c2 < c1 {
            return false
        } else {
            lc = c;
        }        
    }
    true
}