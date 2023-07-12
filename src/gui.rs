use std::fs::{File};
use std::io::{BufReader, Read, Seek, SeekFrom, stdin};
use std::sync::{Arc, Mutex, MutexGuard};
use std::thread;
use std::time::Duration;
use crossbeam_queue::SegQueue;
use crossterm::event::{Event, KeyCode, KeyEvent, KeyEventKind, KeyModifiers};
use crossterm::terminal::disable_raw_mode;
use termion::async_stdin;
use termion::event::Key;
use termion::input::TermRead;
use crate::{Commands, Playlist, Terminal};
use crate::app::{App, AppEvent};
use crate::Commands::{PAUSE, PLAY, PLAYRESUME, END_SONG};
use crate::playlist::Song;

pub struct Gui {
    to_gui_queue: Arc<SegQueue<Commands>>,
    from_gui_queue: Arc<SegQueue<Commands>>,
    playlist: Playlist,
    playlist_index: usize,
    terminal: Terminal
}

impl Gui {
    pub fn new(from_gui_queue: Arc<SegQueue<Commands>>, to_gui_queue: Arc<SegQueue<Commands>>) -> Self {
        Gui {
            to_gui_queue,
            from_gui_queue,
            playlist: Playlist::new(),
            playlist_index: 0,
            terminal: Terminal::new()
        }
    }

    pub fn draw(&mut self) {
        while let Some(command) = self.to_gui_queue.pop() {
            match command {
                END_SONG => {
                    self.playlist_index += 1;

                    let buffer = self.load_buffer(self.playlist_index);
                    self.from_gui_queue.push(PLAY {
                        buffer
                    });
                }
                _ => {}
            }
        }

        let songs = &self.playlist.songs;
        let active_song = self.playlist.songs.get(self.playlist_index).unwrap();

        self.terminal.clear();
        for song in songs {
            self.terminal.write(song);

            self.terminal.cursor_row += 1;
            self.terminal.cursor_col = 1;
            self.terminal.set_cursor();
        }

        self.terminal.cursor_row += 2;
        self.terminal.cursor_col = 1;
        self.terminal.set_cursor();
        self.terminal.clear_line();
        self.terminal.write(active_song)
    }

    pub fn handle_key_event(&mut self, event: KeyEvent) -> Option<AppEvent> {
        match event {
            KeyEvent {
                code: KeyCode::Char('q'),
                modifiers: KeyModifiers::NONE,
                ..
            } => Some(AppEvent::Exit),
            KeyEvent {
                code: KeyCode::Char(' '),
                modifiers: KeyModifiers::NONE,
                ..
            } => {
                let buffer = self.load_buffer(self.playlist_index);
                self.from_gui_queue.push(PLAY {
                    buffer
                });
                Some(AppEvent::Continue)
            }
            KeyEvent {
                code: KeyCode::Char('p'),
                modifiers: KeyModifiers::NONE,
                ..
            } => {
                self.from_gui_queue.push(PAUSE);
                Some(AppEvent::Continue)
            }
            KeyEvent {
                code: KeyCode::Char('n'),
                modifiers: KeyModifiers::NONE,
                ..
            } => {
                self.from_gui_queue.push(PLAYRESUME);
                Some(AppEvent::Continue)
            }
            KeyEvent {
                code: KeyCode::Char('n'),
                modifiers: KeyModifiers::CONTROL,
                ..
            } => {
                self.playlist_index += 1;

                let buffer = self.load_buffer(self.playlist_index);
                self.from_gui_queue.push(PLAY {
                    buffer
                });
                Some(AppEvent::Continue)
            }
            KeyEvent {
                code: KeyCode::Char('p'),
                modifiers: KeyModifiers::CONTROL,
                ..
            } => {
                self.playlist_index -= 1;

                let buffer = self.load_buffer(self.playlist_index);
                self.from_gui_queue.push(PLAY {
                    buffer
                });
                Some(AppEvent::Continue)
            }
            _ => Some(AppEvent::Continue)
        }
    }

    // pub fn get_buffer(&mut self, index: u16) -> Vec<u8> {
    //     return match &self.pre_loaded_buffer {
    //         // make sure this does not clone the data since it's a very expensive allocation
    //         Some(buffer) => {
    //             let pre_loaded_buffer = buffer.clone();
    //             self.pre_loaded_buffer = None;
    //
    //             pre_loaded_buffer
    //         },
    //         None => self.load_buffer(index)
    //     }
    // }

    pub fn load_buffer(&self, playlist_index: usize) -> Vec<u8> {
        let song: &Song = self.playlist.songs.get(playlist_index).unwrap();
        let file = File::open(&song.path).unwrap();

        let mut reader = BufReader::new(file);

        // set seek position after the RIFF header
        reader.seek(SeekFrom::Start(44)).unwrap();

        let mut buffer = vec![0u8; song.wav.header.data.chunk_size as usize];
        reader.read_exact(&mut *buffer).unwrap();

        buffer
    }

    // pub fn pre_load_buffer(&mut self, index: u16) {
    //     self.pre_loaded_buffer = Some(self.load_buffer(index));
    // }
}