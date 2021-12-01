use std::fs;
use array2d::Array2D;
use regex::Regex;
use std::cmp::{max,min,Ordering};

pub fn day6_part1() {
    let NUM_DAYS = 256;

    let file_string = fs::read_to_string("day6/input.txt").expect("File not found");
    let mut lines = file_string.lines();

    let to_usize = |x: &str| x.parse::<usize>().unwrap();
    let lanternfish = lines.next().unwrap().split(",").map(to_usize);
    
    let mut fish_residues = vec![0; 9];
    for lf in lanternfish {
        fish_residues[lf] += 1;
    }

    for day in 0..NUM_DAYS {
        let current_residue = day % 7;
        let new_fish = fish_residues[current_residue];
        fish_residues[current_residue] += fish_residues[7];
        fish_residues[7] = fish_residues[8];
        fish_residues[8] = new_fish;

        //println!("{:?}", fish_residues);
    }

    println!("{}", fish_residues.iter().sum::<usize>());
    
}


pub fn day6_part2() {
    
}