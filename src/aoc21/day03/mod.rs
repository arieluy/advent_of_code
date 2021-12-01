use std::fs;
use std::cmp::Ordering;

pub fn day3_part1() {
    let file_string = fs::read_to_string("day3/input.txt").expect("File not found");
    let lines = file_string.lines();

    let map_fn = |x: &str| -> Vec<i32> {
        x.chars().map(|s| s.to_digit(2).unwrap()).map(|b| if b == 0 { -1 } else { 1 }).collect()
    };
    let reduce_fn = |x: Vec<i32>, y: Vec<i32>| -> Vec<i32> {
        x.iter().zip(y.iter()).map(|xx| xx.0 + xx.1).collect()
    };
    let sum: Vec<i32> = lines.map(map_fn).reduce(reduce_fn).unwrap();

    println!("{:?}", sum);

    let fold_fn = |acc: i32, x: (usize, i32)| -> i32 {
        let (exp, val) = x;
        acc + (1 << exp) * val
    };
    let gamma: i32 = sum.clone().into_iter().map(|x| if x < 0 { 0 } else { 1 }).rev().enumerate().fold(0, fold_fn);
    let epsilon: i32 = sum.into_iter().map(|x| if x < 0 { 1 } else { 0 }).rev().enumerate().fold(0, fold_fn);

    println!("{:?}", gamma*epsilon);
}


pub fn day3_part2() {
    let file_string = fs::read_to_string("day3/input.txt").expect("File not found");

    let map_fn = |x: &str| -> Vec<i32> {
        x.chars().map(|s| s.to_digit(2).unwrap()).map(|b| if b == 0 { -1 } else { 1 }).collect()
    };
    let mut lines = file_string.lines().map(map_fn).collect::<Vec<_>>().into_iter();
    let num_bits = 12; // CHANGE ME

    fn most_common_bit<T>(lines: T, index: usize) -> i32
    where T: Iterator<Item=Vec<i32>>, {
        match lines.fold(0, |acc, x| acc + x[index]).cmp(&0) {
            Ordering::Less => -1, 
            _ => 1
        }
    }

    let mut lcb_lines = lines.clone();
    for i in 0..num_bits {
        if lines.len() == 1 {
            break;
        }
        let mcb = most_common_bit(lines.clone(), i);
        lines = lines.filter(|line| line[i] == mcb).collect::<Vec<_>>().into_iter()
    }

    for i in 0..num_bits {
        if lcb_lines.len() == 1 {
            break;
        }
        let mcb = most_common_bit(lcb_lines.clone(), i);
        let lcb = if mcb == 1 { -1 } else { 1 };
        lcb_lines = lcb_lines.filter(|line| line[i] == lcb).collect::<Vec<_>>().into_iter()
    }

    let fold_fn = |acc: i32, x: (usize, i32)| -> i32 {
        let (exp, val) = x;
        acc + (1 << exp) * val
    };

    let gamma: i32 = lines.next().unwrap().into_iter().map(|x| if x < 0 { 0 } else { 1 }).rev().enumerate().fold(0, fold_fn);
    let epsilon: i32 = lcb_lines.next().unwrap().into_iter().map(|x| if x < 0 { 0 } else { 1 }).rev().enumerate().fold(0, fold_fn);

    println!("{:?}", gamma*epsilon);
}