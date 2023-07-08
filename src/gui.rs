use std::fs::File;
use std::io::{BufReader, Read, Seek, SeekFrom, stdin};
use std::sync::{Arc, Mutex, MutexGuard};
use std::thread;
use std::time::Duration;
use crossbeam_queue::SegQueue;
use termion::event::Key;
use termion::input::TermRead;
use crate::{Commands, Playlist, Terminal};
use crate::Commands::{PAUSE, PLAY, PLAYRESUME};
use crate::gui::Commands::SELECT;
use crate::playlist::Song;

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

    pub fn draw(&self, from_gui_queue: Arc<SegQueue<Commands>>) {
        let mut terminal = Terminal::new();
        let playlist = self.playlist.clone();
        let index = self.active_song.clone();

        thread::spawn(move || {
            loop {
                thread::sleep(Duration::from_secs(1));
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
            let playlist = self.playlist.lock().unwrap();

            match key.unwrap() {
                Key::Char('q') => {
                    break;
                }
                Key::Char(' ') => {
                    let index = self.active_song.lock().unwrap();
                    let buffer = self.get_buffer(index, playlist);
                    from_gui_queue.push(PLAY {
                        buffer
                    });
                }
                Key::Char('p') => {
                    from_gui_queue.push(PAUSE);
                }
                Key::Char('n') => {
                    from_gui_queue.push(PLAYRESUME);
                }
                Key::Ctrl(key) => {
                    match key {
                        'n' => {
                            let mut index = self.active_song.lock().unwrap();
                            *index += 1;

                            let buffer = self.get_buffer(index, playlist);
                            from_gui_queue.push(PLAY {
                                buffer
                            });
                        },
                        'p' => {
                            let mut index = self.active_song.lock().unwrap();
                            *index -= 1;

                            let buffer = self.get_buffer(index, playlist);
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

    pub fn get_buffer(&self, index: MutexGuard<u16>, playlist: MutexGuard<Playlist>) -> Vec<u8> {
        let song: &Song = playlist.songs.get(*index as usize).unwrap();
        let file = File::open(&song.path).unwrap();

        let mut reader = BufReader::new(file);

        // set seek position after the RIFF header
        reader.seek(SeekFrom::Start(44)).unwrap();

        let mut buffer = vec![0u8; song.wav.header.data.chunk_size as usize];
        reader.read_exact(&mut *buffer).unwrap();

        buffer
    }
}