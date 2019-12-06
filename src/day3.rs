use std::collections::HashMap;
use std::error::Error;
use std::fmt::{Display, Formatter};
use std::fmt;
use std::io::BufRead;

use crate::day::{Day, Solution};
use std::mem::size_of;

pub const DAY3: Day = Day {
    title: "Crossed Wires",
    solution: Solution {
        part1,
        part2,
    },
};

#[derive(Debug)]
struct InvalidDirectionError(char);

impl Display for InvalidDirectionError {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result<(), fmt::Error> {
        write!(f, "{:?}", self)
    }
}

impl Error for InvalidDirectionError {}

#[derive(Clone, Copy, Debug)]
enum Axis {
    X,
    Y,
}

struct Step {
    axis: Axis,
    length: isize,
}

#[derive(Debug)]
struct Run {
    along_axis: Axis,
    lateral_offset: isize,
    interval: (isize, usize),
}

impl Run {
    fn visited_cell(&self, x: isize, y: isize) -> bool {
        let (index_along, index_lateral) =
            match self.along_axis {
                Axis::X => (x, y),
                Axis::Y => (y, x)
            };
        let (interval_start, interval_length) = self.interval;
        let interval_end = interval_start + interval_length as isize;

        let on_lateral_offset = index_lateral == self.lateral_offset;
        let within_interval = index_along > interval_start && index_along < interval_end;

        on_lateral_offset && within_interval
    }
}

#[derive(Debug)]
struct Wire {
    runs: Vec<Run>
}

impl Wire {
    fn trace(steps: &[Step]) -> Wire {
        let mut current_coords = (0, 0);
        let mut runs = Vec::with_capacity(steps.len());
        for step in steps {
            let (current_x, current_y) = current_coords;
            let (lateral_offset, interval_start) =
                match step.axis {
                    Axis::X => (current_y, current_x),
                    Axis::Y => (current_x, current_y)
                };
            let (interval_start, interval_length) =
                if step.length < 0 {
                    (interval_start + step.length, -step.length)
                } else {
                    (interval_start, step.length)
                };

            runs.push(Run {
                along_axis: step.axis,
                lateral_offset,
                interval: (interval_start, interval_length as usize),
            });

            current_coords =
                match step.axis {
                    Axis::X => (current_x + step.length, current_y),
                    Axis::Y => (current_x, current_y + step.length)
                }
        }

        Wire { runs }
    }
}

fn part1(input: &mut dyn BufRead) {
    let wire_steps_vec =
        input
            .lines()
            .map(|r| parse_wire_steps(&r?))
            .collect::<Result<Vec<_>, Box<dyn Error>>>()
            .expect("Unable to parse input");

    let wire1 = Wire::trace(&wire_steps_vec[0]);
    let wire2 = Wire::trace(&wire_steps_vec[1]);

    println!("{:#?}", wire1);
}

fn part2(input: &mut dyn BufRead) {}

fn parse_wire_steps(input: &str) -> Result<Vec<Step>, Box<dyn Error>> {
    input
        .split(',')
        .map(parse_step)
        .collect()
}

fn parse_step(input: &str) -> Result<Step, Box<dyn Error>> {
    let direction_char = input.chars().next().unwrap();
    let magnitude = input[1..].parse::<isize>()?;
    let (axis, direction) =
        match direction_char {
            'U' => (Axis::Y, 1),
            'R' => (Axis::X, 1),
            'D' => (Axis::Y, -1),
            'L' => (Axis::X, -1),
            _ => return Err(InvalidDirectionError(direction_char).into())
        };

    Ok(Step {
        axis,
        length: magnitude * direction,
    })
}