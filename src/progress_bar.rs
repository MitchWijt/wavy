use crate::playback_duration::PlaybackDuration;
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

    pub fn update(&self, playback_duration: &PlaybackDuration, total_duration: WavDuration, terminal: &mut Terminal) {
        terminal.write(&playback_duration);
        terminal.write(String::from("["));
        terminal.set_cursor_right(self.max_ticks as u16);
        terminal.write(String::from("]"));
        terminal.write(&total_duration);
        terminal.set_cursor_left((self.max_ticks + 6.0) as u16);

        let ticks_per_second: f32 = self.max_ticks / total_duration.raw_seconds;

        let ms_per_tick: u128 = if ticks_per_second < 1.0 {
            (1.0 / ticks_per_second) * 1000.0
        } else {
            ticks_per_second * 1000.0
        } as u128;

        let ticks = playback_duration.milliseconds / ms_per_tick;
        for _ in 0..ticks {
            terminal.write(String::from("#"));
        }
    }
}