use std::{fmt, fmt::Formatter};

#[derive(Debug, PartialEq)]
pub struct Clock {
    hours: i32,
    minutes: i32,
}

impl fmt::Display for Clock {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        let (hours, minutes) = self.compute();
        write!(f, "{:0>2}:{:0>2}", hours, minutes)
    }
}

impl Clock {
    pub fn new(hours: i32, minutes: i32) -> Self {
        Clock {
            hours,
            minutes,
        }
    }

    fn compute(&self) -> (i32, i32) {
        let mut hours = self.hours;
        if self.hours < 0 {
           hours = 24 + self.hours;
        }

        let mut total_minutes = hours * 60 + self.minutes;

        hours = total_minutes / 60;
        hours = hours % 24;

        let minutes = total_minutes % 60;

        (hours, minutes)
    }

    pub fn add_minutes(mut self, minutes: i32) -> Self {
        self.minutes += minutes;
        let (hours, minutes) = self.compute();

        self.hours = hours;
        self.minutes = minutes;

        self
    }
}
