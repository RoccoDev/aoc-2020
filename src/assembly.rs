use std::{
    cell::Cell,
    sync::atomic::{AtomicBool, Ordering},
};

use Opcode::*;

#[derive(Default)]
pub struct Program {
    instructions: Vec<Instruction>,
    instruction_ptr: Cell<isize>,
    accumulator: Cell<isize>,
    stop: AtomicBool,
}

#[derive(Ord, PartialOrd, Eq, PartialEq, Hash, Copy, Clone, Debug)]
pub struct Instruction {
    pub opcode: Opcode,
    pub arg: isize,
}

#[derive(Ord, PartialOrd, Eq, PartialEq, Hash, Copy, Clone, Debug)]
pub enum Opcode {
    Accumulate,
    Jump,
    NoOp,
}

impl Instruction {
    /// Returns whether to increment the instruction pointer by 1
    fn run(&self, program: &Program) -> bool {
        match self.opcode {
            Accumulate => {
                program
                    .accumulator
                    .set(program.accumulator.get() + self.arg);
                true
            }
            Jump => {
                program
                    .instruction_ptr
                    .set(program.instruction_ptr.get() + self.arg);
                false
            }
            NoOp => true,
        }
    }

    fn from(opcode: &str, arg: &str) -> Instruction {
        Instruction {
            arg: arg.parse().unwrap(),
            opcode: match opcode {
                "acc" => Accumulate,
                "jmp" => Jump,
                "nop" => NoOp,
                _ => panic!("Invalid opcode"),
            },
        }
    }
}

impl Program {
    pub fn with(instructions: Vec<Instruction>) -> Program {
        Program {
            instructions,
            ..Default::default()
        }
    }

    pub fn from_input(input: &str) -> Program {
        let insts = input
            .lines()
            .map(|l| {
                let mut parts = l.split(' ');
                let opcode = parts.next().unwrap();
                let arg = parts.next().unwrap();
                Instruction::from(opcode, arg)
            })
            .collect();
        Self::with(insts)
    }

    pub fn run<F>(&self, callback: F) -> i32
    where
        F: Fn(&Instruction),
    {
        while !self.stop.load(Ordering::Relaxed) {
            let inst_ptr = self.instruction_ptr.get();
            if inst_ptr as usize >= self.instructions.len() {
                return 0; // Program terminated naturally
            }
            let inst = &self.instructions[inst_ptr as usize];
            callback(inst);
            let update_ptr = inst.run(self);
            if update_ptr {
                self.instruction_ptr.set(inst_ptr + 1);
            }
        }
        1 // Program was manually stopped
    }

    pub fn stop(&self) {
        self.stop.store(true, Ordering::Relaxed);
    }

    pub fn accumulator(&self) -> isize {
        self.accumulator.get()
    }

    pub fn instruction_ptr(&self) -> isize {
        self.instruction_ptr.get()
    }

    pub fn instructions(&self) -> &[Instruction] {
        return &self.instructions;
    }
}
