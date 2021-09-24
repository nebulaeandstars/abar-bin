use chrono::prelude::*;

/// Return a string representing the current time.
/// Format: DAY/MONTH/YEAR HOUR:MINUTE(AM|PM)
pub fn clock() -> String {
    let now: DateTime<Local> = Local::now();

    format!(
        "{:#02}/{:#02}/{:#04} {:#02}:{}{}",
        now.day(),
        now.month(),
        now.year(),
        now.hour12().1,
        now.minute(),
        match now.hour12().0 {
            false => "AM",
            true => "PM",
        }
    )
}
