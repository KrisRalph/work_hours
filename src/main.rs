use work_hours::{is_weekday, is_work_hours};
use chrono::Local;
use std::process::Command;

fn main() {
    let dt = Local::now();
    // println!("is_weekday: {}", is_weekday(dt));
    // println!("is_work_hours: {}", is_work_hours(dt));
    if is_weekday(dt) && is_work_hours(dt) {
        let _ = Command::new("firefox").args(&["-p", "Work"]).spawn();
    } else {
        let _ = Command::new("firefox").spawn();
    }
    0;
}
