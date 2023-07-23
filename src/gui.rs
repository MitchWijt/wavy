use std::collections::HashMap;
use std::fs::File;
use std::io::{BufReader, Read, Seek, SeekFrom};
use std::sync::{Arc};

use crossbeam_queue::SegQueue;
use crossterm::event::{KeyCode, KeyEvent, KeyModifiers};
use rand::{Rng, thread_rng};
use rand::prelude::SliceRandom;

use crate::{GuiToPlayerCommands, PlayerToGuiCommands, Playlist, Terminal};
use crate::app::{AppEvent};
use crate::playback_duration::PlaybackDuration;
use crate::playlist::Song;
use crate::progress_bar::ProgressBar;

pub struct Gui {
    to_gui_queue: Arc<SegQueue<PlayerToGuiCommands>>,
    from_gui_queue: Arc<SegQueue<GuiToPlayerCommands>>,
    playlist: Playlist,
    playlist_index: usize,
    terminal: Terminal,
    prev_index: Option<usize>,
    shuffle: bool,
    playing: bool,
    playback_duration: PlaybackDuration,
    progress_bar: ProgressBar,
    active_song: Option<Song>
}

impl Gui {
    pub fn new(from_gui_queue: Arc<SegQueue<GuiToPlayerCommands>>, to_gui_queue: Arc<SegQueue<PlayerToGuiCommands>>) -> Self {
        Gui {
            to_gui_queue,
            from_gui_queue,
            playlist: Playlist::new(),
            playlist_index: 0,
            terminal: Terminal::new(),
            playback_duration: PlaybackDuration::new(),
            progress_bar: ProgressBar::new(),
            shuffle: false,
            prev_index: None,
            playing: false,
            active_song: None
        }
    }

    pub fn draw(&mut self) {
        while let Some(command) = self.to_gui_queue.pop() {
            match command {
                PlayerToGuiCommands::End => {
                    self.next_song();
                }
                PlayerToGuiCommands::Play => {
                    self.playing = true;

                    let song = self.get_song(self.playlist_index);
                    let active_song  = Song::from_path(song.path.clone());

                    self.active_song = Some(active_song);
                },
                PlayerToGuiCommands::Playing => {
                    self.playing = true;
                }
                PlayerToGuiCommands::Paused => {
                    self.playing = false;
                }
                PlayerToGuiCommands::UpdateDuration {
                    duration
                } => {
                    self.playback_duration.advance(duration)
                }
            }
        }

        let songs = &self.playlist.songs;

        self.terminal.clear();
        for song_idx in 0..songs.len() {
            if let Some(song) = songs.get(song_idx) {
                self.terminal.cursor_row += 1;
                self.terminal.cursor_col = 1;
                self.terminal.set_cursor();
                self.terminal.write(format!("#{} {}", song_idx + 1, song));
            }
        }

        if let Some(active_song) = &self.active_song {
            self.terminal.cursor_row += 2;
            self.terminal.cursor_col = 1;
            self.terminal.set_cursor();
            self.terminal.clear_line();
            self.terminal.write(active_song);

            self.terminal.cursor_row += 1;
            self.terminal.set_cursor();
            self.terminal.clear_line();
            self.progress_bar.update(&self.playback_duration, active_song.wav.duration, &mut self.terminal);
        }

        self.terminal.cursor_row += 2;
        self.terminal.set_cursor();
        self.terminal.clear_line();
        self.terminal.write(format!("Playing: {}", self.playing));

        self.terminal.cursor_row += 1;
        self.terminal.set_cursor();
        self.terminal.clear_line();
        self.terminal.write(format!("Shuffle: {}", self.shuffle));
    }

    pub fn handle_key_event(&mut self, event: KeyEvent) -> Option<AppEvent> {
        match event {
            KeyEvent {
                code: KeyCode::Char('q'),
                modifiers: KeyModifiers::NONE,
                ..
            } => Some(AppEvent::Exit),
            KeyEvent {
                code: KeyCode::Enter,
                modifiers: KeyModifiers::NONE,
                ..
            } => {
                self.play_song(self.playlist_index);
                Some(AppEvent::Continue)
            },
            KeyEvent {
                code: KeyCode::Char(' '),
                modifiers: KeyModifiers::NONE,
                ..
            } => {
                if !self.playing {
                    self.from_gui_queue.push(GuiToPlayerCommands::PlayResume);
                } else {
                    self.from_gui_queue.push(GuiToPlayerCommands::Pause);
                }

                Some(AppEvent::Continue)
            },
            KeyEvent {
                code: KeyCode::Char('s'),
                modifiers: KeyModifiers::NONE,
                ..
            } => {
                self.shuffle();
                Some(AppEvent::Continue)
            }
            KeyEvent {
                code: KeyCode::Right,
                modifiers: KeyModifiers::NONE,
                ..
            } => {
                self.next_song();
                Some(AppEvent::Continue)
            }
            KeyEvent {
                code: KeyCode::Left,
                modifiers: KeyModifiers::NONE,
                ..
            } => {
                self.prev_song();
                Some(AppEvent::Continue)
            }
            KeyEvent {
                code: KeyCode::Char('>'),
                modifiers: KeyModifiers::NONE,
                ..
            } => {
                self.from_gui_queue.push(GuiToPlayerCommands::Forward);
                Some(AppEvent::Continue)
            }
            KeyEvent {
                code: KeyCode::Char('<'),
                modifiers: KeyModifiers::NONE,
                ..
            } => {
                self.from_gui_queue.push(GuiToPlayerCommands::Rewind);
                Some(AppEvent::Continue)
            }
            _ => Some(AppEvent::Continue)
        }
    }

    fn next_song(&mut self) {
        let index = self.next_index();
        self.play_song(index);
    }

    fn prev_song(&mut self) {
        let index = self.prev_index();
        self.play_song(index);
    }

    fn play_song(&mut self, index: usize) {
        let buffer = self.load_buffer(index);
        self.from_gui_queue.push(GuiToPlayerCommands::Play {
            buffer
        });
    }

    pub fn next_index(&mut self) -> usize {
        self.prev_index = Some(self.playlist_index);
        if self.playlist_index + 1 > self.playlist.songs.len() - 1 {
            self.playlist_index = 0;
        } else {
            self.playlist_index += 1;
        }

        self.playlist_index
    }

    pub fn prev_index(&mut self) -> usize {
        if self.playlist_index == 0 {
            self.playlist_index = self.playlist.songs.len() - 1
        } else {
            self.playlist_index -= 1;
        }

        self.playlist_index
    }

    pub fn load_buffer(&self, playlist_index: usize) -> Vec<u8> {
        let song = self.get_song(playlist_index);
        let file = File::open(&song.path).unwrap();

        let mut reader = BufReader::new(file);

        // set seek position after the RIFF header
        reader.seek(SeekFrom::Start(44)).unwrap();

        let mut buffer = vec![0u8; song.wav.header.data.chunk_size as usize];
        reader.read_exact(&mut *buffer).unwrap();

        buffer
    }

    fn get_song(&self, playlist_index: usize) -> &Song {
        let index = self.playlist.indexes.get(playlist_index).unwrap();
        let song: &Song = self.playlist.songs.get(*index).unwrap();

        song
    }

    fn shuffle(&mut self) {
        if self.shuffle {
            self.shuffle = false;
            self.playlist.indexes.sort();
        } else {
            self.shuffle = true;
            self.playlist.indexes.shuffle(&mut thread_rng());
        }
    }
}