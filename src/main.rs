use crossterm::event::read;
use fs_handler::{dump_backup, get_latest_json};
use sha2::{Digest, Sha256};
use std::process::exit;

use backup::pull_albums_data;
use menu::{cls, hide_cursor, reset_cursor, reset_terminal, Choice};

pub mod backup;
mod fs_handler;
pub mod menu;
pub mod spotify_client;

enum Mode {
    Backup,
    Recover,
}

fn exit_proc() {
    cls();
    reset_terminal();
    exit(1)
}

fn main_menu() -> Mode {
    loop {
        menu::main_menu::print_menu();

        match menu::main_menu::handle_input() {
            Choice::Choice(x) => {
                match x {
                    1 => return Mode::Backup,
                    2 => return Mode::Recover,
                    _ => panic!("Bad Value received from main menu."),
                };
            }

            Choice::Exit => exit_proc(),

            Choice::Default => reset_cursor(),

            _ => todo!("Add submit to main menu"),
        }
    }
}

fn backup_menu() -> Vec<bool> {
    let mut selection: Vec<bool> = vec![false, false, false];

    loop {
        menu::backup_menu::print_menu(&mut selection);

        match menu::backup_menu::handle_input() {
            Choice::Exit => exit_proc(),
            Choice::Submit => {
                // If at least 1 option is selected, allow the user to continue
                if selection.iter().fold(false, |acc, v| acc || *v) {
                    break;
                }
            }
            Choice::Default => {}
            Choice::Choice(x) => selection[x - 1] = !selection[x - 1],
        }

        reset_cursor();
    }

    selection
}

async fn handle_album_backup(access_token: &str, album_path: &str) -> Result<String, String> {
    let album_data = pull_albums_data(access_token).await?;

    let old_hash = match get_latest_json(album_path)? {
        Some(val) => Some(format!(
            "{:x}",
            Sha256::digest(
                val.chars()
                    .filter(|c| !c.is_whitespace())
                    .collect::<String>()
                    .as_bytes()
            )
        )),
        None => None,
    };

    let album_json = match serde_json::to_string(&album_data) {
        Ok(val) => val,
        Err(_) => return Err("Could not serialize album backup data".to_string()),
    };

    let new_hash = format!(
        "{:x}",
        Sha256::digest(
            album_json
                .chars()
                .filter(|c| !c.is_whitespace())
                .collect::<String>()
                .as_bytes()
        )
    );

    if old_hash.is_some() && old_hash.clone().unwrap() == new_hash {
        println!("hashes\nn:{}\no:{}", new_hash, old_hash.unwrap());
        return Ok("Latest album backup is up to date.".to_string());
    }
    println!("hashes\nn:{}\no:{}", new_hash, old_hash.unwrap());

    let title = format!("album_{}.json", &new_hash[..10]);

    dump_backup("./albums/", &title, &album_json)?;
    Ok("Successfully updated albums.".to_string())
}

#[tokio::main]
async fn main() {
    hide_cursor();
    cls();

    // Discard resize event on Windows
    if cfg!(windows) {
        _ = read().unwrap();
    }

    let backup_mode: Mode = main_menu();
    cls();

    let access_token = "";
    match backup_mode {
        Mode::Backup => {
            let selection = backup_menu();
            cls();

            // if selection[0] {}

            // if selection[1] {}

            if selection[2] {
                handle_album_backup(access_token, "./albums/")
                    .await
                    .unwrap();
            }
        }

        Mode::Recover => todo!("TODO: Recover menu"),
    }

    reset_terminal();
}
