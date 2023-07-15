use std::time::Duration;

use abar::{StatusBar, StatusBarBuilder, StatusBlock, StatusBlockBuilder};

use crate::blocks;

/// The number of blocks that can update concurrently. Most people won't need to
/// change this, but bumping it up can cause a noticable difference in the
/// initial load time if you have a lot of blocks. Setting it to 1 will disable
/// concurrency.
pub const NUM_WORKERS: usize = 2;

pub fn bar() -> StatusBar {
    // All fields are optional; default refresh rate is 1hz
    StatusBarBuilder::default()
        .blocks(blocks())
        .delimiter(" | ")
        .left_buffer(" | ")
        .right_buffer(" | ")
        .hide_empty_modules(true)
        .build()
}

fn blocks() -> Vec<StatusBlock> {
    use abar::utils::run;

    let ip = StatusBlockBuilder::default()
        .name("ip")
        .function(|| run("ip route get 1.2.3.4 | awk '{print $7}'"))
        .update_interval(Duration::from_secs(60 * 60))
        .build();

    let mail = StatusBlockBuilder::default()
        .name("mail")
        .function(|| run("sb-mailbox"))
        .update_interval(Duration::from_secs(1))
        .build();

    let packages = StatusBlockBuilder::default()
        .name("packages")
        .function(|| run("sb-pacpackages"))
        .update_interval(Duration::from_secs(10))
        .build();

    let keyboard = StatusBlockBuilder::default()
        .name("keyboard")
        .function(blocks::keyboard)
        .update_interval(Duration::from_millis(500))
        .build();

    let weather = StatusBlockBuilder::default()
        .name("weather")
        .function(blocks::weather)
        .update_interval(Duration::from_secs(60 * 60))
        .build();

    let moon = StatusBlockBuilder::default()
        .name("moon")
        .function(blocks::moon)
        .update_interval(Duration::from_secs(60 * 60))
        .build();

    let volume = StatusBlockBuilder::default()
        .name("volume")
        .function(|| run("sb-volume"))
        .update_interval(Duration::from_secs(30))
        .build();

    let power = StatusBlockBuilder::default()
        .name("power")
        .function(|| run("sb-battery"))
        .update_interval(Duration::from_secs(5))
        .build();

    let internet = StatusBlockBuilder::default()
        .name("internet")
        .function(|| run("sb-internet"))
        .update_interval(Duration::from_secs(5))
        .build();

    let clock = StatusBlockBuilder::default()
        .name("clock")
        .function(blocks::clock)
        .update_interval(Duration::from_secs(5))
        .build();

    vec![
        ip, mail, packages, keyboard, weather, moon, volume, power, internet, clock,
    ]
}
