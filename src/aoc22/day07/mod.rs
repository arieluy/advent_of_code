use advent_of_code::Solution;
use std::{str::Lines, collections::HashMap};
use std::cmp::min;

pub struct Day07 {}

#[derive(Debug)]
enum Node {
    File(u32),
    Dir(Vec<String>)
}

fn get_path(current_dir: &Vec<&str>) -> String {
    current_dir.join("/").to_string()
}

fn get_file_path(current_dir: &Vec<&str>, file: &str) -> String {
    format!("{}/{}", current_dir.join("/"), file).to_string()
}

fn parse_files(lines: Lines) -> HashMap<String, Node> {
    let mut files_map: HashMap<String, Node> = HashMap::new();

    let mut current_dir: Vec<&str> = Vec::new();
    current_dir.push("~");
    let mut current_ls: Vec<String> = Vec::new();
    let mut is_ls = false;

    for line in lines.skip(1) {
        let parse: Vec<&str> = line.split(" ").collect();
        match parse.as_slice() {
            ["$", "cd", dir] => {
                if is_ls {
                    files_map.insert(get_path(&current_dir), Node::Dir(current_ls));
                    current_ls = Vec::new();
                    is_ls = false;
                }
                match *dir {
                    ".." => { current_dir.pop(); },
                    _ => { current_dir.push(dir); }
                }
            },
            ["$", "ls"] => {is_ls = true},
            ["dir", dir] => {
                current_ls.push(get_file_path(&current_dir, dir));
            },
            [size, file] => { 
                current_ls.push(get_file_path(&current_dir, file)); 
                files_map.insert(get_file_path(&current_dir, file), Node::File((*size).parse::<u32>().unwrap())); 
            },
            _ => panic!("failed to parse")
        }
    }
    if is_ls {
        files_map.insert(get_path(&current_dir), Node::Dir(current_ls));
    }

    files_map

}

fn get_size(name: &str, files_map: &HashMap<String, Node>) -> u32 {
    match &files_map[name] {
        Node::File(size) => *size,
        Node::Dir(files) => files.iter().map(|name| get_size(name, files_map)).sum()
    }
}

impl Solution for Day07 {

    fn part1(lines: Lines) {
        let files_map = parse_files(lines);

        let mut sum = 0;
        for (name, file) in &files_map {
            if let Node::Dir(_) = file {
                let size = get_size(&name, &files_map);
                if size <= 100000 {
                    sum += size;
                }
            }
        }

        dbg!(sum);
    }

    fn part2(lines: Lines) {
        let files_map = parse_files(lines);
        
        let mut best = 1000000000;
        let used_space = get_size("~", &files_map);

        for (name, file) in &files_map {
            if let Node::Dir(_) = file {
                let size = get_size(&name, &files_map);
                if size >= 30000000 - (70000000 - used_space) {
                    best = min(best, size);
                }
            }
        }

        dbg!(best);
    }
}
