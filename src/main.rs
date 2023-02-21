use crossterm::event::read;
use interface::{
    fs::dump_data,
    spotify::{pull_albums, pull_liked_tracks, pull_playlists},
};
use menu::{backup, cls, hide_cursor, main, reset_terminal, Mode};
use std::{
    process::exit,
    thread::{self},
    time::Duration,
};

use crate::interface::{spotify::pull_playlist_tracks, ReducedData, ReducedPlaylist};

mod fs_handler;
pub mod interface;
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

    let backup_mode: Mode = match main::menu() {
        Some(val) => val,
        _ => return exit_proc(),
    };
    cls();

    let access_token = "BQAF9ljGyDG4tDeDcwcpKTwiIyVm8bHdq4hEso1TV5DylcXLMogZEoC9kYBtMvoAXvwUPKCnVcpSStbiAls6_6YTPG47N6_04qThuloMa0gcUwm4Nl8JNg3XCDYJXObr5vpzLs5ldOvc9pXDyi2oCuJ9U65CjHJSk-R-Gu_TseXPdUhKeMmWhQs9xysNNqxBFuwEUA";

    match backup_mode {
        Mode::Backup => {
            let selection = match backup::menu() {
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
                match pull_playlists(access_token).await {
                    Ok(val) => {
                        println!("Pulled playlists!");

                        let mut playlists_backup: Vec<ReducedData> = vec![];

                        for playlist_metadata in val {
                            match pull_playlist_tracks(access_token, &playlist_metadata.id).await {
                                Ok(reduced_tracks) => {
                                    println!("pulled {}", playlist_metadata.name);
                                    playlists_backup.push(ReducedData::Playlists(ReducedPlaylist {
                                        meta: playlist_metadata,
                                        tracks: reduced_tracks,
                                    }))
                                }
                                Err(e) => {
                                    println!(
                                        "Could not backup playlist {}\n{}",
                                        playlist_metadata.name, e
                                    );
                                }
                            }
                        }

                        match dump_data("./playlists", &playlists_backup) {
                            Ok(val) => {
                                println!("good {}", val)
                            }
                            Err(e) => {
                                println!("bad {}", e)
                            }
                        }
                    }
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
