use std::sync::{Arc, Mutex};
use crate::playback_duration::PlaybackDuration;
use crate::terminal::Terminal;

pub struct ProgressBar {
    max_ticks: f32
}

impl ProgressBar {
    pub fn new() -> Self {
        ProgressBar {
            max_ticks: 100.0
        }
    }

    pub fn update(&self, playback_duration: Arc<Mutex<PlaybackDuration>>, terminal: Arc<Mutex<Terminal>>) {
        let playback_duration = playback_duration.lock().unwrap();
        let mut terminal = terminal.lock().unwrap();

        terminal.write(&playback_duration);
        terminal.write(String::from("["));
        terminal.set_cursor_right(self.max_ticks as u16);
        terminal.write(String::from("]"));
        // terminal.write(&total_duration);
        terminal.set_cursor_left((self.max_ticks + 6.0) as u16);

        // let ticks_per_second: f32 = self.max_ticks / total_duration.raw_seconds;
        let ticks_per_second: f32 = self.max_ticks / self.max_ticks;

        let ms_per_tick: u32 = if ticks_per_second < 1.0 {
            (1.0 / ticks_per_second) * 1000.0
        } else {
            ticks_per_second * 1000.0
        } as u32;

        let ticks = playback_duration.milliseconds / ms_per_tick;
        for _ in 0..ticks {
            terminal.write(String::from("#"));
        }
    }
}