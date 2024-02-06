use chrono::{Duration, Local, NaiveDateTime};

pub struct TimeConverter;

impl TimeConverter {
    pub fn get_relative_time_since_now(time: NaiveDateTime) -> String {
        let now: NaiveDateTime = Local::now().naive_local();

        let duration: Duration = time.signed_duration_since(now);
        let num_seconds: i64 = duration.num_seconds();

        if num_seconds < 0 {
            let num_seconds = num_seconds.abs();
            return match num_seconds {
                0..=59 => format!("{} seconds ago", num_seconds),
                60..=3599 => format!("{} minutes ago", num_seconds / 60),
                3600..=86_399 => format!("{} hours ago", num_seconds / 3600),
                _ => format!("{} days ago", num_seconds / 86_400),
            };
        } else {
            return match num_seconds {
                0..=59 => format!("{} seconds left", num_seconds),
                60..=3599 => format!("{} minutes left", num_seconds / 60),
                3600..=86_399 => format!("{} hours left", num_seconds / 3600),
                _ => format!("{} days left", num_seconds / 86_400),
            };
        }
    }
}
