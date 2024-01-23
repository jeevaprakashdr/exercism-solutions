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
        todo!("Add {minutes} minutes to existing Clock time");
    }

    fn get_hours_string(&self) -> String {
        let hours = match self.hours >= 10 {
            true => self.hours.to_string(),
            false => format!("0{}", self.hours)
        };
        hours
    }

    fn get_minutes_string(&self) -> String {
        let minutes = match self.minutes >= 10 {
            true => self.minutes.to_string(),
            false => format!("0{}", self.minutes)
        };
        minutes
    }
}

impl fmt::Display for Clock {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let hours = self.get_hours_string();
        let minutes = self.get_minutes_string();

        write!(f, "{}:{}", hours, minutes)
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

impl PartialEq for Clock{
    fn eq(&self, other: &Self) -> bool {
        self.hours == other.hours && self.minutes == other.minutes
    }

    fn ne(&self, other: &Self) -> bool {
        !self.eq(other)
    }
}
