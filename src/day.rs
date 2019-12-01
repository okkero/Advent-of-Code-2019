use std::io::BufRead;

pub struct Day {
    pub title: &'static str,
    pub solution: Solution,
}

pub struct Solution {
    pub part1: fn(input: &mut dyn BufRead),
    pub part2: fn(input: &mut dyn BufRead),
}