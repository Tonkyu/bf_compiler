use std::io::{self, Read, Write};

use crate::parser::Op;

pub const TAPE_SIZE: usize = 300_000;

pub struct Interpreter {
    memory: Vec<u8>,
    ptr: usize,
}

impl Interpreter {
    pub fn new() -> Self {
        Self {
            memory: vec![0; TAPE_SIZE],
            ptr: 0,
        }
    }

    pub fn run(&mut self, ops: &[Op]) {
        for op in ops {
            match op {
                Op::Add(val) => {
                    self.memory[self.ptr] = self.memory[self.ptr].wrapping_add(*val);
                }
                Op::Move(offset) => {
                    self.ptr = self.ptr.checked_add_signed(*offset).expect("tape pointer underflow");
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
