use backup::web_interface::{pull_albums, pull_liked_tracks, pull_playlists};
use crossterm::event::read;
use menu::{backup_menu, cls, hide_cursor, main_menu, reset_terminal, Mode};
use std::{
    process::exit,
    thread::{self},
    time::Duration,
};

use crate::backup::dump_data;

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

    let access_token = "BQA79gDyqMKLhKxigPgH-Vwu7lyw94EcVHGk6q4v4QLJK_OLprOUxOBKHwtLx3jgwumoUAR8odAV_ffY3G4BppYEqi6XpRjgO6cKSGXWc5eG-eZppREL0JGCZ0yRDYUVbSejZ_cczkbxz4x3u3vM8x2eAf8D0dPBihH2JPu5n-Y1zmN97VYOH0puNhPepIXDb-JO1w";

    match backup_mode {
        Mode::Backup => {
            let selection = match backup_menu::menu() {
                Some(val) => val,
                None => return exit_proc(),
            };
            cls();

            if selection[0] {
                match pull_liked_tracks(access_token).await {
                    Ok(val) => match dump_data("./tracks", &val) {
                        Ok(msg) => println!("Liked Tracks: {}", msg),
                        Err(msg) => println!("Liked Tracks: {}", msg),
                    },
                    Err(e) => {
                        println!("Could not backup liked tracks\n{}", e);
                    }
                };
            }

            if selection[1] {
                match pull_albums(access_token).await {
                    Ok(val) => match dump_data("./albums", &val) {
                        Ok(msg) => println!("Albums: {}", msg),
                        Err(msg) => println!("Albums: {}", msg),
                    },
                    Err(e) => {
                        println!("Could not backup albums\n{}", e);
                        return exit_proc();
                    }
                }
            }

            if selection[2] {
                let _playlist_ids = match pull_playlists(access_token).await {
                    Ok(val) => println!("Playlist: {:?}", val),
                    Err(e) => {
                        println!("Could not get playlists\n{}", e);
                    }
                };
            }
        }

        Mode::Recover => todo!("TODO: Recover menu"),
    }

    thread::sleep(Duration::from_secs(3));
    reset_terminal();
}
