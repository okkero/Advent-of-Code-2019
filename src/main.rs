use std::env;
use std::fs::File;
use std::io::Read;

use day::Day;

mod day;
mod day1;

const DAYS: [Day; 1] =
    [day1::DAY1];

fn main() {
    let args = env::args();
    let day_string = args.skip(1).next().expect("Missing day argument");
    let day_num: usize = day_string.parse().expect("Unable to parse day");

    run_day(day_num);
}

fn run_day(day_num: usize) {
    let day = DAYS.get(day_num - 1).expect("Invalid day");
    let mut input_file = File::open(format!("input/day{}.txt", day_num)).expect("Input file not found");
    let mut input = String::new();
    input_file.read_to_string(&mut input).expect("Unable to read input");

    println!("--- Day {}: {} ---", day_num, day.title);
    println!();
    println!("Part 1:");
    (day.solution.part1)(&input);
    println!();
    println!("Part 2:");
    (day.solution.part2)(&input);
}