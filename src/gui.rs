use std::fs::File;
use std::io::{BufReader, Read, Seek, SeekFrom, stdin};
use std::sync::{Arc, Mutex, MutexGuard};
use std::thread;
use std::time::Duration;
use crossbeam_queue::SegQueue;
use termion::event::Key;
use termion::input::TermRead;
use crate::{Commands, Playlist, Terminal};
use crate::Commands::{PAUSE, PLAY, PLAYRESUME, END_SONG};
use crate::gui::Commands::SELECT;
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
    playlist: Arc<Playlist>,
    active_song: Arc<Mutex<u16>>,
    pre_loaded_buffer: Option<Vec<u8>>
}

impl Gui {
    pub fn new() -> Self {
        Gui {
            playlist: Arc::new(Playlist::new()),
            active_song: Arc::new(Mutex::new(0)),
            pre_loaded_buffer: None
        }
    }

    pub fn draw(&mut self, from_gui_queue: Arc<SegQueue<Commands>>) {
        let mut terminal = Terminal::new();
        let playlist = self.playlist.clone();
        let index = self.active_song.clone();

        thread::spawn(move || {
            loop {
                thread::sleep(Duration::from_secs(1));
                let songs = &playlist.songs;

                terminal.clear();
                for song in songs {
                    // check if active song. If it is, we format it differently
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
                    let song_index = self.get_current_index();
                    let buffer = self.get_buffer(song_index);
                    from_gui_queue.push(PLAY {
                        buffer
                    });

                    let next_song_index = self.get_next_index();
                    self.pre_load_buffer(next_song_index);
                }
                Key::Char('p') => {
                    from_gui_queue.push(PAUSE);
                }
                Key::Char('n') => {
                    from_gui_queue.push(PLAYRESUME);
                }
                Key::Ctrl(key) => {
                    match key {
                        // next song
                        'n' => {
                            *self.active_song.lock().unwrap() += 1;

                            let song_index = self.get_current_index();
                            let buffer = self.get_buffer(song_index);
                            from_gui_queue.push(PLAY {
                                buffer
                            });

                            // todo: only pre-load next song if there is a next song
                            // todo: this needs to be handled by a separate module
                            // let next_song_index = self.get_next_index();
                            // self.pre_load_buffer(next_song_index);
                        },
                        // previous song
                        'p' => {
                            *self.active_song.lock().unwrap() -= 1;

                            let song_index = self.get_current_index();
                            let buffer = self.get_buffer(song_index);
                            from_gui_queue.push(PLAY {
                                buffer
                            });
                        },
                        _ => {}
                    }
                },
                _ => {}
            }
        };
    }

    pub fn get_buffer(&mut self, index: u16) -> Vec<u8> {
        return match &self.pre_loaded_buffer {
            // make sure this does not clone the data since it's a very expensive allocation
            Some(buffer) => {
                let pre_loaded_buffer = buffer.clone();
                self.pre_loaded_buffer = None;

                pre_loaded_buffer
            },
            None => self.load_buffer(index)
        }
    }

    pub fn load_buffer(&self, index: u16) -> Vec<u8> {
        let song: &Song = self.playlist.songs.get(index as usize).unwrap();
        let file = File::open(&song.path).unwrap();

        let mut reader = BufReader::new(file);

        // set seek position after the RIFF header
        reader.seek(SeekFrom::Start(44)).unwrap();

        let mut buffer = vec![0u8; song.wav.header.data.chunk_size as usize];
        reader.read_exact(&mut *buffer).unwrap();

        buffer
    }

    pub fn pre_load_buffer(&mut self, index: u16) {
        self.pre_loaded_buffer = Some(self.load_buffer(index));
    }

    pub fn get_current_index(&self) -> u16 {
        *self.active_song.lock().unwrap()
    }

    pub fn get_next_index(&self) -> u16 {
        // this can return something different if the state is set to shuffle
        let current_index = *self.active_song.lock().unwrap();
        let next_index = current_index + 1;

        next_index
    }


}