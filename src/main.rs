use std::io::{stdin, stdout};
use std::thread;
use std::thread::Thread;
use std::time::Duration;
use termion::raw::IntoRawMode;
use termion::input::TermRead;
use termion::screen::{AlternateScreen, IntoAlternateScreen};
use crate::player::Player;
use crate::wav::Wav;
use std::io::Write;
use std::process::exit;
use termion::event::Key;
use crate::app::App;
use crate::progress_bar::ProgressBar;
use crate::terminal::Terminal;

mod player;
mod wav;
mod playback_duration;
mod terminal;
mod progress_bar;
mod app;

fn main() {
    let app = App::new();
    match app.start() {
        Ok(..) => {},
        Err(e) => {
            eprintln!("{}", e);
            exit(1);
        }
    }
}
