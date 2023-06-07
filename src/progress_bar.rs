use std::sync::{Arc, Mutex, MutexGuard};
use crate::playback_duration::PlaybackDuration;
use crate::player::PlayerState;
use crate::terminal::Terminal;
use crate::wav::WavDuration;

pub struct ProgressBar {
    max_ticks: f32
}

impl ProgressBar {
    pub fn new() -> Self {
        ProgressBar {
            max_ticks: 100.0
        }
    }

    pub fn update(&self, playback_duration: Arc<Mutex<PlaybackDuration>>, player_state: Arc<Mutex<PlayerState>>, terminal: Arc<Mutex<Terminal>>) {
        let playback_duration = playback_duration.lock().unwrap();
        let total_duration = player_state.lock().unwrap().wav.duration;
        let mut terminal = terminal.lock().unwrap();

        terminal.write(&playback_duration);
        terminal.write(String::from("["));
        terminal.set_cursor_right(self.max_ticks as u16);
        terminal.write(String::from("]"));
        terminal.write(&total_duration);
        terminal.set_cursor_left((self.max_ticks + 6.0) as u16);

        let ticks_per_second: f32 = self.max_ticks / total_duration.raw_seconds;
        let seconds_per_tick: u32 = if ticks_per_second < 1.0 {
            (1.0 / ticks_per_second).ceil()
        } else {
            ticks_per_second.ceil()
        } as u32;

        let extra = (self.max_ticks as u32) / (100 - ((total_duration.raw_seconds as u32) / seconds_per_tick));
        let ticks = playback_duration.raw_seconds / (seconds_per_tick + extra);
        for _ in 0..ticks {
            terminal.write(String::from("#"));
        }
    }
}