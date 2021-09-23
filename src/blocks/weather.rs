use std::process::Command;

// ╔══════╦══════╦══════╦══════╦══════╗
// ║ e300 ║ e301 ║ e302 ║ e303 ║ e304 ║
// ║     ║     ║     ║     ║     ║
// ╠══════╬══════╬══════╬══════╬══════╣
// ║ e305 ║ e306 ║ e307 ║ e308 ║ e309 ║
// ║     ║     ║     ║     ║     ║
// ╠══════╬══════╬══════╬══════╬══════╣
// ║ e30a ║ e30b ║ e30c ║ e30d ║ e30e ║
// ║     ║     ║     ║     ║     ║
// ╠══════╬══════╬══════╬══════╬══════╣
// ║ e30f ║ e310 ║ e311 ║ e312 ║ e313 ║
// ║     ║     ║     ║     ║     ║
// ╠══════╬══════╬══════╬══════╬══════╣
// ║ e314 ║ e315 ║ e316 ║ e317 ║ e318 ║
// ║     ║     ║     ║     ║     ║
// ╠══════╬══════╬══════╬══════╬══════╣
// ║ e319 ║ e31a ║ e31b ║ e31c ║ e31d ║
// ║     ║     ║     ║     ║     ║
// ╠══════╬══════╬══════╬══════╬══════╣
// ║ e31e ║ e31f ║ e320 ║ e321 ║ e322 ║
// ║     ║     ║     ║     ║     ║
// ╠══════╬══════╬══════╬══════╬══════╣
// ║ e323 ║ e324 ║ e325 ║ e326 ║ e327 ║
// ║     ║     ║     ║     ║     ║
// ╠══════╬══════╬══════╬══════╬══════╣
// ║ e328 ║ e329 ║ e32a ║ e32b ║ e32c ║
// ║     ║     ║     ║     ║     ║
// ╠══════╬══════╬══════╬══════╬══════╣
// ║ e32d ║ e32e ║ e32f ║ e330 ║ e331 ║
// ║     ║     ║     ║     ║     ║
// ╠══════╬══════╬══════╬══════╬══════╣
// ║ e332 ║ e333 ║ e334 ║ e335 ║ e336 ║
// ║     ║     ║     ║     ║     ║
// ╠══════╬══════╬══════╬══════╬══════╣
// ║ e337 ║ e338 ║ e339 ║ e33a ║ e33b ║
// ║     ║     ║     ║     ║     ║
// ╠══════╬══════╬══════╬══════╬══════╣
//
// 懲
// 戴
// 揄
// 摒
// 敖
// 晴
// 朗
// 望
// 杖
// 歹
// 殺
// 流
// 滛
// 滋
// 漢
// 瀞
// 煮
// 瞧

pub fn weather() -> String {
    match get_wttr_report(&location(), "j1") {
        Ok(report) => format!("{temp}", temp = get_temp_string(&report),),
        Err(_) => String::new(),
    }
}

/// If currently closer to the minimum temperature, return <temp>°. Otherwise,
/// return <temp>°.
fn get_temp_string(report: &serde_json::Value) -> String {
    let weather = &report["weather"];
    let current_condition = &report["current_condition"];

    let max = parse_json_isize(&weather[0]["maxtempC"]);
    let min = parse_json_isize(&weather[0]["mintempC"]);
    let temp = parse_json_isize(&current_condition[0]["temp_C"]);

    if (temp - min).abs() < (temp - max).abs() {
        format!("{}°", temp)
    } else {
        format!("{}°", temp)
    }
}

/// Returns the user's current location.
fn location() -> String {
    "Canberra".to_string() // TODO: Make this abstract
}

/// Get the current weather report from wttr.in in a given format
fn get_wttr_report(
    location: &str,
    format: &str,
) -> Result<serde_json::Value, serde_json::Error> {
    let weather = Command::new("curl")
        .arg("-sf")
        .arg(format!("wttr.in/{}?format={}", location, format))
        .output()
        .expect("Could not get weather")
        .stdout;
    let weather = String::from_utf8(weather).unwrap().trim().to_string();

    serde_json::from_str(&weather)
}

/// Takes a serde_json String and converts it into an int
fn parse_json_isize(value: &serde_json::Value) -> isize {
    value
        .as_str()
        .expect("could not parse value as str")
        .parse::<isize>()
        .expect("could not parse value as isize")
}
