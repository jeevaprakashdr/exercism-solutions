use std::fmt::{self};

pub struct Clock {
    hours: i32,
    minutes: i32,
}

impl Clock {
    pub fn new(hours: i32, minutes: i32) -> Self {
        Clock { hours, minutes }
    }

    pub fn add_minutes(&self, minutes: i32) -> Self {
        let clock = Self::calculate_hours_and_minutes(self.hours, self.minutes + minutes);
        Clock {
            hours: clock.0,
            minutes: clock.1,
        }
    }

    fn get_hours_and_minutes(&self) -> (String, String) {
        let clock = Self::calculate_hours_and_minutes(self.hours, self.minutes);
        let hours = clock.0;
        let minutes = clock.1;
        
        let hour_str = match hours {
            0..=9 => format!("0{}", hours),
            10..=23 => hours.to_string(),
            24 => "00".to_owned(),
            _ => panic!("Invalid hours"),
        };

        let minute_str = match minutes {
            1..=9 => format!("0{}", minutes),
            10..=59 => minutes.to_string(),
            0 | 60 => "00".to_owned(),
            _ => panic!("Invalid minutes"),
        };

        (hour_str, minute_str)
    }

    fn calculate_hours_and_minutes(h: i32, m:i32 ) -> (i32, i32) {
        let mut hours = h;
        let mut minutes = m;
        
        if minutes < 0 {
            minutes = m.rem_euclid(60);
            hours += m.div_euclid(60);
        } else if minutes >= 60 {
            let val: f32 = m as f32 / 60.0;
            hours += val.trunc() as i32;
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
        let tup = self.get_hours_and_minutes();

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
