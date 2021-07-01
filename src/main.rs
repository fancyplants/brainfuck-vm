use std::{
    collections::HashMap,
    convert::TryFrom,
    io::{self, Read, Write},
    env
};

// static HELLO_WORLD: &'static str = "++++++++[>++++[>++>+++>+++>+<<<<-]>+>+>->>+[<]<-]>>.>---.+++++++..+++.>>.<-.<.+++.------.--------.>>+.>++.";

fn main() {
    let mut args = env::args();
    if let Some(s) = args.nth(1) {
        let res = run_program(&s);
    
        if let Err(err) = res {
            println!("Error encountered: {:?}", err);
        }
    } else {
        println!("Need to provide brainfuck program.");
    }
}

fn run_program(s: &str) -> Result<(), Error> {
    let prog = Program::try_from(s)?;
    prog.run_program()
}

#[derive(Debug, PartialEq)]
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

struct Program {
    ops: Vec<Operation>,
    // hashmap connecting indices of brackets
    bracket_map: HashMap<usize, usize>,
}

#[derive(Debug)]
enum Error {
    BracketMismatch,
}

fn get_bracket_map(ops: &[Operation]) -> Result<HashMap<usize, usize>, Error> {
    let mut map = HashMap::new();
    let mut stack = Vec::new();

    for (i, op) in ops.iter().enumerate() {
        match op {
            Operation::LeftBracket => {
                stack.push(i);
            }
            Operation::RightBracket => {
                if let Some(left_index) = stack.pop() {
                    // map is "double-sided"
                    map.insert(left_index, i);
                    map.insert(i, left_index);
                } else {
                    return Err(Error::BracketMismatch);
                }
            }

            _ => (),
        }
    }

    if !stack.is_empty() {
        return Err(Error::BracketMismatch);
    }

    Ok(map)
}

impl Program {
    fn run_program(&self) -> Result<(), Error> {
        let bracket_map = &self.bracket_map;

        let stdout = io::stdout();
        let stdin = io::stdin();

        // usually the brainfuck memory span is 30000
        let mut data = [0u8; 30_000];
        let mut data_index = 0usize;

        let mut op_ptr = 0usize;
        let prog_len = self.ops.len();

        use Operation::*;

        while op_ptr < prog_len {
            match self.ops[op_ptr] {
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
                    data[data_index] = stdin
                        .lock()
                        .bytes()
                        .next()
                        .and_then(|res| res.ok())
                        .unwrap_or(data[data_index]);
                }
                LeftBracket => {
                    let byte = data[data_index];
                    if byte == 0 {
                        let right_bracket = bracket_map.get(&op_ptr).unwrap();
                        op_ptr = *right_bracket;
                    }
                }
                RightBracket => {
                    let byte = data[data_index];
                    if byte != 0 {
                        let left_bracket = bracket_map.get(&op_ptr).unwrap();
                        op_ptr = *left_bracket;
                    }
                }
            };
            op_ptr += 1;
        }

        Ok(())
    }
}

impl TryFrom<&str> for Program {
    type Error = Error;

    fn try_from(s: &str) -> Result<Self, Self::Error> {
        let arr: Vec<_> = s.chars().filter_map(Operation::from_char).collect();
        let bracket_map = get_bracket_map(&arr)?;

        let prog = Program {
            ops: arr,
            bracket_map,
        };

        Ok(prog)
    }
}
