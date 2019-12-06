// Advent of Code - Day 3
use std::io::prelude::*;
use std::io::BufReader;
use std::fs::File;
use std::string::String;
use std::fmt;
use std::collections::HashSet;

enum Direction {
    Up,
    Down,
    Left,
    Right
}

struct Wire {
    // a Vec of Segments sorted ascending by their leftmost coordinate 
    segments: Vec<Segment>,
}

// Segment represents a linear part of a wire that is either horizonal or vertical of a set length.
// x1,y1 to x2,y2 represent the coordinates of this line.
// Direction represents the direction where Up/Down == vertical segments and Left/Right == horizontal.
struct Segment {
    x1: i32,
    x2: i32,
    y1: i32,
    y2: i32,
    direction: Direction,
    distance: i32,
}

impl Segment {

    fn new(x1: i32, y1: i32, x2: i32, y2: i32, direction: Direction, distance: i32) -> Segment {  
        Segment{x1: x1, y1: y1, x2: x2, y2: y2, direction: direction, distance: distance}
    }

}

impl fmt::Display for Direction {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let d = match self {
            Direction::Up => "Up",
            Direction::Down => "Down",
            Direction::Left => "Left",
            Direction::Right => "Right",
        };
        write!(f, "{}", d)
    }
}

impl fmt::Display for Segment {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{},{} - {},{}, {}->{}", self.x1, self.y1, self.x2, self.y2, self.direction, self.distance)
    }
}

impl Wire {
    fn num(&self) -> usize {
        self.segments.len()
    }

    fn new(v: &String) -> Wire {
        // Set the origin port coordinates. This will move to the end of each segment.
        let mut x = 0;
        let mut y = 0;
        let mut segments: Vec<Segment> = Vec::new(); 
        let segments_s: Vec<String> = v.split(",").map(|x| x.to_string()).collect();

        for seg_s in segments_s {
            let spl = seg_s.split_at(1);
            let direction = dir_string_to_enum(spl.0);
            let distance = spl.1.parse::<i32>().unwrap();
            // Match on the direction and create the coordinates for the segment based on the current end of the wire.
            // nx, ny contain the new end of wire based on the direction.
            let (x1, y1, x2, y2, nx, ny) = match direction {
                Direction::Up => {                                                                       
                    (x,y-distance,x,y,x,y-distance)
                }
                Direction::Down => {
                    (x,y,x,y+distance,x,y+distance)
                }
                Direction::Left => {
                    (x-distance,y,x,y,x-distance,y)
                }
                Direction::Right => {
                    (x,y,x+distance,y,x+distance,y)
                }             
            };       
            // Update our end of wire coordinates.
            x = nx;
            y = ny;     
            // Create a new segment and add to the Vec
            let seg = Segment::new(x1, y1, x2, y2, direction, distance);                                    
            segments.push(seg);
        };

        // Return the Wire with the Segments
        Wire{segments: segments}           
    }
}

fn main() {    
    // Create a vec
    let wires_input: Vec<String>  = read_input().lines().map(|x| x.unwrap()).collect();
    
    // Create our Wires
    let wire1 = Wire::new(&wires_input[0]);
    println!("Segment count: {}", wire1.num());
    let wire2 = Wire::new(&wires_input[1]);
    println!("Segment count: {}", wire2.num());

    // Create hashset to build unique X value range
    let mut xv = HashSet::new();
    for s in wire1.segments {
        xv.insert(s.x1);
        xv.insert(s.x2);
    };
    for s in wire2.segments {
        xv.insert(s.x1);
        xv.insert(s.x2);
    };
    // Convert hashset into sorted Vec of x values
    let mut x: Vec<i32> = xv.into_iter().collect();    
    x.sort();

    // test scan of lines by x
    for scanline_x in &x {
        println!("Scanline cursor at: {}", scanline_x);
    };
}

fn read_input() -> BufReader<std::fs::File> {
    let file_in = File::open("./etc/three.txt").unwrap();
    let file_reader = BufReader::new(file_in);
    return file_reader
}

fn dir_string_to_enum(dir: &str) -> Direction {
    match dir {
        "U" => Direction::Up,
        "D" => Direction::Down,
        "L" => Direction::Left,
        "R" => Direction::Right,
        _ => panic!("Unknown direction: {}", dir),
    }
}