use backup::{
    dump_data,
    web_interface::{dummy_pull, pull_albums_data},
};
use crossterm::event::read;
use spotify_client::ReducedAlbumData;
use std::{future::Future, pin::Pin, process::exit, thread, time::Duration};

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

    let access_token = "BQDP_chcndqQHDKm_74ak7MpwzB2dUIMWdZZU77Ola3ZrYLGR8BmSswe6p8d0w4lXWXTuqcrYhk5coqfYQTGszaa7dVMkZIQ_XhxrHtn1ow7JslvSr1Bbq0fpSG73VmMCFSr_d__j1QLkugi8g9-S-BrjmxuoWokvNn6o5ubx_Vv4YYv3gjS7ughJmgBDrxuXBlnng";

    match backup_mode {
        Mode::Backup => {
            let selection = match backup_menu::menu() {
                Some(val) => val,
                None => return exit_proc(),
            };
            cls();

            let pull_tracks_req: Pin<
                Box<dyn Future<Output = Result<Vec<ReducedAlbumData>, String>>>,
            > = if selection[0] {
                Box::pin(pull_albums_data(access_token))
            } else {
                Box::pin(dummy_pull::<ReducedAlbumData>())
            };

            let pull_albums_req: Pin<
                Box<dyn Future<Output = Result<Vec<ReducedAlbumData>, String>>>,
            > = if selection[1] {
                Box::pin(pull_albums_data(access_token))
            } else {
                Box::pin(dummy_pull::<ReducedAlbumData>())
            };

            let (_tracks_res, albums_res) = tokio::join!(pull_tracks_req, pull_albums_req);

            if selection[1] {
                match dump_data("./albums", &albums_res.unwrap()) {
                    Ok(e) => println!("{}", e),
                    Err(e) => println!("{}", e),
                }
            }
        }

        Mode::Recover => todo!("TODO: Recover menu"),
    }

    thread::sleep(Duration::from_secs(3));
    reset_terminal();
}
