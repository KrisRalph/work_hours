#[cfg(test)]
mod tests {
    use chrono::{Utc, TimeZone};
    use workday_finder::{is_work_hours, is_weekday};

    #[test]
    fn test_8_59() {
        let dt = Utc::now().date().and_hms(8, 59, 0);
        assert_eq!(is_work_hours(dt), false);
    }

    #[test]
    fn test_9_01() {
        let dt = Utc::now().date().and_hms(9, 1, 0);
        assert_eq!(is_work_hours(dt), true);
    }

    #[test]
    fn test_5_29() {
        let dt = Utc::now().date().and_hms(17, 29, 0);
        assert_eq!(is_work_hours(dt), true);
    }

    #[test]
    fn test_5_31() {
        let dt = Utc::now().date().and_hms(17, 31, 0);
        assert_eq!(is_work_hours(dt), false);
    }

    #[test]
    fn test_monday() {
        let dt = Utc.ymd(2020, 12, 7).and_hms(8, 30, 0); // It's a monday
        assert_eq!(is_weekday(dt), true);
    }

    #[test]
    fn test_friday() {
        let dt = Utc.ymd(2020, 12, 11).and_hms(8, 30, 0); // It's a friday
        assert_eq!(is_weekday(dt), true);
    }

    #[test]
    fn test_weekends() {
        let dt = Utc.ymd(2020, 12, 5).and_hms(8, 30, 0); // It's a saturday
        assert_eq!(is_weekday(dt), false);

        let dt = Utc.ymd(2020, 12, 6).and_hms(8, 30, 0); // It's a sunday
        assert_eq!(is_weekday(dt), false);
    }
}