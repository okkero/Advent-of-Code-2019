use std::io::BufRead;
use std::marker::PhantomData;

pub struct Puzzle<'a, Input: 'a, InputReader: PuzzleInputReader<'a, Input>> {
    pub title: &'static str,
    pub solution: Solution<Input>,
    pub input_reader: InputReader,
    _phantom: PhantomData<&'a ()>
}

impl<Input, InputReader> Puzzle<Input, InputReader> {
    pub fn new(title: &'static str, solution: Solution<Input>, input_reader: InputReader) -> Self {
        Puzzle {
            title,
            solution,
            input_reader,
            _phantom: PhantomData
        }
    }
}

pub struct Solution<Input> {
    pub part1: fn(input: Input),
    pub part2: fn(input: Input),
}

pub trait PuzzleInputReader<'a, ParsedInput: 'a> {
    fn read_input(&self, input: &'a mut dyn BufRead) -> ParsedInput;
}

pub struct RawInputReader;

impl<'a> PuzzleInputReader<'a, &'a mut (dyn BufRead + 'a)> for RawInputReader {
    fn read_input(&self, input: &'a mut (dyn BufRead + 'a)) -> &'a mut (dyn BufRead + 'a) {
        input
    }
}

pub type RawInput<'a> = &'a mut (dyn BufRead + 'a);

#[macro_export]
macro_rules! use_puzzle {
    ($module:ident) => {
        Box::new($module::PUZZLE)
    }
}

#[macro_export]
macro_rules! use_puzzles {
    [$($module:ident),*] => {
        [$(use_puzzle!($module)),*]
    }
}

#[macro_export]
macro_rules! puzzle {
    ($title:literal, $input:ty, $input_reader:expr) => {
        pub const PUZZLE: Puzzle<$input, $input_reader> = Puzzle::new(
            $title,
            Solution {
                part1,
                part2,
            },
            $input_reader
        );
    }
}