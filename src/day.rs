pub struct Day {
    pub title: &'static str,
    pub solution: Solution,
}

pub struct Solution {
    pub part1: fn(),
    pub part2: fn(),
}