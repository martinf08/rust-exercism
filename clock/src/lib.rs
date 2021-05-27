use std::{fmt, fmt::Formatter};

#[derive(Debug, PartialEq)]
pub struct Clock {
    hours: i32,
    minutes: i32,
}

impl fmt::Display for Clock {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f, "{:0>2}:{:0>2}", self.hours, self.minutes)
    }
}

impl Clock {
    pub fn new(hours: i32, minutes: i32) -> Self {
        let mut clock = Clock {
            hours,
            minutes,
        };
        clock.compute();

        clock
    }

    fn compute(&mut self) {
        let mut hours = self.hours;
        if self.hours < 0 {
           hours = 24 + self.hours;
        }

        let total_minutes = hours * 60 + self.minutes;

        hours = total_minutes / 60;
        hours = hours % 24;

        let mut minutes = total_minutes % 60;
        if minutes.is_negative() {
            let sub_hours = minutes / 60 - 1;
            hours += sub_hours;
            minutes = 60 + minutes;
        }

        if hours.is_negative() {
            hours = 24 + hours;
        }

        if hours >= 24 {
           hours = hours % 24;
        }

        self.hours = hours;
        self.minutes = minutes;
    }

    pub fn add_minutes(mut self, minutes: i32) -> Self {
        self.minutes += minutes;
        self.compute();

        self
    }
}
