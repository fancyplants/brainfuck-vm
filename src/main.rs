use std::io::{self, Read, Write};

fn main() {
    let program = Program::from(",.");
    program.run_program();
}

#[derive(Debug)]
enum Operation {
    IncrementDp,
    DecrementDp,
    IncrementByte,
    DecrementByte,
    OutputByte,
    InputByte,
    LeftBracket,
    RightBracket,
}

impl Operation {
    fn from_char(c: char) -> Option<Operation> {
        use Operation::*;
        match c {
            '>' => Some(IncrementDp),
            '<' => Some(DecrementDp),
            '+' => Some(IncrementByte),
            '-' => Some(DecrementByte),
            '.' => Some(OutputByte),
            ',' => Some(InputByte),
            '[' => Some(LeftBracket),
            ']' => Some(RightBracket),
            _ => None,
        }
    }
}

struct Program(Vec<Operation>);

impl Program {
    fn run_program(&self) {
        let stdout = io::stdout();
        let stdin = io::stdin();

        // usually the brainfuck memory span is 30000
        let mut data = [0u8; 30_000];
        let mut data_index = 0usize;

        let mut input_buffer = [0u8; 1];

        use Operation::*;
        for o in &self.0 {
            match o {
                IncrementDp => data_index = data_index.wrapping_add(1),
                DecrementDp => data_index = data_index.wrapping_sub(1),
                IncrementByte => data[data_index] = data[data_index].wrapping_add(1),
                DecrementByte => data[data_index] = data[data_index].wrapping_sub(1),
                OutputByte => {
                    stdout
                        .lock()
                        .write(&data[data_index..data_index + 1])
                        .expect("Unable to write data to screen");
                }
                InputByte => {
                    stdin
                        .lock()
                        .read_exact(&mut input_buffer)
                        .expect("Unable to read byte");
                    data[data_index] = input_buffer[0];
                }
                _ => (),
                // LeftBracket =>
            };
        }
    }
}

impl From<&str> for Program {
    fn from(s: &str) -> Program {
        Program(str_to_ops(s))
    }
}

fn str_to_ops(s: &str) -> Vec<Operation> {
    s.chars()
        .into_iter()
        .filter_map(|c| Operation::from_char(c))
        .collect()
}
