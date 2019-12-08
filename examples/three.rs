// Advent of Code - Day 3
use std::io::prelude::*;
use std::io::BufReader;
use std::fs::File;
use std::string::String;
use std::fmt;

#[derive(PartialEq)]
#[derive(Clone)]
#[derive(Copy)] 
#[derive(Debug)] 
enum Direction {
    Up,
    Down,
    Left,
    Right
}

// struct Wire {
//     // a Vec of Segments sorted ascending by their leftmost coordinate 
//     segments: Vec<Segment>,
// }

// Segment represents a linear part of a wire that is either horizonal or vertical of a set length.
// x1,y1 to x2,y2 represent the coordinates of this line.
// Direction represents the direction where Up/Down == vertical segments and Left/Right == horizontal.
#[derive(PartialEq)]
#[derive(Clone)]
#[derive(Copy)] 
struct Segment {
    x1: i32,
    x2: i32,
    y1: i32,
    y2: i32,
    direction: Direction,
    distance: i32,
    total_distance: i32,
    wireid: usize,
}

#[derive(Debug)] 
struct Intersection {
    x: i32,
    y: i32,
    distance: i32,
}

impl Intersection {

    fn mdist(&self) -> i32 {
        (self.x.abs())+(self.y.abs())
    }

}



impl Segment {

    fn new(x1: i32, y1: i32, x2: i32, y2: i32, direction: Direction, distance: i32, total_distance: i32, wireid: usize) -> Segment {  
        Segment{x1: x1, y1: y1, x2: x2, y2: y2, direction: direction, distance: distance, total_distance: total_distance, wireid: wireid}
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
        write!(f, "{},{} - {},{}, {} Total Dist:{} Seg Dist:{} WIRE:{}", self.x1, self.y1, self.x2, self.y2, self.direction, self.total_distance, self.distance, self.wireid)
    }
}

impl fmt::Debug for Segment {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{},{} - {},{}, {} Total Dist:{} Seg Dist:{} WIRE:{}", self.x1, self.y1, self.x2, self.y2, self.direction, self.total_distance, self.distance, self.wireid)
    }
}

