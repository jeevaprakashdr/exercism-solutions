use std::fmt::{self};

#[derive(Debug, PartialEq)]
pub struct Clock {
    hours: i32,
    minutes: i32,
}

impl Clock {
    pub fn new(hours: i32, minutes: i32) -> Self {
        Clock {
            hours: hours,
            minutes: minutes,
        }
        .calculate_hours_and_minutes()
    }

    pub fn add_minutes(&self, minutes: i32) -> Self {
        Clock::new(self.hours, self.minutes + minutes)
    }

    fn calculate_hours_and_minutes(&self) -> Self {
        let hours_in_a_day = 24;
        let minutes_in_a_hour = 60;

        let mut hours = self.hours;
        let mut minutes = self.minutes;

        if minutes < 0 {
            hours += minutes.div_euclid(minutes_in_a_hour);
        } else if minutes >= minutes_in_a_hour {
            hours += minutes / minutes_in_a_hour;
        }

        minutes = minutes.rem_euclid(minutes_in_a_hour);

        if hours < 0 {
            hours = hours_in_a_day + (hours % hours_in_a_day);
        } else if hours >= hours_in_a_day {
            hours %= hours_in_a_day;
        }

        if hours == hours_in_a_day {
            hours = 0
        }

        Clock { hours, minutes }
    }
}

impl fmt::Display for Clock {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{:02}:{:02}", self.hours, self.minutes)
    }
}
