use backup::web_interface::{pull_albums_data, pull_liked_tracks_data};
use crossterm::event::read;
use std::{
    process::exit,
    thread::{self},
    time::Duration,
};
use tokio::task::JoinSet;

use menu::{
    backup_menu::{self},
    cls, hide_cursor,
    main_menu::{self},
    reset_terminal, Mode,
};

pub mod backup;
mod fs_handler;
pub mod menu;
pub mod spotify_client;

fn exit_proc() {
    cls();
    reset_terminal();
    exit(1)
}

#[tokio::main]

async fn main() {
    hide_cursor();
    cls();

    // Discard resize event on Windows
    if cfg!(windows) {
        _ = read().expect("failed to read input. main");
    }

    let backup_mode: Mode = match main_menu::menu() {
        Some(val) => val,
        _ => return exit_proc(),
    };
    cls();

    let access_token = "";

    match backup_mode {
        Mode::Backup => {
            let selection = match backup_menu::menu() {
                Some(val) => val,
                None => return exit_proc(),
            };
            cls();

            let mut task_set = JoinSet::new();

            if selection[0] {
                task_set.spawn(pull_liked_tracks_data(access_token));
            }

            if selection[1] {
                task_set.spawn(pull_albums_data(access_token));
            }

            while let Some(x) = task_set.join_next().await {
                let y = x.as_ref().unwrap();
                let z = y.as_ref().unwrap();

                println!("A");
                println!("{:?}", z[0]);
            }
        }

        Mode::Recover => todo!("TODO: Recover menu"),
    }

    thread::sleep(Duration::from_secs(3));
    reset_terminal();
}
