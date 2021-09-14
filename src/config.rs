use std::time::Duration;

use abar::{StatusBar, StatusBlock};

use crate::blocks;

pub fn bar() -> StatusBar {
    // All fields are optional; default refresh rate is 1hz
    StatusBar::new()
        .blocks(blocks())
        .refresh_rate(Duration::from_millis(500))
        .delimiter(" | ")
        .left_buffer(" >>> ")
        .right_buffer(" <<< ")
}

fn blocks() -> Vec<StatusBlock> {
    use crate::utils::run;

    let ip = StatusBlock::new()
        .name("ip")
        .command(&|| run("ip route get 1.2.3.4 | awk '{print $7}'"));

    let mail = StatusBlock::new()
        .name("mail")
        .command(&|| run("sb-mailbox"))
        .poll_interval(Duration::from_secs(1));

    let packages = StatusBlock::new()
        .name("packages")
        .command(&|| run("sb-pacpackages"))
        .poll_interval(Duration::from_secs(60 * 30));

    let weather = StatusBlock::new()
        .name("weather")
        .command(&|| blocks::weather())
        .poll_interval(Duration::from_secs(60 * 60));

    let moon = StatusBlock::new()
        .name("moon")
        .command(&|| run("sb-moonphase"))
        .poll_interval(Duration::from_secs(60 * 60 * 24));

    let volume = StatusBlock::new()
        .name("volume")
        .command(&|| run("sb-volume"))
        .poll_interval(Duration::from_millis(500));

    let power = StatusBlock::new()
        .name("power")
        .command(&|| run("sb-battery"))
        .poll_interval(Duration::from_secs(10));

    let internet = StatusBlock::new()
        .name("internet")
        .command(&|| run("sb-internet"))
        .poll_interval(Duration::from_secs(10));

    let clock = StatusBlock::new()
        .name("internet")
        .command(&|| run("sb-clock"))
        .poll_interval(Duration::from_secs(15));

    vec![
        ip, mail, packages, weather, moon, volume, power, internet, clock,
    ]
}
