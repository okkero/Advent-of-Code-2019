use day::Day;
use std::env;

mod day;
mod day1;

const DAYS: [Day; 1] =
    [day1::DAY1];

fn main() {
    let args = env::args();
    let day_string = args.skip(1).next().expect("Missing day argument");
    let day_num: usize = day_string.parse().expect("Unable to parse day");
    let day = DAYS.get(day_num - 1).expect("Invalid day");

    run_day(day_num, &day);
}

fn run_day(day_num: usize, day: &Day) {
    println!("--- Day {}: {} ---", day_num, day.title);
    println!();
    println!("Part 1:");
    (day.solution.part1)();
    println!();
    println!("Part 2:");
    (day.solution.part2)();
}