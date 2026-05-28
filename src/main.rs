pub mod lexer;
pub mod parser;

pub enum Tokentype {
     IncreamentPointer, // >
     DecreamentPointer, // <
     Increament,  // +
     Decreament,  // -
     Input, // ,
     Output, // .
     LoopBegin, // [
     LoopEnd // ]
    
}

pub enum Instruction {
    IncreamentPointer, // >
    DecreamentPointer, // <
    Increament,  // +
    Decreament,  // -
    Input, // ,
    Output, 
    Loop(Vec<Instruction>)
}

fn main() {
    println!("Hello, world!");
}
