use advent_of_code::Solution;
use itertools::Itertools;
use std::collections::HashSet;
use std::str::Lines;
use std::cmp::Ordering::{Less, Equal, Greater};

pub struct Day09 {}

fn parse(lines: Lines) -> Vec<Move> {
    let mut moves: Vec<Move> = Vec::new();
    for line in lines {
        let parse = line.split_whitespace().collect_vec();
        let n = (parse[1]).parse::<i32>().unwrap();
        let mv = match parse[0] {
            "U" => Move::Up,
            "D" => Move::Down,
            "R" => Move::Right,
            "L" => Move::Left,
            _ => panic!("Unexpected input")
        };
        for _ in 0..n {
            moves.push(mv);
        };
    }

    moves
}

#[derive(Copy, Clone)]
enum Move {
    Up,
    Down,
    Right,
    Left
}

#[derive(Hash, Eq, PartialEq, Clone, Copy, Debug)]
struct Position {
    x: i32,
    y: i32
}

impl Position {
    fn new() -> Position {
        Position {x: 0, y: 0}
    }

    fn make_move(&mut self, mv: Move) {
        match mv {
            Move::Up => self.y += 1,
            Move::Down => self.y -= 1,
            Move::Right => self.x += 1,
            Move::Left => self.x -= 1
        }
    }

    fn is_touching(&self, head: &Position) -> bool {
        (self.x - head.x).abs() <= 1 && (self.y - head.y).abs() <= 1
    }

    fn follow_head(&mut self, head: &Position) {
        if self.is_touching(head) {
            return
        }

        match self.x.cmp(&head.x) {
            Less => self.make_move(Move::Right),
            Greater => self.make_move(Move::Left),
            Equal => ()
        }
        match self.y.cmp(&head.y) {
            Less => self.make_move(Move::Up),
            Greater => self.make_move(Move::Down),
            Equal => ()
        }
    }
}

trait MoveRope {
    fn make_move(&mut self, mv: Move);
    fn get_tail(&self) -> Position;
}

struct Rope {
    head: Position,
    tail: Position
}

impl Rope {
    fn new() -> Rope {
        let head = Position::new();
        let tail = Position::new();

        Rope {head: head, tail: tail}
    }
}

impl MoveRope for Rope {
    fn make_move(&mut self, mv: Move) {
        self.head.make_move(mv);
        self.tail.follow_head(&self.head);
    }

    fn get_tail(&self) -> Position {
        self.tail
    }
}

struct LongRope {
    head: Position,
    tail: Vec<Position>
}

impl LongRope {
    fn new() -> LongRope {
        let head = Position::new();
        let tail = vec![Position::new(); 9];

        LongRope{head: head, tail: tail}
    }
}

impl MoveRope for LongRope {
    fn make_move(&mut self, mv: Move) {
        self.head.make_move(mv);
        let mut follow = self.head;
        for i in 0..9 {
            self.tail[i].follow_head(&follow);
            follow = self.tail[i];
        }
    }

    fn get_tail(&self) -> Position {
        self.tail[8]
    }
}

fn run_simulation(mut rope: impl MoveRope, moves: Vec<Move>) -> usize {
    let mut tail_visited: HashSet<Position> = HashSet::new();
    for mv in moves {
        rope.make_move(mv);
        tail_visited.insert(rope.get_tail());
    }
    tail_visited.len()
}

impl Solution for Day09 {

    fn part1(lines: Lines) {
        let moves = parse(lines);
        let rope = Rope::new();
        let ans = run_simulation(rope, moves);

        println!("{}", ans);
    }

    fn part2(lines: Lines) {
        let moves = parse(lines);
        let rope = LongRope::new();
        let ans = run_simulation(rope, moves);

        println!("{}", ans);
    }
}
