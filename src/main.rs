use std::io;

use crate::repl::start;

pub mod lexer;
pub mod token;
pub mod repl;

fn main() {
    println!("Hello! This is the Mokey Programming Language!");
    println!("Feel free to type in some code and play around!");
    start(io::stdin(), io::stdout());
}
