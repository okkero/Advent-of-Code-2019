use std::io::BufRead;

use crate::puzzle::{Puzzle, Solution, RawInputReader, RawInput, PuzzleInputReader};
use crate::puzzle;

puzzle!("The Tyranny of the Rocket Equation", RawInput, RawInputReader);

fn part1(input: &mut dyn BufRead) {
    let sum: u32 =
        input
            .lines()
            .map(|line| {
                let mass = line.unwrap().parse::<u32>().unwrap();
                calculate_fuel(mass)
            })
            .sum();
    println!("Fuel requirements: {}", sum);
}

fn part2(input: &mut dyn BufRead) {
    let sum: u32 =
        input
            .lines()
            .map(|line| {
                let mass = line.unwrap().parse::<u32>().unwrap();
                let mut required_fuel = calculate_fuel(mass);
                let mut previous_fuel = required_fuel;
                loop {
                    let additional_fuel = calculate_fuel(previous_fuel);
                    if additional_fuel == 0 {
                        break;
                    }

                    required_fuel += additional_fuel;
                    previous_fuel = additional_fuel;
                }

                required_fuel
            })
            .sum();

    println!("Fuel requirements: {}", sum);
}

fn calculate_fuel(mass: u32) -> u32 {
    if mass < 9 {
        return 0;
    }

    mass / 3 - 2
}