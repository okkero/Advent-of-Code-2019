use std::error::Error;
use std::fmt::{Display, Formatter};
use std::fmt;
use std::io::BufRead;
use std::ops::Range;

use crate::day::{Day, Solution};

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

#[derive(PartialEq, Clone, Copy, Debug)]
enum Axis {
    X,
    Y,
}

struct Step {
    axis: Axis,
    length: isize,
}

struct RunIntersection {
    global_coords: (isize, isize),
    run1_offset: usize,
    run2_offset: usize,
}

#[derive(Debug)]
struct Run {
    along_axis: Axis,
    lateral_offset: isize,
    interval: (isize, usize),
}

impl Run {
    fn interval_range(&self) -> Range<isize> {
        let (start, length) = self.interval;
        start + 1..start + length as isize - 1
    }

    fn intersects(&self, other: &Run) -> Option<RunIntersection> {
        if self.along_axis == other.along_axis {
            return None;
        }

        let other_crosses_self_lateral = other.interval_range().contains(&self.lateral_offset);
        let self_crosses_other_lateral = self.interval_range().contains(&other.lateral_offset);

        if !other_crosses_self_lateral || !self_crosses_other_lateral {
            return None;
        }

        let global_coords =
            match self.along_axis {
                Axis::X => (other.lateral_offset, self.lateral_offset),
                Axis::Y => (self.lateral_offset, other.lateral_offset)
            };

        Some(RunIntersection {
            global_coords,
            run1_offset: (other.lateral_offset - self.interval.0) as usize,
            run2_offset: (self.lateral_offset - other.interval.0) as usize,
        })
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
    let (wire1, wire2) = parse_wires(input);

    let closest_intersection =
        wire1.runs.iter()
            .flat_map(|run1|
                wire2.runs.iter()
                    .filter_map(move |run2|
                        run1.intersects(run2)))
            .map(|intersection| intersection.global_coords)
            .min_by_key(|(x, y)| x.abs() + y.abs())
            .expect("Unable to find an intersection");

    let (x, y) = closest_intersection;
    println!("Closest intersection: ({}, {})", x, y);
    println!("Manhattan distance to origin: {}", x.abs() + y.abs());
}

fn part2(input: &mut dyn BufRead) {
    let (wire1, wire2) = parse_wires(input);

    let mut steps1 = 0;
    let shortest_path_intersection =
        wire1.runs.iter()
            .flat_map(|run1| {
                let mut steps2 = 0;
                let res =
                    wire2.runs.iter()
                        .filter_map(move |run2| {
                            let res =
                                if let Some(intersection) = run1.intersects(run2) {
                                    Some((intersection.global_coords, steps1 + intersection.run1_offset, steps2 + intersection.run2_offset))
                                } else {
                                    None
                                };
                            steps2 += run2.interval.1;
                            res
                        });
                steps1 += run1.interval.1;
                res
            });
            //.min_by_key(|(steps1, steps2)| steps1 + steps2)
            //.expect("Unable to find an intersection");

    //let (steps1, steps2) = shortest_path_intersection;
    for (coords, steps1, steps2) in shortest_path_intersection {
        println!("First reached intersection - Wire 1: {} steps, Wire 2: {} steps", steps1, steps2);
        println!("Coords: ({}, {})", coords.0, coords.1);
        println!("Answer: {}", steps1 + steps2);
    }
}

fn parse_wires(input: &mut dyn BufRead) -> (Wire, Wire) {
    let wire_steps_vec =
        input
            .lines()
            .map(|r| parse_wire_steps(&r?))
            .collect::<Result<Vec<_>, Box<dyn Error>>>()
            .expect("Unable to parse input");
    let wire1 = Wire::trace(&wire_steps_vec[0]);
    let wire2 = Wire::trace(&wire_steps_vec[1]);

    (wire1, wire2)
}

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