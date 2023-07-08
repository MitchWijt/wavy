use std::io::stdin;
use std::sync::{Arc, Mutex};
use std::thread;
use crossbeam_queue::SegQueue;
use termion::event::Key;
use termion::input::TermRead;
use crate::{Commands, Playlist, Terminal};
use crate::Commands::PLAY;
use crate::gui::Commands::SELECT;

pub struct Gui {
    playlist: Arc<Mutex<Playlist>>,
    active_song: Arc<Mutex<u16>>
}

impl Gui {
    pub fn new() -> Self {
        Gui {
            playlist: Arc::new(Mutex::new(Playlist::new())),
            active_song: Arc::new(Mutex::new(0))
        }
    }

    pub fn draw(&self, from_gui_queue: SegQueue<Commands>) {
        let mut terminal = Terminal::new();
        let playlist = self.playlist.clone();
        let index = self.active_song.clone();

        thread::spawn(move || {
            loop {
                let songs = &playlist.lock().unwrap().songs;

                terminal.clear();
                for song in songs {
                    terminal.write(format!("{} - {}", song.title, song.artist));

                    terminal.cursor_row += 1;
                    terminal.cursor_col = 1;
                    terminal.set_cursor();
                }

                let index = index.lock().unwrap();
                terminal.write(format!("{index}"));
            }
        });

        for key in stdin().keys() {
            match key.unwrap() {
                Key::Char('q') => {
                    break;
                }
                Key::Char(' ') => {
                    from_gui_queue.push(PLAY);
                }
                Key::Ctrl(key) => {
                    match key {
                        'n' => {
                            from_gui_queue.push(SELECT);
                            *self.active_song.lock().unwrap() += 1;
                        },
                        'p' => {
                            from_gui_queue.push(SELECT);
                            *self.active_song.lock().unwrap() -= 1;
                        },
                        _ => {}
                    }
                },
                _ => {}
            }
        };
    }
}