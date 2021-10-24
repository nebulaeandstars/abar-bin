mod blocks;
mod config;
mod utils;

use std::env;
use std::io::{Read, Write};
use std::net::{TcpListener, TcpStream};

fn listen()
{
    // start listening for external commands
    let listener = TcpListener::bind("127.0.0.1:2227").unwrap();

    for stream in listener.incoming() {
        match stream {
            Ok(mut stream) => {
                let mut data = String::new();
                stream.read_to_string(&mut data).unwrap();
                println!("{}", data);
            },
            Err(e) => {
                println!("Error: {}", e);
            },
        }
    }
}

fn main()
{
    let args: Vec<String> = env::args().collect();

    if args.len() == 1 {
        let mut statusbar = config::bar();
        let mut status = statusbar.to_string();

        // spin up the worker threads
        for _ in 1..config::NUM_THREADS {
            statusbar.spawn_worker();
        }

        std::thread::spawn(|| listen());

        loop {
            let new_status = statusbar.to_string();

            if status != new_status {
                std::process::Command::new("xsetroot")
                    .arg("-name")
                    .arg(new_status.as_str())
                    .output()
                    .unwrap();

                status = new_status;
            }

            statusbar.update();
            statusbar.sleep();
        }
    }
    else {
        let block = args.get(1).unwrap();

        let mut stream =
            TcpStream::connect("127.0.0.1:2227").unwrap_or_else(|_| {
                println!(
                    "Could not connect to running abar instance! Are you sure \
                     that it's running?"
                );
                std::process::exit(1);
            });

        stream.write(block.as_bytes()).unwrap();
    }
}
