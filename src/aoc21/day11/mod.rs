use std::fs;
use array2d::Array2D;

const NUM_ROWS: usize = 10;
const NUM_COLS: usize = 10;

pub fn day11_part1() {
    let NUM_STEPS = 10000;

    let file_string = fs::read_to_string("day11/input.txt").expect("File not found");
    let lines = file_string.lines();

    let map_char = |x: char| -> u32 {
        x.to_digit(10).unwrap()
    };

    let map_line = |line: &str| -> Vec<u32> {
        line.chars().map(map_char).collect()
    };

    let mut octopi: &mut Array2D<u32> = &mut Array2D::from_rows(&lines.map(map_line).collect::<Vec<Vec<_>>>());
    let mut num_flashes = 0;

    fn try_flash((i,j): (usize, usize), octopi: &mut Array2D<u32>, flashed: &mut Array2D<bool>, num_flashes: &mut u32) {
        let get_nbrs = |i: usize, j: usize| -> Vec<(usize, usize)> {
            let mut nbrs: Vec<(usize, usize)> = Vec::new();
            if i > 0 {
                nbrs.push((i-1, j));
                if j > 0 {
                    nbrs.push((i-1, j-1));
                }
                if j < NUM_COLS-1 {
                    nbrs.push((i-1, j+1));
                }
            }
            if i < NUM_ROWS-1 {
                nbrs.push((i+1, j));
                if j > 0 {
                    nbrs.push((i+1, j-1));
                }
                if j < NUM_COLS-1 {
                    nbrs.push((i+1, j+1));
                }
            }
            if j > 0 {
                nbrs.push((i, j-1))
            }
            if j < NUM_COLS-1 {
                nbrs.push((i, j+1))
            }
            nbrs
        };
        if octopi[(i,j)] >= 10 && !flashed[(i,j)] {
            flashed[(i,j)] = true;
            octopi[(i,j)] = 0;
            *num_flashes += 1;
            for nbr in get_nbrs(i,j) {
                if !flashed[nbr] {
                    octopi[nbr] += 1;
                }
                try_flash(nbr, octopi, flashed, num_flashes);
            }
        }
    }

    for step in 0..NUM_STEPS {
        let mut flashed: &mut Array2D<bool> = &mut Array2D::filled_with(false, octopi.column_len(), octopi.row_len());

        for i in 0..NUM_ROWS {
            for j in 0..NUM_COLS {
                octopi[(i,j)] += 1;
            }
        }
        for i in 0..NUM_ROWS{
            for j in 0..NUM_COLS {
                try_flash((i,j), &mut octopi, &mut flashed, &mut num_flashes);
            }
        }

        let mut sum = 0;
        for i in 0..NUM_ROWS{
            for j in 0..NUM_COLS {
                sum += octopi[(i,j)];
            }
        }
        if sum == 0 {
            println!("{}", step+1);
            break;
        }
    }
    
    //println!("{}", num_flashes);
    
}


pub fn day11_part2() {
    
}