use chrono;
use durationfmt;

pub struct Clock {
    stored_time: chrono::DateTime<chrono::Utc>,
}
impl Clock {
    pub fn new() -> Clock {
        Clock {
            stored_time: chrono::Utc::now(),
        }
    }
    pub fn tick(&mut self) -> &mut Clock {
        self.stored_time = chrono::Utc::now();
        self
    }
    pub fn tock(self) -> String {
        let time_spent: chrono::Duration =
            chrono::Utc::now().signed_duration_since(self.stored_time);
        durationfmt::to_string(time_spent.to_std().unwrap())
    }
}
