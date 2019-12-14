// Advent of Code - Day 6 part 1

use std::io::prelude::*;
use std::io::BufReader;
use std::fs::File;
use std::string::String;
use std::collections::BTreeMap;
use std::fmt;

// Objects are Vertexs in our graph
// key: is the unique 3 char string for the vertex
// distance: relative to the vertex and calculated to a source in part 2
// orbit: is the type of orbit for this vertex relative to the item in our a-list
struct Object {
    key: String,
    distance: i32, // -1 is our null, greater than -1 is a true distance to a source
    orbit: Orbit,
}

impl fmt::Debug for Object {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{:?}:{}(D:{})", self.orbit, self.key, self.distance)
    }
}

// Two types of orbits
#[derive(PartialEq)]
enum Orbit {
    Parent,
    Child
}

impl fmt::Debug for Orbit {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Orbit::Parent => write!(f, "P"),
            Orbit::Child => write!(f, "C")
        }        
    }
}

fn main() {
    // This will be our adjacency list
    let mut alist: BTreeMap<String, Vec<Object>> = BTreeMap::new();
    // Build adjacency list
    for l in read_input() {
        let parent = &l[..3];
        let child = &l[4..];
        // Lazy init a Vec if new key
        if !alist.contains_key(parent) {
            alist.insert(parent.to_string(), vec![]);
        }
        // Lazy init a Vec if new key
        // I am creating items in the list even
        // if they might not have an orbit
        if !alist.contains_key(child) {
            alist.insert(child.to_string(), vec![]);
        }

        // Add child to parent list in a-list
        let pv = alist.get_mut(parent).unwrap();
        let co = Object{key: child.to_string(), distance: -1, orbit: Orbit::Child};
        pv.push(co);

        // Add parent to child list in a-list
        let cv = alist.get_mut(child).unwrap();
        let po = Object{key: parent.to_string(), distance: -1, orbit: Orbit::Parent};
        cv.push(po);        
    };

    // debug print a-list
    for (k, v) in &alist {
        println!("{:?}, {:?}", k, v);
    };


    // Part onealist
    // "What is the total number of diralist and indirect orbits in your map data?"
    let count = count_total_orbits(&alist);    
    println!("Total # of objects: {}", alist.len());
    println!("Total # of orbits (direct & indirect): {}", count);

    // Part two
    // "What is the minimum number of orbital transfers required to move
    //  from the object YOU are orbiting to the object SAN is orbiting?"
    
    // // This is our spaceship object
    // let source: String = "YOU".to_string();
    // // This is Santa's object. We want to find a path to
    // // an orbit of who he is directly orbitting.
    // let peer_object = "SAN".to_string();
    // // Lets find who is he orbitting form our a-list
    // // let destination: String = "SAN";
}

// Walk a-list and count direct and indirect from each object as a parent.
// Loop for keys
fn count_total_orbits(o: &BTreeMap<String, Vec<Object>>) -> u32 {
    let mut count: u32 = 0;
    for (_, c) in o.iter() {
        if c.len() > 0  {    
            count+=count_orbit(c, o);
        };
    };
    count
}

// This fn chases objects in a vec back into our a-list until it reaches on without an orbit.
// It counts all the orbits on the way down (child only) and returns the sum from the walk.
fn count_orbit(v: &Vec<Object>, o: &BTreeMap<String, Vec<Object>>) -> u32 {
    let mut count: u32 = 0;
    for c in v {
        // Only count child direct orbits
        if c.orbit == Orbit::Child {
            count+=1;            
            if o[&c.key].len() > 0 {
                count+=count_orbit(&o[&c.key], o);
            };
        };
    };
    count
}

fn read_input() -> Vec<String> {
    let file_in = File::open("./etc/six.txt").unwrap();    
    let l: Vec<String> = BufReader::new(file_in).lines().map(|l| l.unwrap()).collect();
    l
}