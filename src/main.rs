use std::env;

pub mod backends;
pub mod lexer;
pub mod parser;

#[derive(Clone, Debug)]
pub enum Tokentype {
    IncrementPointer, // >
    DecrementPointer, // <
    Increment,        // +
    Decrement,        // -
    Input,            // ,
    Output,           // .
    LoopBegin,        // [
    LoopEnd,          // ]
}

pub enum Instruction {
    IncrementPointer, // >
    DecrementPointer, // <
    Increment,        // +
    Decrement,        // -
    Input,            // ,
    Output,
    Loop(Vec<Instruction>),
}

pub fn main() {
    
}
