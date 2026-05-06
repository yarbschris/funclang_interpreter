# Welcome to funclang, a super tiny and simple functional programming language.

### Description

Funclang is essentially a very small subset of OCaml. The language is purely functional, and all programs are one (large?) expression.

The language is currently a work in progress, but it has support for some basic features that you would expect from a functional programming language:

- Variable Assignments to int, bool, functions
- Functions, Recursive Functions
- Partial Application of Functions
- Lists (not typed (yet))
- Basic Pattern Matching

### Instructions

1. Make sure you have Rust (and everything that comes with it) installed on your system.
2. Clone this repository
3. cd funclang_interpreter

#### To run Read, Evaluate, Print, Loop

cargo run

#### To run a program

1. Write your program
2. cargo build
3. cargo run -- path/to/program

### Writing a Program

This language is kinda just becoming a tiny subset of OCaml, so if you are familiar with ML style languages, the syntax should be very easy to pick up.
There are a few example programs in the test_programs directory
To run one:
cargo run -- test_programs/peano_nums.fl

There are a few cool programs in the test_programs dir

### Command Line Arguments

To show AST:

-s or --show-tree

Runs program in file and prints AST:

cargo run -- test_programs/list_and_high_orders_out_of_functions.fl -s

REPL with AST printed:

cargo run -- -s

### File Descriptions

- main.rs: entry point, either detects a file, runs interpreter and exits, or REPL

- funclang.lalrpop: Defines the grammar for the programming language and creates parser that builds AST from source

- ast.rs: Defines the Abstract Syntax Tree structure that the lalrpop parser will use to build the AST which will be evaluated by the interpreter. This file contains pretty printing helpers if you want to display the AST (All pretty printing code was written by an LLM)

- value.rs: Defines the values (and errors) to which a program can evaluate (and to which variables can be assigned). This file also defines envs, which is the linked list structure used to keep track of scope and variable assignments from "let ... in ..." expressions.

- interpreter.rs: The Tree-Walking Interpreter (and helper functions). eval function takes in an AST and env (for scope) and recurses over entire AST to produce a value
