use advent_of_code::Solution;
use std::str::Lines;

pub struct Day04 {}

impl Solution for Day04 {

    fn part1(lines: Lines) {
        let mut sum = 0;
        for line in lines {
            let parse: Vec<Vec<_>> = line.split(",").map(|x| x.split("-").map(|y| y.parse::<u32>().unwrap()).collect()).collect();
            let elf1_start = parse[0][0];
            let elf1_end = parse[0][1];
            let elf2_start = parse[1][0];
            let elf2_end = parse[1][1];

            if (elf1_start <= elf2_start && elf1_end >= elf2_end)
            || (elf2_start <= elf1_start && elf2_end >= elf1_end) {
                sum += 1;
            }
        }
        
        println!("{}", sum);
    }


    fn part2(lines: Lines) {
        let mut sum = 0;
        for line in lines {
            let parse: Vec<Vec<_>> = line.split(",").map(|x| x.split("-").map(|y| y.parse::<u32>().unwrap()).collect()).collect();
            let elf1_start = parse[0][0];
            let elf1_end = parse[0][1];
            let elf2_start = parse[1][0];
            let elf2_end = parse[1][1];

            if !(elf1_end < elf2_start) && !(elf2_end < elf1_start) {
                sum += 1;
            }
        }
        
        println!("{}", sum);
    }
}