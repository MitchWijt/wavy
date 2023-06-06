use std::fmt::Display;
use std::io::{Stdout, stdout, Write};
use termion::raw::{IntoRawMode, RawTerminal};
use termion::screen::{AlternateScreen, IntoAlternateScreen};

pub struct Terminal {
    stdout: AlternateScreen<RawTerminal<Stdout>>
}

impl Terminal {
    pub fn new() -> Self {
        Terminal {
            stdout: stdout().into_raw_mode().unwrap().into_alternate_screen().unwrap()
        }
    }

    pub fn write<P: Display>(&mut self, text: P) {
        write!(self.stdout, "{}", text).unwrap();
        self.stdout.flush().unwrap();
    }

    pub fn clear(&mut self) {
        write!(self.stdout, "{}{}",termion::cursor::Goto(1,1), termion::clear::CurrentLine).unwrap();
        self.stdout.flush().unwrap();
    }

    pub fn set_cursor_right(&mut self, col: u16) {
        write!(self.stdout, "{}", termion::cursor::Right(col)).unwrap();
        self.stdout.flush().unwrap();
    }

    pub fn set_cursor_left(&mut self, col: u16) {
        write!(self.stdout, "{}", termion::cursor::Left(col)).unwrap();
        self.stdout.flush().unwrap();
    }
}