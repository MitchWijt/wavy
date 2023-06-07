use std::fmt::{Display, Formatter};

pub struct PlaybackDuration {
    pub raw_seconds: u32,
    pub seconds: u32,
    pub minutes: u32,
}

impl PlaybackDuration {
    pub fn new() -> Self {
        PlaybackDuration {
            raw_seconds: 0,
            seconds: 0,
            minutes: 0,
        }
    }

    pub fn advance(&mut self) {
        self.raw_seconds += 1;
        self.seconds += 1;
        if self.seconds == 60 {
            self.minutes += 1;
            self.seconds = 0;
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