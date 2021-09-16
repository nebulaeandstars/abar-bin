use std::collections::HashMap;
use std::process::Command;

type WeatherReport = HashMap<String, f64>;

pub fn weather() -> String {
    let report = report(location());
    let weather: WeatherReport = parse_prometheus(report);

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

    let min = weather
        .get("temperature_celsius_minimum{forecast=\"0d\"}")
        .unwrap();

    let max = weather
        .get("temperature_celsius_maximum{forecast=\"0d\"}")
        .unwrap();

    let precipitation = weather
        .get("precipitation_mm{forecast=\"current\"}")
        .unwrap();

    format!(
        "{prec}mm {min}°{max}°",
        prec = precipitation,
        min = min,
        max = max,
    )
}

/// Returns the user's current location.
fn location() -> String {
    let location = Command::new("echo")
        .arg("Canberra") // TODO: Finish making this abstract
        .output()
        .expect("Could not get location")
        .stdout;
    String::from_utf8(location).unwrap().trim().to_string()
}

/// Fetches the current weather from wttr.in in prometheus format.
fn report(location: String) -> String {
    let weather = Command::new("curl")
        .arg("-sf")
        .arg(format!("wttr.in/{}?format=p1", location))
        .output()
        .expect("Could not get weather")
        .stdout;
    String::from_utf8(weather).unwrap().trim().to_string()
}

/// Parses a prometheus string into a HashMap.
fn parse_prometheus(prometheus: String) -> WeatherReport {
    let mut output: WeatherReport = HashMap::new();

    for line in prometheus.lines() {
        // skip comments
        if line.chars().next() == Some('#') {
            continue;
        }

        let line: Vec<&str> = line.split_whitespace().collect();
        let (key, val) = (line[0].to_string(), line[1]);

        if let Ok(val) = val.parse::<f64>() {
            output.insert(key, val);
        } else if let Ok(val) = val.parse::<i32>() {
            output.insert(key, val.into());
        }
    }

    output
}
