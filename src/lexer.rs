use crate::{Instruction, Tokentype};

//this just tokenise the tokentypes to a string from the source to a vec of tokentypes
pub fn lexer(source: String) -> Vec<Tokentype> {
    let mut lex = Vec::new();

    for char in source.chars() {
        let op = match char {
            '>' => Some(Tokentype::IncrementPointer),
            '<' => Some(Tokentype::DecrementPointer),
            '+' => Some(Tokentype::Increment),
            '-' => Some(Tokentype::Decrement),
            ',' => Some(Tokentype::Input),
            '.' => Some(Tokentype::Output),
            '[' => Some(Tokentype::LoopBegin),
            ']' => Some(Tokentype::LoopEnd),
            _ => None,
        };

        match op {
            Some(op) => lex.push(op),
            None => (),
        }
    }

    lex
}
