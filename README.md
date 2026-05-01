# Welcome to funclang, a super tiny and simple functional programming language.

### Instructions

1. Make sure you have Rust (and everything that comes with it) installed on your system.
2. Clone this repository
3. cd funclang_interpreter

#### To run Read, Evaluate, Print, Loop

cargo run

#### To run a program

1. Write your program
2. cargo build
3. cargo run -- {Program filename}

#### Writing a Program

This language is kinda just becoming a tiny subset of OCaml, so if you are familiar with ML style languages, the syntax should be very easy to pick up.
There is an example program.fl which can serve as an example and test the validity of the interpreter.
To run program.fl:
cargo run -- program.fl
