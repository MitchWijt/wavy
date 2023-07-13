use std::sync::{Arc};

use crossbeam_queue::SegQueue;

use crate::app::App;
use crate::output::Output;
use crate::player::Player;
use crate::playlist::Playlist;
use crate::terminal::Terminal;
use crate::wav::Wav;

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
    Paused,
    UpdateDuration {
        duration: u128
    }
}

fn main() {
    let from_gui_queue = Arc::new(SegQueue::new());
    let to_gui_queue = Arc::new(SegQueue::new());

    let _stream = Output::new(from_gui_queue.clone(), to_gui_queue.clone());
    let _app = App::new(from_gui_queue.clone(), to_gui_queue.clone());
}
