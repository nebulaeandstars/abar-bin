use std::collections::HashMap;
use std::process::Command;

type WeatherReport = HashMap<String, f64>;

pub fn weather() -> String {
    let report = report(location());
    let weather: WeatherReport = parse_prometheus(report);

    let min = weather
        .get("temperature_celsius_minimum{forecast=\"0d\"}")
        .unwrap();

    let max = weather
        .get("temperature_celsius_maximum{forecast=\"0d\"}")
        .unwrap();

    let precipitation = weather
        .get("precipitation_mm{forecast=\"current\"}")
        .unwrap();

    format!("🌧{}% ❄️{}° 🔥{}°", min, max, precipitation)
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
