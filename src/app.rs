use std::fmt::format;
use std::io::stdin;
use std::path::Path;
use std::sync::{Arc, mpsc, Mutex};
use std::thread;
use std::time::Duration;
use termion::event::Key;
use termion::input::TermRead;
use crate::{Player, ProgressBar, Terminal, Wav};
use crate::playback_duration::PlaybackDuration;
use crate::player::PlayerState;
use crate::playlist::Playlist;

pub struct App;

impl App {
    pub fn start(&self) -> Result<(), &'static str> {
        let mut terminal = Terminal::new();
        let playlist = Playlist::new();



        // let progress_bar = ProgressBar::new();
        // let player = Player::new();

        thread::spawn(move || {
            // player.stream().unwrap();

            loop {
                thread::sleep(Duration::from_secs(1));
                let songs = &playlist.0;

                terminal.clear();
                for song in songs {
                    terminal.write(format!("{} - {}", song.title, song.artist));

                    terminal.cursor_col += 1;
                    terminal.cursor_row = 1;
                    terminal.set_cursor();
                }

                // progress_bar.update(
                //     player.playback_duration.clone(),
                //     player.player_state.clone(),
                //     terminal.clone()
                // );
            }
        });

        for key in stdin().keys() {
            match key.unwrap() {
                Key::Char('q') => {
                    break;
                }
                Key::Ctrl(key) => {
                    if key == 'c' {
                        break;
                    }
                },
                _ => {}
            }
        };

        Ok(())
    }
}