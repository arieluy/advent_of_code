use std::fs;
use std::collections::{HashMap, HashSet};

pub fn day12_part1() {
    let file_string = fs::read_to_string("day12/input.txt").expect("File not found");
    let lines = file_string.lines();

    let parse_line = |line: &str| {
        match line.split("-").collect::<Vec<_>>()[..] {
            [a, b] => (a.to_owned(), b.to_owned()),
            _ => panic!()
        }
    };
    let edges = lines.map(parse_line);

    // build the graph from edge list 
    let mut graph: HashMap<String, Vec<String>> = HashMap::new();

    let mut add_nbr = |node: String, nbr: String| {
        graph.entry(node).and_modify(|nbrs| nbrs.push(nbr.clone())).or_insert(vec![nbr]);
    };

    for (a,b) in edges {
        add_nbr(a.clone(), b.clone());
        add_nbr(b.clone(), a.clone());
    }

    // DFS from start to end, but don't mark big caves as visited
    let curr = "start";
    let mut walk = vec![curr.to_string()];
    let mut visited: HashSet<String> = HashSet::new();
    visited.insert(curr.to_string());

    fn visit_node(curr: &str, graph: &HashMap<String, Vec<String>>, walk: &mut Vec<String>, visited: &mut HashSet<String>) -> u32 {
        let next = &graph[curr];
        let mut num_walks = 0;
        if curr == "end" {
            return 1;
        }
        for node in next {
            if !visited.contains(node) {
                walk.push(node.to_string());
                if node.chars().next().unwrap().is_lowercase() {
                    visited.insert(node.to_string());
                }
                num_walks += visit_node(node, graph, walk, visited);
                visited.remove(node);
                walk.pop();
            }
        }
        num_walks
    }

    let num_walks = visit_node(curr, &graph, &mut walk, &mut visited);
    

    println!("{:?}", num_walks);
}


pub fn day12_part2() {
    let file_string = fs::read_to_string("day12/input.txt").expect("File not found");
    let lines = file_string.lines();

    let parse_line = |line: &str| {
        match line.split("-").collect::<Vec<_>>()[..] {
            [a, b] => (a.to_owned(), b.to_owned()),
            _ => panic!()
        }
    };
    let edges = lines.map(parse_line);

    // build the graph from edge list 
    let mut graph: HashMap<String, Vec<String>> = HashMap::new();
    let mut visited: HashMap<String, u32> = HashMap::new();

    let mut add_nbr = |node: String, nbr: String| {
        graph.entry(node.clone()).and_modify(|nbrs| nbrs.push(nbr.clone())).or_insert(vec![nbr]);
        visited.entry(node).or_insert(0);
    };

    for (a,b) in edges {
        add_nbr(a.clone(), b.clone());
        add_nbr(b.clone(), a.clone());
    }

    // DFS from start to end, but don't mark big caves as visited
    let curr = "start";
    let mut walk = vec![curr.to_string()];
    visited.entry(curr.to_string()).and_modify(|n| *n += 1);

    fn visit_node(curr: &str, graph: &HashMap<String, Vec<String>>, walk: &mut Vec<String>, visited: &mut HashMap<String, u32>) -> u32 {
        fn any_visited_twice(visited: &HashMap<String, u32>) -> bool {
            visited.values().fold(false, |acc, v| acc || *v == 2)
        }

        let next = &graph[curr];
        let mut num_walks = 0;
        if curr == "end" {
            return 1;
        }
        for node in next {
            if visited[node] == 0 || (visited[node] == 1 && node != "start" && node != "end" && !any_visited_twice(visited)) {
                walk.push(node.to_string());
                if node.chars().next().unwrap().is_lowercase() {
                    visited.entry(node.to_string()).and_modify(|n| *n += 1);
                }
                num_walks += visit_node(node, graph, walk, visited);
                visited.entry(node.to_string()).and_modify(|n| { if *n > 0 { *n -= 1 }});
                walk.pop();
            }
        }
        num_walks
    }

    let num_walks = visit_node(curr, &graph, &mut walk, &mut visited);
    

    println!("{:?}", num_walks);
}