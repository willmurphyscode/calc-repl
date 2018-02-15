#[macro_use]
extern crate nom;
extern crate termion;

mod interpreter;
mod parser;
mod repl;
mod runtime_error;
mod token;
mod tokenization_error;

fn main() {
    repl::repl();
}

// TODO write some end to end tests that exercise the parser and interpreter together