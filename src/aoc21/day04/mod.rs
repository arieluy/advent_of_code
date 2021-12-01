use array2d;
use std::fs;
use array2d::Array2D;
use itertools::Itertools;
use std::collections::HashSet;

pub fn day4_part1() {
    let file_string = fs::read_to_string("day4/input.txt").expect("File not found");
    let mut lines = file_string.lines();

    let to_int = |x: &str| x.parse::<i32>().unwrap();
    let numbers_to_draw = lines.next().unwrap().split(",").map(to_int);

    let to_board = |mut b: itertools::Chunk<'_, std::str::Lines<'_>>| -> Array2D<i32> {
        b.next(); // skip the blank line
        let to_row = |row: &str| row.split_whitespace().map(to_int).collect::<Vec<_>>();
        Array2D::from_rows(&b.into_iter().map(to_row).collect::<Vec<_>>())
    };

    let chunks = lines.chunks(6);
    let boards = chunks.into_iter().map(to_board).enumerate().collect::<Vec<_>>();

    let mut drawn_numbers = HashSet::new();
    for n in numbers_to_draw {
        drawn_numbers.insert(n);
        let check_row_win = |board: &Array2D<i32>| {
            board.rows_iter().fold(false, |acc, row| acc || row.fold(true, |acc2, num| acc2 && drawn_numbers.contains(num)))
        };
        let check_col_win = |board: &Array2D<i32>| {
            board.columns_iter().fold(false, |acc, col| acc || col.fold(true, |acc2, num| acc2 && drawn_numbers.contains(num)))
        };
        let winner: Option<usize> = boards.clone().iter().fold(None, |winning_board, (index, board)| { 
            if winning_board == None {
                match check_row_win(board) || check_col_win(board) {
                    true => Some(*index),
                    false => None
                }
            } else { winning_board }
        });
        match winner {
            Some(w) => {
                println!("Winner: {:?}", w);
                let (_, winning_board) = &boards[w];
                let sum_unmarked: i32 = winning_board.elements_row_major_iter().filter(|n| !drawn_numbers.contains(n)).sum();
                println!("sum unmarked: {:?}, n: {:?}, ans: {:?}", sum_unmarked, n, sum_unmarked*n);
                return
            },
            None => ()
        }
        
    }
    
}


pub fn day4_part2() {
    let file_string = fs::read_to_string("day4/input.txt").expect("File not found");
    let mut lines = file_string.lines();

    let to_int = |x: &str| x.parse::<i32>().unwrap();
    let numbers_to_draw = lines.next().unwrap().split(",").map(to_int);

    let to_board = |mut b: itertools::Chunk<'_, std::str::Lines<'_>>| -> Array2D<i32> {
        b.next(); // skip the blank line
        let to_row = |row: &str| row.split_whitespace().map(to_int).collect::<Vec<_>>();
        Array2D::from_rows(&b.into_iter().map(to_row).collect::<Vec<_>>())
    };

    let chunks = lines.chunks(6);
    let boards = chunks.into_iter().map(to_board).enumerate().collect::<Vec<_>>();

    let mut drawn_numbers = HashSet::new();
    let mut num_winners = 0;
    let num_boards = boards.len();
    let mut winners_vec = vec![false; num_boards];
    println!("{}", num_boards);

    for n in numbers_to_draw {
        drawn_numbers.insert(n);
        let check_row_win = |board: &Array2D<i32>| {
            board.rows_iter().fold(false, |acc, row| acc || row.fold(true, |acc2, num| acc2 && drawn_numbers.contains(num)))
        };
        let check_col_win = |board: &Array2D<i32>| {
            board.columns_iter().fold(false, |acc, col| acc || col.fold(true, |acc2, num| acc2 && drawn_numbers.contains(num)))
        };
        let mut winner: Option<usize> = None;

        boards.clone().iter().for_each(|(index, board)| { 
            if !winners_vec[*index] && (check_row_win(board) || check_col_win(board)) {
                winners_vec[*index] = true;
                num_winners += 1;
                println!("{}", *index);
                if num_winners == num_boards {
                    winner = Some(*index);
                }
            }
        });
        match winner {
            Some(w) => {
                println!("Winner: {:?}", w);
                let (_, winning_board) = &boards[w];
                let sum_unmarked: i32 = winning_board.elements_row_major_iter().filter(|n| !drawn_numbers.contains(n)).sum();
                println!("sum unmarked: {:?}, n: {:?}, ans: {:?}", sum_unmarked, n, sum_unmarked*n);
                return
            },
            None => ()
        }
        
    }
}