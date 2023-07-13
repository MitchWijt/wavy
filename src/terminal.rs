use std::fmt::Display;
use std::io::{Stdout, stdout, Write};
use crossterm::execute;
use crossterm::terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen};


pub struct Terminal {
    stdout: Stdout,
    pub cursor_row: u16,
    pub cursor_col: u16
}

impl Terminal {
    pub fn new() -> Self {
        let mut stdout = stdout();
        execute!(stdout, EnterAlternateScreen).unwrap();
        enable_raw_mode().unwrap();

        Terminal {
            stdout,
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
        self.cursor_row = 1;
        write!(self.stdout, "{}{}{}",termion::cursor::Goto(1,1), termion::clear::BeforeCursor, termion::cursor::Hide).unwrap();
        self.stdout.flush().unwrap();
    }

    pub fn clear_line(&mut self) {
        write!(self.stdout, "{}", termion::clear::CurrentLine).unwrap();
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
        write!(self.stdout, "{}", termion::cursor::Goto(self.cursor_col, self.cursor_row)).unwrap();
        self.stdout.flush().unwrap();
    }
}