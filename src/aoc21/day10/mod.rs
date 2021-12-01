use std::fs;

#[derive(PartialEq, Eq)]
enum PT {
    Round,
    Square,
    Curly,
    Angled
}

enum Parens {
    Open(PT),
    Close(PT)
}

fn score_of(pt: PT) -> u32 {
    match pt {
        PT::Round => 3,
        PT::Square => 57,
        PT::Curly => 1197,
        PT::Angled => 25137
    }
}

fn paren_parse(c: char) -> Parens {
    match c {
        '(' => Parens::Open(PT::Round),
        ')' => Parens::Close(PT::Round),
        '[' => Parens::Open(PT::Square),
        ']' => Parens::Close(PT::Square),
        '{' => Parens::Open(PT::Curly),
        '}' => Parens::Close(PT::Curly),
        '<' => Parens::Open(PT::Angled),
        '>' => Parens::Close(PT::Angled),
        _ => panic!()
    }
}

pub fn day10_part1() {
    let file_string = fs::read_to_string("day10/input.txt").expect("File not found");
    let lines = file_string.lines();

    let mut score = 0;

    for line in lines {
        let mut parens: Vec<PT> = Vec::new();
        for c in line.chars() {
            match paren_parse(c) {
                Parens::Open(pt) => {parens.push(pt);},
                Parens::Close(pt) => {
                    match parens.pop() {
                        Some(pt2) => {
                            if pt != pt2 {
                                score += score_of(pt);
                                break;
                            }
                        },
                        _ => { 
                            score += score_of(pt); 
                            break;
                        }
                    }
                }
            }
        }
    }
    
    println!("{}", score);
    
}

fn auto_score_of(pt: &PT) -> u64 {
    match pt {
        PT::Round => 1,
        PT::Square => 2,
        PT::Curly => 3,
        PT::Angled => 4
    }
}

pub fn day10_part2() {
    let file_string = fs::read_to_string("day10/input.txt").expect("File not found");
    let lines = file_string.lines();

    let mut scores: Vec<u64> = Vec::new();

    'outer: for line in lines {
        let mut parens: Vec<PT> = Vec::new();
        for c in line.chars() {
            match paren_parse(c) {
                Parens::Open(pt) => {parens.push(pt);},
                Parens::Close(pt) => {
                    match parens.pop() {
                        Some(pt2) => {
                            if pt != pt2 {
                                continue 'outer; // corrupted
                            }
                        },
                        _ => { continue 'outer; /* corrupted */ }
                    }
                }
            }
        }
        // unfinished?
        let mut score = 0;
        for p in parens.iter().rev() {
            score *= 5;
            score += auto_score_of(p);
        }
        scores.push(score);

    }

    scores.sort();
    
    println!("{:?}", scores[(scores.len() - 1)/2]);
}