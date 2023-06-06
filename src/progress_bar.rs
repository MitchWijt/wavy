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

    pub fn initiate(&self, terminal: &mut Terminal) {
        terminal.write(String::from("["));
        terminal.set_cursor_right(self.max_ticks as u16);
        terminal.write(String::from("]"));

        terminal.set_cursor_left(self.max_ticks as u16);
    }

    pub fn print(&self, current_duration_seconds: u32, mut terminal: &mut Terminal) {
        let total_duration: f32 = 260.0;
        let ticks_per_second: f32 = self.max_ticks / total_duration;

        let seconds_per_tick: u32 = if ticks_per_second < 1.0 {
            (1.0 / ticks_per_second).ceil()
        } else {
            ticks_per_second.ceil()
        } as u32;

        if current_duration_seconds % seconds_per_tick == 0 {
            terminal.write(String::from("#"));
        }
    }
}