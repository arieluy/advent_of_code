use advent_of_code::Solution;
use std::collections::HashSet;
use std::str::Lines;

pub struct Day06 {}

fn are_all_unique(slice: &[char]) -> bool {
    let hs: HashSet<&char> = HashSet::from_iter(slice);
    return hs.len() == slice.len();
}

impl Solution for Day06 {

    fn part1(mut lines: Lines) {
        let chars: Vec<_> = lines.next().unwrap().chars().collect();
        let mut index = 4;
        for slice in chars.windows(4) {
            if are_all_unique(slice) {
                break;
            }
            index += 1;
        }
        dbg!(index);
    }

    fn part2(mut lines: Lines) {
        let chars: Vec<_> = lines.next().unwrap().chars().collect();
        let mut index = 14;
        for slice in chars.windows(14) {
            if are_all_unique(slice) {
                break;
            }
            index += 1;
        }
        dbg!(index);
    }
}
