use std::fmt::{self, Display, Formatter};

#[derive(Debug, Eq, PartialEq)]
pub struct Clock {
    minutes: i32,
}

impl Clock {
    pub fn new(hours: i32, minutes: i32) -> Self {
        let total_minutes = hours * 60 + minutes;
        Clock { minutes: Self::normalize_minutes(total_minutes) }
    }

    pub fn add_minutes(&self, minutes: i32) -> Self {
        Clock::new(0, self.minutes + minutes)
    }

    fn normalize_minutes(minutes: i32) -> i32 {
        let day_minutes = 24 * 60;
        let mut normalized = minutes % day_minutes;
        if normalized < 0 {
            normalized += day_minutes;
        }
        normalized
    }
}

impl Display for Clock {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        let hours = (self.minutes / 60) % 24;
        let minutes = self.minutes % 60;
        write!(f, "{:02}:{:02}", hours, minutes)
    }
}
