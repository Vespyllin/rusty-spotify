use crossterm::{
    event::read,
    terminal::{disable_raw_mode, enable_raw_mode},
};
use interface::{
    fs::dump_data,
    spotify::{pull_albums, pull_liked_tracks, pull_playlists},
};
use menu::{
    backup, cls, hide_cursor,
    log::{push_print, LogType},
    main, reset_terminal, Mode,
};
use std::process::exit;

// use uuid::Uuid;

use crate::interface::{spotify::pull_playlist_tracks, ReducedData, ReducedPlaylist};

mod interface;
mod menu;
mod spotify_client;

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

    // let uuid = Uuid::new_v4();

    let access_token = "BQC_iVK_lBTZYiKW6TB-A3yZPq7mLmxm8AEqZ6_GKjQorrLFi8JbfZgxGkTQzYJXLiiewlEmfdr_NbPVTzK6obWk9WZp8HzZKqt3m7ZDLBM8G8Iib4RetuRVFWAVQ7suWCGaMPhaiPgnv08FhZp9ATg1mcp3zX9gjrvt30VAPphqmXfKnw49Fqimv5CVA00PWFD5bw";

    match backup_mode {
        Mode::Backup => {
            let selection = match backup::menu() {
                Some(val) => val,
                None => return exit_proc(),
            };
            cls();

            let mut logs: Vec<LogType> = vec![];

            if selection[0] {
                push_print(LogType::Status("Pulling liked tracks".to_owned()), &mut logs);
                match pull_liked_tracks(access_token).await {
                    Ok(val) => match dump_data("./tracks", &val) {
                        Ok(msg) => push_print(LogType::Completed(msg.to_owned()), &mut logs),
                        Err(msg) => push_print(LogType::Error(msg.to_owned()), &mut logs),
                    },

                    Err(e) => push_print(LogType::Error(e.to_string()), &mut logs),
                };
            }

            if selection[1] {
                push_print(LogType::Status("Pulling liked albums".to_owned()), &mut logs);
                match pull_albums(access_token).await {
                    Ok(val) => match dump_data("./albums", &val) {
                        Ok(msg) => push_print(LogType::Completed(msg.to_owned()), &mut logs),
                        Err(msg) => push_print(LogType::Error(msg.to_owned()), &mut logs),
                    },

                    Err(e) => push_print(LogType::Error(e.to_string()), &mut logs),
                }
            }

            if selection[2] {
                push_print(LogType::Status("Pulling playlists".to_owned()), &mut logs);
                match pull_playlists(access_token).await {
                    Ok(val) => {
                        push_print(LogType::Completed("Acquired list of saved playlists".to_owned()), &mut logs);
                        let mut playlists_backup: Vec<ReducedData> = vec![];

                        for metadata in val {
                            push_print(LogType::Status(format!("Pulling {}.", metadata.name).to_owned()), &mut logs);

                            match pull_playlist_tracks(access_token, &metadata.id).await {
                                Ok(reduced_tracks) => {
                                    push_print(LogType::Completed(format!("Pulled {}", metadata.name).to_owned()), &mut logs);

                                    playlists_backup.push(ReducedData::Playlists(ReducedPlaylist {
                                        meta: metadata,
                                        tracks: reduced_tracks,
                                    }))
                                }

                                Err(_) => {
                                    push_print(
                                        LogType::Error(format!("Could not backup playlist {}", metadata.name).to_owned()),
                                        &mut logs,
                                    );
                                }
                            }
                        }

                        match dump_data("./playlists", &playlists_backup) {
                            Ok(msg) => push_print(LogType::Completed(msg.to_owned()), &mut logs),
                            Err(msg) => push_print(LogType::Error(msg.to_owned()), &mut logs),
                        }
                    }

                    Err(msg) => push_print(LogType::Error(msg.to_string()), &mut logs),
                };
            }
        }

        Mode::Recover => todo!("TODO: Recover menu"),
    }

    _ = enable_raw_mode();
    _ = read();
    _ = disable_raw_mode();
    exit_proc();
}
