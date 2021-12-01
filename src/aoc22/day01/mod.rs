use advent_of_code::Solution;
use std::cmp::max;
use std::str::Lines;

pub struct Day01 {}

impl Solution for Day01 {
    
    fn part1(lines: Lines) {
        let mut sum = 0;
        let mut best_sum = 0;
    
        for line in lines {
            let s = line.trim();
            if s.is_empty() {
                best_sum = max(sum, best_sum);
                sum = 0;
            } else {
                sum += s.parse::<i32>().unwrap();
            }
        }
        println!("{}", best_sum);
    }
    
    fn part2(lines: Lines) {
        let mut sum = 0;
        let mut sums: Vec<i32> = Vec::new();
    
        for line in lines {
            let s = line.trim();
            if s.is_empty() {
                sums.push(sum);
                sum = 0;
            } else {
                sum += s.parse::<i32>().unwrap();
            }
        }
        
        sums.sort();
        let best = &sums[sums.len()-3..sums.len()];
        println!("{}", best[0]+best[1]+best[2]);
    }
}
