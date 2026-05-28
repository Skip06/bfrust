# Brainfuck Interpreter in Rust

A simple Brainfuck interpreter that supports standard operations and nested loops.
To run a program, use: `cargo run "<your_brainfuck_code>"`
Example: `cargo run "++++++[>++++++++++<-]>+++++."` (outputs 'A')
The interpreter features a 30,000-cell memory array with wrapping byte arithmetic.
