use std::sync::Arc;
use std::time::Duration;

use abar::{StatusBar, StatusBlock};

use crate::blocks;

/// The number of blocks that can update concurrently. Most people won't need to
/// change this, but bumping it up can cause a noticable difference in the
/// initial load time if you have a lot of blocks. Setting it to 1 will disable
/// concurrency.
pub const NUM_THREADS: u8 = 1;

pub fn bar() -> StatusBar {
    // All fields are optional; default refresh rate is 1hz
    StatusBar::new()
        .blocks(blocks())
        .refresh_rate(Duration::from_millis(500))
        .delimiter(" | ")
        .left_buffer(" | ")
        .right_buffer(" | ")
}

fn blocks() -> Vec<StatusBlock> {
    use crate::utils::run;

    let ip = StatusBlock::new()
        .name("ip")
        .command(Arc::new(|| run("ip route get 1.2.3.4 | awk '{print $7}'")));

    let mail = StatusBlock::new()
        .name("mail")
        .command(Arc::new(|| run("sb-mailbox")))
        .poll_interval(Duration::from_secs(1));

    let packages = StatusBlock::new()
        .name("packages")
        .command(Arc::new(|| run("sb-pacpackages")))
        .poll_interval(Duration::from_secs(60 * 30));

    let weather = StatusBlock::new()
        .name("weather")
        .command(Arc::new(blocks::weather))
        .poll_interval(Duration::from_secs(60 * 60))
        .update_in_background(true);

    let moon = StatusBlock::new()
        .name("moon")
        .command(Arc::new(blocks::moon))
        .poll_interval(Duration::from_secs(60 * 60 * 24))
        .update_in_background(true);

    let volume = StatusBlock::new()
        .name("volume")
        .command(Arc::new(|| run("sb-volume")))
        .poll_interval(Duration::from_millis(500));

    let power = StatusBlock::new()
        .name("power")
        .command(Arc::new(|| run("sb-battery")))
        .poll_interval(Duration::from_secs(10));

    let internet = StatusBlock::new()
        .name("internet")
        .command(Arc::new(|| run("sb-internet")))
        .poll_interval(Duration::from_secs(10));

    let clock = StatusBlock::new()
        .name("internet")
        .command(Arc::new(|| run("sb-clock")))
        .poll_interval(Duration::from_secs(15));

    vec![
        ip, mail, packages, weather, moon, volume, power, internet, clock,
    ]
}
