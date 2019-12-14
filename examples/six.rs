// Advent of Code - Day 6 part 1

use std::io::prelude::*;
use std::io::BufReader;
use std::fs::File;
use std::string::String;
use std::collections::BTreeMap;


fn main() {
    // This will be our adjacency list
    let mut obj: BTreeMap<String, Vec<String>> = BTreeMap::new();
    // Build adjacency list
    for l in read_input() {
        let parent = &l[..3];
        let child = &l[4..];
        // Lazy init a Vec if new key
        if !obj.contains_key(parent) {
            obj.insert(parent.to_string(), vec![]);
        }
        // Lazy init a Vec if new key
        // I am creating items in the list even
        // if they might not have an orbit
        if !obj.contains_key(child) {
            obj.insert(child.to_string(), vec![]);
        }
        let v = obj.get_mut(parent).unwrap();
        v.push(child.to_string());        
    };

    // Walk a-list and count direct and indirect from each object as a parent.
    let mut count: u32 = 0;
    for (p, c) in &obj {        
        if c.len() > 0 {
            count+=count_orbit(&c, &obj);
        };
    };
    println!("Count: # of objects: {}, number of orbits (direct & indirect): {}", obj.len(), count);
}

// This fn chases objects in a vec back into our a-list until it reaches on without an orbit.
// It counts all the orbits on the way down and returns the sum from the walk.
fn count_orbit(v: &Vec<String>, o: &BTreeMap<String, Vec<String>>) -> u32 {
    let mut count: u32 = 0;
    for c in v {
        count+=1;
        if o[c].len() > 0 {
            count+=count_orbit(&o[c], o);
        };
    };
    count
}

fn read_input() -> Vec<String> {
    let file_in = File::open("./etc/six.txt").unwrap();    
    let l: Vec<String> = BufReader::new(file_in).lines().map(|l| l.unwrap()).collect();
    l
}