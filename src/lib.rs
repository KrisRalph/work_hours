use chrono::{Utc, DateTime, Timelike, Datelike, Weekday};
static WEEKENDS: &'static [Weekday] = &[Weekday::Sat, Weekday::Sun];

pub fn is_work_hours(dt: DateTime::<Utc>) -> bool {
    let cur_hour   = dt.hour();
    let cur_minute = dt.minute();
    if cur_hour == 17 {
        if cur_minute > 30 {
            return false;
        }
    }

    if cur_hour >= 9 && cur_hour <= 17 {
        return true;
    }
    return false;
}

pub fn is_weekday(dt: DateTime::<Utc>) -> bool {
    let weekday = dt.weekday();
    for &x in WEEKENDS {
        if weekday == x {
            return false;
        }
    }
    return true;
}