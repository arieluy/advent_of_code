use std::fs;
use itertools::Itertools;
use std::collections::HashSet;

pub fn day8_part1() {
    let file_string = fs::read_to_string("day8/input.txt").expect("File not found");
    let lines = file_string.lines();

    let lens: Vec<Vec<_>> = lines.map(|line| {
        let halves: Vec<_> = line.split("|").collect();
        halves[1].trim().split(" ").map(|x| x.len()).collect()
    }).collect();

    let count = lens.into_iter().flatten().filter(|x| *x == 2 || *x == 3 || *x == 4 || *x == 7).count();
    
    println!("{:?}", count);
    
}

fn alphabetize(s: &str) -> String {
    s.chars().sorted().collect()
}

fn subset(s: &str, t: &str) -> bool {
    let s_chars: HashSet<char, std::collections::hash_map::RandomState> = HashSet::from(s.chars().collect());
    let t_chars = HashSet::from(t.chars().collect());
    s_chars.is_subset(&t_chars)
}

fn union_eq(s: &str, t: &str, r: &str) -> bool {
    let s_chars: HashSet<char, std::collections::hash_map::RandomState> = HashSet::from(s.chars().collect());
    let t_chars = HashSet::from(t.chars().collect());
    let r_chars = HashSet::<char, std::collections::hash_map::RandomState>::from(r.chars().collect());
    let union = HashSet::<char, std::collections::hash_map::RandomState>::from(s_chars.union(&t_chars).collect::<String>().chars().collect());
    union.is_subset(&r_chars) && union.is_superset(&r_chars)
}

fn solve_line(line: &str) -> usize {
    let halves: Vec<_> = line.split("|").collect();
    let (digits, values) = (halves[0].trim().split(" ").map(alphabetize).collect::<Vec<_>>(), halves[1].trim().split(" ").map(alphabetize));
    let mut translation = vec![""; 10];

    let find_dig_len = |n: usize| {
        for i in 0..digits.len() {
            if digits[i].len() == n {
                return &digits[i]
            }
        }
        panic!()
    };

    translation[1] = find_dig_len(2);
    translation[4] = find_dig_len(4);
    translation[7] = find_dig_len(3);
    translation[8] = find_dig_len(7);

    // 3 is the only 5-length digit such that 1 \subseteq 3
    for i in 0..digits.len() {
        if digits[i].len() == 5 && subset(translation[1], &digits[i]) {
            translation[3] = &digits[i];
            break;
        }
    }

    // 5 is the only 5-length digit st 3 \cup 5 = a 6-length digit, i.e. 9
    'outer: for i in 0..digits.len() {
        if digits[i].len() == 5 && translation[3] != &digits[i] {
            for j in 0..digits.len() {
                if digits[j].len() == 6 && union_eq(translation[3], &digits[i], &digits[j]) {
                    translation[5] = &digits[i];
                    translation[9] = &digits[j];
                    break 'outer;
                }
            }
        }
    }

    // 2 is the only remaining 5-length digit
    for i in 0..digits.len() {
        if digits[i].len() == 5 && translation[3] != &digits[i] && translation[5] != &digits[i] {
            translation[2] = &digits[i];
            break;
        }
    }

    // 5 is a subset of 6 but not 0 (the 2 remaining 6-length digits)
    for i in 0..digits.len() {
        if digits[i].len() == 6 && translation[9] != &digits[i] && subset(translation[5], &digits[i]) {
            translation[6] = &digits[i];
            break;
        }
    }

    // 0 is remainingS
    for i in 0..digits.len() {
        if digits[i].len() == 6 && translation[6] != &digits[i] && translation[9] != &digits[i] {
            translation[0] = &digits[i];
            break;
        }
    }

    let mut num = 0;
    for value in values {
        for i in 0..digits.len() {
            if value == translation[i] {
                num *= 10;
                num += i;
            }
        }
    }
    num
}

pub fn day8_part2() {
    let file_string = fs::read_to_string("day8/input.txt").expect("File not found");
    let lines = file_string.lines();

    let mut sum = 0;
    for line in lines {
        sum += solve_line(line);
    }
    
    println!("{:?}", sum);
}