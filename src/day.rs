pub struct Day {
    pub title: &'static str,
    pub solution: Solution,
}

pub struct Solution {
    pub part1: fn(input: &str),
    pub part2: fn(input: &str),
}