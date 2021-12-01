#![allow(dead_code, unused_variables)]

mod aoc22;
use advent_of_code::Solution;
use clap::Parser;

#[derive(Parser)]
struct Cli {
    #[arg(short, long, value_parser = clap::value_parser!(u8).range(1..=2))]
    part: u8,
    #[arg(short = 's', long)]
    use_sample_input: bool
}

fn main() {
    let args = Cli::parse();
	aoc22::day04::Day04::run(args.part, args.use_sample_input);
}