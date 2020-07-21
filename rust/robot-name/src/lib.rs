use rand::{thread_rng, Rng};

pub struct Robot {
    name: String,
}

impl Robot {
    pub fn new() -> Self {
        Robot {
            name: Robot::gen_name(),
        }
    }

    pub fn name(&self) -> &str {
        &self.name
    }

    pub fn reset_name(&mut self) {
        self.name = Robot::gen_name();
    }

    fn gen_name() -> String {
        let mut name = String::new();
        let mut rng = thread_rng();
        for i in 0..5 {
            match i {
                0..=1 => name.push(rng.gen_range(65, 90) as u8 as char),
                _ => name.push_str(&rng.gen_range(0, 9).to_string()),
            }
        }
        name
    }
}
