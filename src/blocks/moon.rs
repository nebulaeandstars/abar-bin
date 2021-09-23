use chrono::prelude::*;

pub fn moon() -> String {
    const PHASE_ICONS: [char; 28] = [
        '', '', '', '', '', '', '', '', '', '', '', '', '', '',
        '', '', '', '', '', '', '', '', '', '', '', '', '', '',
    ];

    let lunar_day: f64 = get_lunar_day();
    let phase_index: usize = (lunar_day % PHASE_ICONS.len() as f64) as usize;

    format!("{}", PHASE_ICONS[phase_index])
}

fn get_lunar_day() -> f64 {
    const LUNAR_CYCLE: f64 = 29.53058770576;
    const LUNAR_CYCLE_IN_SECONDS: f64 = LUNAR_CYCLE * 24.0 * 60.0 * 60.0;

    let fixed_new_moon: DateTime<Utc> = Utc.ymd(2020, 1, 24).and_hms(9, 44, 00);
    let now: DateTime<Utc> = Utc::now();

    let diff = (now - fixed_new_moon).num_seconds() as f64;

    let lunar_second = diff % LUNAR_CYCLE_IN_SECONDS;
    let lunar_day = lunar_second / 60.0 / 60.0 / 24.0;

    lunar_day
}
