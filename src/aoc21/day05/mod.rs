use std::fs;
use array2d::Array2D;
use regex::Regex;
use std::cmp::{max,min,Ordering};

pub fn day5_part1() {
    let MAX_DIM = 1000;

    let file_string = fs::read_to_string("day5/input.txt").expect("File not found");
    let lines = file_string.lines();

    let to_usize = |x: &str| x.parse::<usize>().unwrap();

    let re = Regex::new(r"^(\d+),(\d+) -> (\d+),(\d+)$").unwrap();
    let mut ocean_floor: Array2D<i32> = Array2D::filled_with(0, MAX_DIM, MAX_DIM);

    for line in lines {
        let caps = re.captures(line).unwrap();
        let (x0, y0, x1, y1) = (to_usize(&caps[1]), to_usize(&caps[2]), to_usize(&caps[3]), to_usize(&caps[4]));
        //println!("{:?}", caps);

        if x0 == x1 {
            for y in min(y0, y1)..=max(y0,y1) {
                ocean_floor[(x0, y)] += 1;
            }
        } else if y0 == y1 {
            for x in min(x0,x1)..=max(x0,x1) {
                ocean_floor[(x, y0)] += 1;
            }
        }
    }

    let ans = ocean_floor.elements_row_major_iter().filter(|x| **x >= 2).count();
    println!("{:?}", ans);
    
    
}


pub fn day5_part2() {
    let MAX_DIM = 1000;

    let file_string = fs::read_to_string("day5/input.txt").expect("File not found");
    let lines = file_string.lines();

    let to_usize = |x: &str| x.parse::<usize>().unwrap();

    let re = Regex::new(r"^(\d+),(\d+) -> (\d+),(\d+)$").unwrap();
    let mut ocean_floor: Array2D<i32> = Array2D::filled_with(0, MAX_DIM, MAX_DIM);

    for line in lines {
        let caps = re.captures(line).unwrap();
        let (mut x0, mut y0, x1, y1) = (to_usize(&caps[1]), to_usize(&caps[2]), to_usize(&caps[3]), to_usize(&caps[4]));
        //println!("{:?}", caps);

        if x0 == x1 {
            for y in min(y0, y1)..=max(y0,y1) {
                ocean_floor[(x0, y)] += 1;
            }
        } else if y0 == y1 {
            for x in min(x0,x1)..=max(x0,x1) {
                ocean_floor[(x, y0)] += 1;
            }
        } else {
            let dir = (
                match x0.cmp(&x1) {
                    Ordering::Less => 1,
                    _ => -1
                },
                match y0.cmp(&y1) {
                    Ordering::Less => 1,
                    _ => -1
                });
            while x0 != x1 && y0 != y1 {
                ocean_floor[(x0, y0)] += 1;
                x0 = (x0 as i32 + dir.0) as usize;
                y0 = (y0 as i32 + dir.1) as usize;
            }
            ocean_floor[(x0, y0)] += 1;
        }
    }

    let ans = ocean_floor.elements_row_major_iter().filter(|x| **x >= 2).count();
    println!("{:?}", ans); 
}