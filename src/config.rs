use std::time::Duration;

use abar::{StatusBar, StatusBlock};

use crate::blocks;

/// The number of blocks that can update concurrently. Most people won't need to
/// change this, but bumping it up can cause a noticable difference in the
/// initial load time if you have a lot of blocks. Setting it to 1 will disable
/// concurrency.
pub const NUM_THREADS: u8 = 1;

pub fn bar() -> StatusBar
{
    // All fields are optional; default refresh rate is 1hz
    StatusBar::default()
        .blocks(blocks())
        .refresh_rate(Duration::from_millis(500))
        .delimiter(" | ")
        .left_buffer(" | ")
        .right_buffer(" | ")
        .hide_empty_modules(true)
}

fn blocks() -> Vec<StatusBlock>
{
    use crate::utils::run;

    let ip = StatusBlock::default()
        .name("ip")
        .command(|| run("ip route get 1.2.3.4 | awk '{print $7}'"));

    let mail = StatusBlock::default()
        .name("mail")
        .command(|| run("sb-mailbox"))
        .poll_interval(Duration::from_secs(1));

    let packages = StatusBlock::default()
        .name("packages")
        .command(|| run("sb-pacpackages"))
        .poll_interval(Duration::from_secs(1));

    let keyboard = StatusBlock::default()
        .name("keyboard")
        .command(blocks::keyboard)
        .poll_interval(Duration::from_millis(500));

    let weather = StatusBlock::default()
        .name("weather")
        .command(blocks::weather)
        .poll_interval(Duration::from_secs(60 * 60))
        .update_in_background(true);

    let moon = StatusBlock::default()
        .name("moon")
        .command(blocks::moon)
        .poll_interval(Duration::from_secs(60 * 60));

    let volume = StatusBlock::default()
        .name("volume")
        .command(|| run("sb-volume"))
        .poll_interval(Duration::from_millis(500));

    let power = StatusBlock::default()
        .name("power")
        .command(|| run("sb-battery"))
        .poll_interval(Duration::from_secs(10));

    let internet = StatusBlock::default()
        .name("internet")
        .command(|| run("sb-internet"))
        .poll_interval(Duration::from_secs(5));

    let clock = StatusBlock::default()
        .name("internet")
        .command(blocks::clock)
        .poll_interval(Duration::from_secs(5));

    vec![
        ip, mail, packages, keyboard, weather, moon, volume, power, internet,
        clock,
    ]
}
