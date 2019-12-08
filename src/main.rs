use std::env;
use std::fs::File;
use std::io::{BufReader, Seek, SeekFrom};

use puzzle::Puzzle;
use std::any::Any;

mod puzzle;
mod day1;
mod day2;
mod day3;

const PUZZLES: [Box<dyn Any>; 3] =
    use_puzzles![day1, day2, day3];

fn main() {
    let args = env::args();
    let day_string = args.skip(1).next().expect("Missing day argument");
    let day: usize = day_string.parse().expect("Unable to parse day");

    run_puzzle(day);
}

fn run_puzzle(day: usize) {
    let puzzle = PUZZLES.get(day - 1).expect("Invalid day");
    let mut input_file = File::open(format!("input/day{}.txt", puzzle)).expect("Input file not found");

    println!("--- Day {}: {} ---", puzzle, puzzle.title);
    println!();
    println!("Part 1:");
    (puzzle.solution.part1)(&mut BufReader::new(&mut input_file));
    println!();
    input_file.seek(SeekFrom::Start(0)).unwrap();
    println!("Part 2:");
    (puzzle.solution.part2)(&mut BufReader::new(&mut input_file));
}