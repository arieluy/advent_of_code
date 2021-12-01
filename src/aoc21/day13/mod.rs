use std::fs;
use std::collections::{HashMap, HashSet};
use std::iter::FromIterator;

/*

fold along x=655
fold along y=447
fold along x=327
fold along y=223
fold along x=163
fold along y=111
fold along x=81
fold along y=55
fold along x=40
fold along y=27
fold along y=13
fold along y=6

*/

pub fn day13_part1() {
    let file_string = fs::read_to_string("day13/input.txt").expect("File not found");
    let lines = file_string.lines();

    let parse_line = |line: &str| -> (u32, u32) {
        match line.split(",").collect::<Vec<_>>()[..] {
            [a, b] => (a.parse().unwrap(), b.parse().unwrap()),
            _ => panic!()
        }
    };
    let points = lines.map(parse_line);
    let fold = ("x", 655);

    let fold_1d = |point: u32, line: u32| -> u32 {
        if point < line {
            point
        } else {
            line - (point - line)
        }
    };
    let fold_point = |(x,y): (u32, u32)| -> (u32, u32) {
        match fold.0 {
            "x" => (fold_1d(x, fold.1), y),
            _ => (x, fold_1d(y, fold.1))
        }
    };
    let folded_points: HashSet<(u32, u32)> = HashSet::from_iter(points.map(fold_point));

    println!("{:?}", folded_points.len());

} 


pub fn day13_part2() {
    let file_string = fs::read_to_string("day13/input.txt").expect("File not found");
    let lines = file_string.lines();

    let parse_line = |line: &str| -> (u32, u32) {
        match line.split(",").collect::<Vec<_>>()[..] {
            [a, b] => (a.parse().unwrap(), b.parse().unwrap()),
            _ => panic!()
        }
    };
    let mut tmp = HashSet::<(u32, u32)>::from_iter(lines.map(parse_line));
    let mut points = tmp.iter();
    let folds = [("x", 655), ("y", 447), ("x", 327), ("y", 223), ("x", 163), 
    ("y", 111), ("x", 81), ("y", 55), ("x", 40), ("y", 27), ("y", 13), ("y", 6)];
    //let folds = [("y", 7), ("x", 5)];

    
    fn fold_point(fold: (&str, u32)) -> impl FnMut(&(u32, u32)) -> (u32, u32) + '_{
        let fold_1d = |point: u32, line: u32| -> u32 {
            if point < line {
                point
            } else {
                line - (point - line)
            }
        };
        move |(x,y): &(u32, u32)| -> (u32, u32) {
            match fold.0 {
                "x" => (fold_1d(*x, fold.1), *y),
                _ => (*x, fold_1d(*y, fold.1))
            }
        }
    }

    for fold in folds {
        tmp = HashSet::from_iter(points.map(fold_point(fold)));
        points = tmp.iter();
    }

    for y in 0..7 {
        for x in 0..40 {
            if tmp.contains(&(x,y)) {
                print!("#");
            } else {
                print!(".")
            }
        }
        print!("\n");
    }
}