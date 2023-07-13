use std::fmt::{Display, Formatter};

pub struct PlaybackDuration {
    pub milliseconds: u128,
    seconds: u32,
    minutes: u32,
}

impl PlaybackDuration {
    pub fn new() -> Self {
        PlaybackDuration {
            milliseconds: 0,
            seconds: 0,
            minutes: 0,
        }
    }

    pub fn advance(&mut self, ms: u128) {
        self.milliseconds = ms;

        if self.milliseconds < 1000 {
            self.seconds = 0;
            self.minutes = 0;
        }

        if self.milliseconds % 1000 == 0 {
            self.seconds = ((self.milliseconds / 1000) % 60) as u32;
            self.minutes = ((self.milliseconds / 1000) / 60) as u32;
        }
    }
}

impl Display for PlaybackDuration {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let minutes = if self.minutes < 10 {
            format!("0{}", self.minutes)
        } else {
            format!("{}", self.minutes)
        };

        let seconds = if self.seconds < 10 {
            format!("0{}", self.seconds)
        } else {
            format!("{}", self.seconds)
        };

        write!(f, "{}:{}", minutes, seconds)
    }
}