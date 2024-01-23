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

    fn get_hours_and_minutes(&self) -> (String, String) {
        let mut hours = self.hours;
        let mut minutes= self.minutes ;

        if minutes < 0 {
            minutes = self.minutes.rem_euclid(60);            
            hours -= 1;
        } else if minutes >= 60 {
            let val: f32 = self.minutes as f32/ 60.0;
            hours += val.trunc() as i32;
            minutes =  self.minutes.rem_euclid(60)
        }
        println!("{}", minutes);
        if hours < 0 {
            hours = 24 + (hours % 24); 
        } else if hours > 24 {
            hours = hours % 24
        }
        
        let hour_str = match hours {
            0..=9 => format!("0{}", hours),
            10..=23 => hours.to_string(),
            24 => "00".to_owned(),
            _ => panic!("Invalid hours"),
        };
        
        let minute_str = match minutes {
            1..=9 => format!("0{}", minutes),
            10..=58 => minutes.to_string(),
            0 | 59 => "00".to_owned(),
            60 => "00".to_owned(),
            _ => panic!("Invalid minutes"),
        };

        (hour_str, minute_str)
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
