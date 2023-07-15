use chrono::prelude::*;

const PHASE_ICONS: [char; 28] = [
    '', '', '', '', '', '', '', '', '', '', '', '', '', '', '',
    '', '', '', '', '', '', '', '', '', '', '', '', '',
];
const LUNAR_CYCLE: f64 = 29.53058770576;
const LUNAR_CYCLE_IN_SECONDS: f64 = LUNAR_CYCLE * 24.0 * 60.0 * 60.0;

pub fn moon() -> String {
    let lunar_day: f64 = get_lunar_day();
    let phase_index: usize =
        (lunar_day % PHASE_ICONS.len() as f64).round() as usize;

    format!("{}", PHASE_ICONS[phase_index])
}

fn get_lunar_day() -> f64 {
    let fixed_new_moon: DateTime<Utc> =
        Utc.with_ymd_and_hms(2020, 1, 24, 9, 44, 00).unwrap();
    let now: DateTime<Utc> = Utc::now();

    let diff = (now - fixed_new_moon).num_seconds() as f64;

    let lunar_second = diff % LUNAR_CYCLE_IN_SECONDS;

    lunar_second / 60.0 / 60.0 / 24.0
}
