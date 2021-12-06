mod blocks;
mod config;
mod utils;

use abar::threadpool::ThreadPool;

fn main() {
    // TODO: Replace this with mpsc
    let (monitor_tx, monitor_rx) = flume::bounded(100);

    let threadpool = ThreadPool::new(config::NUM_THREADS, monitor_tx);
    let statusbar = config::bar();

    statusbar.attach_threadpool(&threadpool);

    let mut bar = String::new();
    loop {
        let new_bar = statusbar.to_string();

        if bar != new_bar {
            bar = new_bar;
            println!("{}", &bar);

            std::process::Command::new("xsetroot")
                .arg("-name")
                .arg(&bar)
                .output()
                .unwrap();
        }

        if let Ok(()) = monitor_rx.try_recv() {
            continue;
        }
        else if let Some(time) = statusbar.time_until_next_update() {
            let _ = monitor_rx.recv_timeout(time);
        }
        else {
            monitor_rx.recv().unwrap();
        }
    }
}
