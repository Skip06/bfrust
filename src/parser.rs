use crate::{Instruction, Tokentype};

// so the Vec<Instruction> is basically AST here. it just says in what order it should execute.
pub fn parser(lex: Vec<Tokentype>) -> Vec<Instruction> {
    let mut ast = Vec::new();

    let mut loop_start = 0;
    let mut loop_stack = 0;

    for (i, op) in lex.iter().enumerate() {
        if loop_stack == 0 {
            let instruction = match op {
                Tokentype::IncrementPointer => Some(Instruction::IncrementPointer),
                Tokentype::DecrementPointer => Some(Instruction::DecrementPointer),
                Tokentype::Increment => Some(Instruction::Increment),
                Tokentype::Decrement => Some(Instruction::Decrement),
                Tokentype::Input => Some(Instruction::Input),
                Tokentype::Output => Some(Instruction::Output),

                Tokentype::LoopBegin => {
                    loop_stack += 1;
                    loop_start = i;
                    None
                }
                Tokentype::LoopEnd => {
                    panic!("invalid brainfuck program as no [ to match your ]");
                }
            };
            match instruction {
                Some(instruction) => ast.push(instruction),
                None => (),
            }
        } else {
            match op {
                Tokentype::LoopBegin => {
                    loop_stack += 1;
                }
                Tokentype::LoopEnd => {
                    loop_stack -= 1;

                    if loop_stack == 0 {
                        // recursively parsing the part inside loop//
                        ast.push(Instruction::Loop(parser(lex[loop_start + 1..i].to_vec())));
                    }
                }
                _ => (),
            }
        }
    }
    ast
}

//EXAMPLE// => ++>+[++<[--+]-]

// [
//     Instruction::Increment,
//     Instruction::Increment,
//     Instruction::IncrementPointer,
//     Instruction::Increment,
//     Instruction::Loop([
//         Instruction::Increment,
//         Instruction::Increment,
//         Instruction::DecrementPointer,
//         Instruction::Loop([
//             Instruction::Decrement,
//             Instruction::Decrement,
//             Instruction::Increment
//         ]),
//         Instruction::Decrement
//     ]),
//     Instruction::Decrement
// ]
