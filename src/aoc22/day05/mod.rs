use advent_of_code::Solution;
use std::str::Lines;
use regex::Regex;

pub struct Day05 {}

fn parse_crates(line: &str) -> Vec<&str> {
    let re = Regex::new(r"(.),").unwrap();
    let crates_capture = re.captures_iter(line);
    let mut crates = Vec::new();
    for c in crates_capture {
        crates.push(c.get(1).unwrap().as_str());
    }
    return crates;
} 

fn parse_moves(line: &str) -> (usize, usize, usize) {
    let re = Regex::new(r"move (\d+) from (\d+) to (\d+)").unwrap();
    let capture = re.captures(line).unwrap();
    let num = capture[1].parse::<usize>().unwrap();
    let src = capture[2].parse::<usize>().unwrap();
    let dst = capture[3].parse::<usize>().unwrap();
    
    return (num, src, dst);
}

impl Solution for Day05 {

    fn part1(lines: Lines) {
        let mut parse_towers = true;
        let mut towers: Vec<Vec<_>> = Vec::new();
    
        for line in lines {
            if parse_towers {
                if line.is_empty() {
                    parse_towers = false;
                    continue;
                }

                towers.push(parse_crates(line));
            } else {
                let (num, src, dst) = parse_moves(line);

                for _ in 0..num {
                    let object = towers[src-1].pop().unwrap();
                    towers[dst-1].push(object);
                }
            }
        }

        for tower in towers {
            print!("{}", tower.last().unwrap());
        }
        println!("");
    }

    fn part2(lines: Lines) {
        let mut parse_towers = true;
        let mut towers: Vec<Vec<_>> = Vec::new();
    
        for line in lines {
            if parse_towers {
                if line.is_empty() {
                    parse_towers = false;
                    continue;
                }

                towers.push(parse_crates(line));
            } else {
                let (num, src, dst) = parse_moves(line);

                let mut holder = Vec::new();
                for _ in 0..num {
                    let object = towers[src-1].pop().unwrap();
                    holder.push(object);
                }
                for _ in 0..num {
                    let object = holder.pop().unwrap();
                    towers[dst-1].push(object);
                }
            }

        }

        for tower in towers {
            print!("{}", tower.last().unwrap());
        }
        println!("");
    }
}
