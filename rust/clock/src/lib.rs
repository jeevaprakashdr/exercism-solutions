use std::fmt::{self};

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
        let clock = Self::calculate_hours_and_minutes(self.hours, self.minutes + minutes);
        Clock {
            hours: clock.0,
            minutes: clock.1,
        }
    }

    fn get_hours_and_minutes_string(&self) -> (String, String) {
        let clock = Self::calculate_hours_and_minutes(self.hours, self.minutes);
        let hour_str = match clock.0 {
            1..=9 => format!("0{}", clock.0),
            10..=23 => clock.0.to_string(),
            _ => "00".to_owned(),
        };

        let minute_str = match clock.1 {
            1..=9 => format!("0{}", clock.1),
            10..=59 => clock.1.to_string(),
            _ => "00".to_owned(),
        };

        (hour_str, minute_str)
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
        let tup = self.get_hours_and_minutes_string();

        write!(f, "{}:{}", tup.0, tup.1)
    }
}

impl fmt::Debug for Clock {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("Clock")
            .field("hours", &self.hours)
            .field("minutes", &self.minutes)
            .finish()
    }
}

impl PartialEq for Clock {
    fn eq(&self, other: &Self) -> bool {
        self.hours == other.hours && self.minutes == other.minutes
    }

    fn ne(&self, other: &Self) -> bool {
        !self.eq(other)
    }
}
