use std::error::Error;
use std::io::BufRead;

use crate::puzzle::{Puzzle, PuzzleInputReader, RawInput, RawInputReader, Solution};
use crate::puzzle;

puzzle!("1202 Program Alarm", RawInput, RawInputReader);

struct Program(Vec<usize>);

impl Program {
    fn read(input: &mut dyn BufRead) -> Result<Program, Box<dyn Error>> {
        let program: Result<_, Box<dyn Error>> = input
            .split(b',')
            .map(|r| {
                Ok(String::from_utf8(r?)?.parse()?)
            })
            .collect();

        Ok(Program(program?))
    }

    fn run(&self, noun: usize, verb: usize) -> usize {
        let mut state = self.0.clone();
        state[1] = noun;
        state[2] = verb;

        let mut instruction_pointer = 0;
        let result =
            loop {
                let instruction = parse_instruction(&state[instruction_pointer..])
                    .expect("Unable to parse instruction");
                let halted = perform_instruction(&mut state, instruction);
                if halted {
                    break state[0];
                }

                instruction_pointer += 4;
            };

        result
    }
}

enum Instruction {
    Add(usize, usize, usize),
    Mul(usize, usize, usize),
    Halt,
}

fn part1(input: &mut dyn BufRead) {
    let program = Program::read(input).expect("Unable to read program");
    let result = program.run(12, 2);

    println!("Result: {}", result);
}

fn part2(input: &mut dyn BufRead) {
    const WANTED_OUTPUT: usize = 19690720;
    let program = Program::read(input).expect("Unable to read program");

    for noun in 0..=99 {
        for verb in 0..=99 {
            let result =
                program.run(noun, verb);

            if result == WANTED_OUTPUT {
                println!("Noun: {}, Verb: {}", noun, verb);
                println!("Answer: {}", 100 * noun + verb);
                break;
            }
        }
    }
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