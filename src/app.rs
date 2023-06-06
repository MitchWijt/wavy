use std::io::stdin;
use std::path::Path;
use std::sync::{Arc, mpsc, Mutex};
use std::thread;
use std::time::Duration;
use termion::event::Key;
use termion::input::TermRead;
use crate::{Player, Terminal, Wav};
use crate::player::PlayerState;

pub struct App {
    song_paths: Arc<Vec<String>>,
    terminal: Arc<Mutex<Terminal>>,
}

impl App {
    pub fn new() -> Self {
        App {
            song_paths: Arc::new(vec![String::from("assets/track.wav"), String::from("assets/track2.wav")]),
            terminal: Arc::new(Mutex::new(Terminal::new()))
        }
    }

    pub fn start(&self) -> Result<(), &'static str> {
        let (next_song_tx, next_song_rx) = mpsc::channel();

        let terminal = self.terminal.clone();
        let song_paths = self.song_paths.clone();

        thread::spawn(move || {
            let mut current_song_index = 0;

            let song_path = song_paths.get(current_song_index).unwrap();

            let mut song = Wav::new(song_path);
            let player = Player::new(song);
            let playback_duration = player.playback_duration.clone();

            player.stream(next_song_tx.clone()).unwrap();
            current_song_index += 1;

            loop {
                thread::sleep(Duration::from_secs(1));
                let playback_duration = playback_duration.lock().unwrap();

                terminal.lock().unwrap().clear();
                terminal.lock().unwrap().write(playback_duration);

                if let Ok(next_song) = next_song_rx.try_recv() {
                    if next_song {
                        println!("NEXT");
                        let song_path = song_paths.get(current_song_index).unwrap();
                        let song = Wav::new(song_path);
                        let player_state = PlayerState::from_wav(song);
                        *player.state.lock().unwrap() = player_state;

                        // player.next()
                        // player.next()
                        // player.next()

                        // player.next() will grab the next song in queue and start playing that from
                        // the same thread. Instead of spawning a new thread.
                        // we do not need new threads for every song, because the old thread is obsolete anyways.
                        // better to re-use it.
                    }
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