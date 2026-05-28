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
    let args: Vec<String> = env::args().collect(); // reads cmd line arguments
    if args.len() < 2 {
        println!("forgot to pass brainfuck code!!!!!"); 
        return;
    }

    let source = args[1].clone();   //args[0] is the name of binary
    let tokens = lexer::lexer(source);
    let ast = parser::parser(tokens);

    let mut array: Vec<u8> = vec![0; 30000];
    let mut dp: usize = 0;

    backends::interpreter::run(&ast, &mut array, &mut dp);
}
