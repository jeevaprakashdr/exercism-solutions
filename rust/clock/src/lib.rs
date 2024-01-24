use std::fmt::{self};

#[derive(Debug, PartialEq)]
pub struct Clock {
    hours: i32,
    minutes: i32,
}

impl Clock {
    pub fn new(hours: i32, minutes: i32) -> Self {
        let clock = Self::calculate_hours_and_minutes(hours, minutes);
        Clock {
            hours: clock.0,
            minutes: clock.1,
        }
    }

    pub fn add_minutes(&self, minutes: i32) -> Self {
        Clock::new(self.hours, self.minutes + minutes)
    }   

    fn calculate_hours_and_minutes(h: i32, m: i32) -> (i32, i32) {
        let mut hours = h;
        let mut minutes = m;

        if minutes < 0 {
            minutes = m.rem_euclid(60);
            hours += m.div_euclid(60);
        } else if minutes >= 60 {
            let minutes_div: f32 = m as f32 / 60 as f32;
            hours += minutes_div.trunc() as i32;
            minutes = m.rem_euclid(60)
        }

        if hours < 0 {
            hours = 24 + (hours % 24);
        } else if hours > 24 {
            hours = hours % 24
        }

        hours = match hours {
            0..=23 => hours,
            24 => 0,
            _ => panic!("Invalid hours"),
        };

        minutes = match minutes {
            1..=9 => minutes,
            10..=59 => minutes,
            0 | 60 => 0,
            _ => panic!("Invalid minutes"),
        };

        (hours, minutes)
    }
}

impl fmt::Display for Clock {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{:02}:{:02}", self.hours, self.minutes)
    }
}