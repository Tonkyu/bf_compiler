use std::io::{self, Read, Write};
use std::{env, fs, process};

#[derive(Debug, PartialEq, Clone)]
enum Op{
    Add(u8),
    Move(isize),
    Output,
    Input,
    Loop(Vec<Op>),
}

fn parse(code: &str) -> Vec<Op>{
    let mut ops = Vec::new();
    let chars: Vec<char> = code.chars().collect();
    let mut i = 0;

    while i < chars.len() {
        match chars[i] {
            '+' | '-' => {
                let mut count = 0i32;
                while i < chars.len() && (chars[i] == '+' || chars[i] == '-') {
                    if chars[i] == '+' { count += 1; } else { count -= 1; }
                    i += 1;
                }
                ops.push(Op::Add(count as u8));
                continue;   
            }
            '>' | '<' => {
                let mut offset = 0isize;
                while i < chars.len() && (chars[i] == '>' || chars[i] == '<') {
                    if chars[i] == '>' { offset += 1; } else { offset -= 1; }
                    i += 1;
                }
                ops.push(Op::Move(offset));
                continue;
            }
            '.' => ops.push(Op::Output),
            ',' => ops.push(Op::Input),
            '[' => {
                let start = i + 1;
                let mut depth = 1;
                i += 1;
                while i < chars.len() && depth > 0 {
                    if chars[i] == '[' { depth += 1; }
                    if chars[i] == ']' { depth -= 1; }
                    i += 1;
                }
                let inner = &code[start..i - 1];
                ops.push(Op::Loop(parse(inner)));
                continue;
            }
            _ => {}
        }
        i += 1;
    }
    ops
}

pub struct Interpreter {
    memory: Vec<u8>,
    ptr: usize,
}

impl Interpreter {
    fn new() -> Self {
        Self {
            memory: vec![0; 300000],
            ptr: 0,
        }
    }

    fn run(&mut self, ops: &[Op]) {
        for op in ops {
            match op {
                Op::Add(val) => {
                    self.memory[self.ptr] = self.memory[self.ptr].wrapping_add(*val);
                }
                Op::Move(offset) => {
                    if *offset >= 0 {
                        self.ptr += *offset as usize;
                    } else {
                        self.ptr -= (-offset) as  usize;
                    }
                }
                Op::Output => {
                    io::stdout().write_all(&[self.memory[self.ptr]]).unwrap();
                }
                Op::Input => {
                    let mut buf = [0u8; 1];
                    if io::stdin().read_exact(&mut buf).is_ok() {
                        self.memory[self.ptr] = buf[0];
                    }
                }
                Op::Loop(inner_ops) => {
                    while self.memory[self.ptr] != 0 {
                        self.run(inner_ops);
                    }
                }
            }
        }
    }
}

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() < 2 {
        eprintln!("Usage: {} <path-to-bf-file>", args[0]);
        process::exit(1);
    }

    let file_path = &args[1];
    let code = match fs::read_to_string(file_path) {
        Ok(content) => content,
        Err(err) => {
            eprintln!("Error: Failed to read file '{}': {}", file_path, err);
            process::exit(1);
        }
    };

    let ops = parse(&code);
    let mut interpreter = Interpreter::new();
    interpreter.run(&ops);
}