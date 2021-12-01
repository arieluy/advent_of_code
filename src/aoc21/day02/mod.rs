use std::fs;

enum Move {
    Forward(u32),
    Down(u32),
    Up(u32)
}

pub fn day2_part1() {
    let file_string = fs::read_to_string("day2/input.txt").expect("File not found");

    let map_to_enum = |x: &str| {
        let xx: Vec<_> = x.split(" ").collect();
        let n: u32 = xx[1].trim().parse().unwrap();
        match xx[0] {
            "forward" => Move::Forward(n),
            "down" => Move::Down(n),
            "up" => Move::Up(n),
            _ => Move::Up(0)
        }
    };
    
    let lines: Vec<_> = file_string.split("\n").map(map_to_enum).collect();

    let fold_fn = |acc: (u32, u32), x: &Move| {
        match x {
            Move::Forward(n) => (acc.0 + n, acc.1),
            Move::Down(n) => (acc.0, acc.1 + n),
            Move::Up(n) => (acc.0, acc.1 - n)
        }
    };

    let position: (u32, u32) = lines.iter().fold((0,0), fold_fn);


    println!("{}", position.0 * position.1);
}


pub fn day2_part2() {
    let file_string = fs::read_to_string("day2/input.txt").expect("File not found");

    let map_to_enum = |x: &str| {
        let xx: Vec<_> = x.split(" ").collect();
        let n: u32 = xx[1].trim().parse().unwrap();
        match xx[0] {
            "forward" => Move::Forward(n),
            "down" => Move::Down(n),
            "up" => Move::Up(n),
            _ => Move::Up(0)
        }
    };
    
    let lines: Vec<_> = file_string.split("\n").map(map_to_enum).collect();

    let fold_fn = |acc: (u32, u32, u32), x: &Move| {
        match x {
            Move::Forward(n) => (acc.0 + n, acc.1 + acc.2*n, acc.2),
            Move::Down(n) => (acc.0, acc.1, acc.2 + n),
            Move::Up(n) => (acc.0, acc.1, acc.2 - n)
        }
    };

    let position: (u32, u32, u32) = lines.iter().fold((0,0, 0), fold_fn);


    println!("{}", position.0 * position.1);
}