use std::fs;
use std::path::PathBuf;
use std::str::Lines;
use std::any::type_name;
use regex::Regex;

pub trait Solution {
    fn get_name() -> String {
        let type_name = type_name::<Self>();
        let re = Regex::new(r"advent_of_code::aoc22::(.{5})::.*").unwrap();
        let name = &re.captures_iter(type_name).next().unwrap()[1];
        
        name.to_string()
    }
    fn part1(lines: Lines);
    fn part2(lines: Lines);
    fn run(part: u8, use_sample_input: bool) {
        let input = match use_sample_input { true => "sample_input.txt", false => "input.txt" };
        let path: PathBuf = ["aoc22", &Self::get_name(), input].iter().collect();

        let file_string = fs::read_to_string(path).expect("File not found");
        let lines = file_string.lines();

        if part == 1 {
            Self::part1(lines);
        } else {
            Self::part2(lines);
        }
    }
}