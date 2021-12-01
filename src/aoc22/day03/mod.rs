use advent_of_code::Solution;
use std::str::Lines;
use std::collections::HashSet;
use std::iter::FromIterator;

pub struct Day03 {}

fn get_value(c: char) -> u32 {
    let n = c as u32;
    if n > 96 {
        return n - 96;
    } else {
        return n - 64 + 26;
    }
}

fn find_intersection(a: &str, b: &str, c: &str) -> u32 {
    let a_set: HashSet<char> = HashSet::from_iter(a.chars());
    let b_set: HashSet<char> = HashSet::from_iter(b.chars());
    let c_set: HashSet<char> = HashSet::from_iter(c.chars());

    let intersect_set: HashSet<char> = HashSet::from_iter(a_set.intersection(&b_set).copied());
    let mut intersect_more = intersect_set.intersection(&c_set);
    let ans: char = *intersect_more.next().unwrap();

    return get_value(ans);
}

impl Solution for Day03 {
    
    fn part1(lines: Lines) {
        let mut sum = 0;
        for line in lines {
            let first_half = &line[0..line.len()/2];
            let second_half = &line[line.len()/2..line.len()];
            //println!("first: {}, second: {}", firstHalf, secondHalf);

            let first_set: HashSet<char> = HashSet::from_iter(first_half.chars());
            let second_set: HashSet<char> = HashSet::from_iter(second_half.chars());

            let mut intersect = first_set.intersection(&second_set);
            let c: char = *intersect.next().unwrap();

            //println!("c: {}, u32: {}", c, getValue(c));
            
            sum += get_value(c);
        }
        
        println!("{}", sum);
    }

    fn part2(lines: Lines) {
        let mut lines = lines.peekable();
        let mut sum = 0;
        while lines.peek() != None {
            sum += find_intersection(lines.next().unwrap(), lines.next().unwrap(), lines.next().unwrap());
        }
        
        println!("{}", sum);
    }
}