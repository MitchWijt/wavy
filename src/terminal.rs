use std::fmt::Display;
use std::io::{Stdout, stdout, Write};
use termion::raw::{IntoRawMode, RawTerminal};
use termion::screen::{AlternateScreen, IntoAlternateScreen};

pub struct Terminal {
    stdout: AlternateScreen<RawTerminal<Stdout>>,
    pub cursor_row: u16,
    pub cursor_col: u16
}

impl Terminal {
    pub fn new() -> Self {
        Terminal {
            stdout: stdout().into_raw_mode().unwrap().into_alternate_screen().unwrap(),
            cursor_row: 0,
            cursor_col: 0,
        }
    }

    pub fn write<P: Display>(&mut self, text: P) {
        write!(self.stdout, "{}", text).unwrap();
        self.stdout.flush().unwrap();
    }

    pub fn clear(&mut self) {
        self.cursor_col = 1;
        self.cursor_col = 1;
        write!(self.stdout, "{}{}",termion::cursor::Goto(1,1), termion::clear::CurrentLine).unwrap();
        self.stdout.flush().unwrap();
    }

    pub fn set_cursor_right(&mut self, col: u16) {
        self.cursor_col += col;
        write!(self.stdout, "{}", termion::cursor::Right(col)).unwrap();
        self.stdout.flush().unwrap();
    }

    pub fn set_cursor_left(&mut self, col: u16) {
        self.cursor_col -= col;
        write!(self.stdout, "{}", termion::cursor::Left(col)).unwrap();
        self.stdout.flush().unwrap();
    }

    pub fn set_cursor_down(&mut self, col: u16) {
        self.cursor_row += col;
        write!(self.stdout, "{}", termion::cursor::Down(col)).unwrap();
        self.stdout.flush().unwrap();
    }

    pub fn set_cursor_up(&mut self, col: u16) {
        self.cursor_row -= col;
        write!(self.stdout, "{}", termion::cursor::Up(col)).unwrap();
        self.stdout.flush().unwrap();
    }

    pub fn set_cursor(&mut self) {
        write!(self.stdout, "{}", termion::cursor::Goto(self.cursor_row, self.cursor_col)).unwrap();
        self.stdout.flush().unwrap();
    }
}