fn main() {    
    // Create a vec
    let wires_input: Vec<String>  = read_input().lines().map(|x| x.unwrap()).collect();
    
    // Set the origin port coordinates. This will move to the end of each segment.    
    let mut segments: Vec<Segment> = Vec::new(); 

    for (i, wire) in wires_input.iter().enumerate() {
        let mut x = 0; 
        let mut y = 0;
        let mut t_distance = 0;
        let segments_s: Vec<String> = wire.split(",").map(|x| x.to_string()).collect();
        for seg_s in segments_s {
            let spl = seg_s.split_at(1);
            let direction = dir_string_to_enum(spl.0);
            let distance = spl.1.parse::<i32>().unwrap();
            // Match on the direction and create the coordinates for the segment based on the current end of the wire.
            // nx, ny contain the new end of wire based on the direction.
            // This also makes sure x1 and y1 are the lowest values in the segment whcih supports the sort for segments later.
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
            // Add to total distance
            t_distance+=distance;
            // Create a new segment and add to the Vec
            let seg = Segment::new(x1, y1, x2, y2, direction, distance, t_distance, i);                                    
            segments.push(seg);
        };
    };

    // Sort segments by x1
    segments.sort_by(|s1, s2| s2.x1.cmp(&s1.x1));
    segments.reverse(); 

    // Build a sweep range out of the X1 values
    let mut sweeprange: Vec<i32> = segments.iter().map(|s| s.x1).collect();    
    sweeprange.dedup();

    let mut set_segment: Vec<Segment> = Vec::new();
    let mut intersections: Vec<Intersection> = Vec::new();    
    
    for sweepline in sweeprange {
        
        // Move the any segments that are in the sweep line into the set
        // This checks from the beginning of segments until one does not match        
        let mut done = false;        
        while !done {                
            let f = segments.first();
            match f {
                Some(f) => {
                    if f.x1 <= sweepline && f.x2 >= sweepline {                
                        set_segment.push(segments.remove(0));
                    } else {
                        done = true;
                    }
                },
                None => done = true,
            }
            
        }

        // Remove any segments that are in the set but no longer in the sweep line
        set_segment.retain(|s| s.x1 <= sweepline && s.x2 >= sweepline );

        // uncomment to print the set
        // println!("At sweep line: x:{}", sweepline);
        // for (i, s) in set_segment.iter().enumerate() {
        //     println!("   - segment[{}] - {}", i, s);
        // }

        // Evaluate if any segemnts in the set intersect and if they are from different wires
        // Loop over each segment in the set and compare to any segment in the set that is from a different wire
        //      if the segments intersect, then add to the intersection set
        let mut w1 = set_segment.to_vec();
        w1.retain(|s| s.wireid == 0 );
        let mut w2 = set_segment.to_vec();
        w2.retain(|s| s.wireid == 1 );

        if !w1.is_empty() && !w2.is_empty() {
            for s1 in &w1 {
                // println!("Checking intersection:");
                for s2 in &w2 {
                    if s1.wireid != s2.wireid {
                        // println!("Checking intersection on 1:{},{} AND 2:{},{}", s1.x1, s1.x2, s2.x1, s2.x2);
                        // Vertical line
                        if s1.direction == Direction::Up || s1.direction == Direction::Down {                            
                            if s2.direction == Direction::Left || s2.direction == Direction::Right {
                                if (s2.y1 >= s1.y1 && s2.y1 <= s1.y2) && (s1.x1 >= s2.x1 && s1.x1 <= s2.x2) {
                                    let x = s1.x1;
                                    let y = s2.y1;
                                    // println!("      S1 Vertical line {}", s1);
                                    // println!("              S2 Horizontal line {}", s2);
                                    // println!("                  INTERSECTION: {},{}", s1.x1, s2.y1);                                       
                                    
                                    let mut idist = s1.total_distance + s2.total_distance;
                                    // println!(" --- total distance: {}", idist);

                                    // println!(" --- s1: y1:{} y2:{} y:{}", s1.y1, s1.y2, y);                                                                   
                                    let d1 = match s1.direction {
                                        Direction::Up => (s1.y1 - y).abs(),
                                        Direction::Down => (s1.y2 - y).abs(),
                                        _ => panic!("not possible"),
                                    };
                                    // println!(" --- extra s1 steps delta: {}", d1);
                                    // println!(" --- s2: x1:{} x2:{} x:{}", s2.x1, s2.x2, x);
                                    let d2 = match s2.direction {
                                        Direction::Left => (s2.x1 - x).abs(),
                                        Direction::Right => (s2.x2 - x).abs(),
                                        _ => panic!("not possible"),
                                    };
                                    // println!(" --- extra s2 steps delta: {}", d2);
                                    idist -= d1+d2;
                                    // println!(" --- total distance: {}", idist);
                                    intersections.push(Intersection{x: x, y: y, distance: idist});
                                }
                            };

                        // Horizontal line
                        } else {
                            // println!("      S1 Horizontal line {}", s1);
                            if s2.direction == Direction::Up || s2.direction == Direction::Down {
                                if (s1.y1 >= s2.y1 && s1.y1 <= s2.y2) && (s2.x1 >= s1.x1 && s2.x1 <= s1.x2) {
                                    let x = s2.x1;
                                    let y = s1.y1;
                                    // println!("      S1 Horizontal line {}", s1);
                                    // println!("              S2 Vertical line {}", s2);
                                    // println!("                  INTERSECTION: {},{}", x, y);                        
                                    
                                    let mut idist = s1.total_distance + s2.total_distance;
                                    // println!(" --- total distance: {}", idist);
                                    // println!(" --- s2: y1:{} y2:{} y:{}", s2.y1, s2.y2, y);                                                                  
                                    let d2 = match s2.direction {
                                        Direction::Up => (s2.y1 - y).abs(),
                                        Direction::Down => (s2.y2 - y).abs(),
                                        _ => panic!("not possible"),
                                    };
                                    // println!(" --- extra s1 steps delta: {}", d2);
                                    // println!(" --- s1: x1:{} x2:{} x:{}", s1.x1, s1.x2, x);
                                    let d1 = match s1.direction {
                                        Direction::Left => (s1.x1 - x).abs(),
                                        Direction::Right => (s1.x2 - x).abs(),
                                        _ => panic!("not possible"),
                                    };
                                    // println!(" --- extra s1 steps delta: {}", d1);
                                    idist -= d1+d2;
                                    // println!(" --- total distance: {}", idist);                                                      
                                    intersections.push(Intersection{x: x, y: y, distance: idist});                            
                                }
                            };
                        };
                    }
                }            
            }
        }
    };

    // Uncomment for sorting by manhattan
    // intersections.sort_by(|i1, i2| i1.mdist().cmp(&i2.mdist()));
    // if intersections[0].x == 0 && intersections[0].y == 0 {
    //     intersections.remove(0);
    // }
    // println!("Intersections sorted by lowest manhattan:");
    // for i in intersections {
    //     println!("  {:?} -> MDist: {}", i, i.mdist());
    // };

    // Uncomment for sorting by distance
    intersections.sort_by(|i1, i2| i1.distance.cmp(&i2.distance));
    if intersections[0].x == 0 && intersections[0].y == 0 {
        intersections.remove(0);
    }
    println!("Intersections sorted by lowest travel:");
    for i in intersections {
        println!("  {:?} -> MDist: {}", i, i.mdist());
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