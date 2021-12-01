use std::fs;
use array2d::Array2D;
use regex::Regex;
use std::cmp::{max,min,Ordering};

pub fn day7_part1() {
    let file_string = fs::read_to_string("day7/input.txt").expect("File not found");
    let mut lines = file_string.lines();

    let to_int = |x: &str| x.parse::<i32>().unwrap();
    let mut crabs = lines.next().unwrap().split(",").map(to_int).collect::<Vec<_>>();
    crabs.sort();
    
    let mut best = i32::MAX;

    for crab_meet in crabs[0]..crabs[crabs.len()-1] {
        let curr = crabs.iter().map(|x| {let dist = (x-crab_meet).abs(); dist*(dist+1)/2 }).sum();
        if curr < best {
            best = curr;
        }
    }
    println!("{}", best);
    
}


pub fn day7_part2() {
    
}