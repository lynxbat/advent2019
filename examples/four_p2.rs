// Advent of Code - Day 4 part 2
use std::collections::HashMap;

fn main() {
    let range_start = 138241;
    let range_end = 674034;

    let mut works: Vec<i32> = Vec::new();
    for r in range_start..range_end {            
        if has_double(r) && no_decr(r) {
            works.push(r);
        };
    }
    println!("{:?}", works.len());
}


fn has_double(num: i32) -> bool {
    let mut nh: HashMap<char, i32> = HashMap::new();
    let sd: String = format!("{}", num);
    let mut lcv: Vec<char> =  Vec::new();    
    lcv.push("0".to_owned().chars().next().unwrap());
    for c in sd.chars() {        
        let mut x = 1;
        for lc in &lcv {
            if *lc == c {
                x+=1;
            };            
        }   
        nh.insert(c, x);        
        lcv.push(c);
    }
    for (k, v) in nh {
        if v == 2 {
            return true;
        }
    };
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