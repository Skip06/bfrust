use crate::Instruction;
use std::io::{self, Read};

pub fn run(ast: &Vec<Instruction>, array: &mut Vec<u8>, dp: &mut usize) {
    for instruction in ast {
        match instruction {
            Instruction::IncrementPointer => *dp += 1,
            Instruction::DecrementPointer => *dp -= 1,
            Instruction::Increment => array[*dp] = array[*dp].wrapping_add(1),
            Instruction::Decrement => array[*dp] = array[*dp].wrapping_sub(1),
            Instruction::Output => print!("{}", array[*dp] as char),
            Instruction::Input => {
                let mut buffer = [0; 1];
                if io::stdin().read_exact(&mut buffer).is_ok() {
                    array[*dp] = buffer[0];
                }
            }
            Instruction::Loop(body) => {
                while array[*dp] != 0 {
                    run(&body, array, dp);
                }
            }
        }
    }
}
