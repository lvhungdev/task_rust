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
                0..=59 => format!("-{}s", num_seconds),
                60..=3599 => format!("-{}m {}s", num_seconds / 60, num_seconds % 60),
                3600..=86_399 => format!("-{}h {}m", num_seconds / 3600, (num_seconds % 3600) / 60),
                _ => format!(
                    "-{}d {}h",
                    num_seconds / 86_400,
                    (num_seconds % 86_400) / 3600
                ),
            };
        } else {
            return match num_seconds {
                0..=59 => format!("{}s", num_seconds),
                60..=3599 => format!("{}m {}s", num_seconds / 60, num_seconds % 60),
                3600..=86_399 => format!("{}h {}m", num_seconds / 3600, (num_seconds % 3600) / 60),
                _ => format!(
                    "{}d {}h",
                    num_seconds / 86_400,
                    (num_seconds % 86_400) / 3600
                ),
            };
        }
    }
}
