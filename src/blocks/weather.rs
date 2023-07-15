use std::process::Command;

// ║     ║     ║     ║     ║     ║
// ║     ║     ║     ║     ║     ║
// ║     ║     ║     ║     ║     ║
// ║     ║     ║     ║     ║     ║
// ║     ║     ║     ║     ║     ║
// ║     ║     ║     ║     ║     ║
// ║     ║     ║     ║     ║     ║
// ║     ║     ║     ║     ║     ║
// ║     ║     ║     ║     ║     ║
// ║     ║     ║     ║     ║     ║
// ║     ║     ║     ║     ║     ║
// ║     ║     ║     ║     ║     ║
//
// 懲 戴 揄 摒 敖 晴 朗 望 杖 歹 殺 流 滛 滋 漢 瀞 煮 瞧

pub fn weather() -> String {
    match get_wttr_report(&location(), "j1") {
        Ok(report) => get_temp_string(&report).to_string(),
        Err(_) => String::new(),
    }
}

/// If currently closer to the minimum temperature, return <temp>°. Otherwise,
/// return <temp>°.
fn get_temp_string(report: &serde_json::Value) -> String {
    let weather = &report["weather"];
    // let current_condition = &report["current_condition"];

    let max = parse_json_isize(&weather[0]["maxtempC"]);
    let min = parse_json_isize(&weather[0]["mintempC"]);
    // let temp = parse_json_isize(&current_condition[0]["temp_C"]);

    // if (temp - min).abs() < (temp - max).abs() {
    //     format!("{}°", temp)
    // } else {
    //     format!("{}°", temp)
    // }

    // temporary, as the actual functionality is a bit more complicated than
    // anticipated
    format!("{}{}", min, max)
}

/// Returns the user's current location.
fn location() -> String {
    // "Canberra".to_string()
    String::from("")
}

/// Get the current weather report from wttr.in in a given format
fn get_wttr_report(
    _location: &str,
    format: &str,
) -> Result<serde_json::Value, serde_json::Error> {
    let weather = Command::new("curl")
        .arg("-sf")
        // .arg(format!("wttr.in/{}?format={}", location, format))
        .arg(format!("wttr.in/?format={}", format))
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
