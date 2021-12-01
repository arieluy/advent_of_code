use std::fs;
use std::collections::{HashMap, HashSet};
use std::iter::FromIterator;
use multiset::HashMultiSet;

const NUM_STEPS: u32 = 40;

pub fn day14_part1() {
    let file_string = fs::read_to_string("day14/input.txt").expect("File not found");
    let mut lines = file_string.lines();

    let template = lines.next().unwrap();
    lines.next();

    let parse_line = |line: &str| {
        match line.split(" -> ").collect::<Vec<_>>()[..] {
            [a, b] => {
                let aa = a.to_owned();
                let aa_chars: Vec<char> = aa.chars().collect();
                (aa, (aa_chars[0].to_string() + b, b.to_owned() + &aa_chars[1].to_string()))
            },
            _ => panic!()
        }
    };
    let rules: HashMap<String, (String, String)> = HashMap::from_iter(lines.map(parse_line));

    let mut pairs: HashMultiSet<String> = HashMultiSet::new();

    let temp_chars: Vec<char> = template.chars().collect();
    for i in 0..temp_chars.len()-1 {
        pairs.insert(temp_chars[i].to_string() + &temp_chars[i+1].to_string());
    }
    println!("{:?}", pairs);

    for _ in 0..NUM_STEPS {
        let mut new_pairs: HashMultiSet<String> = HashMultiSet::new();
        for pair in pairs.distinct_elements() {
            let count = pairs.count_of(pair);
            let (new1, new2) = &rules[pair];
            new_pairs.insert_times(new1.to_string(), count);
            new_pairs.insert_times(new2.to_string(), count);
        }
        pairs = new_pairs;
    }

    let mut elems: HashMultiSet<String> = HashMultiSet::new();
    for pair in pairs.distinct_elements() {
        let count = pairs.count_of(pair);
        let (elem1, elem2) = (&pair[0..1].to_string(), &pair[1..2].to_string());
        elems.insert_times(elem1.to_string(), count);
        elems.insert_times(elem2.to_string(), count);
    }

    let mut highest = 0;
    let mut lowest = usize::MAX;
    for elem in elems.distinct_elements() {
        let count = elems.count_of(elem);
        println!("elem: {}, count: {}", elem, count);
        if count > highest {
            highest = count;
        }
        if count < lowest {
            lowest = count;
        }
    }

    println!("highest: {}, lowest: {}, diff: {}", highest, lowest, highest-lowest);

} 


pub fn day14_part2() {
    
}