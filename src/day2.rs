use std::error::Error;
use std::io::BufRead;

use crate::day::{Day, Solution};

pub const DAY2: Day = Day {
    title: "1202 Program Alarm",
    solution: Solution {
        part1,
        part2,
    },
};

enum Instruction {
    Add(usize, usize, usize),
    Mul(usize, usize, usize),
    Halt,
}

fn part1(input: &mut dyn BufRead) {
    let mut program = read_program(input).expect("Unable to read program");
    restore_program(&mut program);

    let mut instruction_pointer = 0;
    let result =
        loop {
            let instruction = parse_instruction(&program[instruction_pointer..])
                .expect("Unable to parse instruction");
            let halted = perform_instruction(&mut program, instruction);
            if halted {
                break program[0];
            }

            instruction_pointer += 4;
        };

    println!("Result: {}", result);
}

fn part2(input: &mut dyn BufRead) {}

fn read_program(input: &mut dyn BufRead) -> Result<Vec<usize>, Box<dyn Error>> {
    input
        .split(b',')
        .map(|r| {
            Ok(String::from_utf8(r?)?.parse()?)
        })
        .collect()
}

fn parse_instruction(slice: &[usize]) -> Result<Instruction, usize> {
    let opcode = slice[0];
    match opcode {
        1 => Ok(Instruction::Add(slice[1], slice[2], slice[3])),
        2 => Ok(Instruction::Mul(slice[1], slice[2], slice[3])),
        99 => Ok(Instruction::Halt),
        _ => Err(opcode)
    }
}

fn perform_instruction(program: &mut [usize], instruction: Instruction) -> bool {
    match instruction {
        Instruction::Add(a, b, address) => program[address] = program[a] + program[b],
        Instruction::Mul(a, b, address) => program[address] = program[a] * program[b],
        Instruction::Halt => return true
    }

    false
}

fn restore_program(program: &mut [usize]) {
    program[1] = 12;
    program[2] = 2;
}