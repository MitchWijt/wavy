use std::fs::{File};
use std::io::{BufReader, Read, Seek, SeekFrom, stdin};
use std::sync::{Arc, Mutex, MutexGuard};
use std::thread;
use std::time::Duration;
use crossbeam_queue::SegQueue;
use crossterm::event::{KeyEvent, read, KeyCode, KeyModifiers};
use termion::event::{Key};
use termion::input::TermRead;
use crate::{Commands, Playlist, Terminal};
use crate::app::Event;
use crate::app::Event::{Continue, Exit};
use crate::Commands::{PAUSE, PLAY, PLAYRESUME, END_SONG};
use crate::playlist::Song;

/*
TODO: These are some notes of some things that really need to change.
      1. I feel that pre_loaded_buffer and the whole get_buffer mechanism should not be part of the GUI. The GUI should draw the GUI and handle
         GUI commands, that's it.
      2. Maybe something needs to change with the playlist and the active_song_index they seem kind of out of place.
      3. In order to handle commands from the Player to the GUI. We need some sort of a callback mechanism in the GUI to start handling these commands.
         Since we cannot handle them in the spawned thread.
      4. Maybe index can also be it's own struct of some sorts.
      5. I need to handle the overflowing or underflowing of the index and circle back to either the start(overflow) or end(underflow)

 */

pub struct Gui {
    to_gui_queue: Arc<SegQueue<Commands>>,
    from_gui_queue: Arc<SegQueue<Commands>>,
    playlist: Playlist,
    playlist_index: usize,
    terminal: Terminal,
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
        // to_gui_commands
        let songs = &self.playlist.songs;

        self.terminal.clear();
        for song in songs {
            self.terminal.write(format!("{} - {}", song.title, song.artist));

            self.terminal.cursor_row += 1;
            self.terminal.cursor_col = 1;
            self.terminal.set_cursor();
        }
    }

    pub fn handle_key_events(&mut self) -> Option<Event> {
        for key in stdin().keys() {
            match key.unwrap() {
                Key::Char('q') => {
                    return Some(Exit)
                }
                Key::Char(' ') => {
                    let buffer = self.load_buffer(self.playlist_index);
                    self.from_gui_queue.push(PLAY {
                        buffer
                    });
                    return Some(Continue)
                }
                Key::Char('p') => {
                    self.from_gui_queue.push(PAUSE);
                    return Some(Continue)
                }
                Key::Char('n') => {
                    self.from_gui_queue.push(PLAYRESUME);
                    return Some(Continue)
                }
                Key::Ctrl(key) => {
                    return match key {
                        // next song
                        'n' => {
                            self.playlist_index += 1;

                            let buffer = self.load_buffer(self.playlist_index);
                            self.from_gui_queue.push(PLAY {
                                buffer
                            });
                            return Some(Continue)
                        },
                        // previous song
                        'p' => {
                            self.playlist_index -= 1;

                            let buffer = self.load_buffer(self.playlist_index);
                            self.from_gui_queue.push(PLAY {
                                buffer
                            });
                            return Some(Continue)
                        },
                        _ => return Some(Continue)
                    }
                },
                _ => return Some(Continue)
            }
        }

        return Some(Continue)
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