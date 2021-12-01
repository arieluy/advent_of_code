use advent_of_code::Solution;
use std::str::Lines;

struct Game {
    opponent: i32,
    me: i32
}

impl Game {
    fn new(opponent_: &str, me_: &str) -> Game {
        let opponent: i32 = match opponent_ {
                                "A" => 0,
                                "B" => 1,
                                "C" => 2,
                                 _ => panic!()
                            };
        let me: i32 = match me_ {
                        "X" => 0,
                        "Y" => 1,
                        "Z" => 2,
                         _ => panic!()
                        };
        Game { opponent: opponent, me: me }
    }

    fn new2(opponent_: &str, me_: &str) -> Game {
        let opponent: i32 = match opponent_ {
                                "A" => 0,
                                "B" => 1,
                                "C" => 2,
                                 _ => panic!()
                            };
        let me: i32 = match me_ {
                        "X" => (opponent + 2) % 3,
                        "Y" => opponent,
                        "Z" => (opponent + 1) % 3,
                         _ => panic!()
                        };
        Game { opponent: opponent, me: me }
    }

    fn count_points(&self) -> i32 {
        let mut points = 0;
        if (self.me - self.opponent + 3) % 3 == 1 {
            points += 6;
        } else if self.opponent == self.me {
            points += 3;
        }

        points += self.me + 1;
        return points;
    }
}

pub struct Day02 {}

impl Solution for Day02 {

    fn part1(lines: Lines) {
        let mut sum = 0;
        for line in lines {
            let game = Game::new(&line[0..1], &line[2..3]);
            sum += game.count_points();
        }
        
        println!("{}", sum);
    }


    fn part2(lines: Lines) {
        let mut sum = 0;
        for line in lines {
            let game = Game::new2(&line[0..1], &line[2..3]);

            sum += game.count_points();
        }
        
        println!("{}", sum);
    }
}