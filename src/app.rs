use std::sync::{Arc, Mutex};
use std::thread;
use std::time::Duration;
use crossbeam_queue::SegQueue;
use crossterm::event::{poll, read};
use crossterm::terminal::is_raw_mode_enabled;
use crossterm::event::Event;
use crate::gui::Gui;
use crate::{GuiToPlayerCommands, PlayerToGuiCommands};
use crate::playlist::Playlist;

pub struct App;

pub enum AppEvent {
    Exit,
    Continue
}

impl App {
    pub fn new(from_gui_queue: Arc<SegQueue<GuiToPlayerCommands>>, to_gui_queue: Arc<SegQueue<PlayerToGuiCommands>>) {
        let mut gui = Gui::new(from_gui_queue, to_gui_queue);

        loop {
            gui.draw();

            if poll(Duration::from_millis(1)).unwrap() {
                let app_event = match read().unwrap() {
                    Event::Key(event) => gui.handle_key_event(event),
                    _ => Some(AppEvent::Continue)
                };

                if let Some(AppEvent::Exit) = app_event {
                    break;
                }
            }
        }
    }
}