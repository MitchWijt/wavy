use std::fmt::format;
use std::fs::File;
use std::io::{BufReader, stdin, stdout};
use std::thread;
use std::thread::Thread;
use std::time::Duration;
use termion::raw::IntoRawMode;
use termion::input::TermRead;
use termion::screen::{AlternateScreen, IntoAlternateScreen};
use crate::player::Player;
use crate::wav::Wav;
use std::io::Write;
use std::path::PathBuf;
use std::process::exit;
use std::sync::{Arc, Mutex};
use crossbeam_queue::SegQueue;
use termion::event::Key;
use crate::app::App;
use crate::gui::Gui;
use crate::output::Output;
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
mod output;
mod app;

pub enum GuiToPlayerCommands {
    Play {
        buffer: Vec<u8>
    },
    PlayResume,
    Pause,
    Forward,
    Rewind
}

pub enum PlayerToGuiCommands {
    End,
    Playing,
    Paused
}

fn main() {
    let from_gui_queue = Arc::new(SegQueue::new());
    let to_gui_queue = Arc::new(SegQueue::new());

    let _stream = Output::new(from_gui_queue.clone(), to_gui_queue.clone());
    let _app = App::new(from_gui_queue.clone(), to_gui_queue.clone());
}
