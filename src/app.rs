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

pub struct App {
    song_paths: Arc<Vec<String>>,
    terminal: Arc<Mutex<Terminal>>,
}

impl App {
    pub fn new() -> Self {
        App {
            song_paths: Arc::new(vec![
                String::from("assets/track.wav"),
                String::from("assets/track2.wav"),
                String::from("assets/wavy.wav"),
            ]),
            terminal: Arc::new(Mutex::new(Terminal::new()))
        }
    }

    pub fn start(&self) -> Result<(), &'static str> {
        let terminal = self.terminal.clone();
        let song_paths = self.song_paths.clone();
        let progress_bar = ProgressBar::new();

        thread::spawn(move || {
            let mut song_paths_iterator = song_paths.iter().peekable();

            let song = Wav::new(song_paths_iterator.next().unwrap());
            let player = Player::new(song);
            player.stream().unwrap();

            loop {
                thread::sleep(Duration::from_secs(1));

                terminal.lock().unwrap().clear();
                progress_bar.update(
                    player.playback_duration.clone(),
                    player.state.clone(),
                    terminal.clone()
                );

                if player.state.lock().unwrap().is_end {
                    let song = Wav::new(song_paths_iterator.next().unwrap());
                    player.next(song);
                }
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