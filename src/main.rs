use crossterm::event::read;
use interface::{
    fs::dump_data,
    spotify::{pull_albums, pull_liked_tracks, pull_playlists},
};
use menu::{
    backup, cls, hide_cursor,
    log::{push_print, LogType},
    main, reset_terminal, Mode,
};
use std::{
    process::exit,
    thread::{self},
    time::Duration,
};

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

    let access_token = "BQB5c4NUtZaEfzo04YtrmrU8POkIPwfM6r2rRs234ijLfCs8ZwDlZFT0-aO5zok91nLSsrR1ylv9UoR0VDFmESTgK4JA8kzFscHu9jVxOhvtEIBVLinbWZWOUX2eYp5Qx0mn9EH3KR2o9osHmixC8SdWPCWJGY5N4eL4rloIutIZnYZg5wIRM8vZ7Ovr6ft-tsadKA";

    match backup_mode {
        Mode::Backup => {
            let selection = match backup::menu() {
                Some(val) => val,
                None => return exit_proc(),
            };
            cls();

            let mut logs: Vec<LogType> = vec![];

            if selection[0] {
                push_print(
                    LogType::Status("Pulling liked tracks.".to_owned()),
                    &mut logs,
                );

                match pull_liked_tracks(access_token).await {
                    Ok(val) => match dump_data("./tracks", &val) {
                        Ok(msg) => push_print(LogType::Completed(msg.to_owned()), &mut logs),
                        Err(msg) => push_print(LogType::Error(msg.to_owned()), &mut logs),
                    },
                    Err(e) => push_print(LogType::Error(e.to_owned()), &mut logs),
                };
            }

            if selection[1] {
                push_print(
                    LogType::Status("Pulling liked albums.".to_owned()),
                    &mut logs,
                );
                match pull_albums(access_token).await {
                    Ok(val) => match dump_data("./albums", &val) {
                        Ok(msg) => push_print(LogType::Completed(msg.to_owned()), &mut logs),
                        Err(msg) => push_print(LogType::Error(msg.to_owned()), &mut logs),
                    },
                    Err(e) => push_print(LogType::Error(e.to_owned()), &mut logs),
                }
            }

            if selection[2] {
                push_print(
                    LogType::Status("Pulling list of saved playlists...".to_owned()),
                    &mut logs,
                );

                match pull_playlists(access_token).await {
                    Ok(val) => {
                        push_print(
                            LogType::Completed("Acquired list of saved playlists".to_owned()),
                            &mut logs,
                        );
                        let mut playlists_backup: Vec<ReducedData> = vec![];

                        for metadata in val {
                            push_print(
                                LogType::Status(format!("Pulling {}.", metadata.name).to_owned()),
                                &mut logs,
                            );

                            match pull_playlist_tracks(access_token, &metadata.id).await {
                                Ok(reduced_tracks) => {
                                    push_print(
                                        LogType::Completed(
                                            format!("Pulled {}", metadata.name).to_owned(),
                                        ),
                                        &mut logs,
                                    );

                                    playlists_backup.push(ReducedData::Playlists(ReducedPlaylist {
                                        meta: metadata,
                                        tracks: reduced_tracks,
                                    }))
                                }
                                Err(e) => {
                                    push_print(
                                        LogType::Error(
                                            format!(
                                                "Could not backup playlist {}\n{}",
                                                metadata.name, e
                                            )
                                            .to_owned(),
                                        ),
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
                    Err(msg) => push_print(LogType::Error(msg.to_owned()), &mut logs),
                };
            }
        }

        Mode::Recover => todo!("TODO: Recover menu"),
    }

    thread::sleep(Duration::from_secs(3));
    reset_terminal();
}
