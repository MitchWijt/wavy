use std::fmt::format;
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
use std::sync::{Arc, Mutex};
use crossbeam_queue::SegQueue;
use termion::event::Key;
use crate::gui::Gui;
use crate::playlist::Playlist;
use crate::progress_bar::ProgressBar;
use crate::terminal::Terminal;

mod player;
mod wav;
mod playback_duration;
mod terminal;
mod progress_bar;
mod playlist;
mod gui;

pub enum Commands {
    PLAY,
    PAUSE,
    SELECT,
    FORWARD,
    BACKWARDS
}

fn main() {
    let gui = Gui::new();

    let from_gui_queue = SegQueue::new();
    // let to_gui_queue = SegQueue::new();

    gui.draw(from_gui_queue);
}
