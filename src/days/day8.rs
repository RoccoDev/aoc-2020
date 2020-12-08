use fxhash::FxHashSet;
use std::sync::{mpsc, Mutex};

use crate::assembly::{Instruction, Opcode, Program};

fn run_program(program: &Program) -> (i32, isize) {
    let (tx, rx) = mpsc::channel();
    let known = Mutex::new(FxHashSet::default());
    let status = program.run(|_inst| {
        if !known.lock().unwrap().insert(program.instruction_ptr()) {
            program.stop();
            tx.send(program.accumulator()).unwrap();
        }
        return;
    });
    (
        status,
        rx.try_recv().unwrap_or_else(|_e| program.accumulator()),
    )
}

fn change_op(insts: &[Instruction], from: Opcode, to: Opcode) -> Option<isize> {
    insts
        .iter()
        .enumerate()
        .filter_map(|(i, inst)| if inst.opcode == from { Some(i) } else { None })
        .filter_map(|idx| {
            let mut new_insts = insts.to_vec();
            let arg = new_insts[idx].arg;
            new_insts[idx] = Instruction { opcode: to, arg };
            let (status, res) = run_program(&Program::with(new_insts));
            if status == 0 {
                Some(res)
            } else {
                None
            }
        })
        .next()
}

#[aoc(day8, part1)]
fn part1(input: &str) -> isize {
    let program = Program::from_input(input);
    run_program(&program).1
}

#[aoc(day8, part2)]
fn part2(input: &str) -> isize {
    let program = Program::from_input(input);
    let insts = program.instructions();
    change_op(insts, Opcode::NoOp, Opcode::Jump)
        .unwrap_or_else(|| change_op(insts, Opcode::Jump, Opcode::NoOp).unwrap())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part1_example() {
        let input = "nop +0
acc +1
jmp +4
acc +3
jmp -3
acc -99
acc +1
jmp -4
acc +6";
        assert_eq!(part1(input), 5);
    }

    #[test]
    fn part2_example() {
        let input = "nop +0
acc +1
jmp +4
acc +3
jmp -3
acc -99
acc +1
jmp -4
acc +6";
        assert_eq!(part2(input), 8);
    }
}
