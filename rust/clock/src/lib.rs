use std::fmt;

#[derive(Debug)]
pub struct Clock {
    total_mins: i32,
}

impl Clock {
    pub fn new(hours: i32, minutes: i32) -> Self {
        Clock {
            total_mins: Clock::adjust_time(hours * 60 + minutes),
        }
    }

    pub fn add_minutes(&self, minutes: i32) -> Self {
        Clock {
            total_mins: Clock::adjust_time(self.total_mins + minutes),
        }
    }

    fn adjust_time(time: i32) -> i32 {
        (time % 1440 + 1440) % 1440
    }

    fn str_time(num: i32) -> String {
        if num < 10 {
            format!("0{}", num)
        } else {
            num.to_string()
        }
    }
}

impl PartialEq for Clock {
    fn eq(&self, other: &Self) -> bool {
        self.total_mins == other.total_mins
    }
}

impl fmt::Display for Clock {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "{}:{}",
            Clock::str_time(self.total_mins / 60),
            Clock::str_time(self.total_mins % 60),
        )
    }
}
