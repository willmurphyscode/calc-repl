use std::io::{Write, stdout, stdin};
use std::process;
use termion::raw::IntoRawMode;
use termion::input::TermRead;
use termion::event::Key;
use termion::clear;
use termion::cursor;
use termion::terminal_size;

use interpreter;
use parser;

pub fn repl() {
    let mut stdout = stdout();

    writeln!(stdout, "{}Welcome to a simple repl. enter q or press Ctrl-C to exit", clear::CurrentLine).unwrap();
    loop {
        write!(stdout, ">>> ");
        stdout.flush().unwrap();
        let mut current_input = String::new();
        let stdin = stdin();

        for c in stdin.keys() {
            match c.unwrap() {
                Key::Char('\n') => break, // done with this entry
                Key::Char('q') => process::exit(0), // quit entirely
                Key::Ctrl('c') => process::exit(0), // quit entirely
                Key::Char(c)   => { 
                    current_input.push(c);
                },
                _ => {},
            }
        }
        let token_result = parser::parse(&current_input.into_bytes()[..]);
        if let Ok(tokens) = token_result {
            let eval_result = interpreter::eval(tokens);
            if let Ok(value) = eval_result {
                writeln!(stdout, "{}", value);
            } else {
                writeln!(stdout, "Runtime error");
            }
        } else {
            write!(stdout, "{}", "A parse error occurred");
        }
        stdout.flush().unwrap();
    }
}


