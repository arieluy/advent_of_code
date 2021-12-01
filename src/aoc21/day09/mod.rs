use std::fs;
use array2d::Array2D;

pub fn day9_part1() {
    let file_string = fs::read_to_string("day9/input.txt").expect("File not found");
    let lines = file_string.lines();

    let map_char = |x: char| -> u32 {
        x.to_digit(10).unwrap()
    };

    let map_line = |line: &str| -> Vec<u32> {
        line.chars().map(map_char).collect()
    };

    let heights: Array2D<u32> = Array2D::from_rows(&lines.map(map_line).collect::<Vec<Vec<_>>>());

    let mut sum = 0;

    let get_nbrs = |i: usize, j: usize| -> Vec<(usize, usize)> {
        let mut nbrs: Vec<(usize, usize)> = Vec::new();
        if i > 0 {
            nbrs.push((i-1, j));
        }
        if i < heights.column_len()-1 {
            nbrs.push((i+1, j));
        }
        if j > 0 {
            nbrs.push((i, j-1))
        }
        if j < heights.row_len()-1 {
            nbrs.push((i, j+1))
        }
        nbrs
    };

    for i in 0..heights.column_len() {
        'mid: for j in 0..heights.row_len() {
            for nbr in get_nbrs(i,j) {
                if heights[nbr] <= heights[(i,j)] {
                    continue 'mid;
                }
            }
            sum += heights[(i,j)] + 1;
        }
    }
    
    println!("{:?}", sum);
    
}


pub fn day9_part2() {
    let file_string = fs::read_to_string("day9/input.txt").expect("File not found");
    let lines = file_string.lines();

    let map_char = |x: char| -> u32 {
        x.to_digit(10).unwrap()
    };

    let map_line = |line: &str| -> Vec<u32> {
        line.chars().map(map_char).collect()
    };

    let heights: Array2D<u32> = Array2D::from_rows(&lines.map(map_line).collect::<Vec<Vec<_>>>());

    let mut sum = 0;

    let get_nbrs = |i: usize, j: usize| -> Vec<(usize, usize)> {
        let mut nbrs: Vec<(usize, usize)> = Vec::new();
        if i > 0 {
            nbrs.push((i-1, j));
        }
        if i < heights.column_len()-1 {
            nbrs.push((i+1, j));
        }
        if j > 0 {
            nbrs.push((i, j-1))
        }
        if j < heights.row_len()-1 {
            nbrs.push((i, j+1))
        }
        nbrs
    };

    let get_upstream_nbrs = |i: usize, j: usize, curr_height| -> Vec<(usize, usize)> {
        let mut nbrs: Vec<(usize, usize)> = Vec::new();
        let mut push_upstream = |nbr| {
            if heights[nbr] > curr_height && heights[nbr] != 9 {
                nbrs.push(nbr);
            }
        };

        if i > 0 {
            push_upstream((i-1, j));
        }
        if i < heights.column_len()-1 {
            push_upstream((i+1, j));
        }
        if j > 0 {
            push_upstream((i, j-1));
        }
        if j < heights.row_len()-1 {
            push_upstream((i, j+1));
        }
        nbrs
    };

    let mut flow: Array2D<Vec<(usize, usize)>> = Array2D::filled_with(Vec::new(), heights.column_len(), heights.row_len());
    let mut visited: Array2D<bool> = Array2D::filled_with(false, heights.column_len(), heights.row_len());
    let mut basin_sizes: Vec<usize> = Vec::new();

    // pre calculate flow
    for i in 0..heights.column_len() {
        for j in 0..heights.row_len() {
            let curr_height = heights[(i,j)];
            flow[(i,j)] = get_upstream_nbrs(i,j, curr_height);
        }
    }

    // find low points again
    for i in 0..heights.column_len() {
        'mid: for j in 0..heights.row_len() {
            for nbr in get_nbrs(i,j) {
                if heights[nbr] <= heights[(i,j)] {
                    continue 'mid;
                }
            }
            // found a low point, find its basin w/ BFS
            let mut basin_size = 1;
            let mut curr: Vec<(usize, usize)> = Vec::new();
            curr.push((i,j));
            visited[(i,j)] = true;
           
            let mut next: Vec<_> = curr.iter().map(|x| flow[*x].clone().into_iter().filter(|x| !visited[*x])).flatten().collect();

            while !next.is_empty() {
                curr = next;
                for point in &curr {
                    if !visited[*point] {
                        basin_size += 1;
                        visited[*point] = true;
                    }
                }
                next = curr.iter().map(|x| flow[*x].clone()).flatten().collect();
            }

            basin_sizes.push(basin_size);
        }
    }

    basin_sizes.sort();
    basin_sizes = basin_sizes.into_iter().rev().collect();

    println!("{:?}", basin_sizes[0]*basin_sizes[1]*basin_sizes[2]);
}