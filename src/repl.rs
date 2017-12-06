use std::io::{Write, stdout, stdin};
use termion::raw::IntoRawMode;
use termion::input::TermRead;
use termion::event::Key;
use termion::clear;
use termion::cursor;

use interpreter;
use parser;

pub fn repl() {
    println!("{}", clear::All);

    let stdin = stdin();
    let mut stdout = stdout().into_raw_mode().unwrap();
    write!(stdout, "{}{}", cursor::Goto(1, 1), clear::CurrentLine).unwrap();
    print!(">>>");
    
    for c in stdin.keys() {
        // Clear the current line.

        // Print the key we type...
        match c.unwrap() {
            // Exit.
            Key::Char('q') => break,
            Key::Char(c)   => print!("{}", c),
            Key::Alt(c)    => print!("Alt-{}", c),
            Key::Ctrl(c)   => print!("Ctrl-{}", c),
            Key::Left      => print!("<left>"),
            Key::Right     => print!("<right>"),
            Key::Up        => print!("<up>"),
            Key::Down      => print!("<down>"),
            _              => print!("Other"),
        }

        // Flush again.
        stdout.flush().unwrap();
    }
}


