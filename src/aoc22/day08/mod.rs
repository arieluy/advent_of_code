use advent_of_code::Solution;
use array2d::Array2D;
use std::str::Lines;

pub struct Day08 {}

fn get_is_visible(i: usize, j: usize, forest: &Array2D<i32>) -> i32 {
    let my_tree = forest[(i,j)];

    for (dx, dy) in [(0, 1), (0, -1), (1, 0), (-1, 0)] {
        let mut ii = i as i32 + dx;
        let mut jj = j as i32 + dy;
        let mut is_visible = true;

        while 0 <= ii && (ii as usize) < forest.num_rows() && 0 <= jj && (jj as usize) < forest.num_columns() {
            let tree = forest[(ii as usize, jj as usize)];
            if tree >= my_tree {
                is_visible = false;
                break;
            }
            ii += dx;
            jj += dy;
        }

        if is_visible {
            return 1;
        }
    }

    return 0;
}

fn get_scenic_score(i: usize, j: usize, forest: &Array2D<i32>) -> i32 {
    let mut score = 1;
    let my_tree = forest[(i,j)];

    for (dx, dy) in [(0, 1), (0, -1), (1, 0), (-1, 0)] {
        let mut ii = i as i32 + dx;
        let mut jj = j as i32 + dy;
        let mut seen_trees = 0;
        while 0 <= ii && (ii as usize) < forest.num_rows() && 0 <= jj && (jj as usize) < forest.num_columns() {
            let tree = forest[(ii as usize, jj as usize)];
            seen_trees += 1;
            if tree >= my_tree {
                break;
            }
            ii += dx;
            jj += dy;
        }
        score *= seen_trees;
    }

    score
}

impl Solution for Day08 {

    fn part1(lines: Lines) {
        let rows: Vec<Vec<i32>> = lines.map(|line| { line.chars().map(|c| c.to_digit(10).unwrap() as i32).collect() }).collect();
        let forest = Array2D::from_rows(&rows);

        let sum: i32 = forest.rows_iter().enumerate().map(|(i, row)| {
            row.enumerate().map(|(j, tree)| {
                get_is_visible(i, j, &forest)
            }).sum::<i32>()
        }).sum();

        dbg!(sum);
    }

    fn part2(lines: Lines) {
        let rows: Vec<Vec<i32>> = lines.map(|line| { line.chars().map(|c| c.to_digit(10).unwrap() as i32).collect() }).collect();
        let forest = Array2D::from_rows(&rows);

        let max = forest.rows_iter().enumerate().map(|(i, row)| {
            row.enumerate().map(|(j, tree)| {
                get_scenic_score(i, j, &forest)
            }).max().unwrap()
        }).max().unwrap();

        dbg!(max);
    }
}
