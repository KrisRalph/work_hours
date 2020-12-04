use workday_finder::{is_weekday, is_work_hours};
use chrono::Utc;
use std::process::Command;

fn main() {
    let dt = Utc::now();
    if is_weekday(dt) && is_work_hours(dt) {
        let _ = Command::new("firefox").arg("-P Work").spawn();
    } else {
        let _ = Command::new("firefox").spawn();
    }
    0;
}