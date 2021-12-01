use std::fs::File;
use std::io::{BufRead, BufReader};

pub fn day1() {
    let file = File::open("day1/input.txt").expect("File not found");
    let reader = BufReader::new(file);
    let mut count = 0;
    let mut is_first_three = 0;
    let mut a = 0;
    let mut b = 0;
    let mut c = 0;

    for line in reader.lines() {
        if let Ok(ip) = line {
            let n: i32 = match ip.trim().parse() {
              Ok(num) => num,
              Err(_) => return
            };

            if is_first_three >= 3 {
                if c < n {
                    count += 1;
                }
            }
            c = b;
            b = a;
            a = n;
            is_first_three += 1;
        }
    }
    println!("{}", count);
}