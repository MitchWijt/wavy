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
    terminal: Arc<Mutex<Terminal>>,
}

impl App {
    pub fn new() -> Self {
        App {
            terminal: Arc::new(Mutex::new(Terminal::new()))
        }
    }

    pub fn start(&self) -> Result<(), &'static str> {
        let terminal = self.terminal.clone();
        let progress_bar = ProgressBar::new();
        let player = Player::new();

        thread::spawn(move || {
            player.stream().unwrap();

            loop {
                thread::sleep(Duration::from_secs(1));

                terminal.lock().unwrap().clear();
                progress_bar.update(
                    player.playback_duration.clone(),
                    player.player_state.clone(),
                    terminal.clone()
                );
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