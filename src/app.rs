use std::sync::{Arc, Mutex};
use std::thread;
use std::time::Duration;
use crossbeam_queue::SegQueue;
use crate::Commands;
use crate::gui::Gui;
use crate::playlist::Playlist;

pub struct App;

pub enum Event {
    Exit,
    Continue
}

impl App {
    pub fn new(from_gui_queue: Arc<SegQueue<Commands>>, to_gui_queue: Arc<SegQueue<Commands>>) {
        let mut gui = Gui::new(from_gui_queue, to_gui_queue);

        loop {
            gui.draw();
            if let Some(event) = gui.handle_key_events() {
                match event {
                    Event::Exit => break,
                    Event::Continue => continue
                }
            }
        }
    }
}